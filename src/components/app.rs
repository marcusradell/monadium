use dioxus::prelude::*;

use crate::components::{Challenge, Course, Module, Study};

pub fn App(cx: Scope) -> Element {
    cx.render(rsx!(
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
    ))
}
