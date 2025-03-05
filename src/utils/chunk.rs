use std::io::{self, Read, Write};

pub trait Chunking {
    fn chunk(&self, size: usize) -> Vec<Vec<u8>>;
    fn unchunk(&self) -> Vec<u8>;
}

impl Chunking for Vec<u8> {
    fn chunk(&self, size: usize) -> Vec<Vec<u8>> {
        self.chunks(size)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    fn unchunk(&self) -> Vec<u8> {
        self.to_vec()
    }
}

pub fn read_chunks<R: Read>(
    reader: &mut R,
    size: usize,
) -> io::Result<Vec<Vec<u8>>> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer.chunk(size))
}

pub fn write_chunks<W: Write>(
    writer: &mut W,
    chunks: &[Vec<u8>],
) -> io::Result<()> {
    for chunk in chunks {
        writer.write_all(chunk)?;
    }
    Ok(())
}
pub struct ChunkReader<R> {
    reader: R,
    buffer: Vec<u8>,
    position: usize,
}

impl<R: Read> ChunkReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
            position: 0,
        }
    }

    pub fn read_chunk(&mut self) -> io::Result<Option<Vec<u8>>> {
        let mut size_str = String::new();
        
        loop {
            let mut byte = [0; 1];
            if self.reader.read(&mut byte)? == 0 {
                return Ok(None);
            }
            if byte[0] == b'\n' {
                break;
            }
            if byte[0] != b'\r' {
                size_str.push(byte[0] as char);
            }
        }

        let chunk_size = usize::from_str_radix(size_str.trim(), 16)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        if chunk_size == 0 {
            return Ok(None);
        }

        let mut chunk = vec![0; chunk_size];
        self.reader.read_exact(&mut chunk)?;

        let mut crlf = [0; 2];
        self.reader.read_exact(&mut crlf)?;

        Ok(Some(chunk))
    }

    pub fn unchunk(&mut self) -> io::Result<Vec<u8>> {
        let mut complete_data = Vec::new();

        while let Some(chunk) = self.read_chunk()? {
            complete_data.extend(chunk);
        }

        Ok(complete_data)
    }
}

pub fn write_chunked<W: Write>(writer: &mut W, data: &[u8], chunk_size: usize) -> io::Result<()> {
    for chunk in data.chunks(chunk_size) {
        writeln!(writer, "{:X}", chunk.len())?;
        writer.write_all(chunk)?;
        writeln!(writer)?;
    }
    writeln!(writer, "0")?;
    writeln!(writer)?;
    Ok(())
}
