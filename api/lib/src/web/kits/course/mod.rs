use dioxus::prelude::*;

use crate::web::primitives::PageTitle;

#[derive(Props)]
pub struct Props<'a> {
    title: &'a str,
    children: Element<'a>,
}

pub fn Course<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx!(
        PageTitle {
            title: cx.props.title
        }
        div {
            &cx.props.children
        }
    ))
}
