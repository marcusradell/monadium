use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct Props<'a> {
    title: &'a str,
}

pub fn ChapterTitle<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx!(h2 {"{cx.props.title}"}))
}
