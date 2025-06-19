// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::io::{self, Write};

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
    use super::*;

    #[test]
    fn test_write_all_count() {
        let mut buffer: Vec<u8> = Vec::new();
        assert_eq!(write_all_count(&mut buffer, b"hello world\n").unwrap(), 12);
    }

}
