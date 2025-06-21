// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use rust_decimal::{Decimal, RoundingStrategy};
use std::io::{self, Write};
use std::sync::LazyLock;

static DECIMAL_PLACES: LazyLock<u32> = LazyLock::new(|| {
    #[cfg(target_pointer_width = "32")]
    return 7;
    #[cfg(target_pointer_width = "64")]
    return 16;
});

/// Gets package name and version.
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

/// Fixes small floating point errors by rounding to the highest decimal place.
/// 
/// Decimal holds a 128 bit number and up to 28/29 decimal places.
/// 
/// `round` reduces the the number of decimal places (`scale`) by no less than the complied pointer
/// width (e.g. 7 decimal for 32 bit and 16 for 64 bit).  Then rounds for the new decimal place.
pub fn round(decimal: Decimal) -> Decimal {
    decimal.round_dp_with_strategy((decimal.scale().max(1) - 1).max(*DECIMAL_PLACES), RoundingStrategy::MidpointAwayFromZero).normalize()
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

    #[test]
    fn test_round() {
        assert_eq!(round(Decimal::from(10) - ((Decimal::from(400) - Decimal::from(300)) / Decimal::from(2))), Decimal::from(-40));
    }

}
