<!--
SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
SPDX-License-Identifier: MIT OR Apache-2.0
-->

[![Rust](https://github.com/wyzzarz/easy-pdf/actions/workflows/rust.yml/badge.svg)](https://github.com/wyzzarz/easy-pdf/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

easy-pdf
========

Easily create PDF documents.

### Example

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

```
$ git clone https://github.com/wyzzarz/easy-pdf.git
$ cd easy-pdf
$ cargo run --example easy-pdf-example
```
