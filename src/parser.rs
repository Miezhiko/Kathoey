use std::collections::HashMap;
use crate::types::*;

pub fn parse_csv(text: &str) -> eyre::Result<Kathoey> {
  let mut map: HashMap<String, Fem> = HashMap::new();
  let mut lemma = false;
  let mut lword = false;
  let mut gword = false;
  let mut fword = false;
  let mut smthn = false;
  let mut lfemm = false;
  let mut addot = true;
  let mut word: &str = "";
  let mut mbfem: &str = "";
  let mut femfem: &str = "";
  let mut other = vec![];
  let mut dict = vec![];
  let mut temp_dict: HashMap<&str, usize> = HashMap::new();
  let mut fem_index : usize = 0;
  let mut lem = Lemma::Other;
  for token in xmlparser::Tokenizer::from(text) {
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
          addot = true;
        } else {
          smthn = true;
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
              lem = Lemma::Verb;
            } else if value.as_str() == "ADJS" {
              lem = Lemma::Adjs;
            } else if value.as_str() == "PRTS" {
              lem = Lemma::Prts;
            } else if value.as_str() == "femn" {
              femfem = word;
              lfemm = true;
            }
          }
        } else if fword {
          if gword {
            if value.as_str() == "femn" {
              femfem = mbfem;
              lfemm = true;
              addot = false;
            } else if value.as_str() == "impr"
                   || value.as_str() == "neut" {
              addot = false;
            }
          } else if local.as_str() == "t" {
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
            if smthn {
              smthn = false;
            } else if gword {
              gword = false;
            } else if fword {
              if addot {
                other.push(mbfem);
              }
              addot = false;
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
              if lfemm {
                for w in other.iter() {
                  if let Some(mut f) = map.get_mut(*w) {
                    if f.lemma == Lemma::Adjs
                    && lem == Lemma::Prts {
                      f.fem = fem_index;
                      f.lemma = lem;
                    }
                  } else {
                    map.insert(
                      w.to_string(),
                      Fem {
                        fem: fem_index,
                        lemma: lem
                      }
                    );
                  }
                }
              }
              lemma = false;
              lem = Lemma::Other;
              lfemm = false;
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
