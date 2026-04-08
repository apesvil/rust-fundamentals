use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the file name is provided
    if args.len() < 2 {
        panic!("Please provide a file name as an argument")
    }

    // Try to open the file
    let file = File::open(&args[1]);

    // Handle the result of opening the file
    let file = match file {
        // If the file is opened successfully, return the file handle
        Ok(file) => file,

        // If there is an error opening the file, handle it based on the error kind
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    // Create a buffered reader for the file and read it line by line
    let reader = BufReader::new(file);

    // Iterate over the lines in the file and print them to the console
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}