// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use rust_decimal::Decimal;
use std::str::FromStr;
use crate::pdf_object::PdfObject;

/// Unit with `0` point value.
pub const UNIT0: Unit = Unit::Point(Decimal::ZERO);

/// Default user space units.
#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    /// A centimeter.
    Cm(Decimal),
    /// A pixel.  A pixel is 1/96 of an Inch.
    Pixel(Decimal),
    /// A point.  A point is 1/72 of an Inch.
    Point(Decimal),
    /// An inch.
    Inch(Decimal),
    /// A mil.  A mil is 1/1000 of an Inch.
    Mil(Decimal),
    /// A millimeter.
    Mm(Decimal),
}

impl Default for Unit {

    /// Defaults to a `Point` with a zero value.
    fn default() -> Self {
        UNIT0
    }

}

impl From<i32> for Unit {

    fn from(value: i32) -> Self {
        Unit::Point(Decimal::from(value))
    }

}

impl From<isize> for Unit {

    fn from(value: isize) -> Self {
        Unit::Point(Decimal::from(value))
    }

}

impl From<u32> for Unit {

    fn from(value: u32) -> Self {
        Unit::Point(Decimal::from(value))
    }

}

impl From<usize> for Unit {

    fn from(value: usize) -> Self {
        Unit::Point(Decimal::from(value))
    }

}

impl From<f32> for Unit {

    fn from(value: f32) -> Self {
        Unit::Point(Decimal::from_str(&value.to_string()).unwrap())
    }

}

impl From<f64> for Unit {

    fn from(value: f64) -> Self {
        Unit::Point(Decimal::from_str(&value.to_string()).unwrap())
    }

}


impl From<Unit> for PdfObject {

    /// Converts the Unit to a PDF object in points.
    fn from(value: Unit) -> Self {
        PdfObject::Number(value.point().real_value())
    }

}

impl Unit {

    /// Decimal value for the unit.
    pub fn decimal_value(&self) -> Decimal {
        match self {
            Unit::Cm(cm) => *cm,
            Unit::Pixel(pixel) => *pixel,
            Unit::Point(point) => *point,
            Unit::Inch(inch) => *inch,
            Unit::Mil(mil) => *mil,
            Unit::Mm(mm) => *mm,
        }
    }

    /// Real numbers are limited to 5 decimal places.
    pub fn real_value(&self) -> Decimal {
        let mut real = self.decimal_value();
        real.rescale(5);
        real.normalize()
    }

    /// Unit in centimeters.
    pub fn cm(&self) -> Unit {
        Unit::Cm(self.inch().decimal_value() * Decimal::new(254, 2))
    }

    /// Unit in pixels.  A pixel is 1/96 of an inch.
    pub fn pixel(&self) -> Unit {
        Unit::Pixel(self.inch().decimal_value() * Decimal::from(96))
    }

    /// Unit in points.  A point is 1/72 of an inch.
    pub fn point(&self) -> Unit {
        Unit::Point(self.inch().decimal_value() * Decimal::from(72))
    }

    /// Unit in inches.
    pub fn inch(&self) -> Unit {
        match self {
            Unit::Cm(cm) => Unit::Inch(cm / Decimal::new(254, 2)),
            Unit::Pixel(point) => Unit::Inch(*point / Decimal::from(96)),
            Unit::Point(point) => Unit::Inch(point / Decimal::from(72)),
            Unit::Inch(inch) => Unit::Inch(*inch),
            Unit::Mil(mil) => Unit::Inch(mil / Decimal::from(1000)),
            Unit::Mm(mm) => Unit::Inch(mm / Decimal::new(254, 1)),
        }
    }

    /// Unit in mils.
    pub fn mil(&self) -> Unit {
        Unit::Mil(self.inch().decimal_value() * Decimal::from(1000))
    }

    /// Unit in milimeters.
    pub fn mm(&self) -> Unit {
        Unit::Mm(self.inch().decimal_value() * Decimal::new(254, 1))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cm() {
        let cm = Unit::Cm(Decimal::new(254, 2));
        assert_eq!(cm.decimal_value(), Decimal::new(254, 2));
        assert_eq!(Unit::Cm(Decimal::new(254, 2)).cm(), cm);
        assert_eq!(Unit::Pixel(Decimal::from(96)).cm(), cm);
        assert_eq!(Unit::Point(Decimal::from(72)).cm(), cm);
        assert_eq!(Unit::Inch(Decimal::from(1)).cm(), cm);
        assert_eq!(Unit::Mil(Decimal::from(1000)).cm(), cm);
        assert_eq!(Unit::Mm(Decimal::new(254, 1)).cm(), cm);
    }

    #[test]
    fn test_pixel() {
        let px = Unit::Pixel(Decimal::from(96));
        assert_eq!(px.decimal_value(), Decimal::from(96));
        assert_eq!(Unit::Cm(Decimal::new(254, 2)).pixel(), px);
        assert_eq!(Unit::Pixel(Decimal::from(96)).pixel(), px);
        assert_eq!(Unit::Point(Decimal::from(72)).pixel(), px);
        assert_eq!(Unit::Inch(Decimal::from(1)).pixel(), px);
        assert_eq!(Unit::Mil(Decimal::from(1000)).pixel(), px);
        assert_eq!(Unit::Mm(Decimal::new(254, 1)).pixel(), px);
    }

    #[test]
    fn test_point() {
        let pt = Unit::Point(Decimal::from(72));
        assert_eq!(pt.decimal_value(), Decimal::from(72));
        assert_eq!(Unit::Cm(Decimal::new(254, 2)).point(), pt);
        assert_eq!(Unit::Pixel(Decimal::from(96)).point(), pt);
        assert_eq!(Unit::Point(Decimal::from(72)).point(), pt);
        assert_eq!(Unit::Inch(Decimal::from(1)).point(), pt);
        assert_eq!(Unit::Mil(Decimal::from(1000)).point(), pt);
        assert_eq!(Unit::Mm(Decimal::new(254, 1)).point(), pt);
    }

    #[test]
    fn test_inch() {
        let inch = Unit::Inch(Decimal::from(1));
        assert_eq!(inch.decimal_value(), Decimal::from(1));
        assert_eq!(Unit::Cm(Decimal::new(254, 2)).inch(), inch);
        assert_eq!(Unit::Pixel(Decimal::from(96)).inch(), inch);
        assert_eq!(Unit::Point(Decimal::from(72)).inch(), inch);
        assert_eq!(Unit::Inch(Decimal::from(1)).inch(), inch);
        assert_eq!(Unit::Mil(Decimal::from(1000)).inch(), inch);
        assert_eq!(Unit::Mm(Decimal::new(254, 1)).inch(), inch);
    }

    #[test]
    fn test_mil() {
        let mil = Unit::Mil(Decimal::from(1000));
        assert_eq!(mil.decimal_value(), Decimal::from(1000));
        assert_eq!(Unit::Cm(Decimal::new(254, 2)).mil(), mil);
        assert_eq!(Unit::Pixel(Decimal::from(96)).mil(), mil);
        assert_eq!(Unit::Point(Decimal::from(72)).mil(), mil);
        assert_eq!(Unit::Inch(Decimal::from(1)).mil(), mil);
        assert_eq!(Unit::Mil(Decimal::from(1000)).mil(), mil);
        assert_eq!(Unit::Mm(Decimal::new(254, 1)).mil(), mil);
    }

    #[test]
    fn test_mm() {
        let mm = Unit::Mm(Decimal::new(254, 1));
        assert_eq!(mm.decimal_value(), Decimal::new(254, 1));
        assert_eq!(Unit::Cm(Decimal::new(254, 2)).mm(), mm);
        assert_eq!(Unit::Pixel(Decimal::from(96)).mm(), mm);
        assert_eq!(Unit::Point(Decimal::from(72)).mm(), mm);
        assert_eq!(Unit::Inch(Decimal::from(1)).mm(), mm);
        assert_eq!(Unit::Mil(Decimal::from(1000)).mm(), mm);
        assert_eq!(Unit::Mm(Decimal::new(254, 1)).mm(), mm);
    }

}
