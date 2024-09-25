#[cfg(test)]
mod tests {
    use std::process::Command;

    const BINARY_NAME: &str = "semantic_search_cli";
    const TEST_TEXT: &str = "test";
    const TEST_FILE_PATH: &str = "./tests/test_file.txt";

    #[test]
    fn test_semantic_search() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg(BINARY_NAME)
            .arg(TEST_TEXT)
            .arg(TEST_FILE_PATH)
            .output()
            .expect("Failed to execute process");

        let output_str = String::from_utf8_lossy(&output.stdout);

        assert!(output.status.success());
        assert!(output_str.contains(TEST_TEXT));
    }
}
