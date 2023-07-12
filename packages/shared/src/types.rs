use serde::{Serialize, de, Deserialize, Deserializer};
use std::{fmt, str::FromStr};

pub type Row = [i32; 4];
pub type Board = [Row; 4];

#[derive(PartialEq)]
pub enum GameStatus {
    Playing,
    Win,
    Fail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetProgressResponse {
    pub success: bool,
    pub board: Option<Board>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveProgressResponse {
    pub success: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressReqeust {
    pub uuid: Option<String>,
    pub board: Board,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UuidQuery {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub uuid: Option<String>
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbBoardRow {
    pub id: i32,
    pub uuid: String,
    pub progress: Board,
  }
  
pub type DbBoard = Vec<DbBoardRow>;
  
#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardRow {
    pub uuid: String,
    pub score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardResponse {
    pub success: bool,
    pub leaderboard: Vec<LeaderboardRow>,
}