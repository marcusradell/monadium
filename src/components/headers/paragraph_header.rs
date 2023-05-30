use dioxus::prelude::*;

pub fn ParagraphHeader(cx: Scope) -> Element {
    cx.render(rsx!(p {"Run code with Rust"}))
}
