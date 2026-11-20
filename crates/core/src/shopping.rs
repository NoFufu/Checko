//! Shopping list: parsing "2x Milch", guessing the aisle, frequent items.

use chrono::{DateTime, Utc};

use crate::{Meta, ShoppingItem};

/// Aisles in the order you usually walk through a supermarket.
pub const SHOP_CATEGORIES: [&str; 8] = [
    "Obst & Gemüse",
    "Brot & Backwaren",
    "Kühlregal",
    "Fleisch & Fisch",
    "Vorrat",
    "Getränke",
    "Drogerie & Haushalt",
    "Sonstiges",
];

const KEYWORDS: [(&str, &[&str]); 7] = [
    (
        "Obst & Gemüse",
        &[
            "apfel",
            "äpfel",
            "banane",
            "birne",
            "orange",
            "zitrone",
            "traube",
            "beere",
            "erdbeer",
            "tomate",
            "gurke",
            "salat",
            "paprika",
            "zwiebel",
            "knoblauch",
            "kartoffel",
            "karotte",
            "möhre",
            "brokkoli",
            "zucchini",
            "pilz",
            "champignon",
            "avocado",
            "kräuter",
            "obst",
            "gemüse",
            "spinat",
            "lauch",
            "ingwer",
            "kiwi",
            "mango",
        ],
    ),
    (
        "Brot & Backwaren",
        &[
            "brot",
            "brötchen",
            "toast",
            "baguette",
            "croissant",
            "brezel",
            "kuchen",
            "semmel",
        ],
    ),
    (
        "Kühlregal",
        &[
            "milch",
            "joghurt",
            "quark",
            "käse",
            "butter",
            "sahne",
            "ei",
            "eier",
            "margarine",
            "frischkäse",
            "mozzarella",
            "schmand",
            "tofu",
        ],
    ),
    (
        "Fleisch & Fisch",
        &[
            "fleisch",
            "hähnchen",
            "huhn",
            "hack",
            "wurst",
            "schinken",
            "salami",
            "fisch",
            "lachs",
            "thunfisch",
            "steak",
        ],
    ),
    (
        "Vorrat",
        &[
            "nudel",
            "pasta",
            "reis",
            "mehl",
            "zucker",
            "salz",
            "pfeffer",
            "öl",
            "essig",
            "müsli",
            "haferflocken",
            "kaffee",
            "tee",
            "honig",
            "marmelade",
            "dose",
            "soße",
            "sauce",
            "ketchup",
            "senf",
            "gewürz",
            "schokolade",
            "chips",
            "kekse",
        ],
    ),
    (
        "Getränke",
        &[
            "wasser", "saft", "cola", "limo", "bier", "wein", "sprudel", "schorle", "getränk",
        ],
    ),
    (
        "Drogerie & Haushalt",
        &[
            "seife",
            "shampoo",
            "duschgel",
            "zahnpasta",
            "zahnbürste",
            "deo",
            "klopapier",
            "toilettenpapier",
            "küchenrolle",
            "taschentücher",
            "spülmittel",
            "waschmittel",
            "müllbeutel",
            "schwamm",
            "creme",
            "rasier",
            "batterie",
        ],
    ),
];

/// Pick an aisle from the item name ("Bio-Milch" -> "Kühlregal").
pub fn guess_category(name: &str) -> &'static str {
    let lower = name.to_lowercase();
    for (cat, words) in KEYWORDS {
        let hit = words.iter().any(|w| {
            // short words must match a whole word ("ei" should not match "Reis")
            if w.chars().count() <= 3 {
                lower
                    .split(|c: char| !c.is_alphanumeric())
                    .any(|part| part == *w)
            } else {
                lower.contains(w)
            }
        });
        if hit {
            return cat;
        }
    }
    "Sonstiges"
}

/// Split "2x Milch", "Milch 2l", "3 Äpfel" into (name, quantity).
pub fn parse_item(input: &str) -> (String, Option<String>) {
    let words: Vec<&str> = input.split_whitespace().collect();
    let is_qty = |w: &str| {
        let w = w.to_lowercase();
        let digits = w.trim_end_matches(|c: char| c.is_alphabetic());
        let unit = &w[digits.len()..];
        !digits.is_empty()
            && digits
                .chars()
                .all(|c| c.is_ascii_digit() || c == ',' || c == '.')
            && ["", "x", "l", "ml", "g", "kg", "stk", "pck", "st"].contains(&unit)
    };
    if words.len() > 1 && is_qty(words[0]) {
        return (words[1..].join(" "), Some(words[0].to_string()));
    }
    if words.len() > 1 && is_qty(words[words.len() - 1]) {
        let last = words.len() - 1;
        return (words[..last].join(" "), Some(words[last].to_string()));
    }
    (input.trim().to_string(), None)
}

impl ShoppingItem {
    pub fn new(input: &str, now: DateTime<Utc>) -> Self {
        let (name, qty) = parse_item(input);
        let category = Some(guess_category(&name).to_string());
        Self {
            meta: Meta::new(now),
            name,
            qty,
            category,
            checked: false,
            times_bought: 0,
        }
    }

    pub fn toggle(&mut self, now: DateTime<Utc>) {
        self.checked = !self.checked;
        if self.checked {
            self.times_bought += 1;
        } else {
            self.times_bought = self.times_bought.saturating_sub(1);
        }
        self.meta.updated_at = now;
    }

    pub fn category_name(&self) -> &str {
        self.category.as_deref().unwrap_or("Sonstiges")
    }
}

/// Visible items grouped by aisle (in shop order). Unchecked first in each group.
pub fn grouped(items: &[ShoppingItem]) -> Vec<(&'static str, Vec<&ShoppingItem>)> {
    SHOP_CATEGORIES
        .iter()
        .filter_map(|cat| {
            let mut v: Vec<_> = items
                .iter()
                .filter(|i| i.meta.deleted_at.is_none() && i.category_name() == *cat)
                .collect();
            if v.is_empty() {
                return None;
            }
            v.sort_by_key(|i| (i.checked, i.meta.created_at));
            Some((*cat, v))
        })
        .collect()
}

/// Items bought before that are not on the list right now, most frequent first.
pub fn suggestions(items: &[ShoppingItem], limit: usize) -> Vec<&ShoppingItem> {
    let on_list = |name: &str| {
        items
            .iter()
            .any(|i| i.meta.deleted_at.is_none() && !i.checked && i.name.eq_ignore_ascii_case(name))
    };
    let mut seen: Vec<&ShoppingItem> = Vec::new();
    for i in items.iter().filter(|i| i.times_bought > 0) {
        if on_list(&i.name) {
            continue;
        }
        match seen
            .iter_mut()
            .find(|s| s.name.eq_ignore_ascii_case(&i.name))
        {
            Some(s) if i.times_bought > s.times_bought => *s = i,
            Some(_) => {}
            None => seen.push(i),
        }
    }
    seen.sort_by_key(|i| std::cmp::Reverse(i.times_bought));
    seen.truncate(limit);
    seen
}

pub fn open_count(items: &[ShoppingItem]) -> usize {
    items
        .iter()
        .filter(|i| i.meta.deleted_at.is_none() && !i.checked)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn categories() {
        assert_eq!(guess_category("Bio-Milch"), "Kühlregal");
        assert_eq!(guess_category("Eier"), "Kühlregal");
        assert_eq!(guess_category("Reis"), "Vorrat"); // not "ei"
        assert_eq!(guess_category("Äpfel"), "Obst & Gemüse");
        assert_eq!(guess_category("Klopapier"), "Drogerie & Haushalt");
        assert_eq!(guess_category("Geschenkpapier"), "Sonstiges");
    }

    #[test]
    fn quantities() {
        assert_eq!(parse_item("2x Milch"), ("Milch".into(), Some("2x".into())));
        assert_eq!(
            parse_item("Wasser 6l"),
            ("Wasser".into(), Some("6l".into()))
        );
        assert_eq!(parse_item("3 Äpfel"), ("Äpfel".into(), Some("3".into())));
        assert_eq!(parse_item("Brot"), ("Brot".into(), None));
        assert_eq!(parse_item("7up"), ("7up".into(), None));
    }

    #[test]
    fn grouping_and_suggestions() {
        let now = Utc::now();
        let mut milk = ShoppingItem::new("Milch", now);
        milk.toggle(now);
        milk.toggle(now);
        milk.toggle(now); // bought twice in total (checked now)
        let bread = ShoppingItem::new("Brot", now);
        let items = vec![milk.clone(), bread];
        let g = grouped(&items);
        assert_eq!(g[0].0, "Brot & Backwaren");
        assert_eq!(g[1].0, "Kühlregal");
        assert_eq!(open_count(&items), 1);
        // milk is checked (bought), so it is suggested again
        assert_eq!(suggestions(&items, 5).len(), 1);
        let mut items2 = items.clone();
        items2.push(ShoppingItem::new("milch", now));
        assert!(suggestions(&items2, 5).is_empty());
    }
}
