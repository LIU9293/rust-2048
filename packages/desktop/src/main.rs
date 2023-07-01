#![allow(non_snake_case)]
use dioxus::prelude::*;
use app::app::App;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            class: "flex flex-col p-4 align-center",
            App {}
        }
    ))
}
