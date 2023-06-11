use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct Props<'a> {
    code: &'a str,
}

pub fn CodeBlockExample<'a>(cx: Scope<'a, Props<'a>>) -> Element {
    cx.render(rsx!(pre { class: "font-mono",
        code {
            "{cx.props.code}"
        }
    }))
}
