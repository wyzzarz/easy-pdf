// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{UNIT0, Unit};

/// Size with a height and width of `0` points.
pub const SIZE0: Size = Size { width: UNIT0, height: UNIT0 };

/// A size.
#[derive(Debug, Clone, PartialEq)]
pub struct Size {
    pub width: Unit,
    pub height: Unit,
}

impl From<(Unit, Unit)> for Size {

    fn from((width, height): (Unit, Unit)) -> Self {
        Size { width, height }
    }

}

impl From<(isize, isize)> for Size {

    fn from((width, height): (isize, isize)) -> Self {
        Size { width: Unit::from(width), height: Unit::from(height) }
    }

}

impl From<(f32, f32)> for Size {

    fn from((width, height): (f32, f32)) -> Self {
        Size { width: Unit::from(width), height: Unit::from(height) }
    }

}

impl Size {

    /// Creates a new size.
    pub fn new(width: Unit, height: Unit) -> Size {
        Size { width: width, height: height }
    }

}
