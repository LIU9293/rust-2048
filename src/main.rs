#![allow(non_snake_case)]
mod components;
use dioxus::prelude::*;
use crate::components::board::Board;

fn main() {
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    dioxus_web::launch(app);
    // dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            class: "flex flex-col p-4 align-center",
            Board {}
        }
    ))
}
