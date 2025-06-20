// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use rust_decimal::Decimal;
use std::str::FromStr;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use crate::helpers::round;
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

impl Add for &Unit {

    type Output = Unit;

    fn add(self, rhs: Self) -> Unit {
        match self {
            Unit::Cm(_) => Unit::Cm(round(self.decimal_value() + rhs.cm().decimal_value())),
            Unit::Pixel(_) => Unit::Pixel(round(self.decimal_value() + rhs.pixel().decimal_value())),
            Unit::Point(_) => Unit::Point(round(self.decimal_value() + rhs.point().decimal_value())),
            Unit::Inch(_) => Unit::Inch(round(self.decimal_value() + rhs.inch().decimal_value())),
            Unit::Mil(_) => Unit::Mil(round(self.decimal_value() + rhs.mil().decimal_value())),
            Unit::Mm(_) => Unit::Mm(round(self.decimal_value() + rhs.mm().decimal_value())),
        }
    }

}

impl AddAssign<&Unit> for Unit {

    fn add_assign(&mut self, rhs: &Self) {
        *self = self.add(&rhs);
    }

}

impl Sub for &Unit {

    type Output = Unit;

    fn sub(self, rhs: Self) -> Unit {
        match self {
            Unit::Cm(_) => Unit::Cm(round(self.decimal_value() - rhs.cm().decimal_value())),
            Unit::Pixel(_) => Unit::Pixel(round(self.decimal_value() - rhs.pixel().decimal_value())),
            Unit::Point(_) => Unit::Point(round(self.decimal_value() - rhs.point().decimal_value())),
            Unit::Inch(_) => Unit::Inch(round(self.decimal_value() - rhs.inch().decimal_value())),
            Unit::Mil(_) => Unit::Mil(round(self.decimal_value() - rhs.mil().decimal_value())),
            Unit::Mm(_) => Unit::Mm(round(self.decimal_value() - rhs.mm().decimal_value())),
        }
    }

}

impl SubAssign<&Unit> for Unit {

    fn sub_assign(&mut self, rhs: &Self) {
        *self = self.sub(&rhs);
    }

}

impl Mul for &Unit {

    type Output = Unit;

    /// Performs the `*` operation.
    /// 
    /// Only the decimal value is used for the `rhs`.  The units are stripped.
    /// 
    /// # Example
    /// assert_eq!(Unit::Inch(Decimal::from(2)) * Unit::Point(Decimal::from(72)), Unit::Inch(Decimal::from(144)))
    fn mul(self, rhs: Self) -> Unit {
        match self {
            Unit::Cm(_) => Unit::Cm(round(self.decimal_value() * rhs.decimal_value())),
            Unit::Pixel(_) => Unit::Pixel(round(self.decimal_value() * rhs.decimal_value())),
            Unit::Point(_) => Unit::Point(round(self.decimal_value() * rhs.decimal_value())),
            Unit::Inch(_) => Unit::Inch(round(self.decimal_value() * rhs.decimal_value())),
            Unit::Mil(_) => Unit::Mil(round(self.decimal_value() * rhs.decimal_value())),
            Unit::Mm(_) => Unit::Mm(round(self.decimal_value() * rhs.decimal_value())),
        }
    }

}

impl MulAssign<&Unit> for Unit {

    fn mul_assign(&mut self, rhs: &Self) {
        *self = self.mul(&rhs);
    }

}

impl Div for &Unit {

    type Output = Unit;

    /// Performs the `/` operation.
    /// 
    /// Only the decimal value is used for the `rhs`.  The units are stripped.
    /// 
    /// # Example
    /// assert_eq!(Unit::Inch(Decimal::from(72)) / Unit::Point(Decimal::from(36)), Unit::Inch(Decimal::from(2)))
    fn div(self, rhs: Self) -> Unit {
        match self {
            Unit::Cm(_) => Unit::Cm(round(self.decimal_value() / rhs.decimal_value())),
            Unit::Pixel(_) => Unit::Pixel(round(self.decimal_value() / rhs.decimal_value())),
            Unit::Point(_) => Unit::Point(round(self.decimal_value() / rhs.decimal_value())),
            Unit::Inch(_) => Unit::Inch(round(self.decimal_value() / rhs.decimal_value())),
            Unit::Mil(_) => Unit::Mil(round(self.decimal_value() / rhs.decimal_value())),
            Unit::Mm(_) => Unit::Mm(round(self.decimal_value() / rhs.decimal_value())),
        }
    }

}

impl DivAssign<&Unit> for Unit {

    fn div_assign(&mut self, rhs: &Self) {
        *self = self.div(&rhs);
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

    #[test]
    fn test_add() {
        let cm = Unit::Cm(Decimal::new(254, 2)); // 1 inch
        let px = Unit::Pixel(Decimal::from(96)); // 1 inch
        let pt = Unit::Point(Decimal::from(72)); // 1 inch
        let inch = Unit::Inch(Decimal::from(1));
        let mil = Unit::Mil(Decimal::from(1000)); // 1 inch
        let mm = Unit::Mm(Decimal::new(254, 1)); // 1 inch

        assert_eq!((&inch + &cm), Unit::Inch(Decimal::from(2)));
        assert_eq!((&inch + &px), Unit::Inch(Decimal::from(2)));
        assert_eq!((&inch + &pt), Unit::Inch(Decimal::from(2)));
        assert_eq!((&inch + &inch), Unit::Inch(Decimal::from(2)));
        assert_eq!((&inch + &mil), Unit::Inch(Decimal::from(2)));
        assert_eq!((&inch + &mm), Unit::Inch(Decimal::from(2)));

        let mut units = UNIT0;
        units += &cm;
        assert_eq!(units, Unit::Point(Decimal::from(72 * 1)));
        units += &px;
        assert_eq!(units, Unit::Point(Decimal::from(72 * 2)));
        units += &pt;
        assert_eq!(units, Unit::Point(Decimal::from(72 * 3)));
        units += &inch;
        assert_eq!(units, Unit::Point(Decimal::from(72 * 4)));
        units += &mil;
        assert_eq!(units, Unit::Point(Decimal::from(72 * 5)));
        units += &mm;
        assert_eq!(units, Unit::Point(Decimal::from(72 * 6)));
    }

    #[test]
    fn test_subtract() {
        let cm = Unit::Cm(Decimal::new(254, 2)); // 1 inch
        let px = Unit::Pixel(Decimal::from(96)); // 1 inch
        let pt = Unit::Point(Decimal::from(72)); // 1 inch
        let inch = Unit::Inch(Decimal::from(1));
        let mil = Unit::Mil(Decimal::from(1000)); // 1 inch
        let mm = Unit::Mm(Decimal::new(254, 1)); // 1 inch

        let inch3 = Unit::Inch(Decimal::from(3));

        assert_eq!((&inch3 - &cm), Unit::Inch(Decimal::from(2)));
        assert_eq!((&inch3 - &px), Unit::Inch(Decimal::from(2)));
        assert_eq!((&inch3 - &pt), Unit::Inch(Decimal::from(2)));
        assert_eq!((&inch3 - &inch), Unit::Inch(Decimal::from(2)));
        assert_eq!((&inch3 - &mil), Unit::Inch(Decimal::from(2)));
        assert_eq!((&inch3 - &mm), Unit::Inch(Decimal::from(2)));

        let mut units = UNIT0;
        units -= &cm;
        assert_eq!(units, Unit::Point(Decimal::from(-72 * 1)));
        units -= &px;
        assert_eq!(units, Unit::Point(Decimal::from(-72 * 2)));
        units -= &pt;
        assert_eq!(units, Unit::Point(Decimal::from(-72 * 3)));
        units -= &inch;
        assert_eq!(units, Unit::Point(Decimal::from(-72 * 4)));
        units -= &mil;
        assert_eq!(units, Unit::Point(Decimal::from(-72 * 5)));
        units -= &mm;
        assert_eq!(units, Unit::Point(Decimal::from(-72 * 6)));
    }

    #[test]
    fn test_multiply() {
        let cm = Unit::Cm(Decimal::new(254, 2)); // 1 inch
        let px = Unit::Pixel(Decimal::from(96)); // 1 inch
        let pt = Unit::Point(Decimal::from(72)); // 1 inch
        let inch = Unit::Inch(Decimal::from(1));
        let mil = Unit::Mil(Decimal::from(1000)); // 1 inch
        let mm = Unit::Mm(Decimal::new(254, 1)); // 1 inch

        assert_eq!((&inch * &cm), Unit::Inch(Decimal::new(254, 2)));
        assert_eq!((&inch * &px), Unit::Inch(Decimal::new(96, 0)));
        assert_eq!((&inch * &pt), Unit::Inch(Decimal::new(72, 0)));
        assert_eq!((&inch * &inch), Unit::Inch(Decimal::new(1, 0)));
        assert_eq!((&inch * &mil), Unit::Inch(Decimal::new(1000, 0)));
        assert_eq!((&inch * &mm), Unit::Inch(Decimal::new(254, 1)));

        let mut units = Unit::from(1);
        units *= &cm;
        assert_eq!(units, Unit::from(2.54));
        units *= &px;
        assert_eq!(units, Unit::from(243.84));
        units *= &pt;
        assert_eq!(units, Unit::from(17556.48));
        units *= &inch;
        assert_eq!(units, Unit::from(17556.48));
        units *= &mil;
        assert_eq!(units, Unit::from(17556480));
        units *= &mm;
        assert_eq!(units, Unit::from(445934592));

        assert_eq!(&Unit::Inch(Decimal::from(2)) * &Unit::Point(Decimal::from(72)), Unit::Inch(Decimal::from(144)))
    }

    #[test]
    fn test_divide() {
        let cm = Unit::Cm(Decimal::new(254, 2)); // 1 inch
        let px = Unit::Pixel(Decimal::from(96)); // 1 inch
        let pt = Unit::Point(Decimal::from(72)); // 1 inch
        let inch = Unit::Inch(Decimal::from(1));
        let mil = Unit::Mil(Decimal::from(1000)); // 1 inch
        let mm = Unit::Mm(Decimal::new(254, 1)); // 1 inch

        assert_eq!((&inch / &cm).real_value(), Decimal::new(39370, 5));
        assert_eq!((&inch / &px).real_value(), Decimal::new(1042, 5));
        assert_eq!((&inch / &pt).real_value(), Decimal::new(1389, 5));
        assert_eq!((&inch / &inch).real_value(), Decimal::new(1, 0));
        assert_eq!((&inch / &mil).real_value(), Decimal::new(1, 3));
        assert_eq!((&inch / &mm).real_value(), Decimal::new(3937, 5));

        let mut units = Unit::from(6912);
        units /= &cm;
        assert_eq!(units.real_value(), Decimal::new(272125984, 5));
        units /= &px;
        assert_eq!(units.real_value(), Decimal::new(2834646, 5));
        units /= &pt;
        assert_eq!(units.real_value(), Decimal::new(3937, 4));
        units /= &inch;
        assert_eq!(units.real_value(), Decimal::new(3937, 4));
        units /= &mil;
        assert_eq!(units.real_value(), Decimal::new(39, 5));
        units /= &mm;
        assert_eq!(units.real_value(), Decimal::new(2, 5));

        assert_eq!(&Unit::Inch(Decimal::from(72)) / &Unit::Point(Decimal::from(36)), Unit::Inch(Decimal::from(2)))
    }

}
