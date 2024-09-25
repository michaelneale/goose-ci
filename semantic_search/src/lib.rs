use std::fs;
use std::path::Path;
use rayon::prelude::*;
use std::sync::mpsc::channel;
use std::result::Result;
use std::error::Error;
use std::fmt;
use ignore::Walk;

#[derive(Debug, Clone)]
pub struct SemanticSearchError(String);

impl fmt::Display for SemanticSearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SemanticSearchError {}

pub fn semantic_search(query: &str, dir_path: &Path) -> Result<Vec<String>, SemanticSearchError> {
    let mut results = vec![];
    let (tx, rx) = channel();

    let walker = Walk::new(dir_path);

    println!("Searching directory: {:?}", dir_path);
    walker.par_bridge().for_each_with(tx.clone(), |tx, entry| {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if let Ok(content) = fs::read_to_string(path) {
                    println!("Checking file: {:?}", path);
                        if semantic_match(query, &content) {
                        println!("Match found in file: {:?}", path);
                        tx.send(path.to_string_lossy().to_string()).expect("Failed to send result");
                    }
                }
            }
        }
    });

    drop(tx);

    for result in rx {
        results.push(result);
    }

    Ok(results)
}

fn semantic_match(query: &str, content: &str) -> bool {
    // Placeholder for semantic matching logic, always returning true for now
    {
        println!("content:{} vs query:{}", content, query);
    let query_tokens: Vec<&str> = query.split_whitespace().collect();
    let content_tokens: Vec<&str> = content.split_whitespace().collect();
    for qt in &query_tokens {
        if content_tokens.contains(qt) {
            return true;
        }
    }
    false}

}