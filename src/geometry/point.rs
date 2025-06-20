// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::Unit;

/// A coordinate.  Origin in the lower left.  Positve x-axis extends right.  Positive y-axis extends up.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: Unit,
    pub y: Unit,
}

impl Point {

    /// Creates a new point.
    pub fn new(x: Unit, y: Unit) -> Point {
        Point { x: x, y: y }
    }

    /// Point at `(0, 0)`.
    pub fn zero() -> Point {
        Point { x: Unit::zero(), y: Unit::zero() }
    }

}
