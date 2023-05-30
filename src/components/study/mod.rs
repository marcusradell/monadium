use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
    children: Element<'a>,
}

pub fn Study<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx!(&cx.props.children))
}
