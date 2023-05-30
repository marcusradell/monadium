use dioxus::prelude::*;

pub fn PageHeader(cx: Scope) -> Element {
    cx.render(rsx!(h2 {"Learning programming fundamentals with Rust"}))
}
