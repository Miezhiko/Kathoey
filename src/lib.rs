#[allow(unused_imports)]
#[macro_use] extern crate eyre;

use serde_derive::{Serialize, Deserialize};

use std::collections::HashMap;

use eyre::Result;

#[derive(Serialize, Deserialize)]
struct Dict where {
  fem: String,
  verb: bool
}

#[derive(Serialize, Deserialize)]
pub struct Kathoey where {
  map: HashMap<String, Dict>
}

impl Kathoey {
  pub fn from_rs(dict: &str) -> Result<Kathoey> {
    let contents = std::fs::read_to_string(dict)?;
    let map = rudano::from_str(&contents)?;
    Ok(Kathoey {
      map
    })
  }
  pub fn new(dict: &str) -> Result<Kathoey> {
    let text = std::fs::read_to_string(dict)?;
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
    let mut origfem = false;
    let mut verb = false;
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
                origfem = true;
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
                if lfem && !origfem {
                  map.insert(
                    word.to_string(),
                    Dict { fem: femfem.to_string()
                        , verb }
                  );
                }
                lemma = false;
                verb = false;
                lfem = false;
                origfem = false;
              }
            }
          }
        },
        _ => {}
      }
    }
    Ok(Kathoey {
      map
    })
  }
  pub fn feminize_word( &self
                      , string: &str
                      , extreme: bool ) -> Option<String> {
    let dict = self.map.get(string)?;
    if !extreme && dict.verb {
      Some( dict.fem.clone() )
    } else {
      None
    }
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
    let rdn = rudano::to_string_compact(&self.map)?;
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
        /* k.print_this(); */
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
      }
      Err(kerr) => {
        return
          Err(eyre!("Failed to import rs {:?}", kerr));
      }
    }
    Ok(())
  }
}
