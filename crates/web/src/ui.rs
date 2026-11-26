//! Small shared display helpers (colours and labels).

use checko_core::Priority;
use leptos::prelude::{use_context, With};

/// Colour for a category dot. Default areas have fixed colours, custom ones
/// get a stable colour from a small palette.
pub fn area_color(area: &str) -> String {
    if let Some(c) = own_style(area).and_then(|s| s.color) {
        return c;
    }
    default_area_color(area).to_string()
}

/// The user's own style for a category (reactive when read inside a view).
fn own_style(name: &str) -> Option<checko_core::CategoryStyle> {
    use_context::<crate::store::Store>()?
        .profile
        .with(|p| p.category_style(name).cloned())
}

/// Colour without the user's own choice.
pub fn default_area_color(area: &str) -> &'static str {
    const PALETTE: [&str; 6] = [
        "#6fa8dc", "#e6a23c", "#d17fb0", "#5bb5a2", "#c9a227", "#8e9aaf",
    ];
    match area {
        "Arbeit" => "var(--lavender)",
        "Privat" => "var(--green)",
        "Gesundheit" => "var(--peach)",
        _ => {
            let h = area
                .to_lowercase()
                .bytes()
                .fold(7u32, |h, b| h.wrapping_mul(31).wrapping_add(b as u32));
            PALETTE[(h % PALETTE.len() as u32) as usize]
        }
    }
}

pub fn priority_name(p: Priority) -> &'static str {
    match p {
        Priority::P1 => "Dringend",
        Priority::P2 => "Hoch",
        Priority::P3 => "Mittel",
        Priority::P4 => "Normal",
    }
}

/// CSS class for the urgency dot.
pub fn priority_class(p: Priority) -> &'static str {
    match p {
        Priority::P1 => "urgency urgency-1",
        Priority::P2 => "urgency urgency-2",
        Priority::P3 => "urgency urgency-3",
        Priority::P4 => "urgency urgency-4",
    }
}

/// Icon for a category (emoji for common names, otherwise the first letter).
pub fn category_icon(name: &str) -> String {
    if let Some(e) = own_style(name).and_then(|s| s.emoji) {
        return e;
    }
    default_category_icon(name)
}

/// Icon without the user's own choice.
pub fn default_category_icon(name: &str) -> String {
    let n = name.to_lowercase();
    let pick = [
        (["arbeit", "job", "büro"].as_slice(), "💼"),
        (&["privat", "zuhause", "home"], "🏠"),
        (&["gesundheit", "wellness"], "❤️"),
        (&["arzt", "zahnarzt", "medizin", "termine arzt"], "🩺"),
        (&["bad", "badezimmer"], "🛁"),
        (&["küche", "kochen", "essen", "rezept"], "🍳"),
        (&["einkauf", "einkaufen", "shopping"], "🛒"),
        (&["garten", "pflanzen"], "🌱"),
        (&["auto", "fahrrad", "auto & co"], "🚗"),
        (&["sport", "fitness", "training"], "🏃"),
        (&["haushalt", "putzen", "wohnung"], "🧹"),
        (&["schule", "uni", "lernen", "studium"], "📚"),
        (&["finanzen", "geld", "steuer", "steuern", "rechnung"], "💶"),
        (&["urlaub", "reise", "reisen"], "✈️"),
        (&["familie", "kinder"], "👨‍👩‍👧"),
        (&["geburtstag", "geschenke", "geschenk"], "🎁"),
        (&["projekt", "projekte", "ideen", "idee"], "💡"),
    ];
    for (names, icon) in pick {
        if names.contains(&n.as_str()) {
            return icon.to_string();
        }
    }
    name.chars()
        .next()
        .map(|c| c.to_uppercase().collect())
        .unwrap_or_default()
}

/// True on devices with a mouse/trackpad. Drag & drop is only enabled there,
/// because on touch screens it gets in the way of tapping.
pub fn fine_pointer() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(pointer: fine)").ok().flatten())
        .is_some_and(|m| m.matches())
}

/// Medal image for an achievement (graphics in assets/achievements/fade/).
pub fn achievement_icon(id: &str) -> String {
    let file = match id {
        "first-task" => "erster-haken",
        "ten-tasks" => "in-fahrt",
        "fifty-tasks" => "profi",
        "urgent" => "feuerwehr",
        "on-time-5" => "puenktlich",
        "early-bird" => "fruehaufsteher",
        "streak-3" => "dranbleiben",
        "streak-7" => "eine-woche-stark",
        "level-5" => "level-5",
        "organizer" => "ordnung-muss-sein",
        "shopper" => "einkaufsheld",
        "writer" => "notizbuch",
        _ => "erster-haken",
    };
    crate::asset(&format!("achievements/fade/{file}.svg"))
}
