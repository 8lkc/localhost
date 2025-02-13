// use server::Localhost;

mod server;

fn main() {
    // let localhost = Localhost::new();
    // localhost.start();
}

// use std::fs::File;
// use std::io::{self, BufRead, BufReader};

// fn main() -> io::Result<()> {
    
//     // Open the file in read-only mode.
//     let file = File::open("assets/header.txt")?;
    
//     // Wrap the file in a BufReader to handle buffering.
//     let reader = BufReader::new(file);

//     // Iterate over each line in the file.
//     for line_result in reader.lines() {
//         // Each line is returned as a Result<String, std::io::Error>.
//         let line = line_result?;
//         println!("   {}", line);
//     }
    
//     Ok(())
// }
