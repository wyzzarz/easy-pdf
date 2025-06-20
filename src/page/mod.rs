// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod page;
pub mod pages;

pub use page::Page;
pub use pages::Pages;

use std::collections::HashMap;
use crate::geometry::{PaperSize, Rect};
use crate::pdf_object::PdfObject;

/// Inherited page attributes that can be applied to `Page` and `Pages`.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct InheritedPageAttributes {
    /// Physical paper size.
    media_box: Option<PaperSize>,
}

impl InheritedPageAttributes {

    /// Gets the media box.
    pub fn media_box(&self) -> Option<PaperSize> {
        self.media_box.clone()
    }

    /// Sets the media box.
    pub fn set_media_box(&mut self, media_box: Option<PaperSize>) {
        self.media_box = media_box;
    }

    /// Adds inherited properties to the dict.
    pub fn extend(&self, dict: &mut HashMap<String, PdfObject>) -> Result<(), Box<dyn std::error::Error>> {
        // add media box
        self.media_box.clone().and_then(|media_box| 
            dict.insert(
                "MediaBox".to_string(), 
                PdfObject::from(Rect::from(media_box)))
        );

        Ok(())
    }

}
