// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use rand::Rng;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use super::{Object, ObjectId};
use super::objects::Objects;

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
        let new_doc = Object::Document { id: ObjectId::from((0, 0)), document_id: new_id };
        let mut objects = Objects::new();
        objects.insert(new_doc);
        self.documents.insert(new_id, objects);
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
    fn test_objects() {
        let mut documents = Documents::new();
        let doc_id = documents.register_document();
        let objects = documents.get(doc_id).unwrap();
        assert_eq!(objects.len(), 1);
        let new_id = objects.new_object_id();
        let new_object = Object::Catalog { id: new_id, pages: None };
        let mut_objects = documents.get_mut(doc_id).unwrap();
        mut_objects.insert(new_object);
        assert_eq!(mut_objects.len(), 2);
        assert_eq!(objects.len(), 1);  // objects is immutable
        assert_eq!(documents.get(doc_id).unwrap().len(), 2);
    }

    #[test]
    fn test_mutate() {
        // start with a new document
        let doc_id = register_document();

        // test objects
        assert_eq!(get(doc_id).unwrap().len(), 1);

        // insert a new object
        let res = mutate(doc_id, |objects| {
            let new_id = objects.new_object_id();
            let new_object = Object::Catalog { id: new_id, pages: None };
            objects.insert(new_object)
        }).unwrap();
        assert_eq!(res, None);

        // test objects
        assert_eq!(get(doc_id).unwrap().len(), 2);
    }

}
