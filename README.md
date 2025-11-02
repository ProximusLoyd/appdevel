# LS_Table

This project have commands scripted to list directory contents in a detailed, table format like nushell. It includes implementations for both Rust and Python.

## Rust Version

### Building

To build the Rust project, you need Rust and Cargo. You can install them from [rust-lang.org](https://www.rust-lang.org/tools/install) or using appropriate package managers.

tu buid it from source in rust simply try

```bash
cargo build --release
```

The executable will be at `target/release/ls_table_rs`.

### Usage

To run the precompiled rust program:
download ls_table_rs or built output from source

```bash
./target/release/ls_table_rs
```

## Python Version

### Usage

The Python script `ls_table.py` provides a similar directory listing.

To run the script:

```bash
ls_table.py
```

mention directory path after the command is an option to that of the directory you want to list. If omitted, it lists the current directory though.

You can also use the `-p` flag to display file permissions:

```bash
ls_table.py -p
```
or
```bash
ls_table_rs -p
```

