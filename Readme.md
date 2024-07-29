# Boyer-Moore String Search in Rust

This project implements the Boyer-Moore string search algorithm in Rust. The Boyer-Moore algorithm is an efficient string searching algorithm that skips sections of text to improve search performance. This implementation reads text files from a directory and searches for multiple patterns specified in a CSV file.

## Features

- Efficient Boyer-Moore string search algorithm.
- Reads and processes multiple text files from a specified directory.
- Searches for multiple patterns provided in a CSV file.
- Displays progress with a progress bar.
- Outputs search results and processing times to the terminal.

## Prerequisites

- Rust programming language (1.60.0 or later).
- `cargo` package manager.

2. **Add dependencies:**

    The project uses the `indicatif` crate for progress bars. The `Cargo.toml` file should include the following dependency:

    ```toml
    [dependencies]
    indicatif = "0.16"
    ```

3. **Build the project:**

    ```bash
    cargo build
    ```

## Usage

1. **Run the program:**

    ```bash
    cargo run
    ```

2. **Provide the required inputs:**

    - **Directory Path:** Enter the path to the directory containing the text files you want to search.
    - **CSV File Path:** Enter the path to the CSV file containing the search patterns. Each line in the CSV file represents a pattern.

    Example:

    ```
    Directory path: /path/to/text/files
    CSV file path: /path/to/patterns.csv
    ```

## How It Works

1. **Read Input Files:**
   - The program reads all text files from the specified directory.
   - Patterns are read from the specified CSV file.

2. **Boyer-Moore Search:**
   - For each file, the Boyer-Moore algorithm is used to search for each pattern.
   - The algorithm uses a bad character heuristic to skip unnecessary comparisons.

3. **Progress Bar:**
   - The `indicatif` crate is used to display a progress bar indicating the search progress.

4. **Output:**
   - The program prints the occurrences of each pattern in the text files.
   - The time taken to process each file is displayed.
   - The total duration for the entire operation is shown at the end.

## Code Overview

### `bad_char_heuristic`
Initializes the bad character heuristic table for the Boyer-Moore algorithm.

### `read_all_bytes`
Reads the content of a file into a byte vector.

### `search`
Performs the Boyer-Moore search on a given file for a specific pattern.

### `main`
Handles user input, initializes the progress bar, performs the searches, and prints the results.
