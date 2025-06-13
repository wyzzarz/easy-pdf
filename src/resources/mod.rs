// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

/*!
Access to embedded resources.
 */

use rust_embed::{Embed, EmbeddedFile};
use tempfile::{NamedTempFile, TempPath};

#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/resources"]
#[exclude = "**/.DS_Store"]
#[exclude = "**/*.log"]
struct _Resources;

/// Gets an embedded file.
pub fn get_resource(name: &str) -> Option<EmbeddedFile> {
    _Resources::get(name)
}

/// Gets data for an embedded file.
pub fn get_resource_data(name: &str) -> Option<Vec<u8>> {
    get_resource(name).map(|f| f.data.into_owned())
}

/// Gets the contents of the embedded file as a string.
pub fn get_resource_string(name: &str) -> Option<String> {
    get_resource_data(name).map(|d| String::from_utf8(d).unwrap())
}

/// Gets temporary path to embedded file.
pub fn get_resource_path(name: &str) -> Option<TempPath> {
    if let Some(data) = get_resource_data(name) {
        if let Ok(path) = NamedTempFile::new() {
            std::fs::write(path.path(), data).unwrap();
            return Some(path.into_temp_path());
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_resource() {
        let resource = get_resource(".keep");
        assert!(resource.is_some());
    }

    #[test]
    fn test_get_resource_data() {
        let data = get_resource_data(".keep");
        assert_eq!(data.unwrap(), br##"{"message": "nothing to see here"}"##);
    }

    #[test]
    fn test_get_resource_string() {
        let string = get_resource_string(".keep");
        assert_eq!(string.unwrap(), r##"{"message": "nothing to see here"}"##);
    }

    #[test]
    fn test_get_resource_path() {
        let path = get_resource_path(".keep");
        assert!(path.is_some());
        let path = path.unwrap();
        assert!(&path.exists());
        let contents = std::fs::read_to_string(&path).unwrap();
        assert_eq!(contents, r##"{"message": "nothing to see here"}"##);
    }
        
}
