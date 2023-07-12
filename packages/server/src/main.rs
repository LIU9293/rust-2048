use axum::{
  response::Json,
  response::IntoResponse,
  routing::get,
  Router,
  extract::{State, Query},
  http::StatusCode
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::Any;
use shared::types::{
  UuidQuery,
  GetProgressResponse,
  SaveProgressResponse,
  ProgressReqeust,
  DbBoard,
  LeaderboardRow,
  LeaderboardResponse
};
use shared::logic::get_initial_board_data;
use postgrest::Postgrest;
use dotenv::dotenv;
use serde_json;

struct AppState {
  supabase_client: Postgrest,
}

impl Clone for AppState {
  fn clone(&self) -> Self {
      let supabase_url: String = dotenv::var("SUPABASE_URL")
      .expect("Need to set environment variable SUPABASE_URL") + "/rest/v1/";
      let client: Postgrest = Postgrest::new(supabase_url)
          .insert_header("apikey", dotenv::var("SUPABASE_API_KEY").unwrap());
      Self {
          supabase_client: client,
      }
  }
}

impl AppState {
  fn create() -> Arc<AppState> {     
      let supabase_url: String = dotenv::var("SUPABASE_URL")
          .expect("Need to set environment variable SUPABASE_URL") + "/rest/v1/";
      let client: Postgrest = Postgrest::new(supabase_url)
          .insert_header("apikey", dotenv::var("SUPABASE_API_KEY").unwrap());                      //2
      Arc::new(AppState { supabase_client: client })
  }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app_state = AppState::create();

    let app = Router::new()
      .route("/", get(ping))
      .route("/progress", get(get_user_progress).post(save_user_progress))
      .route("/leaderboard", get(get_leaderboard))
      .layer(
        tower_http::cors::CorsLayer::new()
          .allow_origin(Any)
          .allow_headers(Any)
          .allow_methods(Any),
      )
      .with_state(Arc::clone(&app_state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
      .serve(app.into_make_service())
      .await
      .expect("server should listen on port 3000");
}

async fn ping() -> &'static str {
    "Hello World!"
}

async fn save_user_progress(
    State(app_state): State<Arc<AppState>>,
    Json(input): Json<ProgressReqeust>
) -> impl IntoResponse {
    match input.uuid {
      Some(uuid) => {
        let client = &app_state.supabase_client;
        let board_str = serde_json::to_string(&input.board).unwrap();
        let score = input.board
          .iter()
          .fold(0, |acc, row| {
            acc + row.iter().fold(0, |acc, cell| {
              acc + cell
            })
          });

        let s = format!("[{{ \"uuid\": \"{}\", \"progress\": {}, \"score\": {} }}]",
          uuid, 
          board_str,
          score);

        match client
            .from("user_progress")
            .upsert(s)
            .on_conflict("uuid")
            .execute()
            .await {
            Ok(_) => {
              (StatusCode::OK, Json(SaveProgressResponse {
                success: true,
              }))
            },
            Err(_) => {
              get_default_save_response()
            },
        }
      }
      None => {
        get_default_save_response()
      },
    }
}

fn get_default_save_response() -> (StatusCode, Json<SaveProgressResponse>) {
    (StatusCode::OK, Json(SaveProgressResponse {
        success: false,
    }))
}

async fn get_user_progress(
  Query(q): Query<UuidQuery>,
  State(app_state): State<Arc<AppState>>,
) -> impl IntoResponse {
  match q.uuid {
      Some(uuid) => {
          let client = &app_state.supabase_client;
          match client
              .from("user_progress")
              .select("*")
              .eq("uuid", uuid)
              .execute()
              .await {
              Ok(res) => match res.text().await {
                  Ok(resp) => {

                    let board_data: DbBoard = serde_json::from_str(&resp).unwrap();
                    if board_data.len() == 0 {
                      return get_default_progress();
                    }
                    (StatusCode::OK, Json(GetProgressResponse {
                        success: true,
                        board: Some(board_data[0].progress),
                    }))
                  },
                  Err(_) => get_default_progress(),
              },
              Err(_) => get_default_progress(),
          }
      }
      None => get_default_progress(),
  }
}

fn get_default_progress() -> (StatusCode, Json<GetProgressResponse>) {
  (StatusCode::OK, Json(GetProgressResponse {
      success: true,
      board: Some(get_initial_board_data()),
  }))
}


async fn get_leaderboard(
  State(app_state): State<Arc<AppState>>,
) -> impl IntoResponse {
  let client = &app_state.supabase_client;

  match client
    .from("user_progress")
    .select("*")
    .order("score.desc")
    .limit(20)
    .execute()
    .await {
    Ok(res) => match res.text().await {
        Ok(resp) => {
          let leadboard_data: Vec<LeaderboardRow> = serde_json::from_str(&resp).unwrap();
          (StatusCode::OK, Json(LeaderboardResponse {
              success: true,
              leaderboard: leadboard_data
          }))
        },
        Err(_) => {
          (StatusCode::OK, Json(LeaderboardResponse {
            success: false,
            leaderboard: vec![]
          }))
        }
    }
    Err(_) => {
      (StatusCode::OK, Json(LeaderboardResponse {
        success: false,
        leaderboard: vec![]
      }))
    },
  }
}