#![allow(non_snake_case)]
use dioxus::prelude::*;
use components::board::Board;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            class: "flex flex-col p-4 align-center",
            Board {}
        }
    ))
}
