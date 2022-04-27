use std::collections::HashMap;

#[derive(bincode::Encode, bincode::Decode, PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum Lemma { Verb   = 0
               , Prts   = 1
               , Adjs   = 2
               , Other  = 3 }

#[derive(bincode::Encode, bincode::Decode)]
pub struct Fem where {
  pub fem: usize,
  pub lemma: Lemma
}

#[derive(bincode::Encode, bincode::Decode)]
pub struct Kathoey where {
  pub dict: Vec<String>,
  pub map: HashMap<String, Fem>
}
