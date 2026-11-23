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

/// Pull the first web link out of the input: ("Kopfhörer https://…", Some(url)).
pub fn extract_link(input: &str) -> (String, Option<String>) {
    let mut url = None;
    let mut rest = Vec::new();
    for word in input.split_whitespace() {
        let lower = word.to_ascii_lowercase();
        if url.is_none() && (lower.starts_with("https://") || lower.starts_with("http://")) {
            url = Some(word.to_string());
        } else {
            rest.push(word);
        }
    }
    (rest.join(" "), url)
}

/// Host without "www.", e.g. "amazon.de".
pub fn link_host(url: &str) -> String {
    let after = url.split_once("://").map_or(url, |(_, r)| r);
    let host = after.split(['/', '?', '#']).next().unwrap_or(after);
    let host = host.rsplit('@').next().unwrap_or(host);
    let host = host.split(':').next().unwrap_or(host);
    host.trim_start_matches("www.").to_lowercase()
}

/// Decode %XX escapes (UTF-8); invalid sequences are kept as they are.
fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(b) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(b);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8(out).unwrap_or_else(|_| s.to_string())
}

/// A readable name for a pasted link. Shops like Amazon put the product name
/// into the path ("/Sony-WH-1000XM5-Kopfhörer/dp/B09…"); otherwise the host is used.
pub fn title_from_url(url: &str) -> String {
    let host = link_host(url);
    let after = url.split_once("://").map_or(url, |(_, r)| r);
    let path = after
        .split(['?', '#'])
        .next()
        .unwrap_or("")
        .split_once('/')
        .map_or("", |(_, p)| p);
    let looks_like_name = |seg: &str| {
        let seg = percent_decode(seg);
        let letters = seg.chars().filter(|c| c.is_alphabetic()).count();
        let words = seg.split(['-', '_', '+']).filter(|w| !w.is_empty()).count();
        (words >= 2 && letters >= 4).then_some(seg)
    };
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    // Amazon: the segment right before "dp" / "gp"
    let amazon = segments
        .iter()
        .position(|s| *s == "dp" || *s == "gp")
        .and_then(|i| i.checked_sub(1))
        .and_then(|i| looks_like_name(segments[i]));
    let name = amazon.or_else(|| segments.iter().rev().find_map(|s| looks_like_name(s)));
    match name {
        Some(seg) => {
            let seg = seg.trim_end_matches(".html").trim_end_matches(".htm");
            let words: Vec<&str> = seg
                .split(['-', '_', '+'])
                .filter(|w| !w.is_empty())
                .collect();
            let mut title = String::new();
            for w in words {
                if title.chars().count() + w.chars().count() > 48 {
                    title.push_str(" …");
                    break;
                }
                if !title.is_empty() {
                    title.push(' ');
                }
                title.push_str(w);
            }
            let mut c = title.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => host,
            }
        }
        None if host.starts_with("amzn.") || host.contains("amazon.") => "Amazon-Artikel".into(),
        None => host,
    }
}

impl ShoppingItem {
    pub fn new(input: &str, now: DateTime<Utc>) -> Self {
        let (text, url) = extract_link(input);
        let (mut name, qty) = parse_item(&text);
        if name.is_empty() {
            name = url.as_deref().map(title_from_url).unwrap_or_default();
        }
        let category = Some(guess_category(&name).to_string());
        Self {
            meta: Meta::new(now),
            name,
            qty,
            category,
            checked: false,
            times_bought: 0,
            url,
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

/// One line in the shopping history.
#[derive(Debug, Clone, PartialEq)]
pub struct Purchase {
    pub name: String,
    pub times: u32,
    pub last: DateTime<Utc>,
    pub url: Option<String>,
    /// Currently on the list (not yet bought).
    pub on_list: bool,
}

/// Everything bought so far, one entry per name, most recently bought first.
pub fn purchase_history(items: &[ShoppingItem]) -> Vec<Purchase> {
    let mut out: Vec<Purchase> = Vec::new();
    for i in items.iter().filter(|i| i.times_bought > 0) {
        match out
            .iter_mut()
            .find(|p| p.name.eq_ignore_ascii_case(&i.name))
        {
            Some(p) => {
                p.times += i.times_bought;
                if i.meta.updated_at > p.last {
                    p.last = i.meta.updated_at;
                }
                if p.url.is_none() {
                    p.url = i.url.clone();
                }
            }
            None => out.push(Purchase {
                name: i.name.clone(),
                times: i.times_bought,
                last: i.meta.updated_at,
                url: i.url.clone(),
                on_list: false,
            }),
        }
    }
    for p in &mut out {
        p.on_list = items.iter().any(|i| {
            i.meta.deleted_at.is_none() && !i.checked && i.name.eq_ignore_ascii_case(&p.name)
        });
    }
    out.sort_by_key(|p| std::cmp::Reverse(p.last));
    out
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

    #[test]
    fn links() {
        let now = Utc::now();
        let amazon = "https://www.amazon.de/Sony-WH-1000XM5-Kopfh%C3%B6rer-Noise-Cancelling/dp/B09Y2MYL5C/ref=sr_1_1?keywords=sony";
        let item = ShoppingItem::new(amazon, now);
        assert_eq!(item.name, "Sony WH 1000XM5 Kopfhörer Noise Cancelling");
        assert_eq!(item.url.as_deref(), Some(amazon));
        assert_eq!(link_host(amazon), "amazon.de");

        let item = ShoppingItem::new("2x Druckerpatronen https://amzn.eu/d/abc123", now);
        assert_eq!(item.name, "Druckerpatronen");
        assert_eq!(item.qty.as_deref(), Some("2x"));
        assert_eq!(item.url.as_deref(), Some("https://amzn.eu/d/abc123"));

        assert_eq!(title_from_url("https://amzn.eu/d/abc123"), "Amazon-Artikel");
        assert_eq!(
            title_from_url("https://www.ikea.com/de/de/p/kallax-regal-weiss-80275887/"),
            "Kallax regal weiss 80275887"
        );
        assert_eq!(title_from_url("https://example.org/"), "example.org");
        assert!(ShoppingItem::new("Milch", now).url.is_none());
    }

    #[test]
    fn history() {
        let t0 = Utc::now();
        let t1 = t0 + chrono::Duration::hours(1);
        let mut milk = ShoppingItem::new("Milch", t0);
        milk.toggle(t0);
        milk.meta.deleted_at = Some(t0); // bought and cleared
        let mut milk2 = ShoppingItem::new("milch", t1);
        milk2.toggle(t1);
        let mut bread = ShoppingItem::new("Brot", t0);
        bread.toggle(t0);
        let bread_again = ShoppingItem::new("Brot", t1); // back on the list
        let never = ShoppingItem::new("Käse", t0);
        let h = purchase_history(&[milk, milk2, bread, bread_again, never]);
        assert_eq!(h.len(), 2);
        assert_eq!(h[0].name, "Milch");
        assert_eq!(h[0].times, 2);
        assert!(!h[0].on_list);
        assert_eq!(h[1].name, "Brot");
        assert!(h[1].on_list);
    }
}
