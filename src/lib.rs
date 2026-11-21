// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-scanner-backend: scanner backend component model.
//!
//! Real implementation: scanners, events, validation, deterministic storage.
//! Replaces the placeholder stub.

/// SANE-style scanner management module.
pub mod sigmascanner;

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scanner {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScannerBackendError {
    EmptyScannerName,
    DuplicateScanner(String),
    EmptyEventSubject,
}

impl fmt::Display for ScannerBackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyScannerName => write!(f, "scanner name must not be empty"),
            Self::DuplicateScanner(id) => write!(f, "duplicate scanner {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ScannerBackendError {}

pub struct ScannerBackend {
    pub name: String,
    scanners: BTreeMap<String, Scanner>,
    events: BTreeMap<String, Event>,
}

impl ScannerBackend {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            scanners: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_scanner(&mut self, item: Scanner) -> Result<(), ScannerBackendError> {
        if item.name.trim().is_empty() {
            return Err(ScannerBackendError::EmptyScannerName);
        }
        let id = item.id.clone();
        if self.scanners.contains_key(&id) {
            return Err(ScannerBackendError::DuplicateScanner(id.clone()));
        }
        self.scanners.insert(id, item);
        Ok(())
    }

    pub fn scanner_count(&self) -> usize {
        self.scanners.len()
    }

    pub fn get_scanner(&self, id: &str) -> Option<&Scanner> {
        self.scanners.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ScannerBackendError> {
        if event.subject.trim().is_empty() {
            return Err(ScannerBackendError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_scanner(id: &str, name: &str) -> Scanner {
        Scanner {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ScannerBackend::new("ScannerBackend");
        assert_eq!(m.name, "ScannerBackend");
        assert_eq!(m.scanner_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_scanner() {
        let mut m = ScannerBackend::new("ScannerBackend");
        m.add_scanner(sample_scanner("a1", "Alpha")).unwrap();
        assert_eq!(m.scanner_count(), 1);
        assert!(m.get_scanner("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ScannerBackend::new("ScannerBackend");
        assert!(m.add_scanner(sample_scanner("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ScannerBackend::new("ScannerBackend");
        m.add_scanner(sample_scanner("x", "One")).unwrap();
        assert!(m.add_scanner(sample_scanner("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ScannerBackend::new("ScannerBackend");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
