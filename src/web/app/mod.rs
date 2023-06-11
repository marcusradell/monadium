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

pub fn App(cx: Scope) -> Element {
    cx.render(rsx!(
        head {
        title { "Monadium.org"}
        script {src:"https://cdn.tailwindcss.com"}
        }

        body { style: BODY_STYLE, class: "font-sans bg-stone-900 text-stone-100",
        Course {
            title: "Programming Fundamentals with Rust"
            Module {
                title: "Introduction"
                Study {
                    title: "Your First Program"
                    Challenge {
                        title: "Hi!",
                        assignment: "Write a program that outputs 'Hi!'."
                    }
                    CodeBlockExample {
                        code: HI_CODE
                    }
                }
            }

            Module {
                title: "Fundamentals"
                Study {
                    title: "Defining Data"
                    Challenge {
                        title: "Variables",
                        assignment: "Declare immutable, mutable, constant, and shadowed variables."
                    }
                }
            }
        }
    }
    ))
}
