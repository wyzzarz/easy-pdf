// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod documents;
pub mod object_id;
pub mod objects;

use std::fmt;
use std::str::FromStr;
pub use documents::DocumentId;
pub use object_id::ObjectId;

/// Object types.
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    Document,
    Catalog,
    Pages,
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectType::Document => write!(f, "Document"),
            ObjectType::Catalog => write!(f, "Catalog"),
            ObjectType::Pages => write!(f, "Pages"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseObjectTypeError;

impl FromStr for ObjectType {
    type Err = ParseObjectTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Document" => Ok(ObjectType::Document),
            "Catalog" => Ok(ObjectType::Catalog),
            "Pages" => Ok(ObjectType::Pages),
            _ => Err(ParseObjectTypeError),
        }
    }
}

/// Objects that are labeled and referenced in a PDF file.
/// 
/// See PDF 1.7 - 7.3.10.
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    /// The pdf document.
    Document { id: ObjectId, document_id: DocumentId },
    /// Primary dictionary of all objects in the pdf document.  See PDF 1.7 - 7.7.2.
    Catalog { id: ObjectId, pages: Option<ObjectId> },
    /// Collection of page(s) within the pages tree of the pdf document.  See PDF 1.7 - 7.7.3. 
    Pages { id: ObjectId, kids: Vec<ObjectId> },
}

/// A trait shared by all indirect (referenced) objects.
pub trait IndirectObject {
    /// Creates a new indirect object.
    fn new(id: ObjectId) -> Self;
    /// Gets the object's id.
    fn get_id(&self) -> ObjectId;
    /// Sets the object's id.
    fn set_id(&mut self, id: ObjectId);
    /// Gets the object's type.
    fn get_type(&self) -> ObjectType;
}

impl IndirectObject for Object {

    /// For the `Object` enum, this method creates a default `Object::Document` variant
    /// with the given `id`.
    /// 
    /// However `Object` variants should be created directly (e.g., `Object::Catalog { ... }`).
    fn new(_id: ObjectId) -> Self {
        // document is not used as a PDF object and is cached at 0
        Object::Document { id: ObjectId::new(0, 0), document_id: 0 }
    }

    fn get_id(&self) -> ObjectId {
        match self {
            Object::Document { id, .. } => *id,
            Object::Catalog { id, .. } => *id,
            Object::Pages { id, .. } => *id,
        }
    }

    fn set_id(&mut self, value: ObjectId) {
        match self {
            Object::Document { id, .. } => *id = value,
            Object::Catalog { id, .. } => *id = value,
            Object::Pages { id, .. } => *id = value,
        }
    }

    fn get_type(&self) -> ObjectType {
        match self {
            Object::Document { .. } => ObjectType::Document,
            Object::Catalog { .. } => ObjectType::Catalog,
            Object::Pages { .. } => ObjectType::Pages,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object() {
        let object_id = ObjectId::new(1, 2);

        // test default object (document)
        let object = Object::new(object_id);
        assert_eq!(object.get_id(), ObjectId::new(0, 0));
        assert_eq!(object.get_type(), ObjectType::Document);

        // test catalog
        let object = Object::Catalog { id: object_id, pages: None };
        assert_eq!(object.get_id(), object_id);
        assert_eq!(object.get_type(), ObjectType::Catalog);

        // test pages
        let object = Object::Pages { id: object_id, kids: vec![] };
        assert_eq!(object.get_id(), object_id);
        assert_eq!(object.get_type(), ObjectType::Pages);
    }

}
