// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use rust_decimal::Decimal;
use super::{POINT0, Size, Rect, Unit};

#[derive(Debug, Clone, PartialEq)]
pub enum PaperSize {
    // ISO A Series (International)
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
    A10,
    // North American Sizes
    Letter,
    Legal,
    Tabloid,
    Ledger,
    Executive,
    // ANSI Architectural Sizes
    AnsiA,
    AnsiB,
    AnsiC,
    AnsiD,
    AnsiE,
}

impl From<PaperSize> for Rect {

    fn from(value: PaperSize) -> Self {
        match value {
            PaperSize::A0 => Rect::from((POINT0, Size::new(Unit::Mm(Decimal::from(841)), Unit::Mm(Decimal::from(1189))))),
            PaperSize::A1 => Rect::from((POINT0, Size::new(Unit::Mm(Decimal::from(594)), Unit::Mm(Decimal::from(841))))),
            PaperSize::A2 => Rect::from((POINT0, Size::new(Unit::Mm(Decimal::from(420)), Unit::Mm(Decimal::from(594))))),
            PaperSize::A3 => Rect::from((POINT0, Size::new(Unit::Mm(Decimal::from(297)), Unit::Mm(Decimal::from(420))))),
            PaperSize::A4 => Rect::from((POINT0, Size::new(Unit::Mm(Decimal::from(210)), Unit::Mm(Decimal::from(297))))),
            PaperSize::A5 => Rect::from((POINT0, Size::new(Unit::Mm(Decimal::from(148)), Unit::Mm(Decimal::from(210))))),
            PaperSize::A6 => Rect::from((POINT0, Size::new(Unit::Mm(Decimal::from(105)), Unit::Mm(Decimal::from(148))))),
            PaperSize::A7 => Rect::from((POINT0, Size::new(Unit::Mm(Decimal::from(74)), Unit::Mm(Decimal::from(105))))),
            PaperSize::A8 => Rect::from((POINT0, Size::new(Unit::Mm(Decimal::from(52)), Unit::Mm(Decimal::from(74))))),
            PaperSize::A9 => Rect::from((POINT0, Size::new(Unit::Mm(Decimal::from(37)), Unit::Mm(Decimal::from(52))))),
            PaperSize::A10 => Rect::from((POINT0, Size::new(Unit::Mm(Decimal::from(26)), Unit::Mm(Decimal::from(37))))),
            PaperSize::Letter => Rect::from((POINT0, Size::new(Unit::Inch(Decimal::new(85, 1)), Unit::Inch(Decimal::from(11))))),
            PaperSize::Legal => Rect::from((POINT0, Size::new(Unit::Inch(Decimal::new(85, 1)), Unit::Inch(Decimal::from(14))))),
            PaperSize::Tabloid => Rect::from((POINT0, Size::new(Unit::Inch(Decimal::from(11)), Unit::Inch(Decimal::from(17))))),
            PaperSize::Ledger => Rect::from((POINT0, Size::new(Unit::Inch(Decimal::from(17)), Unit::Inch(Decimal::from(11))))),
            PaperSize::Executive => Rect::from((POINT0, Size::new(Unit::Inch(Decimal::new(725, 2)), Unit::Inch(Decimal::new(105, 1))))),
            PaperSize::AnsiA => Rect::from((POINT0, Size::new(Unit::Inch(Decimal::new(85, 1)), Unit::Inch(Decimal::from(11))))),
            PaperSize::AnsiB => Rect::from((POINT0, Size::new(Unit::Inch(Decimal::from(11)), Unit::Inch(Decimal::from(17))))),
            PaperSize::AnsiC => Rect::from((POINT0, Size::new(Unit::Inch(Decimal::from(17)), Unit::Inch(Decimal::from(22))))),
            PaperSize::AnsiD => Rect::from((POINT0, Size::new(Unit::Inch(Decimal::from(22)), Unit::Inch(Decimal::from(34))))),
            PaperSize::AnsiE => Rect::from((POINT0, Size::new(Unit::Inch(Decimal::from(34)), Unit::Inch(Decimal::from(44))))),
        }
    }
}
