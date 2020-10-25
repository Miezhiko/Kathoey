use serde_derive::*;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum Lemma { Verb   = 0
               , Prts   = 1
               , Adjs   = 2
               , Other  = 3 }

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
