// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{Point, Size, Unit};
use crate::pdf_object::PdfObject;

/// A rectangle.
#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub bounds: Size,
}

impl Rect {

    /// Lower left coorinate for rectangle.
    pub fn lower_left(&self) -> Point {
        self.origin.clone()
    }

    /// Upper right coordinate for rectangle.
    pub fn upper_right(&self) -> Point {
        Point {
            x: &self.origin.x + &self.bounds.width,
            y: &self.origin.y + &self.bounds.height,
        }
    }
    
    /// Max x value.
    pub fn max_x(&self) -> Unit {
        &self.origin.x + &self.bounds.width
    }

    /// Min x value.
    pub fn min_x(&self) -> Unit {
        self.origin.x.clone()
    }

    /// Max y value.
    pub fn max_y(&self) -> Unit {
        &self.origin.y + &self.bounds.height
    }

    /// Min y value.
    pub fn min_y(&self) -> Unit {
        self.origin.y.clone()
    }

}

impl From<(Point, Size)> for Rect {

    fn from(value: (Point, Size)) -> Self {
        Rect {
            origin: value.0,
            bounds: value.1,
        }
    }

}

impl From<(Unit, Unit, Unit, Unit)> for Rect {

    fn from(value: (Unit, Unit, Unit, Unit)) -> Self {
        Rect {
            origin: Point {
                x: value.0,
                y: value.1,
            },
            bounds: Size {
                width: value.2,
                height: value.3,
            },
        }
    }

}

impl From<Rect> for PdfObject {

    /// Converts Rect to a to a PDF object specified by its lower-left and upper-right corners.  In points.
    fn from(value: Rect) -> Self {
        PdfObject::Array(vec![
            PdfObject::from(value.min_x()),
            PdfObject::from(value.min_y()),
            PdfObject::from(value.max_x()),
            PdfObject::from(value.max_y()),
        ])
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect() {
        let rect = Rect::from((Unit::from(1), Unit::from(20), Unit::from(300), Unit::from(4000)));
        assert_eq!(PdfObject::from(rect).to_string(), "[1 20 301 4020]");
    }

}
