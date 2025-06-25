// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod catalog;
pub mod cross_reference;
pub mod document;
pub mod object_id;
pub mod objects;

use std::fmt;
use std::str::FromStr;
pub use catalog::Catalog;
pub use cross_reference::CrossReferenceTable;
pub use document::{Document, DocumentId, DocInfo};
pub use object_id::ObjectId;
use crate::page::{Page, Pages};

/// Object types.
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    Document,
    DocInfo,
    Catalog,
    Pages,
    Page,
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectType::Document => write!(f, "Document"),
            ObjectType::DocInfo => write!(f, "Information"),
            ObjectType::Catalog => write!(f, "Catalog"),
            ObjectType::Pages => write!(f, "Pages"),
            ObjectType::Page => write!(f, "Page"),
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
            "DocInfo" => Ok(ObjectType::DocInfo),
            "Catalog" => Ok(ObjectType::Catalog),
            "Pages" => Ok(ObjectType::Pages),
            "Page" => Ok(ObjectType::Page),
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
    Document(Document),
    /// Information for the document.
    DocInfo(DocInfo),
    /// Primary dictionary of all objects in the pdf document.  See PDF 1.7 - 7.7.2.
    Catalog(Catalog),
    /// Collection of page(s) within the pages tree of the pdf document.  See PDF 1.7 - 7.7.3. 
    Pages(Pages),
    //// A page displayed in the pdf document.  See PDF 1.7 - 7.7.3.3
    Page(Page),
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
    /// Generates PDF output.
    fn render(&self, doc_id: DocumentId, parent_id: ObjectId, writer: &mut dyn std::io::Write, xref: &mut CrossReferenceTable) -> Result<(), Box<dyn std::error::Error>>;
}

impl IndirectObject for Object {

    /// For the `Object` enum, this method creates a default `Object::Document` variant
    /// with the given `id`.
    /// 
    /// However `Object` variants should be created directly (e.g., `Object::Catalog(new_catalog)`).
    fn new(_id: ObjectId) -> Self {
        // document is not used as a PDF object and is cached at 0
        Object::Document(Document::new(ObjectId::new(0, 0)))
    }

    fn get_id(&self) -> ObjectId {
        match self {
            Object::Document(document) => document.get_id(),
            Object::DocInfo(doc_info) => doc_info.get_id(),
            Object::Catalog(catalog) => catalog.get_id(),
            Object::Pages(pages) => pages.get_id(),
            Object::Page(page) => page.get_id(),
        }
    }

    fn set_id(&mut self, value: ObjectId) {
        match self {
            Object::Document(document) => document.set_id(value),
            Object::DocInfo(doc_info) => doc_info.set_id(value),
            Object::Catalog(catalog) => catalog.set_id(value),
            Object::Pages(pages) => pages.set_id(value),
            Object::Page(page) => page.set_id(value),
        }
    }

    fn get_type(&self) -> ObjectType {
        match self {
            Object::Document(_) => ObjectType::Document,
            Object::DocInfo(_) => ObjectType::DocInfo,
            Object::Catalog(_) => ObjectType::Catalog,
            Object::Pages(_) => ObjectType::Pages,
            Object::Page(_) => ObjectType::Page,
        }
    }

    fn render(&self, doc_id: DocumentId, parent_id: ObjectId, writer: &mut dyn std::io::Write, xref: &mut CrossReferenceTable) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Object::Document(document) => document.render(doc_id, parent_id, writer, xref),
            Object::DocInfo(doc_info) => doc_info.render(doc_id, parent_id, writer, xref),
            Object::Catalog(catalog) => catalog.render(doc_id, parent_id, writer, xref),
            Object::Pages(pages) => pages.render(doc_id, parent_id, writer, xref),
            Object::Page(page) => page.render(doc_id, parent_id, writer, xref),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object() {
        let object_id = ObjectId::new(1, 2);
        let new_object_id = ObjectId::new(3, 4);

        // test default object (document)
        let object = Object::new(object_id);
        assert_eq!(object.get_id(), ObjectId::new(0, 0));
        assert_eq!(object.get_type(), ObjectType::Document);

        // test catalog
        let catalog = Catalog::new(object_id);
        let object = Object::Catalog(catalog);
        assert_eq!(object.get_id(), object_id);
        assert_eq!(object.get_type(), ObjectType::Catalog);

        // test pages
        let pages = Pages::new(object_id);
        let mut object = Object::Pages(pages);
        assert_eq!(object.get_id(), object_id);
        assert_eq!(object.get_type(), ObjectType::Pages);
        object.set_id(new_object_id);
        assert_eq!(object.get_id(), new_object_id);

        // test page
        let page = Page::new(object_id);
        let mut object = Object::Page(page);
        assert_eq!(object.get_id(), object_id);
        assert_eq!(object.get_type(), ObjectType::Page);
        object.set_id(new_object_id);
        assert_eq!(object.get_id(), new_object_id);

        // test document information
        
    }

}
