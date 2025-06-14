// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

/*!
 * Native PDF objects.  See PDF 1.7 - 7.3.
 * 
 * 7.3.2 : Boolean
 * 7.3.3 : Number (integer and real)
 * 7.3.4 : String (literal and hex)
 * 7.3.5 : Name
 * 7.3.6 : Array
 * 7.3.7 : Dictionary
 * 7.3.8 : Stream
 * 7.3.9 : Null
 */

use rust_decimal::Decimal;
use std::{collections::HashMap, str::FromStr};
use crate::helpers::write_all_count;

/// Pdf object types.
#[derive(Debug, Clone, PartialEq)]
pub enum PdfObject {
    Bool(bool),
    Number(Decimal),
    String(String),
    HexString(String),
    Name(String),
    Array(Vec<PdfObject>),
    Dictionary(HashMap<String, PdfObject>),
    Stream(Vec<u8>),
    Null,
}

impl ToString for PdfObject {
    
    fn to_string(&self) -> String {
        let mut vec: Vec<u8> = Vec::new();
        self.render(&mut vec).unwrap();
        String::from_utf8(vec).unwrap()
    }

}

impl From<bool> for PdfObject {

    fn from(b: bool) -> Self {
        PdfObject::Bool(b)
    }

}

impl From<isize> for PdfObject {

    fn from(n: isize) -> Self {
        PdfObject::Number(n.into())
    }

}

impl From<i32> for PdfObject {

    fn from(n: i32) -> Self {
        PdfObject::Number(n.into())
    }

}

impl From<usize> for PdfObject {

    fn from(n: usize) -> Self {
        PdfObject::Number(n.into())
    }

}

impl From<u32> for PdfObject {

    fn from(n: u32) -> Self {
        PdfObject::Number(n.into())
    }

}

impl From<f32> for PdfObject {

    fn from(n: f32) -> Self {
        PdfObject::Number(Decimal::from_str(&n.to_string()).unwrap())
    }

}

impl From<f64> for PdfObject {

    fn from(n: f64) -> Self {
        PdfObject::Number(Decimal::from_str(&n.to_string()).unwrap())
    }

}

impl From <String> for PdfObject {

    fn from(s: String) -> Self {
        PdfObject::String(s)
    }

}

impl From<&str> for PdfObject {

    fn from(s: &str) -> Self {
        PdfObject::String(s.to_string())
    }

}

impl PdfObject {

    pub fn render(&self, writer: &mut dyn std::io::Write) -> Result<usize, Box<dyn std::error::Error>> {
        Ok(match self {
            PdfObject::Bool(b) => self.render_bool(writer, b),
            PdfObject::Number(n) => self.render_number(writer, n),
            PdfObject::String(s) => self.render_string(writer, &s),
            PdfObject::HexString(s) => self.render_hex_string(writer, &s),
            PdfObject::Name(n) => self.render_name(writer, &n),
            PdfObject::Array(a) => self.render_array(writer, &a),
            PdfObject::Dictionary(d) => self.render_dictionary(writer, &d),
            PdfObject::Stream(s) => self.render_stream(writer, &s),
            PdfObject::Null => self.render_null(writer),
        }?)
    }

    /// Generates PDF output for the boolean.
    fn render_bool(&self, writer: &mut dyn std::io::Write, b: &bool) -> Result<usize, Box<dyn std::error::Error>> {
        Ok(write_all_count(writer, b.to_string().as_bytes())?)
    }

    /// Generates PDF output for the number.
    fn render_number(&self, writer: &mut dyn std::io::Write, n: &Decimal) -> Result<usize, Box<dyn std::error::Error>> {
        Ok(write_all_count(writer, n.to_string().as_bytes())?)
    }

    /// Generates PDF output for the string.
    fn render_string(&self, writer: &mut dyn std::io::Write, s: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let mut count = 0;
        count += write_all_count(writer, b"(")?;
        for c in s.bytes() {
            match c {
                b'(' => count += write_all_count(writer, b"\\(")?,
                b')' => count += write_all_count(writer, b"\\)")?,
                b'\\' => count += write_all_count(writer, b"\\\\")?,
                _ => count += write_all_count(writer, &[c])?,
            }
        }
        count += write_all_count(writer, b")")?;
        Ok(count)
    }

    /// Generates PDF output for the string.
    fn render_hex_string(&self, writer: &mut dyn std::io::Write, s: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let mut count = 0;
        count += write_all_count(writer, b"<")?;
        count += write_all_count(writer, s.as_bytes())?;
        count += write_all_count(writer, b">")?;
        Ok(count)
    }

    /// Generates PDF output for the name.
    fn render_name(&self, writer: &mut dyn std::io::Write, n: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let mut count = 0;
        count += write_all_count(writer, b"/")?;
        for c in n.bytes() {
            match c {
                b' ' => count += write_all_count(writer, b"#20")?,
                b'(' => count += write_all_count(writer, b"#28")?,
                b')' => count += write_all_count(writer, b"#29")?,
                _ => count += write_all_count(writer, &[c])?,
            }
        }
        Ok(count)
    }

    /// Generates PDF output for the array.
    fn render_array(&self, writer: &mut dyn std::io::Write, a: &Vec<PdfObject>) -> Result<usize, Box<dyn std::error::Error>> {
        let mut count = 0;
        count += write_all_count(writer, b"[")?;
        for o in a.iter() {
            count += o.render(writer)?;
            if o != a.last().unwrap() {
                count += write_all_count(writer, b" ")?;
            }
        }
        count += write_all_count(writer, b"]")?;
        Ok(count)
    }

    /// Generates PDF output for the dictionary.
    fn render_dictionary(&self, writer: &mut dyn std::io::Write, d: &HashMap<String, PdfObject>) -> Result<usize, Box<dyn std::error::Error>> {
        let mut count = 0;
        let mut entries: Vec<(&String, &PdfObject)> = d.iter().collect();
        entries.sort_by_key(|&(k, _)| k);
        let len = entries.len();
        count += write_all_count(writer, b"<< ")?;
        for (i, (k, v)) in entries.iter().enumerate() {
            count += self.render_name(writer, k)?;
            count += write_all_count(writer, b" ")?;
            count += v.render(writer)?;
            if i < len - 1 {
                count += write_all_count(writer, b" ")?;
            }
        }
        count += write_all_count(writer, b" >>")?;
        Ok(count)
    }

    /// Generates PDF output for the stream.
    fn render_stream(&self, _writer: &mut dyn std::io::Write, _s: &Vec<u8>) -> Result<usize, Box<dyn std::error::Error>> {
        Ok(0)
    }

    /// Generates PDF output for the boolean.
    fn render_null(&self, writer: &mut dyn std::io::Write) -> Result<usize, Box<dyn std::error::Error>> {
        Ok(write_all_count(writer, b"null")?)
    }

}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use super::*;

    #[test]
    fn test_null() {
        let mut bytes: Vec<u8> = Vec::new();
        assert_eq!(PdfObject::Null.render(&mut bytes).ok(), Some(4));
        assert_eq!(PdfObject::Null.to_string(), "null");
    }

    #[test]
    fn test_bool() {
        let mut bytes: Vec<u8> = Vec::new();
        assert_eq!(PdfObject::Bool(true).render(&mut bytes).ok(), Some(4));
        assert_eq!(PdfObject::Bool(true).to_string(), "true");
        assert_eq!(PdfObject::from(true).to_string(), "true");
        assert_eq!(PdfObject::Bool(false).to_string(), "false");
        assert_eq!(PdfObject::from(false).to_string(), "false");
    }

    #[test]
    fn test_number_integer() {
        let mut bytes: Vec<u8> = Vec::new();
        assert_eq!(PdfObject::Number(Decimal::from(123)).render(&mut bytes).ok(), Some(3));
        assert_eq!(PdfObject::Number(Decimal::from(123)).to_string(), "123");
        assert_eq!(PdfObject::from(43445 as usize).to_string(), "43445");
        assert_eq!(PdfObject::from((-98 as i16) as isize).to_string(), "-98");
        assert_eq!(PdfObject::Number(Decimal::from(0)).to_string(), "0");
    }

    #[test]
    fn test_number_float() {
        let mut bytes: Vec<u8> = Vec::new();
        assert_eq!(PdfObject::Number(Decimal::new(345, 1)).render(&mut bytes).ok(), Some(4));
        assert_eq!(PdfObject::Number(Decimal::new(345, 1)).to_string(), "34.5");
        assert_eq!(PdfObject::Number(Decimal::new(-362, 2)).to_string(), "-3.62");
        assert_eq!(PdfObject::Number(Decimal::new(-2, 3)).to_string(), "-0.002");
        assert_eq!(PdfObject::from(-123.456_f32).to_string(), "-123.456");
        assert_eq!(PdfObject::from(-123.456_f64).to_string(), "-123.456");
    }

    #[test]
    fn test_string() {
        let mut bytes: Vec<u8> = Vec::new();
        let input = "This is a string".to_string();
        let output = "(This is a string)";
        assert_eq!(PdfObject::String(input.clone()).render(&mut bytes).ok(), Some(output.len()));
        assert_eq!(PdfObject::String(input).to_string(), output);
        assert_eq!(
            PdfObject::from("Strings may contain newlines\nand such").to_string(),
            "(Strings may contain newlines\nand such)"
        );
        assert_eq!(
            PdfObject::from(r#"Strings may contain balanced parentheses ( ) and special characters (*!&}^% and so on)."#).to_string(), 
//            r#"(Strings may contain balanced parentheses ( ) and special characters (*!&}^% and so on).)"#
            r#"(Strings may contain balanced parentheses \( \) and special characters \(*!&}^% and so on\).)"#
        );
        assert_eq!(
            PdfObject::from("").to_string(), 
            "()"
        );
    }

    #[test]
    fn test_hex_string() {
        let mut bytes: Vec<u8> = Vec::new();
        let input = "4E6F762073686D6F7A206B6120706F702E".to_string();
        let output = "<4E6F762073686D6F7A206B6120706F702E>";
        assert_eq!(PdfObject::HexString(input.clone()).render(&mut bytes).ok(), Some(output.len()));
        assert_eq!(PdfObject::HexString(input).to_string(), output);
    }

    #[test]
    fn test_name() {
        let mut bytes: Vec<u8> = Vec::new();
        assert_eq!(PdfObject::Name("Name1".to_string()).render(&mut bytes).ok(), Some(6));
        assert_eq!(
            PdfObject::Name("Name1".to_string()).to_string(), 
            "/Name1"
        );
        assert_eq!(
            PdfObject::Name("ASomewhatLongerName".to_string()).to_string(), 
            "/ASomewhatLongerName"
        );
        assert_eq!(
            PdfObject::Name(r#"A;Name_With-Various***Characters?"#.to_string()).to_string(), 
            r#"/A;Name_With-Various***Characters?"#
        );
        assert_eq!(
            PdfObject::Name("1.2".to_string()).to_string(), 
            "/1.2"
        );
        assert_eq!(
            PdfObject::Name("$$".to_string()).to_string(), 
            "/$$"
        );
        assert_eq!(
            PdfObject::Name("@pattern".to_string()).to_string(), 
            "/@pattern"
        );
        assert_eq!(
            PdfObject::Name(".notdef".to_string()).to_string(), 
            "/.notdef"
        );
        assert_eq!(
            PdfObject::Name("Lime Green".to_string()).to_string(), 
            "/Lime#20Green"
        );
        assert_eq!(
            PdfObject::Name("paired()parentheses".to_string()).to_string(), 
            "/paired#28#29parentheses"
        );
        assert_eq!(
            PdfObject::Name("The_Key_of_F#_Minor".to_string()).to_string(), 
            "/The_Key_of_F#_Minor"
        );
    }

    #[test]
    fn test_array() {
        let mut bytes: Vec<u8> = Vec::new();

        // test empty
        assert_eq!(PdfObject::Array(Vec::new()).render(&mut bytes).ok(), Some(2));
        assert_eq!(PdfObject::Array(Vec::new()).to_string(), "[]");

        // test full
        let arr = PdfObject::Array(vec![
            PdfObject::from(true),
            PdfObject::from(123 as usize),
            PdfObject::from("This is a string"),
            PdfObject::HexString("4E6F".to_string()),
            PdfObject::Array(vec![PdfObject::from(1 as usize), PdfObject::from(2 as usize), PdfObject::from(3 as usize)]),
            PdfObject::Dictionary(hashmap!["a".to_string() => PdfObject::from(1 as usize), "b".to_string() => PdfObject::from(2 as usize), "c".to_string() => PdfObject::from(3 as usize)]),
            PdfObject::Null,
        ]);
        let output = b"[true 123 (This is a string) <4E6F> [1 2 3] << /a 1 /b 2 /c 3 >> null]";
        let mut bytes: Vec<u8> = Vec::new();
        assert_eq!(arr.render(&mut bytes).ok(), Some(output.len()));
        assert_eq!(arr.to_string(), String::from_utf8(output.to_vec()).unwrap());
    }

    #[test]
    fn test_dictionary() {
        let mut bytes: Vec<u8> = Vec::new();

        // test empty
        assert_eq!(PdfObject::Dictionary(HashMap::new()).render(&mut bytes).ok(), Some(6));
        assert_eq!(PdfObject::Dictionary(HashMap::new()).to_string(), "<<  >>");

        // test full
        let dict = PdfObject::Dictionary(hashmap!{
            "Bool".to_string() => PdfObject::from(true),
            "Number".to_string() => PdfObject::from(123 as usize),
            "String".to_string() => PdfObject::from("This is a string"),
            "HexString".to_string() => PdfObject::HexString("4E6F".to_string()),
            "Array".to_string() => PdfObject::Array(vec![PdfObject::from(1 as usize), PdfObject::from(2 as usize), PdfObject::from(3 as usize)]),
            "Dictionary".to_string() => PdfObject::Dictionary(hashmap!["a".to_string() => PdfObject::from(1 as usize), "b".to_string() => PdfObject::from(2 as usize), "c".to_string() => PdfObject::from(3 as usize)]),
            "Null".to_string() => PdfObject::Null,
        });
        let output = b"<< /Array [1 2 3] /Bool true /Dictionary << /a 1 /b 2 /c 3 >> /HexString <4E6F> /Null null /Number 123 /String (This is a string) >>";
        assert_eq!(dict.to_string(), String::from_utf8(output.to_vec()).unwrap());
    }

}
