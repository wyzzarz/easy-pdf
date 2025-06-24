// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use rand::Rng;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use super::{Content, ContentId};

/// Holds an instance of all contents.
pub static CONTENTS: LazyLock<Mutex<Contents>> = LazyLock::new(|| Mutex::new(Contents::new()));

/// Holds content objects.
#[derive(Debug, Clone, PartialEq)]
pub struct Contents {
    contents: HashMap<ContentId, Content>,
}

impl Contents {

    /// Creates a new instance.
    pub fn new() -> Self {
        Self {
            contents: HashMap::new(),
        }
    }

    /// Generates a new content id.
    pub fn new_content_id(&self) -> ContentId {
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

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_content_id() {
        let contents = Contents { contents: HashMap::new() };
        assert_ne!(contents.new_content_id(), contents.new_content_id());
    }

}
