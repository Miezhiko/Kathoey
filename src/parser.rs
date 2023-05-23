use std::collections::HashMap;
use crate::types::*;

pub fn parse_xml(text: &str) -> anyhow::Result<Kathoey> {
  let mut map: HashMap<String, Fem> = HashMap::new();
  let mut lemma = false;
  let mut lword = false;
  let mut gword = false;
  let mut fword = false;
  let mut smthn = false;
  let mut lfemm = false;
  let mut addvr = false;
  let mut addot = true;
  let mut word: &str    = "";
  let mut mbfem: &str   = "";
  let mut femfem: &str  = "";
  let mut other: Vec<&str>  = vec![];
  let mut dict: Vec<String> = vec![];
  let mut temp_dict: HashMap<&str, usize> = HashMap::new();
  let mut fem_index : usize = 0;
  let mut lem: Lemma = Lemma::Other;
  for token in xmlparser::Tokenizer::from(text) {
    let t: xmlparser::Token = token?;
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
            if lem == Lemma::Verb
            && value.as_str() == "masc" {
              addvr = true;
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
              if lem == Lemma::Verb {
                if addot && addvr {
                  other.push(mbfem);
                }
              } else if addot {
                other.push(mbfem);
              }
              addot = true;
              addvr = false;
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
                  let femfemfem =
                    if femfem.contains('ё') {
                      femfem.replace('ё', "е")
                    } else {
                      femfem.to_string()
                    };
                  dict.push(femfemfem.clone());
                  fem_index += 1;
                  i
                };
              if lfemm {
                for w in other.iter() {
                  let ww =
                    if w.contains('ё') {
                      w.replace('ё', "е")
                    } else {
                      w.to_string()
                    };
                  if let Some(f) = map.get_mut(&ww) {
                    if lem < f.lemma {
                      f.fem   = fem_index;
                      f.lemma = lem;
                    }
                  } else {
                    map.insert(
                      ww,
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
