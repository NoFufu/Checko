mod components;
mod markdown;
mod pages;
mod store;
mod time;
mod ui;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use components::{
    celebrate::CelebrationHost, quick_add::QuickAddDialog, sidebar::Sidebar,
    task_detail::TaskDetail, toast::ToastHost,
};
use pages::{
    calendar::CalendarPage, notes::NotesPage, settings::SettingsPage, shopping::ShoppingPage,
    tasks::TasksPage, today::TodayPage,
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
    let store = store::Store::provide();
    view! {
        <Router base=BASE>
            <div class="app-shell" class:detail-open=move || store.detail.get().is_some()>
                <Sidebar />
                <main class="app-main">
                    <Routes fallback=|| view! { <p class="p-8">"Seite nicht gefunden."</p> }>
                        <Route path=path!("/") view=TodayPage />
                        <Route path=path!("/aufgaben") view=TasksPage />
                        <Route path=path!("/kalender") view=CalendarPage />
                        <Route path=path!("/notizen") view=NotesPage />
                        <Route path=path!("/einkauf") view=ShoppingPage />
                        <Route path=path!("/einstellungen") view=SettingsPage />
                    </Routes>
                </main>
            </div>
            <ToastHost />
            <QuickAddDialog />
            <TaskDetail />
            <CelebrationHost />
        </Router>
    }
}
