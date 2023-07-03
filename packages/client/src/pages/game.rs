use dioxus::prelude::*;
use dioxus::html::input_data::keyboard_types::Key;
use dioxus::html::KeyboardEvent;
use shared::types::{Board, GameStatus, ProgressReqeust};
use reqwest;
use shared::logic::{get_initial_board_data, add_random, move_up, move_down, move_left, move_right, check_and_do_next};
use log;
use crate::components::row::Row;

pub fn Game(cx: Scope) -> Element {
    let game_status = use_state(cx, || GameStatus::Playing);
    let board_data: &UseState<Board> = use_state(cx, || get_initial_board_data());
    let is_first_load = use_state(cx, || true);

    use_effect(cx, (is_first_load, board_data), |(is_first_load, board_data)| async move {
        if !is_first_load.get() {
            return;
        }
            
        let client = reqwest::Client::new();
        let res = client.get("http://localhost:3000/progress").send().await;
        match res {
            Ok(response) => {
                let payload = response.json::<ProgressReqeust>().await;
                match payload {
                    Ok(data) => {
                        is_first_load.set(false);
                        board_data.set(data.board);
                    },
                    Err(err) => {
                        log::error!("Failed to parse JSON: {}", err);
                    }
                }
            },
            Err(err) => {
                log::error!("Failed to send request: {}", err);
            }
        }
    });

    let handle_key_down_event = move |evt: KeyboardEvent| -> () {
        if *game_status.get() != GameStatus::Playing {
            return;
        }

        let new_data: Board = match evt.key() {
            Key::ArrowUp => add_random(&move_up(board_data.get())),
            Key::ArrowDown => add_random(&move_down(board_data.get())),
            Key::ArrowLeft => add_random(&move_left(board_data.get())),
            Key::ArrowRight => add_random(&move_right(board_data.get())),
            _ => board_data.get().clone()
        };

        if new_data == *board_data.get() {
            return;
        }

        match check_and_do_next(&new_data) {
            GameStatus::Win => {
                board_data.set(new_data);
                game_status.set(GameStatus::Win);
            },
            GameStatus::Fail => {
                board_data.set(new_data);
                game_status.set(GameStatus::Fail);
            },
            GameStatus::Playing => { board_data.set(new_data); },
        }    
        
        cx.spawn({
            async move {
                let client = reqwest::Client::new();
                let res = client.post("http://localhost:3000/progress")
                    .json(&ProgressReqeust {
                        board: new_data
                    })
                    .send()
                    .await;
                
                match res {
                    Ok(_) => {},
                    Err(err) => {
                        log::error!("Failed to record progress: {}", err);
                    }
                }
            }
        });
    };
    
    let total_score = board_data.get().iter().flatten().sum::<i32>();
    let highest_score = board_data.get().iter().flatten().max().unwrap_or(&0);

    cx.render(rsx!(
        div {
            style: "align-self: center;",
            class: "flex flex-col",
            div {
                style: "background-color: #bbada0; margin-top: 100px;",
                class: "flex flex-col gap-2 p-4 rounded-md focus:outline-none",
                id: "gamearea",
                onkeydown: handle_key_down_event,
                div {
                    class: "text-center text-2xl font-bold",
                    "Rust 2048"
                }
                div {
                    class: "flex flex-col",
                    div {
                        class: "flex flex-1",
                        format!("Total: {}", total_score)
                    }
                    div {
                        class: "flex flex-1",
                        format!("Highest: {}", highest_score)
                    }
                }
                (0..4).map(|i| rsx!{Row{ cells: board_data[i] }})
                match game_status.get() {
                    GameStatus::Win => rsx!( div {
                        class: "flex flex-col items-center",
                        div {
                            class: "text-center text-lg",
                            "You Win!" 
                        }
                        button {
                                class: "bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 rounded",
                                onclick: |_| {
                                    board_data.set(get_initial_board_data());
                                    game_status.set(GameStatus::Playing);
                                },
                                "Restart"
                        }
                        
                    } ),
                    GameStatus::Fail => rsx!( div { 
                        class: "flex flex-col items-center",
                        div {
                            class: "text-center text-lg",
                            "You Failed!" 
                        }
                        button {
                                class: "bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 rounded",
                                onclick: |_| {
                                    board_data.set(get_initial_board_data());
                                    game_status.set(GameStatus::Playing);
                                },
                                "Restart"
                        }
                    } ),
                    _ => rsx!(div{})
                }
            }
            div {
                class: "flex flex-col mt-4 text-sm",
                p {
                    class: "text-slate-400",
                    "Learn rust with Dioxus"
                }
                a {
                    class: "text-slate-300",
                    href: "https://github.com/LIU9293/rust-2048",
                    "> Github"
                }
            }
        }
    ))
}