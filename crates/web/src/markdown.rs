//! Safe Markdown -> HTML for note previews (raw HTML in notes is shown as text).

use pulldown_cmark::{html, Event, Options, Parser};

pub fn to_html(md: &str) -> String {
    let opts = Options::ENABLE_TASKLISTS | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES;
    let events = Parser::new_ext(md, opts).map(|e| match e {
        // Never render HTML typed into a note; show it as plain text instead.
        Event::Html(t) | Event::InlineHtml(t) => Event::Text(t),
        other => other,
    });
    let mut out = String::new();
    html::push_html(&mut out, events);
    out
}
