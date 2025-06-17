// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::object::documents::DocumentId;
use crate::object::{IndirectObject, Object, ObjectId, ObjectType};

/// The pdf document.
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    id: ObjectId,
    document_id: DocumentId,
}

impl IndirectObject for Document {

    fn new(id: ObjectId) -> Self {
        Self {
            id,
            document_id: 0,
        }
    }

    fn get_id(&self) -> ObjectId {
        self.id
    }

    fn set_id(&mut self, id: ObjectId) {
        self.id = id;
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Document
    }

    fn render(&self, _doc_id: DocumentId, _parent: ObjectId, _writer: &mut dyn std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

}

impl TryFrom<Object> for Document {

    type Error = String;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Document(document) => Ok(document),
            _ => Err("Object is not a document.".to_string()),
        }
    }

}

impl Document {

    /// Creates a new document.
    pub fn new_with_id(document_id: DocumentId) -> Self {
        Self {
            id: ObjectId::new(0, 0),
            document_id: document_id,
        }
    }

    /// Gets the document id.
    pub fn document_id(&self) -> DocumentId {
        self.document_id
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_object() {
        let id: DocumentId = 123;
        let document = Document::new_with_id(id);
        let object = Object::Document(document.clone());
        let document1 = Document::try_from(object.clone()).unwrap();
        assert_eq!(document, document1);
        assert_eq!(document1.id, ObjectId::new(0, 0));
        assert_eq!(document1.document_id, id);
    }

}
