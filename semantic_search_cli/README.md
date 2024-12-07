# Semantic Search CLI

Semantic Search CLI is a command-line utility written in Rust that allows users to search for specific text within files in a directory. It is designed to work on macOS as well as Linux.

## Usage

To use the CLI, provide the text to search for and the path to the file:

```sh
cargo run --bin semantic_search_cli "<text>" "<path>"
```

Example:

```sh
cargo run --bin semantic_search_cli "test" "./tests/test_file.txt"
```

This will search for the word "test" in the file `test_file.txt` located in the `tests` directory and print the matching lines.

## Development

To build the CLI from source, you'll need to have Rust installed. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/).

To build the project, run:

```sh
cargo build
```

To run tests, run:

```sh
cargo test
```

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
