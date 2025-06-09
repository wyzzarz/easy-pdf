// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

/// PDF indirect object identifier.
/// 
/// See PDF 1.7 - 7.3.10
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectId {
    pub object_number: u32,
    pub generation_number: u16,
}

impl ToString for ObjectId {

    fn to_string(&self) -> String {
        format!("{} {} obj", self.object_number, self.generation_number)
    }

}

impl From<ObjectId> for (u32, u16) {

    /// Converts an `ObjectId` into a tuple `(object_number, generation_number)`.
    fn from(val: ObjectId) -> Self {
        (val.object_number, val.generation_number)
    }

}

impl From<(u32, u16)> for ObjectId {

    /// Converts a tuple `(object_number, generation_number)` into an `ObjectId`. {
    fn from(val: (u32, u16)) -> Self {
        ObjectId { 
            object_number: val.0, 
            generation_number: val.1
        }
    }

}

impl ObjectId {

    /// Creates a new object id.
    pub fn new(object_number: u32, generation_number: u16) -> Self {
        ObjectId { 
            object_number, 
            generation_number
        }
    }

    /// The next sequential object id.
    /// 
    /// `object_number` is increased by one.  `generation_number` is reset to zero.
    pub fn next(&self) -> Self {
        ObjectId { 
            object_number: self.object_number + 1, 
            generation_number: 0
        }
    }

    /// Outputs an indirect object reference for use in a PDF document.
    pub fn to_string_ref(&self) -> String {
        format!("{} {} R", self.object_number, self.generation_number)
    }

    /// Outputs an indirect object reference within a PDF object stream.
    pub fn to_stream_ref(&self) -> String {
        format!("{} 0 R", self.object_number)
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_id() {
        let id1 = ObjectId::new(1, 0);
        let id2 = ObjectId::new(2, 0);
        let id3 = ObjectId::new(1, 1);
        let id1_clone = ObjectId::new(1, 0);

        // test equality
        assert_eq!(id1, id1_clone);
        assert_ne!(id1, id2);
        assert_ne!(id1, id3);

        // test ordering
        assert!(id1 < id2);
        assert!(id1 < id3);
        assert!(id2 > id1);
        assert!(id3 > id1);
        assert!(id2 > id3);
    }

    #[test]
    fn test_into_tuple() {
        // test into tuple
        let id = ObjectId::new(42, 7);
        let tuple: (u32, u16) = id.into();
        assert_eq!(tuple, (42, 7));

        // another test into tuple
        let id1 = ObjectId::new(100, 5);
        let tuple1 = <(u32, u16)>::from(id1);
        assert_eq!(tuple1, (100, 5));
    }

    #[test]
    fn test_from_tuple() {
        // test from tuple
        let tuple = (42, 7);
        let id: ObjectId = ObjectId::from(tuple);
        assert_eq!(id, ObjectId::new(42, 7));
    }

    #[test]
    fn test_next() {
        let id = ObjectId::new(1, 2);
        let next_id = id.next();
        assert_eq!(next_id, ObjectId::new(2, 0));
    }

    #[test]
    fn test_to_string() {
        assert_eq!(ObjectId::new(1, 2).to_string(), "1 2 obj");
    }

    #[test]
    fn test_to_string_ref() {
        assert_eq!(ObjectId::new(1, 2).to_string_ref(), "1 2 R")
    }

}
