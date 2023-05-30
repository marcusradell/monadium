use dioxus::prelude::*;

use crate::components::ChapterHeader;

#[derive(PartialEq, Props)]
pub struct Props {
    pub title: String,
    pub assignment: String,
}

pub fn Challenge(cx: Scope<Props>) -> Element {
    cx.render(rsx!(
    div {
        ChapterHeader {text: cx.props.title.clone()}
        div {"{cx.props.assignment}"}
    }))
}
