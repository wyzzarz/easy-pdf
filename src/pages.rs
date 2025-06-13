// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::object::documents::{self, DocumentId};
use crate::object::{IndirectObject, Object, ObjectId, ObjectType};

/// Collection of page(s) within the pages tree of the pdf document.  See PDF 1.7 - 7.7.3. 
#[derive(Debug, Clone, PartialEq)]
pub struct Pages {
    id: ObjectId,
    kids: Vec<ObjectId>,
    count: usize,
}

impl IndirectObject for Pages {

    fn new(id: ObjectId) -> Self {
        Self {
            id,
            kids: Vec::new(),
            count: 0,
        }
    }

    fn get_id(&self) -> ObjectId {
        self.id
    }

    fn set_id(&mut self, id: ObjectId) {
        self.id = id;
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Pages
    }

}

impl TryFrom<Object> for Pages {

    type Error = String;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Pages(pages) => Ok(pages),
            _ => Err("Object is not a pages.".to_string()),
        }
    }

}

impl Pages {

    /// Gets the object ids of the children.
    /// 
    /// A child can be either another `Pages` or a `Page`.
    pub fn kids(&self) -> Vec<ObjectId> {
        self.kids.clone()
    }

    /// Adds a child.
    /// 
    /// A child must be either `Pages` or a `Page`.
    pub fn add_child(&mut self, child: Object) -> Result<(), Box<dyn std::error::Error>> {
        match child {
            Object::Pages(pages) => {
                self.kids.push(pages.get_id());
                self.count += 1;
                Ok(())
            },
            Object::Page(page) => {
                self.kids.push(page.get_id());
                Ok(())
            },
            _ => Err(format!("Child '{}' is not Pages or Page.", child.get_type()).into()),
        }
    }

    /// Gets the page tree for the specified document.
    /// 
    /// Returns `None` if the document does not exist or `Pages` does not exist for the document.
    pub fn get_page_tree(document_id: DocumentId) -> Result<Option<Pages>, Box<dyn std::error::Error>> {
        Ok(documents::get(document_id)
            .and_then(|objects| {
                objects.page_tree()
            }))
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::page::Page;

    #[test]
    fn test_from_object() {
        let id = ObjectId::new(1, 2);
        let pages = Pages::new(id);
        let object = Object::Pages(pages.clone());
        let pages1 = Pages::try_from(object.clone()).unwrap();
        assert_eq!(pages, pages1);
        assert_eq!(pages1.id, id);
    }

    #[test]
    fn test_add_child() {
        let id = ObjectId::new(1, 2);
        let mut pages = Pages::new(id);

        // test adding pages
        let id1 = ObjectId::new(3, 4);
        let pages1 = Object::Pages(Pages::new(id1));
        pages.add_child(pages1.clone()).unwrap();
        assert_eq!(pages.kids().len(), 1);
        assert_eq!(pages.kids()[0], id1);

        // test adding page
        let id2 = ObjectId::new(7, 8);
        pages.add_child(Object::Page(Page::new(id2))).unwrap();
        assert_eq!(pages.kids().len(), 2);
        assert_eq!(pages.kids()[1], id2);

        // test adding invalid
        assert!(pages.add_child(Object::new(ObjectId::new(5, 6))).is_err());
    }

}
