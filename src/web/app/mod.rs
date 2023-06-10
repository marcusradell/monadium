use dioxus::prelude::*;

use crate::web::kits::{Challenge, Course, Module, Study};

const BODY_STYLE: &str = r"
    font-family: Arial, Helvetica, sans-serif;
";

pub fn App(cx: Scope) -> Element {
    cx.render(rsx!(
        head {
        title { "Monadium.org"}
        script {src:"https://cdn.tailwindcss.com"}
        }

        body { style: BODY_STYLE, class: "bg-slate-900 text-slate-100",
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
