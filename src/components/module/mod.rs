use dioxus::prelude::*;

use crate::components::ChapterTitle;

#[derive(Props)]
pub struct Props<'a> {
    title: &'a str,
    children: Element<'a>,
}

pub fn Module<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx!(
        ChapterTitle {
            title: cx.props.title
        } & cx.props.children
    ))
}
