use dioxus::prelude::*;

use crate::web::{
    kits::{Challenge, Course, Module, Study},
    primitives::CodeBlockExample,
};

const BODY_STYLE: &str = r"
    font-size: 16px;
";

const HI_CODE: &str = r#"fn main() {
    println!("Hi!");
}"#;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

pub fn App(cx: Scope) -> Element {
    println!("In App.");

    let fut = use_future(cx, (), |_| async move {
        println!("Future begun...");
        reqwest::get("https://dog.ceo/api/breeds/image/random/")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
    });

    cx.render(match fut.value() {
        Some(Ok(resp)) => rsx! {
            button {
                onclick: move |_| fut.restart(),
                "Click to fetch another doggo"
            }
            div {
                img {
                    max_width: "500px",
                    max_height: "500px",
                    src: "{resp.message}",
                }
            }
        },
        Some(Err(_)) => rsx! { div { "loading dogs failed" } },
        None => rsx! { div { "loading dogs..." } },
    })

    // cx.render(rsx!(
    //     head {
    //     title { "Monadium.org"}
    //     script {src:"https://cdn.tailwindcss.com"}
    //     }

    //     body { style: BODY_STYLE, class: "font-sans bg-stone-900 text-stone-100",
    //     Course {
    //         title: "Programming Fundamentals with Rust"
    //         Module {
    //             title: "Introduction"
    //             Study {
    //                 title: "Your First Program"
    //                 Challenge {
    //                     name: "Hi!",
    //                     content: "Write a program that outputs 'Hi!'."
    //                 }
    //                 CodeBlockExample {
    //                     code: HI_CODE
    //                 }
    //             }
    //         }

    //         Module {
    //             title: "Fundamentals"
    //             Study {
    //                 title: "Defining Data"
    //                 Challenge {
    //                     name: "Variables",
    //                     content: "Declare immutable, mutable, constant, and shadowed variables."
    //                 }
    //             }
    //         }
    //     }
    // }
    // ))
}
