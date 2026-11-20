use leptos::prelude::*;

#[derive(Clone, Copy)]
pub enum Glyph {
    Today,
    Tasks,
    Calendar,
    Notes,
    Shopping,
    Level,
    Streak,
    Pin,
    Add,
    Search,
    Bell,
    Settings,
    Sun,
    Moon,
    Monitor,
    ChevronDown,
    Clock,
}

#[component]
pub fn Icon(glyph: Glyph, #[prop(default = 24)] size: u32) -> impl IntoView {
    let shape = match glyph {
        Glyph::Today => "M12 2.5v2m0 15v2m9.5-9.5h-2m-15 0h-2m16.2-6.7-1.4 1.4M6.7 17.3l-1.4 1.4m13.4 0-1.4-1.4M6.7 6.7 5.3 5.3M16.5 12a4.5 4.5 0 1 1-9 0 4.5 4.5 0 0 1 9 0Z",
        Glyph::Tasks => "M7 12l3 3 7-7M7 3.5h10A3.5 3.5 0 0 1 20.5 7v10a3.5 3.5 0 0 1-3.5 3.5H7A3.5 3.5 0 0 1 3.5 17V7A3.5 3.5 0 0 1 7 3.5Z",
        Glyph::Calendar => "M7.5 3v4m9-4v4M3.5 10h17M7 5h10a3.5 3.5 0 0 1 3.5 3.5V17a3.5 3.5 0 0 1-3.5 3.5H7A3.5 3.5 0 0 1 3.5 17V8.5A3.5 3.5 0 0 1 7 5Z",
        Glyph::Notes => "M8 9h8m-8 4h6m-6 4h4M7 3.5h10A2.5 2.5 0 0 1 19.5 6v12a2.5 2.5 0 0 1-2.5 2.5H7A2.5 2.5 0 0 1 4.5 18V6A2.5 2.5 0 0 1 7 3.5Z",
        Glyph::Shopping => "M3 4h2l2.4 10.2a2 2 0 0 0 2 1.5h7.8a2 2 0 0 0 1.9-1.5L21 7H5.7m4.8 13a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0Zm8 0a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0Z",
        Glyph::Level => "M12 3.5l2.6 5.3 5.9.9-4.2 4.1 1 5.8-5.3-2.8-5.3 2.8 1-5.8-4.2-4.1 5.9-.9L12 3.5Z",
        Glyph::Streak => "M12 3c1 4-1.5 5-1.5 7.5-1.4 0-2.5-1.2-2.5-2.5C5.8 10 5 12 5 14a7 7 0 1 0 14 0c0-4-3-8-7-11Z",
        Glyph::Pin => "M15 3.5l5.5 5.5-2.4.9-2.9 2.9.3 3.2L8 8.5l3.2.3 2.9-2.9.9-2.4ZM11.8 12.2 4 20",
        Glyph::Add => "M12 5v14M5 12h14",
        Glyph::Search => "M20 20l-4.2-4.2M17 10.5a6.5 6.5 0 1 1-13 0 6.5 6.5 0 0 1 13 0Z",
        Glyph::Bell => "M6 16V11a6 6 0 1 1 12 0v5l1.5 2h-15L6 16Zm4 4a2 2 0 0 0 4 0",
        Glyph::Settings => "M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Zm7.4-3a7.4 7.4 0 0 0-.1-1.2l2-1.6-2-3.4-2.4.9a7.5 7.5 0 0 0-2-1.2L14.5 3h-5l-.4 2.5a7.5 7.5 0 0 0-2 1.2l-2.4-.9-2 3.4 2 1.6a7.4 7.4 0 0 0 0 2.4l-2 1.6 2 3.4 2.4-.9a7.5 7.5 0 0 0 2 1.2l.4 2.5h5l.4-2.5a7.5 7.5 0 0 0 2-1.2l2.4.9 2-3.4-2-1.6c.1-.4.1-.8.1-1.2Z",
        Glyph::Sun => "M12 3v1.5m0 15V21m9-9h-1.5m-15 0H3m15.4-6.4-1 1M6.6 17.4l-1 1m12.8 0-1-1M6.6 6.6l-1-1M16 12a4 4 0 1 1-8 0 4 4 0 0 1 8 0Z",
        Glyph::Moon => "M20 14.5A8 8 0 0 1 9.5 4a8 8 0 1 0 10.5 10.5Z",
        Glyph::Monitor => "M4 5h16v10H4zM9 19h6m-3-4v4",
        Glyph::ChevronDown => "M6 9l6 6 6-6",
        Glyph::Clock => "M12 7v5l3 2m6-2a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
    };

    view! {
        <svg class="checko-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
            width=size height=size fill="none" stroke="currentColor" stroke-width="1.75"
            stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" focusable="false">
            <path d=shape />
        </svg>
    }
}
