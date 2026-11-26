//! Managing categories: rename, delete, own emoji/colour (pure, tested).

use chrono::{DateTime, Utc};

use crate::{CategoryStyle, Event, Profile, Task};

fn same(a: &str, b: &str) -> bool {
    a.trim().eq_ignore_ascii_case(b.trim())
}

/// Rename a category on all tasks and events. If a task already has the new
/// name, the old one is simply removed. Returns how many entries changed.
pub fn rename_category(
    tasks: &mut [Task],
    events: &mut [Event],
    old: &str,
    new: &str,
    now: DateTime<Utc>,
) -> usize {
    let new = new.trim().trim_start_matches('#').trim();
    if new.is_empty() || old.trim().is_empty() || old.trim() == new {
        return 0;
    }
    let mut n = 0;
    for t in tasks
        .iter_mut()
        .filter(|t| !t.is_deleted() && t.has_category(old))
    {
        if t.has_category(new) && !same(old, new) {
            t.remove_category(old, now);
        } else {
            if let Some(a) = t.area.as_mut().filter(|a| same(a, old)) {
                *a = new.to_string();
            }
            for tag in t.tags.iter_mut().filter(|x| same(x, old)) {
                *tag = new.to_string();
            }
            t.meta.updated_at = now;
        }
        n += 1;
    }
    for e in events.iter_mut().filter(|e| e.meta.deleted_at.is_none()) {
        if e.category.as_deref().is_some_and(|c| same(c, old)) {
            e.category = Some(new.to_string());
            e.meta.updated_at = now;
            n += 1;
        }
    }
    n
}

/// Remove a category from all tasks and events (the entries stay).
pub fn delete_category(
    tasks: &mut [Task],
    events: &mut [Event],
    name: &str,
    now: DateTime<Utc>,
) -> usize {
    let mut n = 0;
    for t in tasks
        .iter_mut()
        .filter(|t| !t.is_deleted() && t.has_category(name))
    {
        t.remove_category(name, now);
        n += 1;
    }
    for e in events.iter_mut().filter(|e| e.meta.deleted_at.is_none()) {
        if e.category.as_deref().is_some_and(|c| same(c, name)) {
            e.category = None;
            e.meta.updated_at = now;
            n += 1;
        }
    }
    n
}

impl Profile {
    pub fn category_style(&self, name: &str) -> Option<&CategoryStyle> {
        self.category_styles.iter().find(|s| same(&s.name, name))
    }

    fn style_mut(&mut self, name: &str) -> &mut CategoryStyle {
        let i = match self
            .category_styles
            .iter()
            .position(|s| same(&s.name, name))
        {
            Some(i) => i,
            None => {
                self.category_styles.push(CategoryStyle {
                    name: name.trim().to_string(),
                    ..Default::default()
                });
                self.category_styles.len() - 1
            }
        };
        &mut self.category_styles[i]
    }

    /// Set (or clear with `None`/empty) the emoji and colour of a category.
    pub fn set_category_style(
        &mut self,
        name: &str,
        emoji: Option<String>,
        color: Option<String>,
        now: DateTime<Utc>,
    ) {
        let s = self.style_mut(name);
        s.emoji = emoji
            .map(|e| e.trim().to_string())
            .filter(|e| !e.is_empty());
        s.color = color.filter(|c| !c.trim().is_empty());
        self.tidy_styles();
        self.updated_at = Some(now);
    }

    /// Keep the style when a category is renamed.
    pub fn rename_category_style(&mut self, old: &str, new: &str, now: DateTime<Utc>) {
        let new = new.trim().trim_start_matches('#').trim();
        if let Some(i) = self.category_styles.iter().position(|s| same(&s.name, old)) {
            let mut s = self.category_styles.remove(i);
            self.category_styles.retain(|x| !same(&x.name, new));
            s.name = new.to_string();
            s.hidden = false;
            self.category_styles.push(s);
        } else {
            self.style_mut(new).hidden = false;
        }
        if crate::AREAS.iter().any(|a| same(a, old)) {
            self.style_mut(old).hidden = true;
        }
        self.tidy_styles();
        self.updated_at = Some(now);
    }

    /// Forget the style; default categories are hidden instead.
    pub fn delete_category_style(&mut self, name: &str, now: DateTime<Utc>) {
        self.category_styles.retain(|s| !same(&s.name, name));
        if let Some(a) = crate::AREAS.iter().find(|a| same(a, name)) {
            self.style_mut(a).hidden = true;
        }
        self.updated_at = Some(now);
    }

    /// Show a hidden category again (e.g. when it is used for a new task).
    pub fn unhide_category(&mut self, name: &str, now: DateTime<Utc>) {
        if let Some(s) = self
            .category_styles
            .iter_mut()
            .find(|s| same(&s.name, name) && s.hidden)
        {
            s.hidden = false;
            self.tidy_styles();
            self.updated_at = Some(now);
        }
    }

    pub fn category_hidden(&self, name: &str) -> bool {
        self.category_style(name).is_some_and(|s| s.hidden)
    }

    /// Drop default-category styles that no longer carry anything. Own
    /// categories stay, so they are listed even without tasks.
    fn tidy_styles(&mut self) {
        self.category_styles.retain(|s| {
            s.hidden
                || s.emoji.is_some()
                || s.color.is_some()
                || !crate::AREAS.iter().any(|a| same(a, &s.name))
        });
    }

    /// Own categories kept in the profile (e.g. renamed ones without tasks).
    pub fn kept_categories(&self) -> impl Iterator<Item = &str> {
        self.category_styles
            .iter()
            .filter(|s| !s.hidden)
            .map(|s| s.name.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::all_areas;

    fn task(cats: &[&str]) -> Task {
        let now = Utc::now();
        let mut t = Task::new("x", now);
        for c in cats {
            t.add_category(c, now);
        }
        t
    }

    #[test]
    fn rename_main_and_tag() {
        let now = Utc::now();
        let mut ts = vec![task(&["Bad"]), task(&["Arbeit", "bad"]), task(&["Küche"])];
        let n = rename_category(&mut ts, &mut [], "Bad", "Badezimmer", now);
        assert_eq!(n, 2);
        assert_eq!(ts[0].area.as_deref(), Some("Badezimmer"));
        assert_eq!(ts[1].tags, vec!["Badezimmer".to_string()]);
        assert_eq!(ts[2].area.as_deref(), Some("Küche"));
    }

    #[test]
    fn rename_into_existing_merges() {
        let now = Utc::now();
        let mut ts = vec![task(&["Bad", "Haushalt"])];
        rename_category(&mut ts, &mut [], "Bad", "Haushalt", now);
        assert_eq!(ts[0].categories(), vec!["Haushalt"]);
    }

    #[test]
    fn rename_changes_case_only() {
        let now = Utc::now();
        let mut ts = vec![task(&["bad"])];
        rename_category(&mut ts, &mut [], "bad", "Bad", now);
        assert_eq!(ts[0].area.as_deref(), Some("Bad"));
    }

    #[test]
    fn rename_ignores_empty_and_events_follow() {
        let now = Utc::now();
        let mut ts = vec![task(&["Bad"])];
        let mut ev = vec![Event::new("Termin", now.date_naive(), now)];
        ev[0].category = Some("bad".into());
        assert_eq!(rename_category(&mut ts, &mut ev, "Bad", "  ", now), 0);
        assert_eq!(rename_category(&mut ts, &mut ev, "Bad", "#Bäder", now), 2);
        assert_eq!(ev[0].category.as_deref(), Some("Bäder"));
    }

    #[test]
    fn delete_keeps_tasks() {
        let now = Utc::now();
        let mut ts = vec![task(&["Bad", "Küche"]), task(&["Bad"])];
        let mut ev = vec![Event::new("Termin", now.date_naive(), now)];
        ev[0].category = Some("Bad".into());
        assert_eq!(delete_category(&mut ts, &mut ev, "bad", now), 3);
        assert_eq!(ts[0].categories(), vec!["Küche"]);
        assert!(ts[1].categories().is_empty());
        assert_eq!(ev[0].category, None);
        assert!(!all_areas(&ts).contains(&"Bad".to_string()));
    }

    #[test]
    fn styles() {
        let now = Utc::now();
        let mut p = Profile::default();
        p.set_category_style("Bad", Some("🛁".into()), Some("#65a982".into()), now);
        assert_eq!(
            p.category_style("bad").unwrap().emoji.as_deref(),
            Some("🛁")
        );
        p.rename_category_style("Bad", "Badezimmer", now);
        assert!(p.category_style("Bad").is_none());
        assert_eq!(
            p.category_style("Badezimmer").unwrap().color.as_deref(),
            Some("#65a982")
        );
        p.set_category_style("Badezimmer", None, None, now);
        assert!(p.category_style("Badezimmer").unwrap().emoji.is_none());
        assert_eq!(p.kept_categories().collect::<Vec<_>>(), vec!["Badezimmer"]);
        p.set_category_style("Arbeit", Some("💼".into()), None, now);
        p.set_category_style("Arbeit", None, None, now);
        assert!(p.category_style("Arbeit").is_none());
    }

    #[test]
    fn default_categories_get_hidden() {
        let now = Utc::now();
        let mut p = Profile::default();
        p.delete_category_style("Gesundheit", now);
        assert!(p.category_hidden("gesundheit"));
        p.unhide_category("Gesundheit", now);
        assert!(!p.category_hidden("Gesundheit"));
        p.rename_category_style("Arbeit", "Job", now);
        assert!(p.category_hidden("Arbeit"));
        assert!(!p.category_hidden("Job"));
    }
}
