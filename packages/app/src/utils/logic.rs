use rand::seq::SliceRandom;
use rand::Rng;
use crate::utils::types::{Board, GameStatus};

pub fn get_initial_board_data () -> [[i32; 4]; 4] {
    let mut init_board_data: [[i32; 4]; 4] = [[0; 4]; 4];
    let mut rng = rand::thread_rng();
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

pub fn add_random(board: &Board) -> Board {
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

pub fn move_up(board: &Board) -> Board {
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
  new_board
}

pub fn move_down(board: &Board) -> Board {
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
  new_board
}

pub fn move_left(board: &Board) -> Board {
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
  new_board
}

pub fn move_right(board: &Board) -> Board {
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
  new_board
}

pub fn check_win(board: &Board) -> bool {
    for row in board.iter() {
        for &cell in row.iter() {
            if cell == 2048 {
                return true;
            }
        }
    }
    false
}

pub fn check_fail(board: &Board) -> bool {
    let new_board = board.clone();
    for row in new_board.iter() {
        for &cell in row.iter() {
            if cell == 0 {
                return false;
            }
        }
    }
    true
}

pub fn check_and_do_next (board_status: &Board) -> GameStatus {
    if check_win(board_status) {
        return GameStatus::Win;
    }
    if check_fail(board_status) {
        return GameStatus::Fail;
    }
    GameStatus::Playing
}