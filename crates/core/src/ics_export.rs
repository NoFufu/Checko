//! Export as a calendar feed (`.ics`), e.g. to subscribe in Outlook.
//!
//! Contains Checko's own calendar events and tasks that are appointments.
//! Events imported from Outlook (`source_uid`) are left out, so they do not
//! show up twice there. Times are local German time (`Europe/Berlin`).

use chrono::{DateTime, Duration, NaiveDate, NaiveTime, Utc};

use crate::{Event, Freq, Recurrence, Task};

const TZID: &str = "Europe/Berlin";

const VTIMEZONE: &str = "BEGIN:VTIMEZONE\r\nTZID:Europe/Berlin\r\n\
BEGIN:DAYLIGHT\r\nTZOFFSETFROM:+0100\r\nTZOFFSETTO:+0200\r\nTZNAME:CEST\r\n\
DTSTART:19700329T020000\r\nRRULE:FREQ=YEARLY;BYMONTH=3;BYDAY=-1SU\r\nEND:DAYLIGHT\r\n\
BEGIN:STANDARD\r\nTZOFFSETFROM:+0200\r\nTZOFFSETTO:+0100\r\nTZNAME:CET\r\n\
DTSTART:19701025T030000\r\nRRULE:FREQ=YEARLY;BYMONTH=10;BYDAY=-1SU\r\nEND:STANDARD\r\n\
END:VTIMEZONE\r\n";

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace(';', "\\;")
        .replace(',', "\\,")
        .replace("\r\n", "\\n")
        .replace('\n', "\\n")
}

/// Fold a content line at 75 bytes (RFC 5545), never inside a UTF-8 character.
fn fold(line: &str, out: &mut String) {
    let mut len = 0;
    for c in line.chars() {
        if len + c.len_utf8() > 75 {
            out.push_str("\r\n ");
            len = 1;
        }
        out.push(c);
        len += c.len_utf8();
    }
    out.push_str("\r\n");
}

fn rrule(r: &Recurrence) -> String {
    let freq = match r.freq {
        Freq::Daily => "DAILY",
        Freq::Weekly => "WEEKLY",
        Freq::Monthly => "MONTHLY",
        Freq::Yearly => "YEARLY",
    };
    let mut s = format!("RRULE:FREQ={freq}");
    if r.interval > 1 {
        s.push_str(&format!(";INTERVAL={}", r.interval));
    }
    if let Some(u) = r.until {
        s.push_str(&format!(";UNTIL={}", u.format("%Y%m%d")));
    }
    s
}

struct Entry<'a> {
    uid: String,
    title: &'a str,
    date: NaiveDate,
    start: Option<NaiveTime>,
    end: Option<NaiveTime>,
    notes: &'a str,
    category: Option<&'a str>,
    recurrence: Option<Recurrence>,
    updated: DateTime<Utc>,
}

fn write_entry(e: &Entry, stamp: DateTime<Utc>, out: &mut String) {
    let lines = |out: &mut String, l: String| fold(&l, out);
    out.push_str("BEGIN:VEVENT\r\n");
    lines(out, format!("UID:{}", e.uid));
    lines(out, format!("DTSTAMP:{}", stamp.format("%Y%m%dT%H%M%SZ")));
    lines(
        out,
        format!("LAST-MODIFIED:{}", e.updated.format("%Y%m%dT%H%M%SZ")),
    );
    lines(out, format!("SUMMARY:{}", escape(e.title)));
    match e.start {
        Some(start) => {
            let begin = e.date.and_time(start);
            let mut end = e.date.and_time(e.end.unwrap_or(start));
            if end <= begin {
                end = begin + Duration::hours(1);
            }
            lines(
                out,
                format!("DTSTART;TZID={TZID}:{}", begin.format("%Y%m%dT%H%M%S")),
            );
            lines(
                out,
                format!("DTEND;TZID={TZID}:{}", end.format("%Y%m%dT%H%M%S")),
            );
        }
        None => {
            lines(
                out,
                format!("DTSTART;VALUE=DATE:{}", e.date.format("%Y%m%d")),
            );
            let next = e.date + Duration::days(1);
            lines(out, format!("DTEND;VALUE=DATE:{}", next.format("%Y%m%d")));
            out.push_str("TRANSP:TRANSPARENT\r\n");
        }
    }
    if let Some(r) = &e.recurrence {
        lines(out, rrule(r));
    }
    if !e.notes.trim().is_empty() {
        lines(out, format!("DESCRIPTION:{}", escape(e.notes.trim())));
    }
    if let Some(c) = e.category {
        lines(out, format!("CATEGORIES:{}", escape(c)));
    }
    out.push_str("END:VEVENT\r\n");
}

/// Build the calendar feed. `stamp` = time of creation (DTSTAMP).
pub fn write_ics(events: &[Event], tasks: &[Task], stamp: DateTime<Utc>) -> String {
    let mut out = String::from(
        "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//Checko//Checko//DE\r\n\
CALSCALE:GREGORIAN\r\nMETHOD:PUBLISH\r\nX-WR-CALNAME:Checko\r\n\
X-WR-TIMEZONE:Europe/Berlin\r\nREFRESH-INTERVAL;VALUE=DURATION:PT1H\r\n\
X-PUBLISHED-TTL:PT1H\r\n",
    );
    out.push_str(VTIMEZONE);
    for e in events
        .iter()
        .filter(|e| e.meta.deleted_at.is_none() && e.source_uid.is_none())
    {
        write_entry(
            &Entry {
                uid: format!("{}@checko", e.meta.id),
                title: &e.title,
                date: e.date,
                start: e.start,
                end: e.end,
                notes: &e.notes,
                category: e.category.as_deref(),
                recurrence: e.recurrence,
                updated: e.meta.updated_at,
            },
            stamp,
            &mut out,
        );
    }
    for t in tasks
        .iter()
        .filter(|t| !t.is_deleted() && t.is_appointment())
    {
        let Some(date) = t.due else { continue };
        write_entry(
            &Entry {
                uid: format!("{}@checko", t.meta.id),
                title: &t.title,
                date,
                start: t.due_time,
                end: None,
                notes: &t.notes,
                category: t.area.as_deref(),
                recurrence: t.recurrence,
                updated: t.meta.updated_at,
            },
            stamp,
            &mut out,
        );
    }
    out.push_str("END:VCALENDAR\r\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_ics;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }
    fn t(h: u32, m: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(h, m, 0).unwrap()
    }

    #[test]
    fn events_and_appointments() {
        let now = Utc::now();
        let mut e = Event::new("Zahnarzt, Kontrolle; jährlich", d(2026, 12, 3), now);
        e.start = Some(t(14, 0));
        e.end = Some(t(14, 30));
        e.notes = "Zeile 1\nZeile 2".into();
        let mut allday = Event::new("Geburtstag Oma", d(2026, 12, 24), now);
        allday.recurrence = Some(Recurrence::every(Freq::Yearly));
        let mut task = Task::new("Teammeeting", now);
        task.due = Some(d(2026, 12, 1));
        task.due_time = Some(t(9, 0));
        let todo = Task::new("Milch kaufen", now);
        let ics = write_ics(&[e, allday], &[task, todo], now);
        assert!(ics.contains("SUMMARY:Zahnarzt\\, Kontrolle\\; jährlich"));
        assert!(ics.contains("DTSTART;TZID=Europe/Berlin:20261203T140000"));
        assert!(ics.contains("DTEND;TZID=Europe/Berlin:20261203T143000"));
        assert!(ics.contains("DESCRIPTION:Zeile 1\\nZeile 2"));
        assert!(ics.contains("DTSTART;VALUE=DATE:20261224"));
        assert!(ics.contains("DTEND;VALUE=DATE:20261225"));
        assert!(ics.contains("RRULE:FREQ=YEARLY"));
        // task without end gets one hour
        assert!(ics.contains("DTEND;TZID=Europe/Berlin:20261201T100000"));
        assert!(!ics.contains("Milch"));
        assert_eq!(ics.matches("BEGIN:VEVENT").count(), 3);
    }

    #[test]
    fn imported_and_deleted_are_left_out() {
        let now = Utc::now();
        let mut imported = Event::new("Aus Outlook", d(2026, 12, 3), now);
        imported.source_uid = Some("abc".into());
        let mut gone = Event::new("Gelöscht", d(2026, 12, 3), now);
        gone.meta.deleted_at = Some(now);
        let ics = write_ics(&[imported, gone], &[], now);
        assert_eq!(ics.matches("BEGIN:VEVENT").count(), 0);
    }

    #[test]
    fn long_lines_fold_and_read_back() {
        let now = Utc::now();
        let mut e = Event::new(
            &"Sehr langer Titel mit Ümlauten ".repeat(5),
            d(2026, 12, 3),
            now,
        );
        e.start = Some(t(8, 0));
        let mut r = Recurrence::every(Freq::Weekly);
        r.interval = 2;
        r.until = Some(d(2027, 3, 1));
        e.recurrence = Some(r);
        let ics = write_ics(std::slice::from_ref(&e), &[], now);
        assert!(ics.lines().all(|l| l.len() <= 75));
        let back = parse_ics(&ics, 60);
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].title, e.title.trim());
        assert_eq!(back[0].start, Some(t(8, 0)));
        assert_eq!(back[0].recurrence, Some(r));
    }
}
