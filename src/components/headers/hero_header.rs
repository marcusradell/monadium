use dioxus::prelude::*;

pub fn HeroHeader(cx: Scope) -> Element {
    cx.render(rsx!(h1 {"Monadium.org"}))
}
