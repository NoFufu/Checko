//! Quick-add parser: turns "Zahnarzt morgen 14:00 #gesundheit !1" into a task.
//!
//! Recognised (German):
//! - dates: heute, morgen, übermorgen, Wochentage (montag … sonntag),
//!   "nächsten/kommenden montag", "nächste woche", "in 3 tagen", "in 2 wochen",
//!   "24.11.", "24.11.2026", "24.11.26" – optional preceded by "am", "bis", "ab"
//! - times: "14:00", "9:30", "14 uhr", "14uhr", "um 14", "um 14:30"
//! - priority: "!1" … "!4" or "p1" … "p4"
//! - area / tags: "#arbeit" (known area) or any other "#tag"
//!
//! Only the first date and the first time are taken; everything that is not
//! recognised stays in the title.

use chrono::{Datelike, Duration, NaiveDate, NaiveTime, Weekday};

use crate::{Priority, AREAS};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Date,
    Time,
    Priority,
    Area,
    Tag,
}

/// Something the parser recognised, for highlighting in the UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    /// The original words, e.g. "nächsten Montag".
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ParsedTask {
    pub title: String,
    pub priority: Option<Priority>,
    pub area: Option<String>,
    pub tags: Vec<String>,
    pub due: Option<NaiveDate>,
    pub time: Option<NaiveTime>,
    pub tokens: Vec<Token>,
}

const WEEKDAYS: [(&str, Weekday); 7] = [
    ("montag", Weekday::Mon),
    ("dienstag", Weekday::Tue),
    ("mittwoch", Weekday::Wed),
    ("donnerstag", Weekday::Thu),
    ("freitag", Weekday::Fri),
    ("samstag", Weekday::Sat),
    ("sonntag", Weekday::Sun),
];

/// Words that may introduce a date or time and are dropped together with it.
const DATE_PREFIX: [&str; 3] = ["am", "bis", "ab"];

fn norm(w: &str) -> String {
    w.trim_matches(|c: char| matches!(c, ',' | ';' | '!' | '?') && c != '!')
        .trim_end_matches([',', ';'])
        .to_lowercase()
        .replace("ae", "ä")
        .replace("ue", "ü")
}

fn weekday(w: &str) -> Option<Weekday> {
    WEEKDAYS.iter().find(|(n, _)| *n == w).map(|(_, d)| *d)
}

/// Next date with weekday `wd`; today counts unless `strictly_after`.
fn next_weekday(today: NaiveDate, wd: Weekday, strictly_after: bool) -> NaiveDate {
    let mut diff =
        (wd.num_days_from_monday() as i64 - today.weekday().num_days_from_monday() as i64 + 7) % 7;
    if diff == 0 && strictly_after {
        diff = 7;
    }
    today + Duration::days(diff)
}

fn parse_numeric_date(w: &str, today: NaiveDate) -> Option<NaiveDate> {
    let parts: Vec<&str> = w.trim_end_matches('.').split('.').collect();
    if !(2..=3).contains(&parts.len()) {
        return None;
    }
    let day: u32 = parts[0].parse().ok()?;
    let month: u32 = parts[1].parse().ok()?;
    match parts.get(2) {
        Some(y) if !y.is_empty() => {
            let mut year: i32 = y.parse().ok()?;
            if year < 100 {
                year += 2000;
            }
            NaiveDate::from_ymd_opt(year, month, day)
        }
        _ => {
            let d = NaiveDate::from_ymd_opt(today.year(), month, day)?;
            // A date without year that has already passed means next year.
            if d < today {
                NaiveDate::from_ymd_opt(today.year() + 1, month, day)
            } else {
                Some(d)
            }
        }
    }
}

fn parse_clock(w: &str) -> Option<NaiveTime> {
    let w = w.trim_end_matches("uhr");
    let (h, m) = match w.split_once(':') {
        Some((h, m)) => (h.parse().ok()?, m.parse().ok()?),
        None => (w.parse().ok()?, 0),
    };
    NaiveTime::from_hms_opt(h, m, 0)
}

fn priority(w: &str) -> Option<Priority> {
    match w {
        "!1" | "p1" => Some(Priority::P1),
        "!2" | "p2" => Some(Priority::P2),
        "!3" | "p3" => Some(Priority::P3),
        "!4" | "p4" => Some(Priority::P4),
        _ => None,
    }
}

/// Try to read a date starting at word `i`. Returns (date, words used).
fn date_at(words: &[String], i: usize, today: NaiveDate) -> Option<(NaiveDate, usize)> {
    let w = words[i].as_str();
    let next = words.get(i + 1).map(String::as_str);
    match w {
        "heute" => return Some((today, 1)),
        "morgen" => return Some((today + Duration::days(1), 1)),
        "übermorgen" => return Some((today + Duration::days(2), 1)),
        "nächste" | "nächster" | "nächstes" | "nächsten" | "kommende" | "kommenden" => {
            if let Some(n) = next {
                if n == "woche" {
                    return Some((next_weekday(today, Weekday::Mon, true), 2));
                }
                if let Some(wd) = weekday(n) {
                    return Some((next_weekday(today, wd, true), 2));
                }
            }
        }
        "in" => {
            let n: i64 = next?.parse().ok()?;
            let unit = words.get(i + 2)?.as_str();
            let days = match unit {
                "tag" | "tagen" | "tage" => n,
                "woche" | "wochen" => n * 7,
                _ => return None,
            };
            return Some((today + Duration::days(days), 3));
        }
        _ => {}
    }
    if let Some(wd) = weekday(w) {
        return Some((next_weekday(today, wd, false), 1));
    }
    parse_numeric_date(w, today).map(|d| (d, 1))
}

/// Try to read a time starting at word `i`. Returns (time, words used).
fn time_at(words: &[String], i: usize) -> Option<(NaiveTime, usize)> {
    let w = words[i].as_str();
    let next = words.get(i + 1).map(String::as_str);
    if w == "um" {
        let n = next?;
        let t = parse_clock(n)?;
        let extra = usize::from(words.get(i + 2).is_some_and(|x| x == "uhr"));
        return Some((t, 2 + extra));
    }
    if w.contains(':') || w.ends_with("uhr") && w.len() > 3 {
        return parse_clock(w).map(|t| (t, 1));
    }
    if next == Some("uhr") {
        return parse_clock(w).map(|t| (t, 2));
    }
    None
}

pub fn parse_quick_add(input: &str, today: NaiveDate) -> ParsedTask {
    let raw: Vec<&str> = input.split_whitespace().collect();
    let words: Vec<String> = raw.iter().map(|w| norm(w)).collect();
    let mut out = ParsedTask::default();
    let mut title: Vec<&str> = Vec::new();
    let mut i = 0;

    while i < words.len() {
        let w = words[i].as_str();
        let original = |from: usize, n: usize| raw[from..from + n].join(" ");

        if out.priority.is_none() {
            if let Some(p) = priority(w) {
                out.priority = Some(p);
                out.tokens.push(Token {
                    kind: TokenKind::Priority,
                    text: original(i, 1),
                });
                i += 1;
                continue;
            }
        }

        if let Some(tag) = w.strip_prefix('#').filter(|t| !t.is_empty()) {
            let tag_raw = raw[i].trim_start_matches('#').trim_end_matches([',', ';']);
            match AREAS.iter().find(|a| a.to_lowercase() == tag) {
                Some(a) if out.area.is_none() => {
                    out.area = Some(a.to_string());
                    out.tokens.push(Token {
                        kind: TokenKind::Area,
                        text: original(i, 1),
                    });
                }
                _ => {
                    out.tags.push(tag_raw.to_string());
                    out.tokens.push(Token {
                        kind: TokenKind::Tag,
                        text: original(i, 1),
                    });
                }
            }
            i += 1;
            continue;
        }

        // Optional "am/bis/ab" in front of a date.
        let prefix = usize::from(DATE_PREFIX.contains(&w) && i + 1 < words.len());

        if out.due.is_none() {
            if let Some((d, n)) = date_at(&words, i + prefix, today) {
                out.due = Some(d);
                out.tokens.push(Token {
                    kind: TokenKind::Date,
                    text: original(i, prefix + n),
                });
                i += prefix + n;
                continue;
            }
        }
        if out.time.is_none() {
            if let Some((t, n)) = time_at(&words, i) {
                out.time = Some(t);
                out.tokens.push(Token {
                    kind: TokenKind::Time,
                    text: original(i, n),
                });
                i += n;
                continue;
            }
        }

        title.push(raw[i]);
        i += 1;
    }

    // A time without a date means today.
    if out.time.is_some() && out.due.is_none() {
        out.due = Some(today);
    }
    out.title = title.join(" ").trim().to_string();
    if out.title.is_empty() {
        out.title = input.trim().to_string();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // Friday, 20 November 2026
    fn today() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 11, 20).unwrap()
    }
    fn d(m: u32, day: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(2026, m, day)
    }
    fn t(h: u32, m: u32) -> Option<NaiveTime> {
        NaiveTime::from_hms_opt(h, m, 0)
    }
    fn p(s: &str) -> ParsedTask {
        parse_quick_add(s, today())
    }

    #[test]
    fn full_example() {
        let r = p("Zahnarzt morgen 14:00 #gesundheit !1");
        assert_eq!(r.title, "Zahnarzt");
        assert_eq!(r.due, d(11, 21));
        assert_eq!(r.time, t(14, 0));
        assert_eq!(r.area.as_deref(), Some("Gesundheit"));
        assert_eq!(r.priority, Some(Priority::P1));
        assert_eq!(r.tokens.len(), 4);
    }

    #[test]
    fn plain_title() {
        let r = p("Milch und Eier kaufen");
        assert_eq!(r.title, "Milch und Eier kaufen");
        assert!(r.due.is_none() && r.time.is_none() && r.tokens.is_empty());
    }

    #[test]
    fn relative_days() {
        assert_eq!(p("x heute").due, d(11, 20));
        assert_eq!(p("x übermorgen").due, d(11, 22));
        assert_eq!(p("x uebermorgen").due, d(11, 22));
        assert_eq!(p("x in 3 tagen").due, d(11, 23));
        assert_eq!(p("x in 2 wochen").due, d(12, 4));
    }

    #[test]
    fn weekdays() {
        assert_eq!(p("x montag").due, d(11, 23));
        assert_eq!(p("x Freitag").due, d(11, 20)); // today
        assert_eq!(p("x nächsten freitag").due, d(11, 27));
        assert_eq!(p("x naechste woche").due, d(11, 23));
        assert_eq!(p("Steuer bis Freitag").title, "Steuer");
        assert_eq!(p("Treffen am Dienstag").due, d(11, 24));
    }

    #[test]
    fn numeric_dates() {
        assert_eq!(p("x 24.12.").due, d(12, 24));
        assert_eq!(p("x am 3.1.").due, NaiveDate::from_ymd_opt(2027, 1, 3)); // passed -> next year
        assert_eq!(p("x 5.6.2027").due, NaiveDate::from_ymd_opt(2027, 6, 5));
        assert_eq!(p("x 5.6.27").due, NaiveDate::from_ymd_opt(2027, 6, 5));
        assert_eq!(p("x 31.2.").due, None);
    }

    #[test]
    fn times() {
        assert_eq!(p("x 9:30").time, t(9, 30));
        assert_eq!(p("x 14 uhr").time, t(14, 0));
        assert_eq!(p("x 14uhr").time, t(14, 0));
        assert_eq!(p("x um 8").time, t(8, 0));
        assert_eq!(p("x um 8:15 uhr").time, t(8, 15));
        assert_eq!(p("x um 8").due, d(11, 20)); // time only -> today
        assert_eq!(p("x 25:00").time, None);
    }

    #[test]
    fn tags_and_priority() {
        let r = p("Bericht #Arbeit #projekt-x p2");
        assert_eq!(r.area.as_deref(), Some("Arbeit"));
        assert_eq!(r.tags, vec!["projekt-x"]);
        assert_eq!(r.priority, Some(Priority::P2));
        assert_eq!(r.title, "Bericht");
    }

    #[test]
    fn only_first_date_counts() {
        let r = p("Ticket für 24.12. kaufen morgen");
        assert_eq!(r.due, d(12, 24));
        assert_eq!(r.title, "Ticket für kaufen morgen");
    }

    #[test]
    fn words_that_look_like_prefixes_stay() {
        assert_eq!(p("Bis bald sagen").title, "Bis bald sagen");
        assert_eq!(p("am Ende").title, "am Ende");
    }

    #[test]
    fn only_tokens_keeps_input_as_title() {
        assert_eq!(p("morgen").title, "morgen");
    }
}
