//! Read calendar entries from an Outlook data file (`.pst`).
//!
//! Uses Microsoft's open `outlook-pst` reader. Start, end, all-day, location
//! and the repeat rule are Outlook "named" properties (PSETID_Appointment),
//! looked up through the file's name map; the standard start/end dates are the
//! fallback. The result goes through `merge_ics`, so a re-import updates
//! instead of duplicating.

use std::{collections::HashMap, io::Cursor, rc::Rc};

use checko_core::{Freq, IcsEvent, Recurrence};
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use outlook_pst::{
    ltp::prop_context::PropertyValue,
    messaging::store::{AnsiStore, UnicodeStore},
    messaging::{
        named_prop::{NamedPropertyGuid, NamedPropertyId},
        store::Store,
    },
    ndb::node_id::NodeId,
    AnsiPstFile, UnicodePstFile,
};

const PR_MESSAGE_CLASS: u16 = 0x001A;
const PR_SUBJECT: u16 = 0x0037;
const PR_START_DATE: u16 = 0x0060;
const PR_END_DATE: u16 = 0x0061;
const PR_BODY: u16 = 0x1000;
const PR_SEARCH_KEY: u16 = 0x300B;

/// PSETID_Appointment {00062002-0000-0000-C000-000000000046}: data1.
const PSETID_APPOINTMENT: u32 = 0x0006_2002;
const LID_LOCATION: u32 = 0x8208;
const LID_START: u32 = 0x820D;
const LID_END: u32 = 0x820E;
const LID_ALL_DAY: u32 = 0x8215;
const LID_RECUR: u32 = 0x8216;
const LID_RECURRING: u32 = 0x8223;

/// Property ids of the appointment fields in this file (they differ per file).
fn appointment_ids(store: &Rc<dyn Store>) -> HashMap<u32, u16> {
    let mut ids = HashMap::new();
    let Ok(map) = store.named_property_map() else {
        return ids;
    };
    let props = map.properties();
    let (Ok(entries), Ok(guids)) = (props.stream_entry(), props.stream_guid()) else {
        return ids;
    };
    for e in entries {
        let (NamedPropertyId::Number(lid), NamedPropertyGuid::GuidIndex(g)) = (e.id(), e.guid())
        else {
            continue;
        };
        // own lookup: the library's `lookup_guid` can index past the list
        if guids
            .get(g as usize)
            .is_some_and(|g| g.data1() == PSETID_APPOINTMENT)
        {
            ids.insert(lid, e.prop_id());
        }
    }
    ids
}

/// Open a PST from its bytes. The header version tells Unicode (Outlook 2003+)
/// from the old ANSI format, so the file is not copied for a second try.
fn open(bytes: Vec<u8>) -> Result<Rc<dyn Store>, String> {
    let bad = || "Keine gültige Outlook-Datei (.pst).".to_string();
    if bytes.get(0..4) != Some(b"!BDN") {
        return Err(bad());
    }
    let version = u16::from_le_bytes([bytes[10], bytes[11]]);
    let reader = Box::new(Cursor::new(bytes));
    let store: Rc<dyn Store> = if version >= 23 {
        let f = UnicodePstFile::read_from(reader).map_err(|_| bad())?;
        UnicodeStore::read(Rc::new(f)).map_err(|e| e.to_string())?
    } else {
        let f = AnsiPstFile::read_from(reader).map_err(|_| bad())?;
        AnsiStore::read(Rc::new(f)).map_err(|e| e.to_string())?
    };
    Ok(store)
}

fn text(v: Option<&PropertyValue>) -> String {
    match v {
        Some(PropertyValue::Unicode(s)) => s.to_string(),
        Some(PropertyValue::String8(s)) => s.to_string(),
        _ => String::new(),
    }
}

/// FILETIME (100 ns since 1601) to UTC.
fn filetime(v: Option<&PropertyValue>) -> Option<DateTime<Utc>> {
    let Some(PropertyValue::Time(t)) = v else {
        return None;
    };
    DateTime::from_timestamp(t / 10_000_000 - 11_644_473_600, 0)
}

/// Read all appointments. `to_local` turns UTC into local time (with summer time).
pub fn read_pst_events(
    bytes: Vec<u8>,
    to_local: impl Fn(DateTime<Utc>) -> NaiveDateTime,
) -> Result<Vec<IcsEvent>, String> {
    let store = open(bytes)?;
    let named = appointment_ids(&store);
    let pid = |lid: u32| named.get(&lid).copied();
    let mut wanted = vec![
        PR_MESSAGE_CLASS,
        PR_SUBJECT,
        PR_START_DATE,
        PR_END_DATE,
        PR_BODY,
        PR_SEARCH_KEY,
    ];
    wanted.extend(
        [
            LID_LOCATION,
            LID_START,
            LID_END,
            LID_ALL_DAY,
            LID_RECUR,
            LID_RECURRING,
        ]
        .into_iter()
        .filter_map(pid),
    );
    let props = store.properties();
    let root = props
        .ipm_sub_tree_entry_id()
        .map_err(|e| format!("Ordner nicht lesbar: {e}"))?;
    let mut out = Vec::new();
    let mut todo = vec![root];
    let mut seen = 0usize;
    while let Some(id) = todo.pop() {
        seen += 1;
        if seen > 5000 {
            break;
        }
        let Ok(folder) = store.open_folder(&id) else {
            continue;
        };
        if let Some(h) = folder.hierarchy_table() {
            for row in h.rows_matrix() {
                let node = NodeId::from(u32::from(row.id()));
                if let Ok(e) = props.make_entry_id(node) {
                    todo.push(e);
                }
            }
        }
        let Some(contents) = folder.contents_table() else {
            continue;
        };
        for row in contents.rows_matrix() {
            let node = NodeId::from(u32::from(row.id()));
            let Ok(eid) = props.make_entry_id(node) else {
                continue;
            };
            let Ok(msg) = store.open_message(&eid, Some(&wanted)) else {
                continue;
            };
            let p = msg.properties();
            let class = text(p.get(PR_MESSAGE_CLASS));
            if !class.to_ascii_lowercase().starts_with("ipm.appointment") {
                continue;
            }
            let named = |lid: u32| pid(lid).and_then(|i| p.get(i));
            let start = filetime(named(LID_START)).or(filetime(p.get(PR_START_DATE)));
            let end = filetime(named(LID_END)).or(filetime(p.get(PR_END_DATE)));
            let Some(start) = start else {
                continue;
            };
            let all_day = matches!(named(LID_ALL_DAY), Some(PropertyValue::Boolean(true)));
            let recurring = matches!(named(LID_RECURRING), Some(PropertyValue::Boolean(true)));
            let recurrence = match named(LID_RECUR) {
                Some(PropertyValue::Binary(b)) if recurring => parse_recurrence(b.buffer()),
                _ => None,
            };
            let mut notes = text(p.get(PR_BODY)).trim().to_string();
            let location = text(named(LID_LOCATION));
            if !location.trim().is_empty() {
                notes = format!("Ort: {}\n\n{notes}", location.trim())
                    .trim()
                    .to_string();
            }
            let uid = match p.get(PR_SEARCH_KEY) {
                Some(PropertyValue::Binary(b)) => {
                    b.buffer().iter().map(|x| format!("{x:02x}")).collect()
                }
                _ => format!("pst-{}-{}", start.timestamp(), text(p.get(PR_SUBJECT))),
            };
            let mut ev = to_event(
                format!("pst:{uid}"),
                subject(&text(p.get(PR_SUBJECT))),
                to_local(start),
                end.map(&to_local),
                notes,
            );
            if all_day {
                // all-day dates are stored as local midnight
                ev.start = None;
                ev.end = None;
                ev.date = to_local(start + Duration::hours(12)).date();
            }
            ev.recurrence = recurrence;
            out.push(ev);
        }
    }
    Ok(out)
}

/// Outlook stores "RE: "-style prefixes in front: `\u{1}<len>` + text.
fn subject(s: &str) -> String {
    let mut c = s.chars();
    if s.starts_with('\u{1}') {
        c.next();
        c.next();
    }
    c.as_str().to_string()
}

/// Minutes since 1601 (as used in recurrence blobs) to a date.
fn minutes_to_date(m: u32) -> Option<NaiveDate> {
    let base = NaiveDate::from_ymd_opt(1601, 1, 1)?;
    Some(base + Duration::minutes(m as i64))
}

/// Read the repeat rule from PidLidAppointmentRecur (MS-OXOCAL 2.2.1.44.1).
/// Several weekdays or "every 2nd Tuesday" become the plain rule (weekly/monthly).
pub fn parse_recurrence(b: &[u8]) -> Option<Recurrence> {
    let u16_at = |o: usize| Some(u16::from_le_bytes(b.get(o..o + 2)?.try_into().ok()?));
    let u32_at = |o: usize| Some(u32::from_le_bytes(b.get(o..o + 4)?.try_into().ok()?));
    let freq = u16_at(4)?;
    let pattern = u16_at(6)?;
    let period = u32_at(14)?;
    let specific = match pattern {
        0x0000 => 0,
        0x0001 | 0x0002 | 0x0004 | 0x000A | 0x000C => 4,
        0x0003 | 0x000B => 8,
        _ => return None,
    };
    let o = 22 + specific;
    let end_type = u32_at(o)?;
    let (freq, interval) = match freq {
        0x200A => (Freq::Daily, (period / 1440).max(1)),
        0x200B => (Freq::Weekly, period.max(1)),
        0x200C => (Freq::Monthly, period.max(1)),
        0x200D => (Freq::Yearly, (period / 12).max(1)),
        _ => return None,
    };
    // deleted / modified instance lists, then start and end date
    let deleted = u32_at(o + 12)? as usize;
    let o2 = o + 16 + deleted * 4;
    let modified = u32_at(o2)? as usize;
    let end_date = u32_at(o2 + 4 + modified * 4 + 4)?;
    let until = match end_type {
        0x2021 | 0x2022 => minutes_to_date(end_date),
        _ => None,
    };
    Some(Recurrence {
        freq,
        interval,
        until,
    })
}

/// One appointment; midnight-to-midnight (whole days) becomes all-day.
pub fn to_event(
    uid: String,
    title: String,
    start: NaiveDateTime,
    end: Option<NaiveDateTime>,
    notes: String,
) -> IcsEvent {
    let midnight = NaiveTime::MIN;
    let all_day = start.time() == midnight && end.is_none_or(|e| e.time() == midnight && e > start);
    let title = Some(title.trim().to_string())
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| "(ohne Titel)".into());
    let (s, e) = if all_day {
        (None, None)
    } else {
        // end on the same day only; longer events keep the start time
        let e = end
            .filter(|e| e.date() == start.date() && *e > start)
            .map(|e| e.time())
            .or(Some(start.time() + Duration::hours(1)).filter(|t| *t > start.time()));
        (Some(start.time()), e)
    };
    IcsEvent {
        uid,
        title,
        date: start.date(),
        start: s,
        end: e,
        notes: notes.trim().replace("\r\n", "\n"),
        recurrence: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(d: u32, h: u32, m: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2026, 12, d)
            .unwrap()
            .and_hms_opt(h, m, 0)
            .unwrap()
    }

    #[test]
    fn timed_event() {
        let e = to_event(
            "a".into(),
            " Meeting ".into(),
            at(3, 9, 30),
            Some(at(3, 10, 15)),
            "x\r\ny".into(),
        );
        assert_eq!(e.title, "Meeting");
        assert_eq!(e.start, NaiveTime::from_hms_opt(9, 30, 0));
        assert_eq!(e.end, NaiveTime::from_hms_opt(10, 15, 0));
        assert_eq!(e.notes, "x\ny");
    }

    #[test]
    fn whole_days_are_all_day() {
        let e = to_event(
            "a".into(),
            "Urlaub".into(),
            at(3, 0, 0),
            Some(at(5, 0, 0)),
            String::new(),
        );
        assert_eq!((e.start, e.end), (None, None));
        assert_eq!(e.date, NaiveDate::from_ymd_opt(2026, 12, 3).unwrap());
    }

    #[test]
    fn empty_title_and_overnight() {
        let e = to_event(
            "a".into(),
            "  ".into(),
            at(3, 22, 0),
            Some(at(4, 1, 0)),
            String::new(),
        );
        assert_eq!(e.title, "(ohne Titel)");
        assert_eq!(e.start, NaiveTime::from_hms_opt(22, 0, 0));
        assert_eq!(e.end, NaiveTime::from_hms_opt(23, 0, 0));
    }

    /// Blob like Outlook writes it (header, pattern, counts, dates).
    fn blob(
        freq: u16,
        pattern: u16,
        period: u32,
        specific: &[u8],
        end_type: u32,
        end: u32,
    ) -> Vec<u8> {
        let mut b = vec![0x04, 0x30, 0x04, 0x30];
        b.extend(freq.to_le_bytes());
        b.extend(pattern.to_le_bytes());
        b.extend(0u16.to_le_bytes()); // calendar type
        b.extend(0u32.to_le_bytes()); // first date time
        b.extend(period.to_le_bytes());
        b.extend(0u32.to_le_bytes()); // sliding
        b.extend(specific);
        b.extend(end_type.to_le_bytes());
        b.extend(10u32.to_le_bytes()); // occurrence count
        b.extend(1u32.to_le_bytes()); // first day of week
        b.extend(1u32.to_le_bytes()); // one deleted instance
        b.extend(5u32.to_le_bytes());
        b.extend(0u32.to_le_bytes()); // no modified instances
        b.extend(0u32.to_le_bytes()); // start date
        b.extend(end.to_le_bytes());
        b
    }

    #[test]
    fn recurrence_blobs() {
        // weekly every 2 weeks (Monday), ends 2027-03-01
        let end = (NaiveDate::from_ymd_opt(2027, 3, 1).unwrap()
            - NaiveDate::from_ymd_opt(1601, 1, 1).unwrap())
        .num_minutes() as u32;
        let r = parse_recurrence(&blob(0x200B, 1, 2, &[2, 0, 0, 0], 0x2021, end)).unwrap();
        assert_eq!(r.freq, Freq::Weekly);
        assert_eq!(r.interval, 2);
        assert_eq!(r.until, NaiveDate::from_ymd_opt(2027, 3, 1));
        // daily every day, never ends
        let r = parse_recurrence(&blob(0x200A, 0, 1440, &[], 0x2023, 0x5AE9_80DF)).unwrap();
        assert_eq!((r.freq, r.interval, r.until), (Freq::Daily, 1, None));
        // yearly: period in months, "2nd Tuesday" pattern
        let r =
            parse_recurrence(&blob(0x200D, 3, 12, &[4, 0, 0, 0, 2, 0, 0, 0], 0x2023, 0)).unwrap();
        assert_eq!((r.freq, r.interval), (Freq::Yearly, 1));
        assert!(parse_recurrence(&[1, 2, 3]).is_none());
    }

    #[test]
    fn subject_prefix() {
        assert_eq!(subject("\u{1}\u{1}Test"), "Test");
        assert_eq!(subject("Plain"), "Plain");
    }

    #[test]
    fn garbage_is_rejected() {
        assert!(read_pst_events(b"nicht pst".to_vec(), |u| u.naive_utc()).is_err());
    }

    #[test]
    fn filetime_converts() {
        // 2026-01-01T00:00:00Z
        let ft = (1_767_225_600i64 + 11_644_473_600) * 10_000_000;
        assert_eq!(
            filetime(Some(&PropertyValue::Time(ft)))
                .unwrap()
                .timestamp(),
            1_767_225_600
        );
    }
}
