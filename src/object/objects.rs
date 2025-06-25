// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashMap;
use super::{
    Object, IndirectObject, ObjectType, object_id::ObjectId,
    Pages,
};

/// Holds all indirect (referenced) objects to be included in a pdf document.
#[derive(Debug, Clone, PartialEq)]
pub struct Objects {
    max_id: ObjectId,
    objects: HashMap<ObjectId, Object>,
}

impl Objects {

    /// Creates a new instance.
    /// 
    /// Includes an initial document object.
    pub fn new() -> Self {
        Self {
            max_id: ObjectId::new(0, 0),
            objects: HashMap::new(),
        }
    }

    /// Number of objects.
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    /// All keys.
    pub fn keys(&self) -> impl Iterator<Item = &ObjectId> {
        self.objects.keys()
    }

    /// All values iterator.
    pub fn values(&self) -> impl Iterator<Item = &Object> {
        self.objects.values()
    }

    /// All key-value pairs iterator.
    pub fn iter(&self) -> impl Iterator<Item = (&ObjectId, &Object)> {
        self.objects.iter()
    }

    /// Gets the next available object id.
    pub fn new_object_id(&self) -> ObjectId {
        self.max_id.next()
    }

    /// Gets an object by its id.
    pub fn get(&self, id: &ObjectId) -> Option<&Object> {
        self.objects.get(id)
    }

    /// Gets all objects by type.
    pub fn by_type(&self, object_type: ObjectType) -> Vec<Object> {
        self.objects.values().filter(|object| object.get_type() == object_type).cloned().collect()
    }

    /// Gets the `Pages` page tree.
    pub fn page_tree(&self) -> Option<Pages> {
        self.by_type(ObjectType::Pages)
            .first()
            .and_then(|object| Pages::try_from(object.clone()).ok())
    }

    /// Inserts a new object.
    /// 
    /// If the object exists, it is replaced and the old object returned.
    pub fn insert(&mut self, object: Object) -> Option<Object> {
        let id = object.get_id();
        if id > self.max_id { self.max_id = id; }
        self.objects.insert(object.get_id(), object)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::Catalog;

    #[test]
    fn test_objects() {
        let mut objects = Objects::new();
   
        // test object id
        let new_id = objects.new_object_id();
        assert_eq!(new_id, ObjectId::new(1, 0));

        // test object (Document)
        let object = Object::new(new_id);
        objects.insert(object.clone());
        assert_eq!(objects.len(), 1);
        assert_eq!(objects.get(&ObjectId::from((0, 0))), Some(&object));

        // test catalog
        let new_id = objects.new_object_id();
        let catalog = Catalog::new(new_id);
        let object = Object::Catalog(catalog);
        objects.insert(object.clone());
        assert_eq!(objects.len(), 2);
        assert_eq!(objects.get(&new_id), Some(&object));

        // test next id
        assert_eq!(objects.new_object_id(), ObjectId::new(2, 0));
        assert_eq!(objects.get(&objects.new_object_id()), None);

        // test by type
        assert_eq!(objects.by_type(ObjectType::Document).len(), 1);
        assert_eq!(objects.by_type(ObjectType::Catalog).len(), 1);
    }

}
