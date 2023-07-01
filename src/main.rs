mod controller;
mod model;
mod view;

use crate::{
  controller::PokemonController,
  model::PokemonEntity
};
use axum::{
  routing::{get, post, patch, delete},
  Router
};
use std::{
  net::SocketAddr,
  sync::{Arc, Mutex},
  collections::HashMap
};

#[tokio::main]
async fn main() {
  let app = app();
  let address = SocketAddr::from(([127, 0, 0, 1], 3000)); // <- localhost:3000
  axum::Server::bind(&address)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

fn app() -> Router {
  let mut hash_map = HashMap::<i32, PokemonEntity>::new(); // <- adjust to your needs

  // insert some initial state:
  hash_map.insert(1, PokemonEntity {
    name: "Bulbasaur".to_string(),
    nick_name: "Hasso".to_string(),
    number: 1,
    r#type: "Grass".to_string(),
  });
  hash_map.insert(9, PokemonEntity {
    name: "Blastoise".to_string(),
    nick_name: "Blaster".to_string(),
    number: 9,
    r#type: "Water".to_string(),
  });

  let thread_safe_hash_map = Arc::new(Mutex::new(hash_map));
  let database = thread_safe_hash_map;

  let app = Router::new()
    .route("/pokemon", get(PokemonController::list))
    .route("/pokemon", post(PokemonController::create))
    .route("/pokemon/:id", get(PokemonController::read))
    .route("/pokemon/:id", patch(PokemonController::update))
    .route("/pokemon/:id", delete(PokemonController::delete))
    .with_state(database); // <- gives your server (handlers) access to database
  app
}
