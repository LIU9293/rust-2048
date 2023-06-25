use dioxus::html::input_data::keyboard_types::Key;
use dioxus::html::KeyboardEvent;
use dioxus::prelude::*;

use rand::seq::SliceRandom;
use rand::Rng;
use log::{info};
use crate::components::cell::Cell;

// [0,2,0,0]
// [0,2,0,0]
// [0,2,0,0]
// [0,2,0,0]
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

fn get_initial_board_data () -> [[i32; 4]; 4] {
  let mut init_board_data: [[i32; 4]; 4] = [[0; 4]; 4];
  let mut rng = rand::thread_rng();

  // Create a vector of all possible positions
  let mut positions: Vec<(usize, usize)> = (0..4).flat_map(|i| (0..4).map(move |j| (i, j))).collect();

  for _ in 0..3 {
      // Randomly choose a position and remove it from the vector
      if let Some((row, col)) = positions.choose(&mut rng).cloned() {
          positions.retain(|&r| r != (row, col));
          init_board_data[row][col] = 2;
      }
  }

  init_board_data
}

type Board = [[i32; 4]; 4];

fn add_random(board: &Board) -> Board {
  let mut rng = rand::thread_rng();
  let mut new_board = board.clone();
  let mut empty_cells = Vec::new();

  // Find all empty cells
  for (i, row) in new_board.iter().enumerate() {
      for (j, &cell) in row.iter().enumerate() {
          if cell == 0 {
              empty_cells.push((i, j));
          }
      }
  }

  // Choose a random empty cell
  if let Some(&(row, col)) = empty_cells.choose(&mut rng) {
      // Assign it a 2 or 4
      new_board[row][col] = if rng.gen_bool(0.5) { 2 } else { 4 };
  }

  new_board
}

fn move_up(board: &Board) -> Board {
  let mut new_board = board.clone();
  for col in 0..4 {
      let mut last_merge_position = 0;
      for row in 1..4 {
          if new_board[row][col] != 0 {
              let mut current_row = row;
              while current_row > last_merge_position && new_board[current_row - 1][col] == 0 {
                  // Move the number up
                  new_board[current_row - 1][col] = new_board[current_row][col];
                  new_board[current_row][col] = 0;
                  current_row -= 1;
              }
              if current_row > last_merge_position && new_board[current_row - 1][col] == new_board[current_row][col] {
                  // Merge the numbers
                  new_board[current_row - 1][col] *= 2;
                  new_board[current_row][col] = 0;
                  last_merge_position = current_row;
              }
          }
      }
  }
  add_random(&new_board)
}

fn move_down(board: &Board) -> Board {
  let mut new_board = board.clone();
  for col in 0..4 {
      let mut last_merge_position = 3;
      for row in (0..3).rev() {
          if new_board[row][col] != 0 {
              let mut current_row = row;
              while current_row < last_merge_position && new_board[current_row + 1][col] == 0 {
                  // Move the number down
                  new_board[current_row + 1][col] = new_board[current_row][col];
                  new_board[current_row][col] = 0;
                  current_row += 1;
              }
              if current_row < last_merge_position && new_board[current_row + 1][col] == new_board[current_row][col] {
                  // Merge the numbers
                  new_board[current_row + 1][col] *= 2;
                  new_board[current_row][col] = 0;
                  last_merge_position = current_row;
              }
          }
      }
  }
  add_random(&new_board)
}

fn move_left(board: &Board) -> Board {
  let mut new_board = board.clone();
  for row in 0..4 {
      let mut last_merge_position = 0;
      for col in 1..4 {
          if new_board[row][col] != 0 {
              let mut current_col = col;
              while current_col > last_merge_position && new_board[row][current_col - 1] == 0 {
                  // Move the number left
                  new_board[row][current_col - 1] = new_board[row][current_col];
                  new_board[row][current_col] = 0;
                  current_col -= 1;
              }
              if current_col > last_merge_position && new_board[row][current_col - 1] == new_board[row][current_col] {
                  // Merge the numbers
                  new_board[row][current_col - 1] *= 2;
                  new_board[row][current_col] = 0;
                  last_merge_position = current_col;
              }
          }
      }
  }
  add_random(&new_board)
}

fn move_right(board: &Board) -> Board {
  let mut new_board = board.clone();
  for row in 0..4 {
      let mut last_merge_position = 3;
      for col in (0..3).rev() {
          if new_board[row][col] != 0 {
              let mut current_col = col;
              while current_col < last_merge_position && new_board[row][current_col + 1] == 0 {
                  // Move the number right
                  new_board[row][current_col + 1] = new_board[row][current_col];
                  new_board[row][current_col] = 0;
                  current_col += 1;
              }
              if current_col < last_merge_position && new_board[row][current_col + 1] == new_board[row][current_col] {
                  // Merge the numbers
                  new_board[row][current_col + 1] *= 2;
                  new_board[row][current_col] = 0;
                  last_merge_position = current_col;
              }
          }
      }
  }
  add_random(&new_board)
}

pub fn Board(cx: Scope) -> Element {
  let init_board_data: Board = get_initial_board_data();
  let board_data: &UseState<Board> = use_state(cx, || init_board_data);

  let handle_key_down_event = move |evt: KeyboardEvent| match evt.key() {
      Key::ArrowUp => { board_data.set( move_up(board_data.get()) ) },
      Key::ArrowDown => { board_data.set( move_down(board_data.get()) ) },
      Key::ArrowLeft => { board_data.set( move_left(board_data.get()) ) },
      Key::ArrowRight => { board_data.set( move_right(board_data.get()) ) },
      _ => {}
  };

  cx.render(rsx!(
    div {
        style: "background-color: #bbada0; align-self: center;",
        class: "flex flex-col gap-2 p-4 rounded-md",
        onkeydown: handle_key_down_event,
        (0..4).map(|i| rsx!{ Row { cells: board_data[i] } })
        button {
          class: "but",
          onclick: move |evt| {
              println!("{evt:?}");
              info!("{evt:?}");
          },
          "Press me!"
      },
    }
  ))
}