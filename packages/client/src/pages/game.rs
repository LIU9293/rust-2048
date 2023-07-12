use fermi::*;
use dioxus::prelude::*;
use dioxus::html::input_data::keyboard_types::Key;
use dioxus::html::KeyboardEvent;
use shared::types::{Board, GameStatus, ProgressReqeust};
use reqwest;
use shared::logic::{get_initial_board_data, add_random, move_up, move_down, move_left, move_right, check_and_do_next};
use uuid::Uuid;
use crate::components::row::Row;
use shared::translate::TRANSLATION;
use gloo_storage::{LocalStorage, Storage, errors::StorageError};

pub fn Game(cx: Scope) -> Element {
    let game_status = use_state(cx, || GameStatus::Playing);
    let board_data: &UseState<Board> = use_state(cx, || get_initial_board_data());
    let is_first_load = use_state(cx, || true);
    let translator = use_read(cx, TRANSLATION);

    use_effect(cx, (is_first_load, board_data, game_status), |(is_first_load, board_data, game_status)| async move {
        if !is_first_load.get() {
            return;
        }

        let a: Result<String, StorageError> = LocalStorage::get("uuid");
        match a {
            Ok(uuid) => {
                let client = reqwest::Client::new();
                let url = format!("https://rust-2048-api.onrender.com/progress?uuid={uuid}", uuid=uuid);
                let res = client.get(url).send().await;
                match res {
                    Ok(response) => {
                        let payload = response.json::<ProgressReqeust>().await;
                        match payload {
                            Ok(data) => {
                                is_first_load.set(false);
                                board_data.set(data.board);

                                match check_and_do_next(&data.board) {
                                    GameStatus::Win => {
                                        game_status.set(GameStatus::Win);
                                    },
                                    GameStatus::Fail => {
                                        game_status.set(GameStatus::Fail);
                                    },
                                    GameStatus::Playing => { game_status.set(GameStatus::Playing); },
                                }
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
            },
            StorageError => {
                log::info!("No uuid found, creating one...");
                let uuid = Uuid::new_v4();
                LocalStorage::set("uuid", uuid.to_string()).unwrap();
            }
        };
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
                let a: Result<String, StorageError> = LocalStorage::get("uuid");
                match a {
                    Ok(uuid) => {
                        let client = reqwest::Client::new();
                        let res = client.post("https://rust-2048-api.onrender.com/progress")
                            .json(&ProgressReqeust {
                                board: new_data,
                                uuid: Some(uuid),
                            })
                            .send()
                            .await;
                        
                        match res {
                            Ok(_) => {},
                            Err(err) => {
                                log::error!("Failed to record progress: {}", err);
                            }
                        }
                    },
                    StorageError => {
                        log::info!("No uuid found, skip...");
                    }
                }
            }
        });
    };

    let restart_game = |_| -> () {
        board_data.set(get_initial_board_data());
        game_status.set(GameStatus::Playing);
        
        // reset game uuid
        let uuid = Uuid::new_v4();
        LocalStorage::set("uuid", uuid.to_string()).unwrap();
    };
    
    let total_score = board_data.get().iter().flatten().sum::<i32>();
    let highest_score = board_data.get().iter().flatten().max().unwrap_or(&0);

    cx.render(rsx!(
        div {
            class: "flex flex-col items-center bg-base-200",
            div {
                class: "card inline-block bg-base-100 shadow-xl mx-auto my-16 p-8",
                id: "gamearea",
                onkeydown: handle_key_down_event,
                div {
                    class: "stats flex my-4",
                    div {
                        class: "stat",
                        div { class: "stat-title", translator.get("game.total_score".to_string()) }
                        div { class: "stat-value text-primary", format!("{}", total_score) }
                    }
                    div {
                        class: "stat",
                        div { class: "stat-title", translator.get("game.highest_block".to_string()) }
                        div { class: "stat-value text-primary", format!("{}", highest_score) }
                    }
                }
                div {
                    class: "flex flex-col gap-4",
                    (0..4).map(|i| rsx!{Row{ cells: board_data[i] }})
                }
                match game_status.get() {
                    GameStatus::Win => rsx!( div {
                        class: "flex flex-col items-center",
                        div {
                            class: "text-center text-lg",
                            "You Win!" 
                        }
                        button {
                                class: "bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 rounded",
                                onclick: restart_game,
                                "Restart"
                        }
                    } ),
                    GameStatus::Fail => rsx!( div { 
                        class: "flex flex-col items-center mt-8",
                        div {
                            class: "text-center text-xl font-bold",
                            "You Failed!"
                        }
                        button {
                                class: "btn btn-primary mt-4",
                                onclick: restart_game,
                                "Restart"
                        }
                    } ),
                    _ => rsx!(div{})
                }
            }    
        }
    ))
}