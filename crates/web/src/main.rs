mod components;
mod pages;
mod time;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use components::{aurora::Aurora, dock::Dock};
use pages::{
    calendar::CalendarPage, notes::NotesPage, shopping::ShoppingPage, tasks::TasksPage,
    today::TodayPage,
};

/// URL prefix when hosted in a sub-folder (e.g. GitHub Pages: "/Checko").
/// Set at build time with `CHECKO_BASE=/Checko`.
pub const BASE: &str = match option_env!("CHECKO_BASE") {
    Some(b) => b,
    None => "",
};

/// Absolute URL for a file in `assets/`.
pub fn asset(path: &str) -> String {
    format!("{BASE}/assets/{path}")
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router base=BASE>
            <Aurora />
            <Dock />
            <div class="app-content relative z-10 min-h-screen">
                <main class="mx-auto max-w-6xl px-4 pt-8 pb-28 sm:px-8 sm:pb-12">
                    <Routes fallback=|| view! { <p>"Seite nicht gefunden."</p> }>
                        <Route path=path!("/") view=TodayPage />
                        <Route path=path!("/aufgaben") view=TasksPage />
                        <Route path=path!("/kalender") view=CalendarPage />
                        <Route path=path!("/notizen") view=NotesPage />
                        <Route path=path!("/einkauf") view=ShoppingPage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
