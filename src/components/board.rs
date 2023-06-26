use dioxus::prelude::*;
use dioxus::html::input_data::keyboard_types::Key;
use dioxus::html::KeyboardEvent;

use crate::components::cell::Cell;
use crate::components::logic::{Board, move_up, move_down, move_left, move_right, add_random, get_initial_board_data, check_fail, check_win};

#[derive(PartialEq, Props)]
pub struct RowProps {
    cells: [i32; 4],
}

pub fn Row (cx: Scope<RowProps>) -> Element {
  cx.render(rsx!(
    div {
        class: "flex flex-row gap-2",
        (0..4).map(|i| rsx!{ Cell { score: cx.props.cells[i] } })
    }
  ))
}

#[derive(PartialEq)]
enum GameStatus {
    Win,
    Fail,
    Playing,
}

fn check_and_do_next (board_status: &Board) -> GameStatus {
    if check_win(board_status) {
        return GameStatus::Win;
    }
    if check_fail(board_status) {
        return GameStatus::Fail;
    }
    GameStatus::Playing
}

pub fn Board(cx: Scope) -> Element {
    let game_status = use_state(cx, || GameStatus::Playing);
    let init_board_data: Board = get_initial_board_data();
    let board_data: &UseState<Board> = use_state(cx, || init_board_data);
    
    let handle_key_down_event = move |evt: KeyboardEvent|  {
        if *game_status.get() != GameStatus::Playing {
            return;
        }

        let new_data: Option<Board> = match evt.key() {
            Key::ArrowUp => Some(add_random(&move_up(board_data.get()))),
            Key::ArrowDown => Some(add_random(&&move_down(board_data.get()))),
            Key::ArrowLeft => Some(add_random(&move_left(board_data.get()))),
            Key::ArrowRight => Some(add_random(&&move_right(board_data.get()))),
            _ => None
        };

        match new_data {
            Some(new_data) => {
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
            },
            None => {}
        }
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
    
                (0..4).map(|i| rsx!{ Row { cells: board_data[i] } })
    
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
                    class: "text-sm text-slate-400",
                    "Learning rust using Dioxus"
                }
                a {
                    class: "text-sm text-slate-300",
                    href: "https://github.com/LIU9293/rust-2048",
                    "Github"
                }
            }
        }
    ))
}