// SPDX-FileCopyrightText: 2025 Warner Zee <warner@zoynk.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

/// An indirect object reference in the cross-reference table.
#[derive(Debug, Clone, PartialEq)]
pub struct CrossReferenceTableEntry {
    /// Byte offset within the body of the file.
    pub offset: usize,
    /// Generation number for the indirect object.
    pub generation: u16,
    /// Whether the indirect object is currently in use or deleted (free).
    pub in_use: bool,
}

/// Cross-Reference Table lists all indirect objects in the pdf document.
/// 
/// Each object is listed by its byte offset within the body of the file.
#[derive(Debug, Clone, PartialEq)]
pub struct CrossReferenceTable {
    /// Total number of bytes written.
    total_bytes: usize,
    /// Table of indirect object references.
    entries: Vec<CrossReferenceTableEntry>,
}

impl CrossReferenceTable {

    pub fn new() -> Self {
        Self {
            total_bytes: 0,
            entries: vec![
                CrossReferenceTableEntry {
                    offset: 0,
                    generation: 65535,
                    in_use: false,
                }
            ],
        }
    }

    /// Total number of bytes written.
    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    /// Adds bytes to the offset.
    pub fn add_bytes(&mut self, bytes_added: usize) {
        self.total_bytes += bytes_added;
    }

    /// Number of entries.
    pub fn num_entries(&self) -> usize {
        self.entries.len()
    }

    /// All entries.
    pub fn entries(&self) -> impl Iterator<Item = &CrossReferenceTableEntry> {
        self.entries.iter()
    }

    /// Adds an indirect object entry.
    pub fn add_entry(&mut self, generation: u16, in_use: bool) {
        self.entries.push(CrossReferenceTableEntry {
            offset: self.total_bytes + 1,
            generation: generation,
            in_use: in_use,
        });
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_reference_table() {
        // setup cross reference table
        let mut bytes: Vec<u8> = b"%PDF-1.7\n".to_vec();
        let mut table = CrossReferenceTable::new();
        table.add_bytes(bytes.len());
        assert_eq!(table.num_entries(), 1);
        assert_eq!(table.total_bytes, (bytes.len()));
        assert_eq!(table.entries.first().unwrap(), &CrossReferenceTableEntry { offset: 0, generation: 65535, in_use: false });
    
        // add entry
        let entry = b"1234567890\n";
        table.add_entry(0, true);
        assert_eq!(table.num_entries(), 2);
        bytes.extend_from_slice(entry);
        table.add_bytes(entry.len());
        assert_eq!(table.entries.get(1).unwrap(), &CrossReferenceTableEntry { offset: 10, generation: 0, in_use: true });

        // add another entry
        table.add_entry(0, true);
        assert_eq!(table.num_entries(), 3);
        bytes.extend_from_slice(entry);
        table.add_bytes(entry.len());
        assert_eq!(table.entries.get(2).unwrap(), &CrossReferenceTableEntry { offset: 21, generation: 0, in_use: true });
    }

}
