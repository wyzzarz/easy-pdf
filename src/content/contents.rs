// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use rand::Rng;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use super::{Content, ContentId, Layout};

/// Holds an instance of all contents.
pub(crate) static CONTENTS: LazyLock<Mutex<Contents>> = LazyLock::new(|| Mutex::new(Contents::new()));

/// Holds content objects.
#[derive(Debug, Clone, PartialEq)]
pub struct Contents {
    contents: HashMap<ContentId, Content>,
}

impl Contents {

    /// Creates a new instance.
    fn new() -> Self {
        Self {
            contents: HashMap::new(),
        }
    }

    /// Generates a new content id.
    fn new_content_id(&self) -> ContentId {
        // start a new number generator
        let mut rng = rand::rng();

        // return a new id
        loop {
            let new_id: ContentId = rng.random();
            if !self.contents.contains_key(&new_id) {
                return new_id;
            }
        }
    }

    /// Gets content by id.
    fn get_content(&self, id: ContentId) -> Option<Content> {
        self.contents
            .get(&id)
            .map(|content| content.clone())
    }

    // Inseerts the content.
    fn insert_content(&mut self, content: Content) -> Option<Content> {
        let id = content.get_id();
        self.contents.insert(id, content)
    }

}

/// Generates a new content id.
pub fn new_content_id() -> ContentId {
    CONTENTS.lock().unwrap().new_content_id()
}

/// Gets content by id.
pub fn get_content(id: ContentId) -> Option<Content> {
    CONTENTS.lock().unwrap().get_content(id)
}

/// Inserts the content.
pub fn insert_content(content: Content) -> Option<Content> {
    CONTENTS.lock().unwrap().insert_content(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::Frame;
    use crate::geometry::RECT0;

    #[test]
    fn test_new_content_id() {
        let contents = Contents { contents: HashMap::new() };
        assert_ne!(contents.new_content_id(), contents.new_content_id());
    }

    #[test]
    fn test_insert_and_get_content() {
        let content_id = 123;
        let frame = Frame::new(content_id, RECT0);
        let content = Content::Frame(frame);
        assert!(insert_content(content.clone()).is_none());
        let content1 = get_content(content_id).unwrap();
        assert_eq!(content, content1);
    }

}
