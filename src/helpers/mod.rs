// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::io::{self, Write};

/// Gets package name and version
pub fn get_lib_name() -> String {
    format!("{} v{}", env!("CARGO_PKG_NAME").to_string(), env!("CARGO_PKG_VERSION").to_string())
}

/// Similar to `std::io::Write::write_all`.  With the addition of returning the number of bytes added.
pub fn write_all_count<W: Write + ?Sized>(writer: &mut W, mut buf: &[u8]) -> io::Result<usize> {
    let mut count = 0;
    while !buf.is_empty() {
        match writer.write(buf) {
            Ok(0) => return Err(io::Error::new(io::ErrorKind::WriteZero, "failed to write whole buffer")),
            Ok(n) => {
                count += n;
                buf = &buf[n..];
            },
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {},
            Err(e) => return Err(e),
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use super::*;

    #[test]
    fn test_get_lib_name() {
        let re = Regex::new(r"^easy-pdf v\d.\d.\d").unwrap();
        assert!(re.is_match(&get_lib_name()));
    }

    #[test]
    fn test_write_all_count() {
        let mut buffer: Vec<u8> = Vec::new();
        assert_eq!(write_all_count(&mut buffer, b"hello world\n").unwrap(), 12);
    }

}
