use dioxus::prelude::*;

use crate::web::components::SectionTitle;

#[derive(Props)]
pub struct Props<'a> {
    title: &'a str,
    children: Element<'a>,
}

pub fn Study<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx!(
        SectionTitle {
            title: cx.props.title
        } & cx.props.children
    ))
}
