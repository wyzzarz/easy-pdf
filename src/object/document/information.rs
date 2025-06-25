// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use chrono::prelude::*;
use std::collections::HashMap;
use super::{DocumentId, documents};
use crate::cross_reference::CrossReferenceTable;
use crate::helpers::get_lib_name;
use crate::object::{IndirectObject, Object, ObjectId, ObjectType};
use crate::pdf_object::PdfObject;
use crate::helpers::write_all_count;

/// Primary dictionary of all objects in the pdf document.  See PDF 1.7 - 7.7.2.
#[derive(Debug, Clone, PartialEq)]
pub struct DocInfo {
    id: ObjectId,
    title: Option<String>,
    author: Option<String>,
    subject: Option<String>,
    keywords: Option<String>,
    creator: Option<String>,
    producer: Option<String>,
    creation_date: Option<DateTime<Local>>,
    modification_date: Option<DateTime<Local>>,
}

impl IndirectObject for DocInfo {

    fn new(id: ObjectId) -> Self {
        Self {
            id,
            title: None,
            author: None,
            subject: None,
            keywords: None,
            creator: Some(get_lib_name()),
            producer: Some(get_lib_name()),
            creation_date: Some(Local::now()),
            modification_date: Some(Local::now()),
        }
    }

    fn get_id(&self) -> ObjectId {
        self.id
    }

    fn set_id(&mut self, id: ObjectId) {
        self.id = id;
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::DocInfo
    }

    fn render(&self, _doc_id: DocumentId, _parent_id: ObjectId, writer: &mut dyn std::io::Write, xref: &mut CrossReferenceTable) -> Result<(), Box<dyn std::error::Error>> {
        // add catalog to cross reference table
        xref.add_entry(self.get_id().generation_number, true);

        // write object id
        xref.add_bytes(write_all_count(writer, self.get_id().to_string().as_bytes())?);
        xref.add_bytes(write_all_count(writer, b"\n")?);

        // write dictionary
        let mut dict: HashMap<String, PdfObject>  = HashMap::new();
        self.title.as_ref().and_then(|title| dict.insert("Title".to_string(), PdfObject::String(title.clone())));
        self.author.as_ref().and_then(|author| dict.insert("Author".to_string(), PdfObject::String(author.clone())));
        self.subject.as_ref().and_then(|subject| dict.insert("Subject".to_string(), PdfObject::String(subject.clone())));
        self.keywords.as_ref().and_then(|keywords| dict.insert("Keywords".to_string(), PdfObject::String(keywords.clone())));
        self.creator.as_ref().and_then(|creator| dict.insert("Creator".to_string(), PdfObject::String(creator.clone())));
        self.producer.as_ref().and_then(|producer| dict.insert("Producer".to_string(), PdfObject::String(producer.clone())));
        self.creation_date.as_ref().and_then(|creation_date| dict.insert("CreationDate".to_string(), PdfObject::from(creation_date.clone())));
        self.modification_date.as_ref().and_then(|modification_date| dict.insert("ModDate".to_string(), PdfObject::from(modification_date.clone())));
        xref.add_bytes(PdfObject::Dictionary(dict).render(writer)?);

        // end object
        xref.add_bytes(write_all_count(writer, b"\nendobj\n")?);

        Ok(())
    }

}

impl TryFrom<Object> for DocInfo {

    type Error = String;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::DocInfo(doc_info) => Ok(doc_info),
            _ => Err("Object is not a doc_info.".to_string()),
        }
    }

}

impl DocInfo {

    /// Gets the doc_info for the specified document.
    /// 
    /// Returns `None` if the document does not exist.
    pub fn get_doc_info(document_id: DocumentId) -> Result<Option<DocInfo>, Box<dyn std::error::Error>> {
        Ok(documents::get(document_id)
            .and_then(|objects| {
                objects
                    .values()
                    .find_map(|object| DocInfo::try_from(object.clone()).ok())
                    .and_then(|doc_info| Some(doc_info))
        }))
    }

    /// Updates the doc_info for the specified document.
    /// 
    /// Returns a copy of the old doc_info.  Or `None` if the document does not exist.
    pub fn update_doc_info(document_id: DocumentId, new_doc_info: DocInfo) -> Result<Option<DocInfo>, Box<dyn std::error::Error>> {
        if let Some(_) = DocInfo::get_doc_info(document_id)? {
            let old_object = documents::mutate(document_id, |objects| {
                objects.insert(Object::DocInfo(new_doc_info))
            });
            match old_object {
                Ok(Some(object)) =>
                    DocInfo::try_from(object)
                        .map(Some)
                        .map_err(|e| e.into()),
                Ok(None) => Ok(None),
                Err(e) => Err(e),
            }
        } else {
            Ok(None)
        }
    }

    /// Gets the title.
    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }

    /// Sets the title.
    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    /// Gets the author.
    pub fn author(&self) -> Option<String> {
        self.author.clone()
    }

    /// Sets the author.
    pub fn set_author(&mut self, author: Option<String>) {
        self.author = author;
    }

    /// Gets the subject.
    pub fn subject(&self) -> Option<String> {
        self.subject.clone()
    }

    /// Sets the subject.
    pub fn set_subject(&mut self, subject: Option<String>) {
        self.subject = subject;
    }

    /// Gets the keywords.
    pub fn keywords(&self) -> Option<String> {
        self.keywords.clone()
    }

    /// Sets the keywords.
    pub fn set_keywords(&mut self, keywords: Option<String>) {
        self.keywords = keywords;
    }

    /// Gets the creator.
    pub fn creator(&self) -> Option<String> {
        self.creator.clone()
    }

    /// Sets the creator.
    pub fn set_creator(&mut self, creator: Option<String>) {
        self.creator = creator;
    }

    /// Gets the producer.
    pub fn producer(&self) -> Option<String> {
        self.producer.clone()
    }

    /// Sets the producer.
    pub fn set_producer(&mut self, producer: Option<String>) {
        self.producer = producer;
    }

    /// Gets the creation_date.
    pub fn creation_date(&self) -> Option<DateTime<Local>> {
        self.creation_date.clone()
    }

    /// Sets the creation_date.
    pub fn set_creation_date(&mut self, creation_date: Option<DateTime<Local>>) {
        self.creation_date = creation_date;
    }

    /// Gets the modification_date.
    pub fn modification_date(&self) -> Option<DateTime<Local>> {
        self.modification_date.clone()
    }

    /// Sets the modification_date.
    pub fn set_modification_date(&mut self, modification_date: Option<DateTime<Local>>) {
        self.modification_date = modification_date;
    }

    /// Sets the modification date to now.
    pub fn set_modification_date_now(&mut self) {
        self.modification_date = Some(Local::now());
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::Document;

    #[test]
    fn test_from_object() {
        let id = ObjectId::from((1, 2));
        let doc_info = DocInfo::new(id);
        let object = Object::DocInfo(doc_info.clone());
        let doc_info1 = DocInfo::try_from(object.clone()).unwrap();
        assert_eq!(doc_info, doc_info1);

        let object2 = Object::Document(Document::new_with_id(123));
        let doc_info2 = DocInfo::try_from(object2.clone());
        assert!(doc_info2.is_err());
    }

    #[test]
    fn test_render() {
        // setup doc_info
        let id = ObjectId::from((1, 2));
        let mut doc_info = DocInfo::new(id);
        doc_info.set_author(Some("some author".to_string()));
        doc_info.set_title(Some("some title".to_string()));
        doc_info.set_subject(Some("some subject".to_string()));
        doc_info.set_keywords(Some("some keywords".to_string()));
        doc_info.set_creation_date(Some(Local.with_ymd_and_hms(2025, 6, 19, 18, 38, 58).unwrap()));
        doc_info.set_modification_date(Some(Local.with_ymd_and_hms(2025, 7, 19, 18, 38, 58).unwrap()));

        // test render
        let mut writer = Vec::new();
        let mut xref = CrossReferenceTable::new();
        doc_info.render(123, id, &mut writer, &mut xref).unwrap();
        assert_eq!(String::from_utf8(writer).unwrap(), format!("1 2 obj\n\
<< \
/Author (some author) \
/CreationDate (D:20250619183858-07'00) \
/Creator ({}) \
/Keywords (some keywords) \
/ModDate (D:20250719183858-07'00) \
/Producer ({}) \
/Subject (some subject) \
/Title (some title) \
>>\n\
endobj\n",
            get_lib_name(),
            get_lib_name(),
        ));
    }

    #[test]
    fn test_get_doc_info() {
        // test no document
        let document_id = 123;
        let doc_info = DocInfo::get_doc_info(document_id);
        assert!(doc_info.is_ok());
        assert!(doc_info.unwrap().is_none());

        // test with document
        let document_id = documents::register_document();
        let doc_info = DocInfo::get_doc_info(document_id);
        assert!(doc_info.as_ref().ok().is_some());
    }

    #[test]
    fn test_update_doc_info() {
        let document_id = documents::register_document();

        // test title
        let title = Some("some title".to_string());
        DocInfo::get_doc_info(document_id).unwrap()
            .and_then(|mut doc_info| {
                doc_info.set_title(title.clone());
                DocInfo::update_doc_info(document_id, doc_info).ok()
            });
        DocInfo::get_doc_info(document_id).unwrap()
            .and_then(|doc_info| {
                assert_eq!(doc_info.title(), title);
                Some(doc_info)
            });

        // test author
        let author = Some("some author".to_string());
        DocInfo::get_doc_info(document_id).unwrap()
            .and_then(|mut doc_info| {
                doc_info.set_author(author.clone());
                DocInfo::update_doc_info(document_id, doc_info).ok()
            });
        DocInfo::get_doc_info(document_id).unwrap()
            .and_then(|doc_info| {
                assert_eq!(doc_info.author(), author);
                Some(doc_info)
            });

        // test subject
        let subject = Some("some subject".to_string());
        DocInfo::get_doc_info(document_id).unwrap()
            .and_then(|mut doc_info| {
                doc_info.set_subject(subject.clone());
                DocInfo::update_doc_info(document_id, doc_info).ok()
            });
        DocInfo::get_doc_info(document_id).unwrap()
            .and_then(|doc_info| {
                assert_eq!(doc_info.subject(), subject);
                Some(doc_info)
            });

        // test keywords
        let keywords = Some("some keywords".to_string());
        DocInfo::get_doc_info(document_id).unwrap()
            .and_then(|mut doc_info| {
                doc_info.set_keywords(keywords.clone());
                DocInfo::update_doc_info(document_id, doc_info).ok()
            });
        DocInfo::get_doc_info(document_id).unwrap()
            .and_then(|doc_info| {
                assert_eq!(doc_info.keywords(), keywords);
                Some(doc_info)
            });

        // test creator
        let creator = Some("some creator".to_string());
        DocInfo::get_doc_info(document_id).unwrap()
            .and_then(|mut doc_info| {
                doc_info.set_creator(creator.clone());
                DocInfo::update_doc_info(document_id, doc_info).ok()
            });
        DocInfo::get_doc_info(document_id).unwrap()
            .and_then(|doc_info| {
                assert_eq!(doc_info.creator(), creator);
                Some(doc_info)
            });

        // test producer
        let producer = Some("some producer".to_string());
        DocInfo::get_doc_info(document_id).unwrap()
            .and_then(|mut doc_info| {
                doc_info.set_producer(producer.clone());
                DocInfo::update_doc_info(document_id, doc_info).ok()
            });
        DocInfo::get_doc_info(document_id).unwrap()
            .and_then(|doc_info| {
                assert_eq!(doc_info.producer(), producer);
                Some(doc_info)
            });

        // test creation_date
        let creation_date = Some(Local::now());
        DocInfo::get_doc_info(document_id).unwrap()
            .and_then(|mut doc_info| {
                doc_info.set_creation_date(creation_date.clone());
                DocInfo::update_doc_info(document_id, doc_info).ok()
            });
        DocInfo::get_doc_info(document_id).unwrap()
            .and_then(|doc_info| {
                assert_eq!(doc_info.creation_date(), creation_date);
                Some(doc_info)
            });

        // test modification_date
        let modification_date = Some(Local::now());
        DocInfo::get_doc_info(document_id).unwrap()
            .and_then(|mut doc_info| {
                doc_info.set_modification_date(modification_date.clone());
                DocInfo::update_doc_info(document_id, doc_info).ok()
            });
        DocInfo::get_doc_info(document_id).unwrap()
            .and_then(|doc_info| {
                assert_eq!(doc_info.modification_date(), modification_date);
                Some(doc_info)
            });
    }

}
