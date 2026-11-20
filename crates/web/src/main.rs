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

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Aurora />
            <div class="relative z-10 min-h-screen pb-32">
                <header class="mx-auto flex max-w-6xl items-center px-4 pt-6 sm:px-8">
                    <a href="/" class="brand"><img src="/assets/brand/icon.png" alt="" />"checko"</a>
                </header>
                <main class="mx-auto max-w-6xl px-4 pt-6 sm:px-8">
                    <Routes fallback=|| view! { <p>"Seite nicht gefunden."</p> }>
                        <Route path=path!("/") view=TodayPage />
                        <Route path=path!("/aufgaben") view=TasksPage />
                        <Route path=path!("/kalender") view=CalendarPage />
                        <Route path=path!("/notizen") view=NotesPage />
                        <Route path=path!("/einkauf") view=ShoppingPage />
                    </Routes>
                </main>
            </div>
            <Dock />
        </Router>
    }
}
