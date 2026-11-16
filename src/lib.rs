// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-omarchy-update: omarchy update component model.
//!
//! Real implementation: updates, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update {
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
pub enum OmarchyUpdateError {
    EmptyUpdateName,
    DuplicateUpdate(String),
    EmptyEventSubject,
}

impl fmt::Display for OmarchyUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyUpdateName => write!(f, "update name must not be empty"),
            Self::DuplicateUpdate(id) => write!(f, "duplicate update {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for OmarchyUpdateError {}

pub struct OmarchyUpdate {
    pub name: String,
    updates: BTreeMap<String, Update>,
    events: BTreeMap<String, Event>,
}

impl OmarchyUpdate {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            updates: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_update(&mut self, item: Update) -> Result<(), OmarchyUpdateError> {
        if item.name.trim().is_empty() {
            return Err(OmarchyUpdateError::EmptyUpdateName);
        }
        let id = item.id.clone();
        if self.updates.contains_key(&id) {
            return Err(OmarchyUpdateError::DuplicateUpdate(id.clone()));
        }
        self.updates.insert(id, item);
        Ok(())
    }

    pub fn update_count(&self) -> usize {
        self.updates.len()
    }

    pub fn get_update(&self, id: &str) -> Option<&Update> {
        self.updates.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), OmarchyUpdateError> {
        if event.subject.trim().is_empty() {
            return Err(OmarchyUpdateError::EmptyEventSubject);
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

    fn sample_update(id: &str, name: &str) -> Update {
        Update {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = OmarchyUpdate::new("OmarchyUpdate");
        assert_eq!(m.name, "OmarchyUpdate");
        assert_eq!(m.update_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_update() {
        let mut m = OmarchyUpdate::new("OmarchyUpdate");
        m.add_update(sample_update("a1", "Alpha")).unwrap();
        assert_eq!(m.update_count(), 1);
        assert!(m.get_update("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = OmarchyUpdate::new("OmarchyUpdate");
        assert!(m.add_update(sample_update("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = OmarchyUpdate::new("OmarchyUpdate");
        m.add_update(sample_update("x", "One")).unwrap();
        assert!(m.add_update(sample_update("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = OmarchyUpdate::new("OmarchyUpdate");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
