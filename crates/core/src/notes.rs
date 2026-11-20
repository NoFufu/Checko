//! Notes and the global search across tasks, notes and shopping items.

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{Meta, Note, ShoppingItem, Task};

impl Note {
    pub fn new(now: DateTime<Utc>) -> Self {
        Self {
            meta: Meta::new(now),
            title: String::new(),
            body_md: String::new(),
            pinned: false,
            tags: Vec::new(),
        }
    }

    /// Title, or the first line of the body, or "Neue Notiz".
    pub fn display_title(&self) -> String {
        let t = self.title.trim();
        if !t.is_empty() {
            return t.to_string();
        }
        self.body_md
            .lines()
            .map(|l| l.trim_start_matches(['#', ' ', '-', '*']).trim())
            .find(|l| !l.is_empty())
            .map(|l| l.chars().take(60).collect())
            .unwrap_or_else(|| "Neue Notiz".to_string())
    }

    /// Short plain-text preview of the body.
    pub fn preview(&self, max: usize) -> String {
        let text: String = self
            .body_md
            .lines()
            .map(|l| {
                l.trim_start_matches(['#', '-', '*', '>', ' '])
                    .trim_start_matches("[ ] ")
                    .trim_start_matches("[x] ")
                    .replace("**", "")
                    .replace('`', "")
                    .trim()
                    .to_string()
            })
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .join(" · ");
        let mut out: String = text.chars().take(max).collect();
        if text.chars().count() > max {
            out.push('…');
        }
        out
    }

    pub fn is_empty(&self) -> bool {
        self.title.trim().is_empty() && self.body_md.trim().is_empty()
    }
}

/// Visible notes: pinned first, then most recently edited.
pub fn sorted_notes(notes: &[Note]) -> Vec<&Note> {
    let mut v: Vec<_> = notes
        .iter()
        .filter(|n| n.meta.deleted_at.is_none())
        .collect();
    v.sort_by_key(|n| (!n.pinned, std::cmp::Reverse(n.meta.updated_at)));
    v
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitKind {
    Task,
    Note,
    Shopping,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchHit {
    pub kind: HitKind,
    pub id: Uuid,
    pub title: String,
    pub detail: String,
}

fn matches(haystack: &str, words: &[String]) -> bool {
    let h = haystack.to_lowercase();
    words.iter().all(|w| h.contains(w.as_str()))
}

/// Find tasks, notes and shopping items that contain all words of `query`.
pub fn search(
    query: &str,
    tasks: &[Task],
    notes: &[Note],
    shopping: &[ShoppingItem],
    limit: usize,
) -> Vec<SearchHit> {
    let words: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
    if words.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for t in tasks.iter().filter(|t| !t.is_deleted()) {
        let hay = format!(
            "{} {} {} {}",
            t.title,
            t.notes,
            t.area.as_deref().unwrap_or(""),
            t.tags.join(" ")
        );
        if matches(&hay, &words) {
            out.push(SearchHit {
                kind: HitKind::Task,
                id: t.meta.id,
                title: t.title.clone(),
                detail: if t.is_open() {
                    "Aufgabe".into()
                } else {
                    "Erledigte Aufgabe".into()
                },
            });
        }
    }
    for n in notes.iter().filter(|n| n.meta.deleted_at.is_none()) {
        if matches(&format!("{} {}", n.title, n.body_md), &words) {
            out.push(SearchHit {
                kind: HitKind::Note,
                id: n.meta.id,
                title: n.display_title(),
                detail: n.preview(60),
            });
        }
    }
    for s in shopping.iter().filter(|s| s.meta.deleted_at.is_none()) {
        if matches(&s.name, &words) {
            out.push(SearchHit {
                kind: HitKind::Shopping,
                id: s.meta.id,
                title: s.name.clone(),
                detail: format!("Einkauf · {}", s.category_name()),
            });
        }
    }
    out.truncate(limit);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn titles_and_previews() {
        let now = Utc::now();
        let mut n = Note::new(now);
        assert_eq!(n.display_title(), "Neue Notiz");
        n.body_md = "# Urlaub\n- [x] Pass\n- **Sonnencreme**".into();
        assert_eq!(n.display_title(), "Urlaub");
        assert_eq!(n.preview(100), "Urlaub · Pass · Sonnencreme");
        assert_eq!(n.preview(6), "Urlaub…");
    }

    #[test]
    fn pinned_first() {
        let now = Utc::now();
        let a = Note::new(now);
        let mut b = Note::new(now);
        b.pinned = true;
        let notes = vec![a, b.clone()];
        assert_eq!(sorted_notes(&notes)[0].meta.id, b.meta.id);
    }

    #[test]
    fn search_everything() {
        let now = Utc::now();
        let t = Task::new("Zahnarzt anrufen", now);
        let mut n = Note::new(now);
        n.body_md = "Zahnarzt: Dr. Meier, Tel. 1234".into();
        let s = ShoppingItem::new("Zahnseide", now);
        let (t, n, s) = (vec![t], vec![n], vec![s]);
        let hits = search("zahn", &t, &n, &s, 10);
        assert_eq!(hits.len(), 3);
        assert_eq!(search("zahnarzt meier", &t, &n, &s, 10).len(), 1);
        assert!(search("  ", &[], &[], &[], 10).is_empty());
    }
}
