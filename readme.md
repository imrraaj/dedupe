# File Deduplication Tool

This is a Rust-based tool for detecting and removing duplicate files from a given directory and its subdirectories. The tool computes the SHA-256 checksum for each file and deletes any duplicate files based on their hash values, keeping only the most recently modified version.

## How It Works

- The tool recursively traverses a specified directory.
- For each file, it computes the SHA-256 hash using the `sha256sum` command.
- If two or more files have the same checksum (hash), the older file (based on modification time) is deleted.
- The program logs every step to a `dedupe.log` file.

## Features

- **Recursive File Processing**: It searches the directory and subdirectories for all files.
- **SHA-256 Checksum Comparison**: Hashes each file using SHA-256 and checks for duplicates.
- **Duplicate Deletion**: Deletes the older file when duplicates are found.
- **Logging**: Logs every operation (processing, duplicates, deletion) to a file (`dedupe.log`).

## Usage

To run the tool, use the following command:

```bash
cargo run --release <directory-path>
```

Where `<directory-path>` is the path to the folder you want to process for duplicate files.

Example:

```bash
cargo run --release /path/to/folder
```

The tool will:

- Recursively traverse the specified folder.
- Identify and delete duplicate files based on their SHA-256 checksum.
- Log all operations (processing, duplicate detection, and deletion) to `dedupe.log`.

### Example Output (Log)

```rust
Processing: "/path/to/file1"
Processing: "/path/to/file2"
DUPLICATE DETECTED: "/path/to/file1" and "/path/to/file2"
DELETING: "/path/to/file1"
```
