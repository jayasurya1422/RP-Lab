5. Read from a File and Write to Another File

use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Read from input.txt
    let mut input_file = File::open("input.txt")?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    // Modify contents (optional)
    let processed = format!("Processed:\n{}", contents);

    // Write to output.txt
    let mut output_file = File::create("output.txt")?;
    output_file.write_all(processed.as_bytes())?;

    println!("File processed successfully.");
    Ok(())
}
