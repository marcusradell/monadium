use dioxus::prelude::*;

use crate::components::ChapterTitle;

#[derive(PartialEq, Props)]
pub struct Props<'a> {
    pub title: &'a str,
    pub assignment: &'a str,
}

pub fn Challenge<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx!(
    div {
        ChapterTitle {title: cx.props.title}
        div {"{cx.props.assignment}"}
    }))
}
