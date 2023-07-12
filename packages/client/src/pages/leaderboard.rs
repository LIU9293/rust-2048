use dioxus::prelude::*;
use shared::types::{LeaderboardResponse, LeaderboardRow};
use reqwest;

pub fn Leaderboard (cx: Scope<>) -> Element {

    let leaderboard_data: &UseState<Vec<LeaderboardRow>>  = use_state(cx, ||vec![]);

    use_effect(cx, leaderboard_data, |leaderboard_data| async move {
      if leaderboard_data.get().len() > 0 {
        return;
      }

      let client = reqwest::Client::new();
      let url = "https://rust-2048-api.onrender.com/leaderboard".to_string();
      let res = client.get(url).send().await;

      match res {
        Ok(response) => {
          let payload = response.json::<LeaderboardResponse>().await;
          match payload {
            Ok(data) => {
                leaderboard_data.set(data.leaderboard);
              },
              Err(err) => {
                  log::error!("Failed to parse JSON: {}", err);
              }
          }
        }
        Err(err) => { log::error!("Failed to send request: {}", err) }
      }
    });

    cx.render(rsx!(
        div {
          class: "py-36 bg-base-200 flex-col flex items-center",
          h1 {
            class: "text-5xl font-bold mb-8",
            "Top 20 players"
          }
          leaderboard_data.get().iter().map(|game| {
            rsx!(div {
              class: "card mb-8 w-96 p-8 bg-base-100 shadow-xl",
              div {
                format!("Player: {}", game.uuid)
              }
              div {
                format!("Score: {}", game.score)
              }
            })
          })
        }
    ))
}