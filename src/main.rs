#![allow(non_snake_case)]

mod components;

use crate::components::{Challenge, Course, Module, Study};
use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;

async fn app_endpoint() -> Html<String> {
    fn app(cx: Scope) -> Element {
        cx.render(rsx!(
            Course {
                title: "Learning programming fundamentals with Rust"
                Module {
                    title: "Introduction"
                    Study {
                        Challenge{
                            title: "Hi!",
                            assignment: "Write a program that outputs 'Hi!'."
                        }
                        Challenge{
                            title: "Variables",
                            assignment: "Declare immutable, mutable, constant, and shadowed variables."
                        }
                    }
                }
            }
        ))
    }

    let mut app = VirtualDom::new(app);

    let _ = app.rebuild();

    Html(dioxus_ssr::render(&app))
}

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(
            Router::new()
                .route("/", get(app_endpoint))
                .into_make_service(),
        )
        .await
        .unwrap();
}
