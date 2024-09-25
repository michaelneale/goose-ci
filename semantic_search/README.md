# Semantic Search CLI in Rust

This project provides a command-line interface (CLI) for performing semantic searches within a specified directory. The CLI tool returns files that have content semantically similar to a given input string.

## Requirements
* macOS or Linux
* Rust and Cargo installed

## Usage

```sh
cargo run -- <query> <path>
```

`<query>`: The text to search for.
`<path>`: The path to the directory to search within.

### Example

```sh
cargo run -- "synthetic text data" ./test_data
```

## Project Structure

- `src/lib.rs`: Core functionality for semantic search.
- `src/main.rs`: CLI implementation.
- `tests/integration_test.rs`: Integration tests.
- `test_data/`: Directory containing synthetic text data for testing.

## Running Tests

```sh
cargo test
```

## License

MIT License. See [LICENSE](./LICENSE) for details.
