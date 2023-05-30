use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct Props {
    text: String,
}

pub fn ChapterHeader(cx: Scope<Props>) -> Element {
    cx.render(rsx!(h3 {"{cx.props.text}"}))
}
