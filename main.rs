use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn write_to_file<P: AsRef<Path>>(path: P, lines: &[&str]) -> io::Result<()> {
    let mut file = File::create(path)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

fn main() {
    let lines = ["Hello", "World", "This is Rust"];
    let file_path = "output.txt";

    match write_to_file(file_path, &lines) {
        Ok(()) => println!("Lines written to file successfully."),
        Err(e) => eprintln!("Failed to write to file: {}", e),
    }
}
