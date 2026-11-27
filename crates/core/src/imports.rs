//! Calendar imports: list them and remove a wrong one again.
//!
//! Events from newer imports carry the file name. Older ones only share the
//! moment they were created (`created_at`, the same for one import run).

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::Event;

#[derive(Debug, Clone, PartialEq)]
pub struct ImportBatch {
    /// Stable key to delete this batch.
    pub key: String,
    /// File name, if known.
    pub name: Option<String>,
    /// When the first event of this batch was imported.
    pub imported_at: DateTime<Utc>,
    pub count: usize,
}

fn key_of(e: &Event) -> Option<String> {
    e.source_uid.as_ref()?;
    Some(match &e.source_name {
        Some(n) => format!("datei:{n}"),
        None => format!("zeit:{}", e.meta.created_at.timestamp()),
    })
}

/// All imports that still have events, newest first.
pub fn import_batches(events: &[Event]) -> Vec<ImportBatch> {
    let mut out: Vec<ImportBatch> = Vec::new();
    for e in events.iter().filter(|e| e.meta.deleted_at.is_none()) {
        let Some(key) = key_of(e) else { continue };
        match out.iter_mut().find(|b| b.key == key) {
            Some(b) => {
                b.count += 1;
                b.imported_at = b.imported_at.min(e.meta.created_at);
            }
            None => out.push(ImportBatch {
                key,
                name: e.source_name.clone(),
                imported_at: e.meta.created_at,
                count: 1,
            }),
        }
    }
    out.sort_by_key(|b| std::cmp::Reverse(b.imported_at));
    out
}

/// Soft-delete all events of one import (synced as deletion). Returns their ids for undo.
pub fn delete_import(events: &mut [Event], key: &str, now: DateTime<Utc>) -> Vec<Uuid> {
    let mut ids = Vec::new();
    for e in events.iter_mut().filter(|e| e.meta.deleted_at.is_none()) {
        if key_of(e).as_deref() == Some(key) {
            e.meta.deleted_at = Some(now);
            e.meta.updated_at = now;
            ids.push(e.meta.id);
        }
    }
    ids
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{merge_ics, IcsEvent};
    use chrono::{Duration, NaiveDate};

    fn ie(uid: &str) -> IcsEvent {
        IcsEvent {
            uid: uid.into(),
            title: uid.into(),
            date: NaiveDate::from_ymd_opt(2026, 12, 1).unwrap(),
            start: None,
            end: None,
            notes: String::new(),
            recurrence: None,
        }
    }

    #[test]
    fn lists_and_deletes_one_import() {
        let t1 = Utc::now();
        let t2 = t1 + Duration::minutes(5);
        let mut ev = vec![Event::new(
            "eigener Termin",
            NaiveDate::from_ymd_opt(2026, 12, 2).unwrap(),
            t1,
        )];
        merge_ics(
            &mut ev,
            vec![ie("a"), ie("b")],
            None,
            Some("arbeit.ics"),
            t1,
        );
        merge_ics(&mut ev, vec![ie("c")], None, Some("falsch.ics"), t2);
        let b = import_batches(&ev);
        assert_eq!(b.len(), 2);
        assert_eq!(b[0].name.as_deref(), Some("falsch.ics"));
        assert_eq!((b[1].name.as_deref(), b[1].count), (Some("arbeit.ics"), 2));

        let ids = delete_import(&mut ev, &b[0].key, t2);
        assert_eq!(ids.len(), 1);
        assert_eq!(import_batches(&ev).len(), 1);
        // own events stay
        assert!(ev[0].meta.deleted_at.is_none());
        // re-import brings them back
        merge_ics(&mut ev, vec![ie("c")], None, Some("falsch.ics"), t2);
        assert_eq!(import_batches(&ev).len(), 2);
    }

    #[test]
    fn old_imports_without_name_group_by_time() {
        let t1 = Utc::now();
        let mut ev = Vec::new();
        merge_ics(&mut ev, vec![ie("a"), ie("b")], None, None, t1);
        merge_ics(&mut ev, vec![ie("c")], None, None, t1 + Duration::hours(1));
        let b = import_batches(&ev);
        assert_eq!(b.len(), 2);
        assert_eq!(b[1].count, 2);
        assert_eq!(delete_import(&mut ev, &b[1].key, t1).len(), 2);
    }
}
