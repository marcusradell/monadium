use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct Props {
    text: String,
}

pub fn PageTitle(cx: Scope<Props>) -> Element {
    cx.render(rsx!(h1 {"{cx.props.text}"}))
}
