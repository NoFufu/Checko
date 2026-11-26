//! Import of calendar files (`.ics`, e.g. exported from Outlook).
//!
//! Small, forgiving reader for `VEVENT`s: title, start/end, all-day, notes
//! (description + location) and simple repeat rules. Re-importing the same file
//! updates the existing events (matched by `UID`) instead of adding duplicates.

use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};

use crate::{Event, Freq, Recurrence};

/// One event read from an `.ics` file.
#[derive(Debug, Clone, PartialEq)]
pub struct IcsEvent {
    pub uid: String,
    pub title: String,
    pub date: NaiveDate,
    pub start: Option<NaiveTime>,
    pub end: Option<NaiveTime>,
    pub notes: String,
    pub recurrence: Option<Recurrence>,
}

/// Lines with leading space/tab continue the previous one (RFC 5545 folding).
fn unfold(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for raw in text.lines() {
        let line = raw.trim_end_matches('\r');
        if let Some(rest) = line.strip_prefix(' ').or_else(|| line.strip_prefix('\t')) {
            if let Some(last) = out.last_mut() {
                last.push_str(rest);
                continue;
            }
        }
        out.push(line.to_string());
    }
    out
}

fn unescape(v: &str) -> String {
    let mut out = String::with_capacity(v.len());
    let mut chars = v.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') | Some('N') => out.push('\n'),
                Some(other) => out.push(other),
                None => {}
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// "DTSTART;TZID=W. Europe Standard Time:20261126T090000" → ("DTSTART", params, value)
fn split(line: &str) -> Option<(String, String, String)> {
    let colon = line.find(':')?;
    let (head, value) = (&line[..colon], &line[colon + 1..]);
    let mut parts = head.splitn(2, ';');
    let name = parts.next()?.to_ascii_uppercase();
    let params = parts.next().unwrap_or("").to_ascii_uppercase();
    Some((name, params, value.to_string()))
}

/// A date or date-time value in local time. UTC values ("…Z") are shifted by `offset_min`.
fn parse_when(
    params: &str,
    value: &str,
    offset_min: i32,
) -> Option<(NaiveDate, Option<NaiveTime>)> {
    let v = value.trim();
    if params.contains("VALUE=DATE") && !params.contains("VALUE=DATE-TIME") || v.len() == 8 {
        return NaiveDate::parse_from_str(v, "%Y%m%d")
            .ok()
            .map(|d| (d, None));
    }
    let (body, utc) = match v.strip_suffix('Z') {
        Some(b) => (b, true),
        None => (v, false),
    };
    let dt = NaiveDateTime::parse_from_str(body, "%Y%m%dT%H%M%S")
        .or_else(|_| NaiveDateTime::parse_from_str(body, "%Y%m%dT%H%M"))
        .ok()?;
    let local = if utc {
        dt + Duration::minutes(offset_min as i64)
    } else {
        dt
    };
    Some((local.date(), Some(local.time())))
}

fn parse_rrule(value: &str, offset_min: i32) -> Option<Recurrence> {
    let mut freq = None;
    let mut interval = 1u32;
    let mut until = None;
    for part in value.split(';') {
        let (k, v) = part.split_once('=')?;
        match k.to_ascii_uppercase().as_str() {
            "FREQ" => {
                freq = match v.to_ascii_uppercase().as_str() {
                    "DAILY" => Some(Freq::Daily),
                    "WEEKLY" => Some(Freq::Weekly),
                    "MONTHLY" => Some(Freq::Monthly),
                    "YEARLY" => Some(Freq::Yearly),
                    _ => None,
                }
            }
            "INTERVAL" => interval = v.parse().unwrap_or(1).max(1),
            "UNTIL" => until = parse_when("", v, offset_min).map(|(d, _)| d),
            _ => {}
        }
    }
    Some(Recurrence {
        freq: freq?,
        interval,
        until,
    })
}

type When = (NaiveDate, Option<NaiveTime>);

/// Fields collected while reading one VEVENT.
#[derive(Default)]
struct Draft {
    uid: String,
    title: String,
    start: Option<When>,
    end: Option<When>,
    desc: String,
    loc: String,
    rec: Option<Recurrence>,
    cancelled: bool,
}

/// Read all events of an `.ics` file. `offset_min` = local UTC offset in minutes
/// (Berlin in winter: 60), used for times given in UTC.
pub fn parse_ics(text: &str, offset_min: i32) -> Vec<IcsEvent> {
    let mut out = Vec::new();
    let mut cur: Option<Draft> = None;
    let mut depth = 0; // nested blocks inside VEVENT (VALARM)
    for line in unfold(text) {
        let Some((name, params, value)) = split(&line) else {
            continue;
        };
        match (name.as_str(), value.to_ascii_uppercase().as_str()) {
            ("BEGIN", "VEVENT") => {
                cur = Some(Draft::default());
                depth = 0;
                continue;
            }
            ("BEGIN", _) if cur.is_some() => {
                depth += 1;
                continue;
            }
            ("END", "VEVENT") => {
                if let Some(Draft {
                    uid,
                    title,
                    start: Some((date, start)),
                    end,
                    desc,
                    loc,
                    rec,
                    cancelled,
                }) = cur.take()
                {
                    if !cancelled {
                        let end_time = end.and_then(|(d, t)| if d == date { t } else { None });
                        let mut notes = Vec::new();
                        if !loc.trim().is_empty() {
                            notes.push(format!("Ort: {}", loc.trim()));
                        }
                        if !desc.trim().is_empty() {
                            notes.push(desc.trim().to_string());
                        }
                        let title = if title.trim().is_empty() {
                            "(ohne Titel)".to_string()
                        } else {
                            title.trim().to_string()
                        };
                        let uid = if uid.is_empty() {
                            format!("{title}@{date}")
                        } else {
                            uid
                        };
                        out.push(IcsEvent {
                            uid,
                            title,
                            date,
                            start,
                            end: end_time,
                            notes: notes.join("\n\n"),
                            recurrence: rec,
                        });
                    }
                }
                continue;
            }
            ("END", _) if cur.is_some() && depth > 0 => {
                depth -= 1;
                continue;
            }
            _ => {}
        }
        if depth > 0 {
            continue;
        }
        let Some(c) = cur.as_mut() else { continue };
        match name.as_str() {
            "UID" => c.uid = value.trim().to_string(),
            "SUMMARY" => c.title = unescape(&value),
            "DTSTART" => c.start = parse_when(&params, &value, offset_min),
            "DTEND" => c.end = parse_when(&params, &value, offset_min),
            "DESCRIPTION" => c.desc = unescape(&value),
            "LOCATION" => c.loc = unescape(&value),
            "RRULE" => c.rec = parse_rrule(&value, offset_min),
            "STATUS" if value.trim().eq_ignore_ascii_case("CANCELLED") => c.cancelled = true,
            _ => {}
        }
    }
    out
}

/// Result of an import.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ImportStats {
    pub added: usize,
    pub updated: usize,
    pub unchanged: usize,
}

/// Add the imported events to `events`; events from an earlier import of the same
/// calendar (same `UID`) are updated in place. Reminder and category set in Checko stay.
pub fn merge_ics(
    events: &mut Vec<Event>,
    imported: Vec<IcsEvent>,
    category: Option<&str>,
    now: DateTime<Utc>,
) -> ImportStats {
    let mut stats = ImportStats::default();
    for ie in imported {
        match events
            .iter_mut()
            .find(|e| e.source_uid.as_deref() == Some(ie.uid.as_str()))
        {
            Some(e) => {
                let changed = e.title != ie.title
                    || e.date != ie.date
                    || e.start != ie.start
                    || e.end != ie.end
                    || e.notes != ie.notes
                    || e.recurrence != ie.recurrence
                    || e.meta.deleted_at.is_some();
                if changed {
                    e.title = ie.title;
                    e.date = ie.date;
                    e.start = ie.start;
                    e.end = ie.end;
                    e.notes = ie.notes;
                    e.recurrence = ie.recurrence;
                    e.meta.deleted_at = None;
                    e.meta.updated_at = now;
                    stats.updated += 1;
                } else {
                    stats.unchanged += 1;
                }
            }
            None => {
                let mut e = Event::new(&ie.title, ie.date, now);
                e.start = ie.start;
                e.end = ie.end;
                e.notes = ie.notes;
                e.recurrence = ie.recurrence;
                e.category = category.map(str::to_string);
                e.source_uid = Some(ie.uid);
                events.push(e);
                stats.added += 1;
            }
        }
    }
    stats
}

#[cfg(test)]
mod tests {
    use super::*;

    const OUTLOOK: &str = "BEGIN:VCALENDAR\r\nPRODID:-//Microsoft Corporation//Outlook 16.0 MIMEDIR//EN\r\nVERSION:2.0\r\n\
BEGIN:VTIMEZONE\r\nTZID:W. Europe Standard Time\r\nBEGIN:STANDARD\r\nDTSTART:16011028T030000\r\nEND:STANDARD\r\nEND:VTIMEZONE\r\n\
BEGIN:VEVENT\r\nUID:040000008200E00074C5B7101A82E008000000001\r\nSUMMARY;LANGUAGE=de-de:Zweigstelle Paderborn\r\n\
DTSTART;TZID=W. Europe Standard Time:20261127T090000\r\nDTEND;TZID=W. Europe Standard Time:20261127T103000\r\n\
LOCATION:Raum 2\\, OG\r\nDESCRIPTION:Bitte Laptop mitbringen\\nund Unterlagen.\r\n\
BEGIN:VALARM\r\nTRIGGER:-PT15M\r\nACTION:DISPLAY\r\nDESCRIPTION:Reminder\r\nEND:VALARM\r\nEND:VEVENT\r\n\
BEGIN:VEVENT\r\nUID:abc-2\r\nSUMMARY:Betriebsausflug mit sehr langem Titel der in der Datei \r\n umgebrochen wurde\r\n\
DTSTART;VALUE=DATE:20261204\r\nDTEND;VALUE=DATE:20261205\r\nEND:VEVENT\r\n\
BEGIN:VEVENT\r\nUID:abc-3\r\nSUMMARY:Jour fixe\r\nDTSTART:20261130T080000Z\r\nDTEND:20261130T083000Z\r\n\
RRULE:FREQ=WEEKLY;INTERVAL=2;UNTIL=20270331T080000Z;BYDAY=MO\r\nEND:VEVENT\r\n\
BEGIN:VEVENT\r\nUID:abc-4\r\nSUMMARY:Abgesagt\r\nSTATUS:CANCELLED\r\nDTSTART:20261201T100000\r\nEND:VEVENT\r\n\
END:VCALENDAR\r\n";

    #[test]
    fn reads_outlook_file() {
        let ev = parse_ics(OUTLOOK, 60);
        assert_eq!(ev.len(), 3, "cancelled event is skipped");
        let a = &ev[0];
        assert_eq!(a.title, "Zweigstelle Paderborn");
        assert_eq!(a.date, NaiveDate::from_ymd_opt(2026, 11, 27).unwrap());
        assert_eq!(a.start, NaiveTime::from_hms_opt(9, 0, 0));
        assert_eq!(a.end, NaiveTime::from_hms_opt(10, 30, 0));
        assert_eq!(
            a.notes,
            "Ort: Raum 2, OG\n\nBitte Laptop mitbringen\nund Unterlagen."
        );
        let b = &ev[1];
        assert_eq!(
            b.title,
            "Betriebsausflug mit sehr langem Titel der in der Datei umgebrochen wurde"
        );
        assert_eq!(b.start, None, "all-day");
        let c = &ev[2];
        assert_eq!(c.start, NaiveTime::from_hms_opt(9, 0, 0), "UTC → Berlin");
        assert_eq!(
            c.recurrence,
            Some(Recurrence {
                freq: Freq::Weekly,
                interval: 2,
                until: NaiveDate::from_ymd_opt(2027, 3, 31)
            })
        );
    }

    #[test]
    fn reimport_updates_instead_of_duplicating() {
        let now = Utc::now();
        let mut events = Vec::new();
        let first = merge_ics(&mut events, parse_ics(OUTLOOK, 60), Some("Arbeit"), now);
        assert_eq!(
            first,
            ImportStats {
                added: 3,
                updated: 0,
                unchanged: 0
            }
        );
        assert_eq!(events[0].category.as_deref(), Some("Arbeit"));
        events[0].remind = Some(15); // set in Checko – must survive a re-import
        let moved = OUTLOOK.replace("20261127T090000", "20261127T100000");
        let second = merge_ics(&mut events, parse_ics(&moved, 60), Some("Arbeit"), now);
        assert_eq!(
            second,
            ImportStats {
                added: 0,
                updated: 1,
                unchanged: 2
            }
        );
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].start, NaiveTime::from_hms_opt(10, 0, 0));
        assert_eq!(events[0].remind, Some(15));
    }
}
