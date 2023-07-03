use serde::{Serialize, Deserialize};

pub type Row = [i32; 4];
pub type Board = [Row; 4];

#[derive(PartialEq)]
pub enum GameStatus {
    Playing,
    Win,
    Fail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressResponse {
    pub success: bool,
    pub board: Option<Board>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressReqeust {
    pub board: Board,
}
