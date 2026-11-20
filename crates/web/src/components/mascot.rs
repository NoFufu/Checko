use leptos::prelude::*;

/// Animation states, one sprite strip each (see assets/mascot).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum Mood {
    Idle,
    Blink,
    Wave,
    WalkRight,
    WalkLeft,
    Crawl,
    Surprised,
    Tablet,
    ThumbsUp,
}

impl Mood {
    fn sprite(self) -> (&'static str, u32, u32) {
        // (file, frames, duration ms for one loop)
        match self {
            Mood::Idle => ("idle", 6, 1400),
            Mood::Blink => ("blink", 6, 1600),
            Mood::Wave => ("wave", 4, 700),
            Mood::WalkRight => ("walk-right", 8, 900),
            Mood::WalkLeft => ("walk-left", 8, 900),
            Mood::Crawl => ("crawl", 5, 900),
            Mood::Surprised => ("surprised", 8, 1400),
            Mood::Tablet => ("tablet", 6, 1500),
            Mood::ThumbsUp => ("thumbs-up", 6, 1200),
        }
    }
}

/// The axolotl mascot, animated with CSS `steps()`.
#[component]
pub fn Mascot(
    #[prop(into)] mood: Signal<Mood>,
    #[prop(default = 140)] size: u32,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let style = move || {
        let (file, frames, ms) = mood.get().sprite();
        format!(
            "--size:{size}px;--frames:{frames};--dur:{ms}ms;background-image:url({})",
            crate::asset(&format!("mascot/{file}.webp"))
        )
    };
    view! { <div class=format!("mascot {class}") style=style role="img" aria-label="Axolotl-Maskottchen"></div> }
}
