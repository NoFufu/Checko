use std::time::Duration;

use leptos::prelude::*;

use crate::store::{Celebration, Store};

const COLORS: [&str; 5] = ["#4caf7a", "#e8c547", "#f08c3a", "#a48cd9", "#f4a587"];

/// Confetti + card for level-ups and new achievements. Closes by itself.
#[component]
pub fn CelebrationHost() -> impl IntoView {
    let store = Store::get();
    let current = move || store.celebrations.with(|c| c.first().cloned());

    // Auto-dismiss after a few seconds.
    Effect::new(move |_| {
        if current().is_some() {
            let len = store.celebrations.with_untracked(Vec::len);
            set_timeout(
                move || {
                    if store.celebrations.with_untracked(Vec::len) == len {
                        store.dismiss_celebration();
                    }
                },
                Duration::from_millis(4200),
            );
        }
    });

    view! {
        {move || current().map(|c| {
            let (emoji, kicker, title, text) = match c {
                Celebration::LevelUp(l) => ("⭐".to_string(), "Level‑Up!".to_string(), format!("Level {l}"), "Stark! Weiter so.".to_string()),
                Celebration::Achievement(a) => (a.emoji.to_string(), "Erfolg freigeschaltet".to_string(), a.title.to_string(), a.description.to_string()),
            };
            let confetti = (0..28)
                .map(|i| {
                    let left = (i * 37) % 100;
                    let delay = (i * 53) % 600;
                    let color = COLORS[i % COLORS.len()];
                    let rot = (i * 47) % 360;
                    view! {
                        <span
                            class="confetti"
                            style=format!("left:{left}%;animation-delay:{delay}ms;background:{color};transform:rotate({rot}deg)")
                        ></span>
                    }
                })
                .collect_view();
            view! {
                <div class="celebration" role="status" aria-live="polite" on:click=move |_| store.dismiss_celebration()>
                    <div class="confetti-layer" aria-hidden="true">{confetti}</div>
                    <div class="celebration-card" on:click=|e| e.stop_propagation()>
                        <div class="celebration-emoji" aria-hidden="true">{emoji}</div>
                        <p class="celebration-kicker">{kicker}</p>
                        <h2 class="celebration-title">{title}</h2>
                        <p class="text-muted">{text}</p>
                        <button type="button" class="btn btn-primary mt-4" on:click=move |_| store.dismiss_celebration()>
                            "Weiter"
                        </button>
                    </div>
                </div>
            }
        })}
    }
}
