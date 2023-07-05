#![allow(non_snake_case)]
mod pages;
mod components;
use fermi::*;
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use crate::pages::game::Game;
use crate::pages::homepage::Homepage;
use crate::components::footer::Footer;
use crate::components::header::Header;

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
                    <link rel="stylesheet" href="tailwind.css">
                    <style>
                        html,body { width: 100%; }
                        #main { min-height: 100vh; width: 100%; }
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
                            setTimeout(function() { counter = 1 }, 20);
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
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);
    cx.render(rsx! (        
        Header{}
        Router {
            Route { to: "/", Homepage{} }
            Route { to: "/game", Game{} }
            Route { to: "/game/", Game{} }
            Route { to: "", Homepage{} }
        }
        Footer{}
    ))
}
