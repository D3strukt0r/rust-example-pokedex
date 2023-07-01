use axum::{
  extract::{Path, Query, State},
  http::StatusCode,
  Json,
};
use axum_macros::debug_handler;
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};
use crate::{
  model::PokemonEntity,
  view::{FilterOptions, CreatePokemon, UpdatePokemon, PokemonShow, PokemonList},
};

pub struct PokemonController;
impl PokemonController {
  // #[debug_handler]
  pub async fn list(
    opts: Option<Query<FilterOptions>>,
    State(db): State<Arc<Mutex<HashMap<i32, PokemonEntity>>>>,
  ) -> Result<Json<PokemonList>, StatusCode> {
    let db = db.lock().unwrap(); // usually you should handle errors ;)
    
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let pokemons = db
      .values()
      .skip(offset)
      .take(limit)
      .map(|model| PokemonShow {
        name: model.name.clone(),
        nick_name: model.nick_name.clone(),
        number: model.number.clone(),
        r#type: model.r#type.clone(),
      })
      .collect::<Vec<_>>();

    let mut view = PokemonList {
      total: db.len(),
      limit,
      offset,
      pokemons,
    };

    Ok(Json(view))
  }

  // #[debug_handler]
  pub async fn create(
    State(db): State<Arc<Mutex<HashMap<i32, PokemonEntity>>>>,
    Json(payload): Json<CreatePokemon>,
  ) -> Result<Json<PokemonShow>, StatusCode> {
    let mut db = db.lock().unwrap(); // usually you should handle errors ;)

    let model = PokemonEntity {
      name: payload.name.clone(),
      nick_name: payload.nick_name.clone(),
      number: payload.number.clone(),
      r#type: payload.r#type.clone(),
    };
    db.insert(payload.number, model);

    let view = PokemonShow {
      name: payload.name.clone(),
      nick_name: payload.nick_name.clone(),
      number: payload.number.clone(),
      r#type: payload.r#type.clone(),
    };
    Ok(Json(view))
  }

  // #[debug_handler]
  pub async fn read(
    Path(id): Path<usize>,
    State(db): State<Arc<Mutex<HashMap<i32, PokemonEntity>>>>,
  ) -> Result<Json<PokemonShow>, StatusCode> {
    let db = db.lock().unwrap(); // usually you should handle errors ;)

    let db_key = i32::try_from(id).unwrap();
    if let Some(model) = db.get(&db_key) {
      let view = PokemonShow {
        name: model.name.clone(),
        nick_name: model.nick_name.clone(),
        number: model.number.clone(),
        r#type: model.r#type.clone(),
      };
      Ok(Json(view))
    } else {
      Err(StatusCode::NOT_FOUND)
    }
  }

  // #[debug_handler]
  pub async fn update(
    Path(id): Path<usize>,
    State(db): State<Arc<Mutex<HashMap<i32, PokemonEntity>>>>,
    Json(payload): Json<UpdatePokemon>,
  ) -> Result<Json<PokemonShow>, StatusCode> {
    let mut db = db.lock().unwrap(); // usually you should handle errors ;)

    let db_key = i32::try_from(id).unwrap();
    if let Some(model) = db.get(&db_key) {
      let newModel = PokemonEntity {
        name: payload.name.clone().unwrap_or(model.name.clone()),
        nick_name: payload.nick_name.clone().unwrap_or(model.nick_name.clone()),
        number: payload.number.clone().unwrap_or(model.number.clone()),
        r#type: payload.r#type.clone().unwrap_or(model.r#type.clone()),
      };
      let view = PokemonShow {
        name: payload.name.clone().unwrap_or(model.name.clone()),
        nick_name: payload.nick_name.clone().unwrap_or(model.nick_name.clone()),
        number: payload.number.clone().unwrap_or(model.number.clone()),
        r#type: payload.r#type.clone().unwrap_or(model.r#type.clone()),
      };
      db.remove(&db_key);
      db.insert(newModel.number, newModel);
      Ok(Json(view))
    } else {
      Err(StatusCode::NOT_FOUND)
    }
  }

  // #[debug_handler]
  pub async fn delete(
    Path(id): Path<usize>,
    State(db): State<Arc<Mutex<HashMap<i32, PokemonEntity>>>>, 
  ) -> Result<StatusCode, StatusCode> {
    let mut db = db.lock().unwrap(); // usually you should handle errors ;)

    let db_key = i32::try_from(id).unwrap();
    if let Some(model) = db.get(&db_key) {
      db.remove(&db_key);
      Ok(StatusCode::NO_CONTENT)
    } else {
      Err(StatusCode::NOT_FOUND)
    }
  }
}
