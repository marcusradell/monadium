use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct Props<'a> {
    title: &'a str,
}

pub fn PageTitle<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx!(h1 {"{cx.props.title}"}))
}
