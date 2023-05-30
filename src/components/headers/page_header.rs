use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct Props {
    text: String,
}

pub fn CourseTitle(cx: Scope<Props>) -> Element {
    cx.render(rsx!(h1 {"{cx.props.text}"}))
}
