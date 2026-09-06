# mini-grep

A simple command-line text search tool written in Rust, inspired by the classic `grep` utility. It searches for a query string inside a file and prints matching lines, with optional case-insensitive search and colored highlighting.

## Features

- Search for a string in a file and print all matching lines
- Case-insensitive search mode
- Highlighted output mode (matching lines are printed in red using ANSI escape codes)
- Simple configuration through command-line arguments and environment variables

## Project Structure

```
src/
  main.rs   -> CLI entry point, argument parsing, and program execution
  lib.rs    -> Core search logic and text styling utilities
```

## How It Works

The program takes two required arguments: a query string and a file path. It reads the file contents and searches each line for the query, printing the lines that match.

Behavior can be adjusted using environment variables:

- `IGNORE_CASE` - if set (to any value), search ignores letter case
- `HIGHLIGHT_CASE` - if set (to any value) and `IGNORE_CASE` is not set, matching lines are printed in red

If neither variable is set, the default case-sensitive search is used.

## Usage

Build the project:

```
cargo build --release
```

Run the program:

```
cargo run -- <query> <file_path>
```

### Examples

Basic search:

```
cargo run -- duct poem.txt
```

Case-insensitive search:

```
IGNORE_CASE=1 cargo run -- rust poem.txt
```

Highlighted search:

```
HIGHLIGHT_CASE=1 cargo run -- rust poem.txt
```

## Core Functions

### `search(query, contents) -> Vec<String>`
Performs a case-sensitive search and returns all lines containing the query.

### `search_case_insensitive(query, contents) -> Vec<String>`
Performs a case-insensitive search by lowercasing both the query and each line before comparison.

### `search_case_highlight(query, contents) -> Vec<String>`
Performs a case-insensitive search and wraps matching lines in ANSI color codes (red) for terminal highlighting. Non-matching lines are returned unchanged.

## Text Styling

The `TextStyle` enum provides ANSI escape code prefixes for styling terminal output, including:

- `BOLD`, `ITALIC`, `UNDERLINE`
- `GRAY`, `RED`, `GREEN`, `YELLOW`, `CYAN`, `VIOLET`, `BLUE`

The `TextStyle::to_string` method wraps a given text in the chosen style's prefix and a reset postfix, producing a styled string ready to print to the terminal.

## Error Handling

- If fewer than the required arguments are provided, the program prints an error and exits with a non-zero status code.
- If the specified file cannot be read, the program prints an application error and exits with a non-zero status code.

## Testing

The project includes unit tests covering:

- Basic case-sensitive search (`one_result`)
- Case-insensitive search (`case_insensitive`)
- Text highlighting output (`case_highlight`)

Run the tests with:

```
cargo test
```

## Requirements

- Rust and Cargo installed (see https://www.rust-lang.org/tools/install)

## License

This project is provided as-is for learning and demonstration purposes.
