use std::fs;
use std::path::Path;
use std::process::Command;
use structopt::StructOpt;

// Define the CLI structure
#[derive(StructOpt, Debug)]
#[structopt(name = "semantic_search_cli")]
struct Cli {
    /// The text to search for
    #[structopt(short = "t", long = "text")]
    text: String,

    /// The path to search within
    #[structopt(short = "p", long = "path")]
    path: String,
}

fn main() {
    let args = Cli::from_args();
    let similar_words = get_similar_words(&args.text);
    let results = search_in_path(&similar_words, &args.path);

    for result in results {
        println!("{}", result);
    }
}

fn get_similar_words(word: &str) -> Vec<String> {
    // Placeholder for a more sophisticated implementation
    vec![word.to_string(), format!("{}_similar", word)]
}

fn search_in_path(words: &[String], path: &str) -> Vec<String> {
    let mut results = Vec::new();
    let entries = fs::read_dir(Path::new(path)).expect("Directory not found");
    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() {
            for word in words {
                let output = Command::new("grep")
                    .arg("-il")
                    .arg(word)
                    .arg(path.to_str().unwrap())
                    .output()
                    .expect("Failed to execute grep");
                if !output.stdout.is_empty() {
                    results.push(path.display().to_string());
                }
            }
        }
    }
    results
}
