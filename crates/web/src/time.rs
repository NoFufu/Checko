//! Local date/time helpers via the browser clock.

const WEEKDAYS: [&str; 7] = [
    "Sonntag",
    "Montag",
    "Dienstag",
    "Mittwoch",
    "Donnerstag",
    "Freitag",
    "Samstag",
];
const MONTHS: [&str; 12] = [
    "Januar",
    "Februar",
    "März",
    "April",
    "Mai",
    "Juni",
    "Juli",
    "August",
    "September",
    "Oktober",
    "November",
    "Dezember",
];

pub fn greeting() -> &'static str {
    match js_sys::Date::new_0().get_hours() {
        5..=10 => "Guten Morgen",
        11..=13 => "Mahlzeit",
        14..=17 => "Hallo",
        18..=22 => "Guten Abend",
        _ => "Noch wach?",
    }
}

/// e.g. "Freitag, 20. November"
pub fn today_label() -> String {
    let d = js_sys::Date::new_0();
    format!(
        "{}, {}. {}",
        WEEKDAYS[d.get_day() as usize],
        d.get_date(),
        MONTHS[d.get_month() as usize]
    )
}
