//! Small shared display helpers (colours and labels).

use checko_core::Priority;

/// Colour for a category dot. Default areas have fixed colours, custom ones
/// get a stable colour from a small palette.
pub fn area_color(area: &str) -> &'static str {
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
