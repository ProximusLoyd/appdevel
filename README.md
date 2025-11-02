# appdevel

This project provides scripts to list directory contents in a detailed, table-like format. It includes implementations in both Rust and Python.

## Rust Version

### Building

To build the Rust project, you need Rust and Cargo. You can install them from [rust-lang.org](https://www.rust-lang.org/tools/install).

Navigate to the project root and run:

```bash
cargo build --release
```

The executable will be at `target/release/appdevel`.

### Usage

To run the compiled program:

```bash
./target/release/appdevel
```

## Python Version

### Usage

The Python script `ls_table.py` provides a similar directory listing.

To run the script:

```bash
python3 ls_table.py [directory]
```

-   `[directory]` is an optional path to the directory you want to list. If omitted, it lists the current directory.

You can also use the `-p` flag to display file permissions:

```bash
python3 ls_table.py -p [directory]
```
