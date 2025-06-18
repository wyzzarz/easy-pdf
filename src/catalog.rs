// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::cross_reference::CrossReferenceTable;
use crate::object::documents::{self, DocumentId};
use crate::object::{IndirectObject, Object, ObjectId, ObjectType};

/// Primary dictionary of all objects in the pdf document.  See PDF 1.7 - 7.7.2.
#[derive(Debug, Clone, PartialEq)]
pub struct Catalog {
    id: ObjectId,
    pages: Option<ObjectId>,
}

impl IndirectObject for Catalog {

    fn new(id: ObjectId) -> Self {
        Self {
            id,
            pages: None,
        }
    }

    fn get_id(&self) -> ObjectId {
        self.id
    }

    fn set_id(&mut self, id: ObjectId) {
        self.id = id;
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Catalog
    }

    fn render(&self, _doc_id: DocumentId, _parent: ObjectId, _writer: &mut dyn std::io::Write, _xref: &mut CrossReferenceTable) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

}

impl TryFrom<Object> for Catalog {

    type Error = String;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Catalog(catalog) => Ok(catalog),
            _ => Err("Object is not a catalog.".to_string()),
        }
    }

}

impl Catalog {

    /// Gets the catalog for the specified document.
    /// 
    /// Returns `None` if the document does not exist.
    pub fn get_catalog(document_id: DocumentId) -> Result<Option<Catalog>, Box<dyn std::error::Error>> {
        Ok(documents::get(document_id)
            .and_then(|objects| {
                objects
                    .values()
                    .find_map(|object| Catalog::try_from(object.clone()).ok())
                    .and_then(|catalog| Some(catalog))
        }))
    }

    /// Updates the catalog for the specified document.
    /// 
    /// Returns a copy of the old catalog.  Or `None` if the document does not exist.
    pub fn update_catalog(document_id: DocumentId, new_catalog: Catalog) -> Result<Option<Catalog>, Box<dyn std::error::Error>> {
        if let Some(_) = Catalog::get_catalog(document_id)? {
            let old_object = documents::mutate(document_id, |objects| {
                objects.insert(Object::Catalog(new_catalog))
            });
            match old_object {
                Ok(Some(object)) =>
                    Catalog::try_from(object)
                        .map(Some)
                        .map_err(|e| e.into()),
                Ok(None) => Ok(None),
                Err(e) => Err(e),
            }
        } else {
            Ok(None)
        }
    }

    /// Gets the object id for the pages tree.
    pub fn pages(&self) -> Option<ObjectId> {
        self.pages
    }

    /// Sets the object id for the pages tree.
    pub fn set_pages(&mut self, pages: Option<ObjectId>) {
        self.pages = pages;
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::Document;

    #[test]
    fn test_from_object() {
        let id = ObjectId::from((1, 2));
        let catalog = Catalog::new(id);
        let object = Object::Catalog(catalog.clone());
        let catalog1 = Catalog::try_from(object.clone()).unwrap();
        assert_eq!(catalog, catalog1);

        let object2 = Object::Document(Document::new_with_id(123));
        let catalog2 = Catalog::try_from(object2.clone());
        assert!(catalog2.is_err());
    }

    #[test]
    fn test_get_catalog() {
        // test no document
        let document_id = 123;
        let catalog = Catalog::get_catalog(document_id);
        assert!(catalog.is_ok());
        assert!(catalog.unwrap().is_none());

        // test with document
        let document_id = documents::register_document();
        let catalog = Catalog::get_catalog(document_id);
        assert!(catalog.as_ref().ok().is_some());

        // test page tree
        let catalog = catalog.unwrap().unwrap();
        assert!(catalog.pages().is_some());
        let pages_id = catalog.pages().unwrap();
        let pages = documents::get_object(document_id, &pages_id);
        assert_eq!(pages.unwrap().get_type(), ObjectType::Pages);
    }

    #[test]
    fn test_update_catalog() {
        let document_id = documents::register_document();
        let mut catalog = Catalog::get_catalog(document_id).unwrap().unwrap();
        let pages_id = documents::get(document_id).unwrap().new_object_id();
        catalog.set_pages(Some(pages_id));
        let _ = Catalog::update_catalog(document_id, catalog);
        let new_catalog = Catalog::get_catalog(document_id).unwrap().unwrap();
        assert_eq!(new_catalog.pages(), Some(pages_id));
    }

}
