#![allow(non_snake_case)]

// mod app;
// pub use app::*;

// mod kits;
// pub mod primitives;

// TODO: convert to v0.4 fullstack
fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("web"));

    log::info!("Message on my console");
    // Launch the web application using the App component as the root.
    // dioxus_web::launch(App);
}
