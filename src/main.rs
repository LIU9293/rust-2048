#![allow(non_snake_case)]
mod components;
use dioxus::prelude::*;
use crate::components::board::Board;
use dioxus_router::{Link, Route, Router};

#[cfg(not(target_arch = "wasm32"))]
use dioxus_desktop::Config;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    dioxus_desktop::launch_cfg(
        app,
        Config::new().with_custom_index(
            r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Rust 2048</title>
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <script src="https://cdn.tailwindcss.com"></script>
                    <style>
                        html,
                        body {
                            background-color: #495057;
                            color: white;
                        }

                        #main {
                            min-height: 100vh;
                            width: 100%;
                        }
                    </style>
                </head>
                <body>
                    <div id="main"></div>
                    <script>
                    // PLEASE HELP ME FIX THIS SHIT !!!
                    let counter = 1;
                    document.addEventListener('keydown', function(e) {
                        if (!document.getElementById('gamearea')) {
                            return
                        }
                        if (counter === 1) {
                        counter = 0
                        new_e = new e.constructor(e.type, e);
                        gamearea.dispatchEvent(new_e);
                        setTimeout(function() {
                            counter = 1
                        }, 100);
                        }
                    });
                    </script>
                </body>
                </html>
            "#
            .into(),
        ),
    );
    
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            class: "flex flex-col p-4 align-center",
            Router {
                Route { to: "/game", Board {} }
                Route {
                    to: "/",
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
                }
            }
        }
    ))
}
