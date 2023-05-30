#![allow(non_snake_case)]

mod components;

use crate::components::{Challenge, Course, Module, PageTitle, Study};
use axum::{response::Html, routing::get, Router};
use dioxus::prelude::*;

async fn app_endpoint() -> Html<String> {
    fn app(cx: Scope) -> Element {
        cx.render(rsx!(
            Course {
                PageTitle{
                    text: "Learning programming fundamentals with Rust".to_string()
                }
                Module {
                    Study {
                        Challenge{
                            title: "Hi!".to_string(),
                            assignment: "Write a program that outputs 'Hi!'.".to_string()
                        }
                        Challenge{
                            title: "Variables".to_string(),
                            assignment: "Declare immutable, mutable, constant, and shadowed variables.".to_string()
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
