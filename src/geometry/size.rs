// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Unit;

/// A size.
#[derive(Debug, Clone, PartialEq)]
pub struct Size {
    pub width: Unit,
    pub height: Unit,
}

impl Size {

    /// Creates a new size.
    pub fn new(width: Unit, height: Unit) -> Size {
        Size { width: width, height: height }
    }

    /// Size of `(0, 0)`.
    pub fn zero() -> Size {
        Size { width: Unit::zero(), height: Unit::zero() }
    }

}
