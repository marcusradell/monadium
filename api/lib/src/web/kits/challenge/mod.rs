use dioxus::prelude::*;

use crate::web::primitives::ChapterTitle;

#[derive(PartialEq, Props)]
pub struct Props<'a> {
    pub name: &'a str,
    pub content: &'a str,
}

pub fn Challenge<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx!(
    div {
        ChapterTitle {title: cx.props.name}
        div {"{cx.props.content}"}
    }))
}
