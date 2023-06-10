use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct Props<'a> {
    title: &'a str,
}

pub fn SectionTitle<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx!(h3 {"{cx.props.title}"}))
}
