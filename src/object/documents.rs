// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use rand::Rng;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use super::{IndirectObject, Object, ObjectId, ObjectType};
use super::objects::Objects;
use crate::catalog::Catalog;
use crate::document::Document;
use crate::pages::Pages;

/// Document identifier.
pub type DocumentId = u16;

/// Holds an instance of all documents.
pub static DOCUMENTS: LazyLock<Mutex<Documents>> = LazyLock::new(|| Mutex::new(Documents::new()));

#[derive(Debug, Clone, PartialEq)]
pub struct Documents {
    documents: HashMap<DocumentId, Objects>,
}

impl Documents {

    /// Creates a new instance.
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }

    /// Generates a new document id.
    fn new_document_id(&self) -> DocumentId {
        // get the existing keys
        let ids = self.documents.keys().cloned().collect::<Vec<DocumentId>>();

        // start a new number generator
        let mut rng = rand::rng();

        // return a new id
        loop {
            let new_id: DocumentId = rng.random();
            if !ids.contains(&new_id) {
                return new_id;
            }
        }
    }

    /// Registers a new document.
    fn register_document(&mut self) -> DocumentId {
        let new_id = self.new_document_id();
        let new_doc = Object::Document(Document::new_with_id(new_id));

        // create new objects array for the new document
        let mut objects = Objects::new();
        
        // add this document as the first object
        objects.insert(new_doc);

        // add page tree for this document
        let pages_id = objects.new_object_id();
        let pages = Pages::new(pages_id);
        objects.insert(Object::Pages(pages.clone()));

        // add a catalog for this document
        let id = objects.new_object_id();
        let mut catalog = Catalog::new(id);
        catalog.set_pages(Some(pages_id));
        objects.insert(Object::Catalog(catalog.clone()));

        // register the new document and its objects
        self.documents.insert(new_id, objects);

        // return the new document id
        new_id
    }

    /// Gets document objects.
    fn get(&self, document_id: DocumentId) -> Option<Objects> {
        self.documents.get(&document_id).cloned()
    }

    /// Gets mutable access to document objects.
    fn get_mut(&mut self, document_id: DocumentId) -> Option<&mut Objects> {
        self.documents.get_mut(&document_id)
    }

}

/// Registers a new document and returns the new document id.
pub fn register_document() -> DocumentId {
    DOCUMENTS.lock().unwrap().register_document()
}

/// Gets the document.
pub fn get_document(document_id: DocumentId) -> Option<Object> {
    DOCUMENTS.lock().unwrap().get(document_id)
        .and_then(|objects| objects.by_type(ObjectType::Document).first().cloned())
}

/// Gets document objects.
pub fn get(document_id: DocumentId) -> Option<Objects> {
    DOCUMENTS.lock().unwrap().get(document_id)
}

/// Gets an object from the document for the specified object id.
pub fn get_object(document_id: DocumentId, object_id: &ObjectId) -> Option<Object> {
    if let Some(objects) = DOCUMENTS.lock().unwrap().get(document_id) {
        objects.get(object_id).cloned()
    } else {
        None
    }
}

/// Accesses mutable document objects using a closure.
/// 
/// Returns a result wrapped around an optional return value.
pub fn mutate<F, R>(
    document_id: DocumentId,
    f: F,
) -> Result<Option<R>, Box<dyn std::error::Error>>
where
    F: FnOnce(&mut Objects) -> Option<R>,
{
    let mut docs_guard = DOCUMENTS.lock().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    Ok(docs_guard.get_mut(document_id).and_then(f))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::Catalog;
    use crate::object::IndirectObject;

    #[test]
    fn test_new_document_id() {
        let documents = Documents::new();
        assert!(documents.new_document_id() > 0);
    }

    #[test]
    fn test_register_document() {
        let mut documents = Documents::new();
        assert_eq!(documents.documents.len(), 0);
        assert!(documents.register_document() > 0);
        assert_eq!(documents.documents.len(), 1);
    }


    #[test]
    fn test_get_document() {
        let doc_id = register_document();
        let document = Document::try_from(get_document(doc_id).unwrap()).unwrap();
        assert_eq!(document.document_id(), doc_id);
    }

    #[test]
    fn test_objects() {
        let mut documents = Documents::new();
        let doc_id = documents.register_document();
        let objects = documents.get(doc_id).unwrap();
        assert_eq!(objects.len(), 3);
        let new_id = objects.new_object_id();
        let new_catalog = Catalog::new(new_id);
        let new_object = Object::Catalog(new_catalog);
        let mut_objects = documents.get_mut(doc_id).unwrap();
        mut_objects.insert(new_object);
        assert_eq!(mut_objects.len(), 4);
        assert_eq!(objects.len(), 3);  // objects is immutable
        assert_eq!(documents.get(doc_id).unwrap().len(), 4);
    }

    #[test]
    fn test_mutate() {
        // start with a new document
        let doc_id = register_document();

        // test objects (includes document and catalog)
        assert_eq!(get(doc_id).unwrap().len(), 3);

        // insert a new object
        let res = mutate(doc_id, |objects| {
            let new_id = objects.new_object_id();
            let new_catalog = Catalog::new(new_id);
            let new_object = Object::Catalog(new_catalog);
            objects.insert(new_object)
        }).unwrap();
        assert_eq!(res, None);

        // test objects
        assert_eq!(get(doc_id).unwrap().len(), 4);
    }

}
