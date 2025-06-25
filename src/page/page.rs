// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use maplit::hashmap;
use super::InheritedPageAttributes;
use crate::cross_reference::CrossReferenceTable;
use crate::object::DocumentId;
use crate::object::{IndirectObject, Object, ObjectId, ObjectType};
use crate::pdf_object::PdfObject;
use crate::helpers::write_all_count;

/// A page.  See PDF 1.7 - 7.7.3. 
#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    id: ObjectId,
    pub inherited: InheritedPageAttributes,
}

impl IndirectObject for Page {

    fn new(id: ObjectId) -> Self {
        Self {
            id: id,
            inherited: InheritedPageAttributes::default(),
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

    fn render(&self, _doc_id: DocumentId, parent: ObjectId, writer: &mut dyn std::io::Write, xref: &mut CrossReferenceTable) -> Result<(), Box<dyn std::error::Error>> {
        // add page to cross reference table
        xref.add_entry(self.get_id().generation_number, true);

        // write object id
        xref.add_bytes(write_all_count(writer, self.get_id().to_string().as_bytes())?);
        xref.add_bytes(write_all_count(writer, b"\n")?);

        // write dictionary
        let mut dict = hashmap! {
            "Type".to_string() => PdfObject::Name(self.get_type().to_string()),
            "Parent".to_string() => PdfObject::Raw(parent.to_string_ref().into()),
        };
        self.inherited.extend(&mut dict)?;
        let obj = PdfObject::Dictionary(dict);
        xref.add_bytes(obj.render(writer)?);

        // end object
        xref.add_bytes(write_all_count(writer, b"\nendobj\n")?);
        
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
    use crate::cross_reference::CrossReferenceTable;
    use crate::geometry::PaperSize;
    use crate::page::Rotation;

    #[test]
    fn test_from_object() {
        let id = ObjectId::new(1, 2);
        let page = Page::new(id);
        let object = Object::Page(page.clone());
        let page1 = Page::try_from(object.clone()).unwrap();
        assert_eq!(page, page1);
        assert_eq!(page1.id, id);
    }

    #[test]
    fn test_render() {
        let parent_id = ObjectId::new(3, 1);
        let id = ObjectId::new(10, 2);
        let mut page = Page::new(id);
        page.inherited.set_media_box(Some(PaperSize::Tabloid));
        page.inherited.set_rotation(Some(Rotation::R90));
        let mut writer: Vec<u8> = Vec::new();
        let mut xref = &mut CrossReferenceTable::new();
        assert!(page.render(0, parent_id, &mut writer, &mut xref).is_ok());
        let results = String::from_utf8(writer).unwrap();
        assert_eq!(results, r#"10 2 obj
<< /MediaBox [0 0 792 1224] /Parent 3 1 R /Rotate 90 /Type /Page >>
endobj
"#);
    }

}
