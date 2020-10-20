pub mod types;
pub mod parser;

#[allow(unused_imports)]
#[macro_use] extern crate eyre;

use types::*;

use std::collections::HashSet;

static SWORDS: &[&str] = &["он", "оно", "они", "ты", "вы", "мы"];

#[allow(clippy::unnecessary_unwrap)]
impl Kathoey {
  pub fn from_rs(rudano: &str) -> eyre::Result<Kathoey> {
    let contents = std::fs::read_to_string(rudano)?;
    let k = rudano::from_str(&contents)?;
    Ok(k)
  }
  pub fn from_xml(csv: &str) -> eyre::Result<Kathoey> {
    let text = std::fs::read_to_string(csv)?;
    parser::parse_xml(text.as_str())
  }
  fn fem( &self
        , string: &str
        , extreme: bool ) -> Option<String> {
    let ff = self.map.get(string)?;
    if extreme || ff.lemma != Lemma::Other {
      if ff.fem < self.dict.len() {
        let fem = self.dict[ff.fem].clone();
        Some( fem )
      } else { None }
    } else { None }
  }
  pub fn feminize_word( &self
                      , string: &str
                      , extreme: bool ) -> Option<String> {
    if let Some(result) = self.fem(string, extreme) {
      Some(result)
    } else if string.contains('е') {
      let yo = string.replace('е', "ё");
      self.fem(&yo, extreme)
    } else {
      None
    }
  }
  fn process_sentance( &self
                     , string: &str
                     , extreme: bool) -> String {
    let mut out = string.to_string();
    let mut processed_words : HashSet<&str> = HashSet::new();
    let words = string.split(&[' ',',','.',';',':','!','?','\n','\r'][..]);
    for word in words {
      if word.is_empty() { continue; }
      if let Some(fw) = self.feminize_word(word, extreme) {
        if !processed_words.contains(&word) {
          out = out.replace(word, &fw);
          processed_words.insert(word);
        }
      }
    }
    out
  }
  pub fn feminize( &self
                 , string: &str ) -> String {
    let lower = string.to_lowercase();
    if lower.contains('я') {
      if let Some(o) = SWORDS.iter().find(|o| lower.contains(*o)) {
        let ipos = lower.find('я');
        let opos = lower.find(o);
        if ipos.is_some() && opos.is_some() {
          let ip = ipos.unwrap();
          let op = opos.unwrap();
          if ip > op {
            let (first, last) = string.split_at(ip);
            format!("{}{}", first.to_string(), self.process_sentance(last, false))
          } else {
            let (first, last) = string.split_at(op);
            format!("{}{}", self.process_sentance(first, false), last.to_string())
          }
        } else {
          self.process_sentance(string, false)
        }
      } else {
        self.process_sentance(string, false)
      }
    } else if SWORDS.iter().any(|o| lower.contains(*o)) {
      string.to_string()
    } else {
      self.process_sentance(string, false)
    }
  }
  pub fn extreme_feminize( &self
                 , string: &str ) -> String {
    self.process_sentance(string, true)
  }
  pub fn print_this(&self) {
    for (kk, vv) in self.map.iter() {
      println!("{} -> {}", kk, vv.fem);
    }
  }
  pub fn save(&self, fname: &str) -> eyre::Result<()> {
    let rdn = rudano::to_string_compact(&self)?;
    std::fs::write(fname, rdn)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn from_csv() -> eyre::Result<()> {
    match Kathoey::from_xml("dict.opcorpora.xml") {
      Ok(k) => {
        assert_eq!("Я сделала это!", k.feminize("Я сделал это!"));
        assert_eq!("Я потеряла ключи", k.feminize("Я потерял ключи"));
        assert_eq!("Хорошо, я ответила.", k.feminize("Хорошо, я ответил."));
        assert_eq!("Я не хотела этого говорить на случай, если ты увидишь",
          k.feminize("Я не хотел этого говорить на случай, если ты увидишь"));
        assert_eq!("Я уверена, что у него была идея получше, он просто забыл",
          k.feminize("Я уверен, что у него была идея получше, он просто забыл"));
        assert_eq!("Вообще-то, я была немного удивлена.",
          k.feminize("Вообще-то, я был немного удивлен."));
        assert_eq!("Мне нравилось, когда я в аниме и не беспокойся о спойлерах.",
          k.feminize("Мне нравилось, когда я в аниме и не беспокойся о спойлерах."));
        assert_eq!("Я скажу ему это.",
          k.feminize("Я скажу ему это."));
        // Exporting test
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
  #[ignore = "Optional test for optimized format"]
  fn from_rudano() -> eyre::Result<()> {
    match Kathoey::from_rs("dict.rs") {
      Ok(k) => {
        assert_eq!("Я уверена, что у него была идея получше, он просто забыл",
          k.feminize("Я уверен, что у него была идея получше, он просто забыл"));
      }
      Err(kerr) => {
        return
          Err(eyre!("Failed to import rs {:?}", kerr));
      }
    }
    Ok(())
  }
}
