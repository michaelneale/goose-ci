use std::{env, fs};
use std::path::Path;

fn read_file_to_string<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    fs::read_to_string(path)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <text> <path>", args[0]);
        std::process::exit(1);
    }

    let text = &args[1];
    let path = &args[2];

    match read_file_to_string(path) {
        Ok(content) => {
            let matching_lines: Vec<&str> = content
                .lines()
                .filter(|line| line.contains(text))
                .collect();
            
            for line in matching_lines {
                println!("{}", line);
            }
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            std::process::exit(1);
        }
    }
}
