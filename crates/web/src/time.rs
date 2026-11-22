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

/// Today's weekday with Monday = 0 … Sunday = 6.
pub fn weekday_index() -> usize {
    (js_sys::Date::new_0().get_day() as usize + 6) % 7
}

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

/// Current instant (browser clock).
pub fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::from_timestamp_millis(js_sys::Date::now() as i64).unwrap_or_default()
}

/// Today's date in the user's local time zone.
pub fn today() -> chrono::NaiveDate {
    let d = js_sys::Date::new_0();
    chrono::NaiveDate::from_ymd_opt(d.get_full_year() as i32, d.get_month() + 1, d.get_date())
        .unwrap_or_default()
}

/// Short, friendly label for a due date: "Heute", "Morgen", "Montag", "3. Dez.".
pub fn due_label(due: chrono::NaiveDate, today: chrono::NaiveDate) -> String {
    use chrono::Datelike;
    const SHORT_MONTHS: [&str; 12] = [
        "Jan.", "Feb.", "März", "Apr.", "Mai", "Juni", "Juli", "Aug.", "Sep.", "Okt.", "Nov.",
        "Dez.",
    ];
    match (due - today).num_days() {
        0 => "Heute".into(),
        1 => "Morgen".into(),
        -1 => "Gestern".into(),
        2..=6 => WEEKDAYS[due.weekday().num_days_from_sunday() as usize].into(),
        _ => format!("{}. {}", due.day(), SHORT_MONTHS[due.month0() as usize]),
    }
}

/// "20.11. 14:03" in the browser's local time.
pub fn local_label(t: chrono::DateTime<chrono::Utc>) -> String {
    let d = js_sys::Date::new(&wasm_bindgen::JsValue::from_f64(t.timestamp_millis() as f64));
    format!(
        "{:02}.{:02}. {:02}:{:02}",
        d.get_date(),
        d.get_month() + 1,
        d.get_hours(),
        d.get_minutes()
    )
}

/// Current local time of day.
pub fn now_local_time() -> chrono::NaiveTime {
    let d = js_sys::Date::new_0();
    chrono::NaiveTime::from_hms_opt(d.get_hours(), d.get_minutes(), 0).unwrap_or_default()
}
