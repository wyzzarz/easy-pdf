// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

/*!
Easily create PDF documents.

# Setup

Run `cargo add easy-pdf` to add the latest version of the `easy-pdf` crate to your Cargo.toml.

# Example

This example shows how to use the `easy-pdf` crate.

```rust
use easy_pdf::hello_world;

fn main() {
    env_logger::init();
    if let Err(e) = example() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }   
}

fn example() -> Result<(), Box<dyn std::error::Error>> {
    hello_world()
}
```

The above example can be run like so:

```ignore
$ git clone https://github.com/wyzzarz/easy-pdf.git
$ cd easy-pdf
$ cargo run --example easy-pdf-rs-example
```

*/

pub mod helpers;
pub mod resources;

pub fn hello_world() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Hello, world!");
    Ok(())
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert!(hello_world().is_ok());
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
