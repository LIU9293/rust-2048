use dioxus::prelude::*;
use dioxus_router::{Link};

pub fn Homepage(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            style: "align-self: center;",
            class: "flex flex-col",            
            div {
                class: "text-center mb-4",
                "homepage"
            },
            Link { to: "/game", 
                button { 
                    class: "text-xs rounded-full bg-sky-500 text-white px-4 py-2",
                    "Start Game"
                }
            }
        }
    ))
}