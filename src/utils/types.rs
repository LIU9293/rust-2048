pub type Row = [i32; 4];

pub type Board = [Row; 4];

#[derive(PartialEq)]
pub enum GameStatus {
    Playing,
    Win,
    Fail,
}