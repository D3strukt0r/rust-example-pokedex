use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default)]
pub struct FilterOptions {
  pub page: Option<usize>,
  pub limit: Option<usize>,
}

#[derive(Deserialize)]
pub struct CreatePokemon {
  pub name: String,
  pub nick_name: String,
  pub number: i32,
  pub r#type: String,
}

#[derive(Deserialize, Default)]
pub struct UpdatePokemon {
  pub name: Option<String>,
  pub nick_name: Option<String>,
  pub number: Option<i32>,
  pub r#type: Option<String>,
}

#[derive(Serialize)]
pub struct PokemonShow {
  pub name: String,
  pub nick_name: String,
  pub number: i32,
  pub r#type: String,
}

#[derive(Serialize)]
pub struct PokemonList {
  pub total: usize,
  pub limit: usize,
  pub offset: usize,
  pub pokemons: Vec<PokemonShow>,
}
