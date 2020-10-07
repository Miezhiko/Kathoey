#[allow(unused_imports)]
#[macro_use] extern crate eyre;

use serde_derive::{Serialize, Deserialize};

use std::collections::HashMap;

use eyre::Result;

#[derive(Serialize, Deserialize)]
struct Fem where {
  fem: usize,
  verb: bool
}

#[derive(Serialize, Deserialize)]
pub struct Kathoey where {
  dict: Vec<String>,
  map: HashMap<String, Fem>
}

impl Kathoey {
  pub fn from_rs(rudano: &str) -> Result<Kathoey> {
    let contents = std::fs::read_to_string(rudano)?;
    let k = rudano::from_str(&contents)?;
    Ok(k)
  }
  pub fn new(csv: &str) -> Result<Kathoey> {
    let text = std::fs::read_to_string(csv)?;
    let mut map = HashMap::new();
    let mut lemma = false;
    let mut lword = false;
    let mut gword = false;
    let mut fword = false;
    let mut something = false;
    let mut lfem = false;
    let mut word: &str = "";
    let mut mbfem: &str = "";
    let mut femfem: &str = "";
    let mut verb = false;
    let mut other = vec![];
    let mut dict = vec![];
    let mut temp_dict: HashMap<&str, usize> = HashMap::new();
    let mut fem_index : usize = 0;
    for token in xmlparser::Tokenizer::from(text.as_str()) {
      let t = token?;
      match t {
        xmlparser::Token::ElementStart { local, .. } => {
          if local.as_str() == "lemma" {
            lemma = true;
          }
          else if local.as_str() == "l" {
            lword = true;
          }
          else if local.as_str() == "g" {
            gword = true;
          }
          else if local.as_str() == "f" {
            fword = true;
          } else {
            something = true;
          }
        },
        xmlparser::Token::Attribute { local, value, .. } => {
          if lword && !gword {
            if local.as_str() == "t" {
              word = value.as_str();
            }
          } else if gword && lword {
            if local.as_str() == "v" {
              if value.as_str() == "VERB" {
                verb = true;
              } else if value.as_str() == "femn" {
                femfem = word;
                lfem = true;
              }
            }
          } else if fword {
            if gword
              && value.as_str() == "femn" {
                femfem = mbfem;
                lfem = true;
              }
            if local.as_str() == "t" {
              mbfem = value.as_str();
              other.push(mbfem);
            }
          }
        },
        xmlparser::Token::ElementEnd { end, .. } => {
          match end {
            xmlparser::ElementEnd::Open => {
              // means > and we not interested in those
            }
            _ => {
              if something {
                something = false;
              } else if gword {
                gword = false;
              } else if fword {
                fword = false;
              } else if lword {
                lword = false;
              } else if lemma {
                let fem_index =
                  if let Some(i) = temp_dict.get(femfem) {
                    *i
                  } else {
                    let i = fem_index;
                    temp_dict.insert(femfem, i);
                    dict.push(femfem.to_string());
                    fem_index += 1;
                    i
                  };
                if lfem {
                  if word != femfem {
                    map.insert(
                      word.to_string(),
                      Fem { fem: fem_index
                          , verb }
                    );
                  }
                  for w in other.iter() {
                    if *w != femfem {
                      map.insert(
                        w.to_string(),
                        Fem { fem: fem_index
                            , verb }
                      );
                    }
                  }
                }
                lemma = false;
                verb = false;
                lfem = false;
                other.clear();
              }
            }
          }
        },
        _ => {}
      }
    }
    Ok(Kathoey {
      dict, map
    })
  }
  pub fn feminize_word( &self
                      , string: &str
                      , extreme: bool ) -> Option<String> {
    let f = self.map.get(string)?;
    if extreme || f.verb {
      if f.fem < self.dict.len() {
        let fem = self.dict[f.fem].clone();
        Some( fem )
      } else { None }
    } else { None }
  }
  pub fn feminize( &self
                 , string: &str ) -> String {
    let words: Vec<&str> = string.split_whitespace().collect();
    let mut out = string.to_string();
    for word in words {
      if let Some(fw) = self.feminize_word(word, false) {
        out = out.replace(word, &fw);
      }
    }
    out
  }
  pub fn extreme_feminize( &self
                 , string: &str ) -> String {
    let words: Vec<&str> = string.split_whitespace().collect();
    let mut out = string.to_string();
    for word in words {
      if let Some(fw) = self.feminize_word(word, true) {
        out = out.replace(word, &fw);
      }
    }
    out
  }
  pub fn print_this(&self) {
    for (kk, vv) in self.map.iter() {
      println!("{} -> {}", kk, vv.fem);
    }
  }
  pub fn save(&self, fname: &str) -> Result<()> {
    let rdn = rudano::to_string_compact(&self)?;
    std::fs::write(fname, rdn)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  #[ignore = "ignored after dict.rs generation"]
  fn from_csv() -> Result<()> {
    match Kathoey::new("dict.opcorpora.xml") {
      Ok(k) => {
        assert_eq!("Я сделала это", k.feminize("Я сделал это"));
        assert_eq!("Я потеряла ключи", k.feminize("Я потерял ключи"));
        assert_eq!("Я не хотела этою говорить на случай, если ты увидишь.",
          k.extreme_feminize("Я не хотел этого говорить на случай, если ты увидишь."));
        // Optional: exporting
        if let Err(exerr) = k.save("dict.rs") {
          return
            Err(eyre!("Failed to export {:?}", exerr));
        }
      }
      Err(kerr) => {
        return
          Err(eyre!("Failed to create {:?}", kerr));
      }
    }
    Ok(())
  }
  #[test]
  fn from_rudano() -> Result<()> {
    match Kathoey::from_rs("dict.rs") {
      Ok(k) => {
        assert_eq!("Я сделала это", k.feminize("Я сделал это"));
        assert_eq!("Я потеряла ключи", k.feminize("Я потерял ключи"));
        assert_eq!("Я не хотела этого говорить на случай, если ты увидишь.",
          k.feminize("Я не хотел этого говорить на случай, если ты увидишь."));
      }
      Err(kerr) => {
        return
          Err(eyre!("Failed to import rs {:?}", kerr));
      }
    }
    Ok(())
  }
}
