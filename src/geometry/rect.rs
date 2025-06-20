// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{POINT0, Point, SIZE0, Size, Unit};
use crate::pdf_object::PdfObject;

/// A rect, in points, at the origin with no size.
pub const RECT0: Rect = Rect { origin: POINT0, size: SIZE0 };

/// A rectangle.
#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {

    /// Lower left coorinate for rectangle.
    pub fn lower_left(&self) -> Point {
        self.origin.clone()
    }

    /// Upper right coordinate for rectangle.
    pub fn upper_right(&self) -> Point {
        Point {
            x: &self.origin.x + &self.size.width,
            y: &self.origin.y + &self.size.height,
        }
    }
    
    /// Max x value.
    pub fn max_x(&self) -> Unit {
        &self.origin.x + &self.size.width
    }

    /// Min x value.
    pub fn min_x(&self) -> Unit {
        self.origin.x.clone()
    }

    /// Mid x value.
    pub fn mid_x(&self) -> Unit {
        &self.origin.x + &(&self.size.width / &Unit::from(2))
    }    

    /// Max y value.
    pub fn max_y(&self) -> Unit {
        &self.origin.y + &self.size.height
    }

    /// Min y value.
    pub fn min_y(&self) -> Unit {
        self.origin.y.clone()
    }

    /// Mid y value.
    pub fn mid_y(&self) -> Unit {
        &self.origin.y + &(&self.size.height / &Unit::from(2))
    }

    /// Center of rect.
    pub fn center(&self) -> Point {
        Point { x: self.mid_x(), y: self.mid_y() }
    }

}

impl From<(Point, Size)> for Rect {

    fn from(value: (Point, Size)) -> Self {
        Rect {
            origin: value.0,
            size: value.1,
        }
    }

}

impl From<(Unit, Unit, Unit, Unit)> for Rect {

    fn from((x, y, width, height): (Unit, Unit, Unit, Unit)) -> Self {
        Rect {
            origin: Point { x, y },
            size: Size { width, height },
        }
    }

}

impl From<(isize, isize, isize, isize)> for Rect {

    fn from((x, y, width, height): (isize, isize, isize, isize)) -> Self {
        Rect {
            origin: Point::from((x, y)),
            size: Size::from((width, height)),
        }
    }

}

impl From<(f32, f32, f32, f32)> for Rect {

    fn from((x, y, width, height): (f32, f32, f32, f32)) -> Self {
        Rect {
            origin: Point::from((x, y)),
            size: Size::from((width, height)),
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

    #[test]
    fn test_center() {
        let rect = Rect::from((Unit::from(1), Unit::from(20), Unit::from(300), Unit::from(4000)));
        assert_eq!(rect.center(), Point { x: Unit::from(151), y: Unit::from(2020) });
    }

}
