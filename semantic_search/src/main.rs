use std::env;
use std::path::Path;
use semantic_search::semantic_search;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <query> <path>", args[0]);
        std::process::exit(1);
    }

    let query = &args[1];
    let path = Path::new(&args[2]);

    match semantic_search(query, &path) {
        Ok(results) => {
            for result in results {
                println!("{}", result);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}