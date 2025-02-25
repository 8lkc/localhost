use std::{fs::File, io::{self, BufRead}};

pub struct Localhost {
} impl Localhost {
    pub fn start() {
        match Self::header() {
            Ok(_) => { println!("Hello, from a server!") },
            Err(output) => println!("⚠️ {}", output),
        }
    }

    fn header() -> io::Result<()> {
        let file = File::open("assets/header.txt")?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() { println!("\t{}", line?) }
        Ok(())
    }
}
