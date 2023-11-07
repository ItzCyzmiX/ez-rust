# ez-rust
ez to use utility functions for rust

---
# Instalation
using cargo:
```bash
cargo add ez-rust
```
or manually by adding `ez-rust = "0.1.6"` to the dependencies in the `Cargo.toml` file

---
# Documentation

## IO
### Input
to get input using the `ez_rust::io` module use the `get_input()` function

Example:
```rust
use ez_rust::io

fn main() {
    let name: String = io::get_input("Whats your name?");   

    println!("Hello {name}!");
}
```
### Colors
to style the output on the terminal we use the `ez_rust::io::Colors` struct that contains easy to use function for styling

Example:
```rust
use ez_rust::io::Colors

fn main() {
    
    println!(
        "Normal {} Danger! {} Blue {} Green",
        Colors::bg_red(),
        Colors::fg_blue(),
        Colors::fg_green()
    );
}
```

Here is a list of all bg colors and fg colors:

| BG  | FG |
| --- | --- |
| bg_red | fg_red | 
| bg_yellow | fg_yellow |
| bg_green |  fg_green | 
| bg_blue | fg_blue |
| bg_magenta | fg_magenta | 
| bg_white | fg_white |
| bg_black | fg_black | 
| bg_reset | fg_reset |
the `bg_reset()` and `fg_reset()` reset the bg and the fg colors to default respectively
## FS
### Read
reading a file using `ez_rust::fs`:
```rust
use ez_rust::fs

fn main() {
    let file: String = fs::read_file("path/to/file.txt");
}
```

### Write
writing to a file using `ez_rust::fs`:
```rust
use ez_rust::fs

fn main() {
    let err: bool = fs::write_file("path/to/file.txt", "hello");
    /* err is a boolean wich will be true if an error occcurd while writing to the file */
}
```
### Create
creating a file using `ez_rust::fs`:
```rust
use ez_rust::fs

fn main() {
    let err: bool = fs::create_file("path/to/file.txt", "hello");
    /* err is a boolean wich will be true if an error occcurd while creating to the file */
}
```
### Create and Write
creating and writing to a file using `ez_rust::fs`:
```rust
use ez_rust::fs

fn main() {
    let err: bool = fs::create_file_write("path/to/file.txt", "hello");
    /* err is a boolean wich will be true if an error occcurd while creating or writing to the file */
}
```