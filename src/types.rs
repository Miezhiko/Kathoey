use serde_derive::*;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Lemma { Verb
               , Adjs
               , Prts
               , Other }

#[derive(Serialize, Deserialize)]
pub struct Fem where {
  pub fem: usize,
  pub lemma: Lemma
}

#[derive(Serialize, Deserialize)]
pub struct Kathoey where {
  pub dict: Vec<String>,
  pub map: HashMap<String, Fem>
}
