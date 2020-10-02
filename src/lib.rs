#[allow(unused_imports)]
#[macro_use] extern crate eyre;

use std::collections::HashMap;

use eyre::Result;

struct Dict where {
  fem: String,
  verb: bool
}

pub struct Kathoey where {
  map: HashMap<String, Dict>
}

impl Kathoey {
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
    let mut verb = false;
    for token in xmlparser::Tokenizer::from(text.as_str()) {
      let t = token?;
      match t {
        xmlparser::Token::ElementStart { prefix, local, span } => {
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
        xmlparser::Token::Attribute { prefix, local, value, span } => {
          if lword {
            if local.as_str() == "t" {
              word = value.as_str();
            }
          }
          if gword {
            if local.as_str() == "v" {
              if value.as_str() == "VERB" {
                verb = true;
              } else if value.as_str() == "femn" {
                femfem = mbfem;
                lfem = true;
              }
            }
          }
          if fword {
            if local.as_str() == "t" {
              mbfem = value.as_str();
            }
          }
        },
        xmlparser::Token::ElementEnd { end, span } => {
          if something {
            something = false;
          } else if fword {
            fword = false;
          } else if gword {
            gword = false;
          } else if lword {
            lword = false;
          } else if lemma {
            if lfem {
              map.insert(
                word.to_string(),
                Dict { fem: mbfem.to_string()
                     , verb: verb }
              );
            }
            lemma = false;
            verb = false;
            lfem = false;
          }
        },
        _ => {}
      }
    }
    Ok(Kathoey {
      map: map
    })
  }
  pub fn feminize_word(&mut self, string: &str) -> Option<String> {
    let dict = self.map.get(string)?;
    if dict.verb {
      Some( dict.fem.clone() )
    } else {
      None
    }
  }
  pub fn feminize(&mut self, string: &str) -> String {
    let words: Vec<&str> = string.split_whitespace().collect();
    let mut out = string.to_string();
    for word in words {
      if let Some(fw) = self.feminize_word(word) {
        out = out.replace(word, &fw);
      }
    }
    out
  }
  pub fn print_this(&self) {
    for (kk, vv) in self.map.iter() {
      print!("{} -> {}", kk, vv.fem);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn constructor() -> Result<()> {
    match Kathoey::new("ru-dict.xml") {
      Ok(k) => {
        k.print_this();
      }
      Err(kerr) => {
        return
          Err(eyre!("Failed to create {:?}", kerr));
      }
    }
    Ok(())
  }
}
