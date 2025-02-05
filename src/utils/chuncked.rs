pub fn encode_chunked(data: &[u8]) -> Vec<u8> {
    let mut chunked_data = Vec::new();
    for chunk in data.chunks(1024) {
        chunked_data.extend(format!("{:x}\r\n", chunk.len()).as_bytes());
        chunked_data.extend(chunk);
        chunked_data.extend(b"\r\n");
    }
    chunked_data.extend(b"0\r\n\r\n");
    chunked_data
}

pub fn decode_chunked(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut decoded_data = Vec::new();
    let mut chunks = data.split(|&b| b == b'\n');
    while let Some(chunk) = chunks.next() {
        let chunk_size = std::str::from_utf8(chunk)?.trim();
        if chunk_size.is_empty() {
            continue;
        }
        let chunk_size = usize::from_str_radix(chunk_size, 16)?;
        if chunk_size == 0 {
            break;
        }
        let chunk_data = chunks.next().ok_or("Invalid chunked data")?;
        decoded_data.extend(&chunk_data[..chunk_size]);
    }
    Ok(decoded_data)
}