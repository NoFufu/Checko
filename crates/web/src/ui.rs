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
