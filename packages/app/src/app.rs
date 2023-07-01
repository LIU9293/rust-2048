#![allow(non_snake_case)]
mod pages;
mod components;
mod utils;
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use crate::pages::game::Game;
use crate::pages::homepage::Homepage;

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            class: "flex flex-col p-4 align-center",
            Router {
                Route { to: "/", Homepage{} }
                Route { to: "/game", Game{} }
            }
        }
    ))
}