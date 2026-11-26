//! Calendar file import (.ics, e.g. from Outlook) and backup download/restore.

use checko_core::{merge_ics, parse_ics, Event, Note, Profile, ShoppingItem, Task};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use super::icon::{Glyph, Icon};
use crate::store::Store;
use crate::time::now;

/// Read the text of the first file from a file input or a drop.
/// Take the file out of the list right away – clearing the input empties the list.
fn first(files: Option<web_sys::FileList>) -> Option<web_sys::File> {
    files?.get(0)
}

async fn read_file(file: Option<web_sys::File>) -> Option<(String, String)> {
    let file = file?;
    let text = JsFuture::from(file.text()).await.ok()?.as_string()?;
    Some((file.name(), text))
}

fn utc_offset_min() -> i32 {
    -(js_sys::Date::new_0().get_timezone_offset() as i32)
}

/// Box to pick or drop an `.ics` file; the events land in the Checko calendar.
#[component]
pub fn IcsImport(#[prop(optional)] compact: bool) -> impl IntoView {
    let store = Store::get();
    let category = RwSignal::new("Arbeit".to_string());
    let over = RwSignal::new(false);
    let result = RwSignal::new(None::<String>);

    let import = move |files: Option<web_sys::FileList>| {
        let file = first(files);
        leptos::task::spawn_local(async move {
            let Some((name, text)) = read_file(file).await else {
                result.set(Some("Datei konnte nicht gelesen werden.".into()));
                return;
            };
            let parsed = parse_ics(&text, utc_offset_min());
            if parsed.is_empty() {
                result.set(Some(format!("In „{name}“ wurden keine Termine gefunden.")));
                return;
            }
            let cat = category.get_untracked();
            let cat = Some(cat.as_str()).filter(|c| !c.is_empty());
            let mut stats = Default::default();
            store
                .events
                .update(|es| stats = merge_ics(es, parsed, cat, now()));
            let checko_core::ImportStats {
                added,
                updated,
                unchanged,
            } = stats;
            let msg = format!(
                "{added} neu, {updated} aktualisiert{}",
                if unchanged > 0 {
                    format!(", {unchanged} unverändert")
                } else {
                    String::new()
                }
            );
            store.notify(format!("Kalender importiert: {msg}"), None);
            result.set(Some(format!("✓ „{name}“: {msg}")));
        });
    };

    view! {
        <div class="ics-import" class:compact=compact>
            <label
                class="ics-drop"
                class:over=move || over.get()
                on:dragover=move |e| { e.prevent_default(); over.set(true); }
                on:dragleave=move |_| over.set(false)
                on:drop=move |e| {
                    e.prevent_default();
                    over.set(false);
                    import(e.data_transfer().and_then(|dt| dt.files()));
                }
            >
                <input
                    type="file"
                    accept=".ics,text/calendar"
                    class="sr-only"
                    on:change=move |e| {
                        let input: web_sys::HtmlInputElement = event_target(&e);
                        import(input.files());
                        input.set_value("");
                    }
                />
                <span class="ics-icon" aria-hidden="true"><Icon glyph=Glyph::Calendar size=20 /></span>
                <span class="min-w-0">
                    <span class="block font-semibold">"Kalender-Datei importieren"</span>
                    <span class="block text-sm text-muted">".ics hierher ziehen oder antippen – erneuter Import aktualisiert, statt zu verdoppeln"</span>
                </span>
            </label>
            <label class="ics-cat">
                <span class="text-sm text-muted">"Kategorie für neue Termine"</span>
                <select bind:value=category>
                    <option value="">"Keine"</option>
                    {move || store.areas().into_iter().map(|a| view! { <option value=a.clone()>{a.clone()}</option> }).collect_view()}
                </select>
            </label>
            {move || result.get().map(|r| view! { <p class="text-sm ics-result">{r}</p> })}
        </div>
    }
}

#[derive(Serialize, Deserialize)]
struct Backup {
    app: String,
    version: u32,
    created: String,
    tasks: Vec<Task>,
    events: Vec<Event>,
    notes: Vec<Note>,
    shopping: Vec<ShoppingItem>,
    profile: Profile,
}

/// Put every item of `from` into `into` (same id → replaced), marked as changed now so it syncs.
fn restore<T, F: Fn(&mut T) -> &mut checko_core::Meta>(
    into: &mut Vec<T>,
    from: Vec<T>,
    meta: F,
) -> usize {
    let mut n = 0;
    for mut item in from {
        meta(&mut item).updated_at = now();
        let id = meta(&mut item).id;
        match into.iter_mut().position(|x| meta(x).id == id) {
            Some(i) => into[i] = item,
            None => into.push(item),
        }
        n += 1;
    }
    n
}

/// Download all data as one file, and restore it again.
#[component]
pub fn BackupTools() -> impl IntoView {
    let store = Store::get();
    let result = RwSignal::new(None::<String>);

    let download = move |_| {
        let backup = Backup {
            app: "checko".into(),
            version: 1,
            created: now().to_rfc3339(),
            tasks: store.tasks.get_untracked(),
            events: store.events.get_untracked(),
            notes: store.notes.get_untracked(),
            shopping: store.shopping.get_untracked(),
            profile: store.profile.get_untracked(),
        };
        let Ok(json) = serde_json::to_string_pretty(&backup) else {
            return;
        };
        let parts = js_sys::Array::of1(&json.into());
        let opts = web_sys::BlobPropertyBag::new();
        opts.set_type("application/json");
        let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&parts, &opts) else {
            return;
        };
        let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) else {
            return;
        };
        if let Some(a) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.create_element("a").ok())
            .and_then(|a| a.dyn_into::<web_sys::HtmlAnchorElement>().ok())
        {
            a.set_href(&url);
            a.set_download(&format!("checko-backup-{}.json", crate::time::today()));
            a.click();
        }
        let _ = web_sys::Url::revoke_object_url(&url);
        result.set(Some("✓ Backup heruntergeladen.".into()));
    };

    let upload = move |e: leptos::ev::Event| {
        let input: web_sys::HtmlInputElement = event_target(&e);
        let file = first(input.files());
        input.set_value("");
        leptos::task::spawn_local(async move {
            let Some((_, text)) = read_file(file).await else {
                return;
            };
            let Ok(b) = serde_json::from_str::<Backup>(&text) else {
                result.set(Some("Das ist keine Checko-Backup-Datei.".into()));
                return;
            };
            let ok = web_sys::window()
                .and_then(|w| w.confirm_with_message(&format!(
                    "Backup vom {} einspielen? {} Aufgaben, {} Termine, {} Notizen und {} Einkäufe werden übernommen (gleiche Einträge werden ersetzt).",
                    b.created.get(..10).unwrap_or(&b.created), b.tasks.len(), b.events.len(), b.notes.len(), b.shopping.len()
                )).ok())
                .unwrap_or(false);
            if !ok {
                return;
            }
            let mut n = 0;
            store
                .tasks
                .update(|v| n += restore(v, b.tasks, |t| &mut t.meta));
            store
                .events
                .update(|v| n += restore(v, b.events, |t| &mut t.meta));
            store
                .notes
                .update(|v| n += restore(v, b.notes, |t| &mut t.meta));
            store
                .shopping
                .update(|v| n += restore(v, b.shopping, |t| &mut t.meta));
            store.profile.update(|p| {
                if b.profile.xp > p.xp {
                    *p = b.profile;
                    p.updated_at = Some(now());
                }
            });
            result.set(Some(format!("✓ {n} Einträge wiederhergestellt.")));
        });
    };

    view! {
        <div class="flex flex-wrap items-center gap-2">
            <button type="button" class="btn btn-primary" on:click=download>
                <Icon glyph=Glyph::Notes size=16 />
                "Backup herunterladen"
            </button>
            <label class="btn btn-outline">
                <input type="file" accept=".json,application/json" class="sr-only" on:change=upload />
                "Backup einspielen"
            </label>
        </div>
        {move || result.get().map(|r| view! { <p class="text-sm mt-2">{r}</p> })}
    }
}
