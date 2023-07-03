use axum::{
  response::Json,
  response::IntoResponse,
  routing::get,
  extract::{State},
  Router,
  http::StatusCode
};
use std::net::SocketAddr;
use std::{collections::HashMap, sync::{Arc, RwLock}};
use tower_http::cors::{Any};
use shared::types::{Board, ProgressResponse, ProgressReqeust};

type Db = Arc<RwLock<HashMap<String, Board>>>;

#[tokio::main]
async fn main() {
    let shared_state = Db::default();

    let app = Router::new()
      .route("/", get(ping))
      .route("/progress", get(get_user_progress).post(save_user_progress))
      .layer(
        tower_http::cors::CorsLayer::new()
          .allow_origin(Any)
          .allow_headers(Any)
          .allow_methods(Any),
      )
      .with_state(Arc::clone(&shared_state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
      .serve(app.into_make_service())
      .await
      .expect("server should listen on port 3000");
}

async fn ping() -> &'static str {
    "Hello World!"
}

async fn save_user_progress(
  State(db): State<Db>,
  Json(input): Json<ProgressReqeust>
) -> impl IntoResponse {
    let board: Board = input.board;    
    match db.write() {
        Ok(mut db_instance) => {
            db_instance.insert("test".to_string(), board);
            (StatusCode::CREATED, Json(ProgressResponse{
              success: true,
              board: Some(board)
            }))
        },
        Err(err) => {
            println!("Error: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ProgressResponse{
              success: false,
              board: None
            }))
        }
    }
}

async fn get_user_progress(State(db): State<Db>) -> impl IntoResponse {
    match db.read() {
      Ok(db_instance) => {
        let board: Option<Board> = db_instance.get("test").cloned();
        match board {
          Some(b) => (StatusCode::OK, Json(ProgressResponse {
              success: true,
              board: Some(b)
          })),
          None => (StatusCode::NOT_FOUND, Json(ProgressResponse{ 
              success: false,
              board: None 
          }))
      }
      },
      Err(err) => {
        println!("Error: {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ProgressResponse{
          success: false,
          board: None
        }))
      }
    }   
}