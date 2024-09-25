use semantic_search::semantic_search;
use std::path::Path;

#[test]
fn test_semantic_search() {
    let query = "synthetic text data";
    let path = Path::new("./test_data");
    let results = semantic_search(query, &path).expect("Failed to perform semantic search");

    println!("Results: {:?}", results);
    assert!(results.contains(&"./test_data/dir1/file1.txt".to_string()));
    assert!(results.contains(&"./test_data/dir1/nested/file2.txt".to_string()));
    assert!(results.contains(&"./test_data/dir2/file3.txt".to_string()));
    assert!(results.contains(&"./test_data/dir2/nested/file4.txt".to_string()));
}