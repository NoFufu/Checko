//! App state shared by all pages, saved in the browser (localStorage).
//!
//! Later phases swap the storage for IndexedDB and add sync; the pages only
//! talk to `Store`, so they do not need to change.

use std::time::Duration;

use checko_core::{complete_task, reopen_task, Priority, Profile, Task};
use chrono::{NaiveDate, NaiveTime};
use leptos::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

use crate::time::{now, today};

const TASKS_KEY: &str = "checko.tasks.v1";
const PROFILE_KEY: &str = "checko.profile.v1";

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

fn load<T: DeserializeOwned + Default>(key: &str) -> T {
    storage()
        .and_then(|s| s.get_item(key).ok().flatten())
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

fn save<T: Serialize>(key: &str, value: &T) {
    if let (Some(s), Ok(raw)) = (storage(), serde_json::to_string(value)) {
        let _ = s.set_item(key, &raw);
    }
}

/// A short message at the bottom of the screen, optionally with "Rückgängig".
#[derive(Clone, PartialEq)]
pub struct Toast {
    pub text: String,
    pub undo: Option<Undo>,
    /// Increases with every toast so an old timer does not hide a newer one.
    pub id: u32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Undo {
    Trash(Uuid),
    Complete(Uuid),
}

/// What a new task starts with.
#[derive(Clone)]
pub struct NewTask {
    pub title: String,
    pub priority: Priority,
    pub area: Option<String>,
    pub due: Option<NaiveDate>,
    pub time: Option<NaiveTime>,
    pub tags: Vec<String>,
}

#[derive(Clone, Copy)]
pub struct Store {
    pub tasks: RwSignal<Vec<Task>>,
    pub profile: RwSignal<Profile>,
    pub toast: RwSignal<Option<Toast>>,
    /// The Strg+K quick-add dialog.
    pub quick_add_open: RwSignal<bool>,
}

impl Store {
    /// Create the store, load saved data and keep saving on every change.
    pub fn provide() -> Self {
        let store = Store {
            tasks: RwSignal::new(load(TASKS_KEY)),
            profile: RwSignal::new(load(PROFILE_KEY)),
            toast: RwSignal::new(None),
            quick_add_open: RwSignal::new(false),
        };
        Effect::new(move |_| store.tasks.with(|t| save(TASKS_KEY, t)));
        Effect::new(move |_| store.profile.with(|p| save(PROFILE_KEY, p)));
        provide_context(store);
        store
    }

    pub fn get() -> Self {
        expect_context::<Store>()
    }

    fn notify(&self, text: impl Into<String>, undo: Option<Undo>) {
        let id = self
            .toast
            .with_untracked(|t| t.as_ref().map_or(0, |t| t.id + 1));
        self.toast.set(Some(Toast {
            text: text.into(),
            undo,
            id,
        }));
        let toast = self.toast;
        set_timeout(
            move || {
                if toast.with_untracked(|t| t.as_ref().is_some_and(|t| t.id == id)) {
                    toast.set(None);
                }
            },
            Duration::from_millis(5000),
        );
    }

    pub fn dismiss_toast(&self) {
        self.toast.set(None);
    }

    fn edit(&self, id: Uuid, f: impl FnOnce(&mut Task)) {
        self.tasks.update(|ts| {
            if let Some(t) = ts.iter_mut().find(|t| t.meta.id == id) {
                f(t);
            }
        });
    }

    pub fn add(&self, new: NewTask) {
        let title = new.title.trim();
        if title.is_empty() {
            return;
        }
        let mut t = Task::new(title, now());
        t.priority = new.priority;
        t.area = new.area;
        t.due = new.due;
        t.due_time = new.time;
        t.tags = new.tags;
        self.tasks.update(|ts| ts.push(t));
    }

    pub fn rename(&self, id: Uuid, title: String) {
        let title = title.trim().to_string();
        if title.is_empty() {
            return;
        }
        self.edit(id, |t| {
            if t.title != title {
                t.title = title;
                t.meta.updated_at = now();
            }
        });
    }

    pub fn set_priority(&self, id: Uuid, p: Priority) {
        self.edit(id, |t| {
            t.priority = p;
            t.meta.updated_at = now();
        });
    }

    pub fn toggle_started(&self, id: Uuid) {
        self.edit(id, |t| match t.status {
            checko_core::TaskStatus::Planned => t.start(now()),
            checko_core::TaskStatus::InProgress => t.pause(now()),
            checko_core::TaskStatus::Done => {}
        });
    }

    pub fn complete(&self, id: Uuid) {
        let mut xp = 0;
        let profile = self.profile;
        self.tasks.update(|ts| {
            profile.update(|p| xp = complete_task(ts, p, id, now(), today()));
        });
        if xp > 0 {
            self.notify(format!("Erledigt! +{xp} XP"), Some(Undo::Complete(id)));
        }
    }

    pub fn reopen(&self, id: Uuid) {
        let profile = self.profile;
        self.tasks
            .update(|ts| profile.update(|p| reopen_task(ts, p, id, now())));
    }

    pub fn trash(&self, id: Uuid) {
        self.edit(id, |t| t.trash(now()));
        self.notify("In den Papierkorb verschoben", Some(Undo::Trash(id)));
    }

    pub fn restore(&self, id: Uuid) {
        self.edit(id, |t| t.restore(now()));
    }

    /// Permanently remove a task from the trash.
    pub fn purge(&self, id: Uuid) {
        self.tasks.update(|ts| ts.retain(|t| t.meta.id != id));
    }

    pub fn empty_trash(&self) {
        self.tasks.update(|ts| ts.retain(|t| !t.is_deleted()));
    }

    pub fn undo(&self, undo: Undo) {
        match undo {
            Undo::Trash(id) => self.restore(id),
            Undo::Complete(id) => self.reopen(id),
        }
        self.dismiss_toast();
    }
}
