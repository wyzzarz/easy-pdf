// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::object::documents::DocumentId;
use crate::object::{IndirectObject, Object, ObjectId, ObjectType};

/// A page.  See PDF 1.7 - 7.7.3. 
#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    id: ObjectId,
}

impl IndirectObject for Page {

    fn new(id: ObjectId) -> Self {
        Self {
            id,
        }
    }

    fn get_id(&self) -> ObjectId {
        self.id
    }

    fn set_id(&mut self, id: ObjectId) {
        self.id = id;
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Page
    }

    fn render(&self, _doc_id: DocumentId, _parent: ObjectId, _writer: &mut dyn std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

}

impl TryFrom<Object> for Page {

    type Error = String;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Page(page) => Ok(page),
            _ => Err("Object is not a page.".to_string()),
        }
    }

}

impl Page {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_object() {
        let id = ObjectId::new(1, 2);
        let page = Page::new(id);
        let object = Object::Page(page.clone());
        let page1 = Page::try_from(object.clone()).unwrap();
        assert_eq!(page, page1);
        assert_eq!(page1.id, id);
    }

}
