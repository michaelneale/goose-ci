use std::process::Command;

#[test]
fn test_search_single_word() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-t", "test", "-p", "sample_data"])
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("file1.txt"));
    assert!(stdout.contains("file2.txt"));
}

#[test]
fn test_search_non_existent_word() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-t", "nonexistent", "-p", "sample_data"])
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(!stdout.contains("file1.txt"));
    assert!(!stdout.contains("file2.txt"));
}
