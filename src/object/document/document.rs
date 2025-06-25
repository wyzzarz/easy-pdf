// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use maplit::hashmap;
use std::path::Path;
use super::{DocumentId, DocInfo};
use crate::object::{
    Catalog,
    CrossReferenceTable,
    IndirectObject, Object, ObjectId, ObjectType
};
use crate::helpers::write_all_count;
use crate::page::Pages;
use crate::pdf_object::PdfObject;

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

    /// The file structure of a PDF document includes:
    /// - One line header
    /// - Body of objects
    /// - Cross-reference table of indirect objects
    /// - Trailer providing the location of the cross-reference table and other special objects
    /// 
    /// See PDF 1.7 - 7.5.1
    fn render(&self, doc_id: DocumentId, _parent: ObjectId, writer: &mut dyn std::io::Write, xref: &mut CrossReferenceTable) -> Result<(), Box<dyn std::error::Error>> {
        // add header
        xref.add_bytes(write_all_count(writer, b"%PDF-1.7\n")?);

        // write document information
        let doc_info = DocInfo::get_doc_info(self.document_id())?
            .ok_or("Document information not found for document.")?;
        doc_info.render(doc_id, self.get_id(), writer, xref)?;
        
        // write catalog
        let catalog = Catalog::get_catalog(self.document_id())?
            .ok_or("Catalog not found for document.")?;
        catalog.render(doc_id, self.get_id(), writer, xref)?;
        
        // write pages tree
        let page_tree = Pages::get_page_tree(self.document_id())?
            .ok_or("Page tree not found for document.")?;
        page_tree.render(doc_id, self.get_id(), writer, xref)?;

        // write cross reference table
        let xref_offset = xref.render(writer)?;

        // add trailer
        write_all_count(writer, b"trailer\n")?;
        let trailer_dict = PdfObject::Dictionary(hashmap! {
            "Info".to_string() => PdfObject::Raw(doc_info.get_id().to_string_ref().as_bytes().to_vec()),
            "Size".to_string() => PdfObject::from(xref.num_entries()),
            "Root".to_string() => PdfObject::Raw(catalog.get_id().to_string_ref().as_bytes().to_vec()),
        });
        trailer_dict.render(writer)?;
        write_all_count(writer, b"\nstartxref\n")?;
        write_all_count(writer, format!("{}\n", xref_offset).as_bytes())?;
        write_all_count(writer, b"%%EOF\n")?;

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

    /// Writes pdf to the specified buffer.
    pub fn write(&self, writer: &mut dyn std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
        let mut xref = CrossReferenceTable::new();
        self.render(self.document_id, self.id, writer, &mut xref)
    }

    /// Saves pdf to the specified filepath.
    pub fn save(&self, filepath: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = std::fs::File::create(filepath)?;
        self.write(&mut file)
    }

}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use tempfile::NamedTempFile;
    use super::*;
    use crate::geometry::PaperSize;
    use crate::object::{Object, document::documents};
    use crate::page::{Page, Rotation};
    use crate::resources;

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

    #[test]
    fn test_render() {
        // get document
        let doc_id = documents::register_document();
        let doc = Document::try_from(documents::get_document(doc_id).unwrap()).unwrap();

        // remove dates from doc info
        assert!(DocInfo::get_doc_info(doc_id).unwrap()
            .and_then(|mut doc_info| {
                doc_info.set_creation_date(Some(Local.with_ymd_and_hms(2025, 6, 19, 18, 38, 58).unwrap()));
                doc_info.set_modification_date(Some(Local.with_ymd_and_hms(2025, 7, 19, 18, 38, 58).unwrap()));
                DocInfo::update_doc_info(doc_id, doc_info).ok()
        }).is_some());
        
        // add a page
        assert!(documents::mutate(doc_id, |objects| {
            // add a new page
            let new_id = objects.new_object_id();
            let mut page = Page::new(new_id);
            page.inherited.set_media_box(Some(PaperSize::Tabloid));
            page.inherited.set_rotation(Some(Rotation::R90));
            objects.insert(Object::Page(page.clone()));

            // add page to (parent) page_tree
            let mut page_tree = objects.page_tree().unwrap();
            assert!(page_tree.add_child(Object::Page(page)).is_ok());
            objects.insert(Object::Pages(page_tree));

            // nothing to return
            None::<Object>
        }).is_ok());

        // render document
        let mut data: Vec<u8> = Vec::new();
        let rc = doc.write(&mut data);
        assert!(rc.is_ok());
        assert_eq!(String::from_utf8(data).unwrap(), resources::get_resource_string("tests/document.pdf").unwrap());
    }

    #[test]
    fn test_save() {
        // get document
        let doc_id = documents::register_document();
        let doc = Document::try_from(documents::get_document(doc_id).unwrap()).unwrap();

        // save document
        let mut file = NamedTempFile::new().unwrap();
        if false {
            file.disable_cleanup(true);
            eprintln!("Access PDF: open -a Preview {}", file.path().display());
        }
        let path = file.path();
        assert!(doc.save(&path).is_ok());
        assert!(path.try_exists().unwrap_or(false));
    }

}
