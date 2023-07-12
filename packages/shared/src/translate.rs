use std::collections::HashMap;
use fermi::*;

pub struct Translator {
  default_language: String,
  translations: HashMap<(String, String), String>,
}

impl Translator {
  fn new() -> Self {
      Self {
          default_language: "en".to_string(),
          translations: HashMap::new(),
      }
  }

  pub fn change_to (&self, language: String) -> Self {
      Self {
          default_language: language,
          translations: self.translations.clone(),
      }
  }

  fn insert(&mut self, key: (String, String), value: String) {
      self.translations.insert(key, value);
  }

  pub fn get(&self, key: String) -> String {
      self.get_raw(&(key, self.default_language.clone()))
  }

  pub fn get_raw(&self, key: &(String, String)) -> String {
      match self.translations.get(key) {
          Some(value) => value.clone(),
          None => {
            let ret = match self.translations.get(&(key.0.clone(), "en".to_string())) {
              Some(value) => value.clone(),
              None => key.0.clone(),
            };

            ret
          },
      }
  }
}

pub static TRANSLATION_MAP: [((&str, &str), &str); 9] = [
    (("homepage.title", "en"), "Rust 2048"),
    (("homepage.subtitle", "en"), "It's a demo application for me to figure out if I can use rust/dioxus in production code, this codebase contains a frontend client that can compiled to Web(wasm) and Desktop, a simple rust server to handle some basic API logic and some shared components between client and server."),
    (("homepage.subtitle", "zh"), "这是一个为了联系Rust而做的2048小游戏，同时也用来测试Dioxus是否适合用作production的环境中来做Full Stack的Web Application."),
    (("homepage.start_game", "en"), "Start Game"),
    (("homepage.start_game", "zh"), "开始游戏"),
    (("game.total_score", "en"), "Total Score"),
    (("game.total_score", "zh"), "总分"),
    (("game.highest_block", "en"), "Highest Block"),
    (("game.highest_block", "zh"), "最高分"),
];

pub static TRANSLATION: Atom<Translator> = |_| {
  let mut translator = Translator::new();

  for (key, value) in TRANSLATION_MAP.iter() {
      translator.insert((key.0.to_string(), key.1.to_string()), value.to_string());
  }
  
  translator
};
