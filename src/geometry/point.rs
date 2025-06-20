// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{UNIT0, Unit};

/// Coordinate in points located at the origin `(0, 0)`.
pub const POINT0: Point = Point { x: UNIT0, y: UNIT0 };

/// A coordinate.  Origin in the lower left.  Positve x-axis extends right.  Positive y-axis extends up.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: Unit,
    pub y: Unit,
}

impl From<(Unit, Unit)> for Point {

    fn from((x, y): (Unit, Unit)) -> Point {
        Point::new(x, y)
    }

}

impl From<(isize, isize)> for Point {

    fn from((x, y): (isize, isize)) -> Point {
        Point::new(Unit::from(x), Unit::from(y))
    }

}

impl From<(f32, f32)> for Point {

    fn from((x, y): (f32, f32)) -> Point {
        Point::new(Unit::from(x), Unit::from(y))
    }

}

impl Point {

    /// Creates a new point.
    pub fn new(x: Unit, y: Unit) -> Point {
        Point { x: x, y: y }
    }

}
