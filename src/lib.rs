pub mod types;
pub mod parser;
pub mod utils;

#[allow(unused_imports)]
#[macro_use] extern crate anyhow;

use types::*;

use std::collections::HashSet;

static SWORDS: &[&str] = &["он", "оно", "они", "ты", "вы", "мы"];
pub static SEPARATORS: [char; 10] = [' ',',','.',';',':','!','?','_','\n','\r'];

impl Kathoey {
  pub fn load(bin: &str) -> anyhow::Result<Kathoey> {
    let mut bin_file: std::fs::File = std::fs::File::open(bin)?;
    let kathoey: Kathoey =
      bincode::decode_from_std_read( &mut bin_file
                                   , bincode::config::standard() )?;
    Ok(kathoey)
  }
  pub fn from_xml(csv: &str) -> anyhow::Result<Kathoey> {
    let text: String = std::fs::read_to_string(csv)?;
    parser::parse_xml(text.as_str())
  }
  fn fem( &self
        , slice: &str
        , extreme: bool ) -> Option<String> {
    let ff: &Fem = self.map.get(slice)?;
    if extreme || ff.lemma != Lemma::Other {
      if ff.fem < self.dict.len() {
        let fem: String = self.dict[ff.fem].clone();
        Some( fem )
      } else { None }
    } else { None }
  }
  pub fn feminize_word( &self
                      , slice: &str
                      , extreme: bool ) -> Option<String> {
    if let Some(result) = self.fem(slice, extreme) {
      Some(result)
    } else if slice.contains('ё') {
      let yo: String = slice.replace('ё', "е");
      self.fem(&yo, extreme)
    } else {
      None
    }
  }
  fn process_sentance( &self
                     , slice: &str
                     , extreme: bool) -> String {
    let mut out: String = slice.into();
    let mut processed_words : HashSet<&str> = HashSet::new();
    let words: std::str::Split<&[char]> = slice.split(&SEPARATORS[..]);
    for word in words {
      if word.is_empty() { continue; }
      let small_word: String = word.to_lowercase();
      if let Some(mut fw) = self.feminize_word(&small_word, extreme) {
        if !processed_words.contains(&word) {
          let mut whole_word_uppercase  = true;
          let mut first_char_uppercase  = true;
          let mut first_char_checked    = false;
          for ch in word.chars() {
            if ch.is_lowercase() {
              if !first_char_checked {
                first_char_uppercase = false;
              }
              whole_word_uppercase = false;
              break;
            }
            first_char_checked = true;
          }
          if whole_word_uppercase {
            fw = fw.to_uppercase();
          } else if first_char_uppercase {
            fw = utils::capital_first(&fw);
          }
          out = out.replace(word, &fw);
          processed_words.insert(word);
        }
      }
    }
    out
  }
  pub fn feminize( &self
                 , slice: &str ) -> String {
    let lower: String = slice.to_lowercase();
    let lwords: Vec<&str> = lower.split(&SEPARATORS[..])
                                 .collect::<Vec<&str>>();
    if lwords.contains(&"я") {
      if let Some(o) = SWORDS.iter().find(|o| lwords.contains(o)) {
        let ipos: Option<usize> = lwords.iter().position(|&w| w == "я");
        let opos: Option<usize> = lwords.iter().position(|&w| w == *o);
        if ipos.is_some() && opos.is_some() {
          let ip: usize = ipos.unwrap_or_default();
          let op: usize = opos.unwrap_or_default();
          if ip > op {
            let pos: usize = lwords[0..ip].join(" ").len();
            let (first, last) = slice.split_at(pos);
            let fem_first: String = self.feminize(first);
            format!("{fem_first}{}", self.process_sentance(last, false))
          } else {
            let pos: usize = lwords[0..op].join(" ").len();
            let (first, last) = slice.split_at(pos);
            let fem_last = self.feminize(last);
            format!("{}{fem_last}", self.process_sentance(first, false))
          }
        } else {
          self.process_sentance(slice, false)
        }
      } else {
        self.process_sentance(slice, false)
      }
    } else if SWORDS.iter().any(|o| lower.contains(*o)) {
      slice.to_string()
    } else {
      self.process_sentance(slice, false)
    }
  }
  pub fn extreme_feminize( &self
                         , string: &str ) -> String {
    self.process_sentance(string, true)
  }
  pub fn print_this(&self) {
    for (kk, vv) in self.map.iter() {
      println!("{kk} -> {}", vv.fem);
    }
  }
  pub fn save(&self, fname: &str) -> anyhow::Result<()> {
    let mut bin_file: std::fs::File = std::fs::File::create(fname)?;
    bincode::encode_into_std_write( self
                                  , &mut bin_file
                                  , bincode::config::standard() )?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use serial_test::serial;
  use super::*;
  #[test]
  #[serial]
  fn from_csv() -> anyhow::Result<()> {
    match Kathoey::from_xml("dict.opcorpora.xml") {
      Ok(k) => {
        assert_eq!("Начала наруто смотреть", k.feminize("Начал наруто смотреть"));
        if let Err(exerr) = k.save("dict.bin") {
          return
            Err(anyhow!("Failed to export {:?}", exerr));
        }
      }
      Err(kerr) => {
        return
          Err(anyhow!("Failed to create {:?}", kerr));
      }
    }
    Ok(())
  }
  #[test]
  #[serial]
  fn from_bincode() -> anyhow::Result<()> {
    match Kathoey::load("dict.bin") {
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
        assert_eq!("Я скажу ему это.", k.feminize("Я скажу ему это."));
        assert_eq!("Ничего страшного и спасибо, что посмотрел на меня, если ты когда-нибудь захочешь вернуться в Воу, я всегда рада играть с тобой.",
          k.feminize("Ничего страшного и спасибо, что посмотрел на меня, если ты когда-нибудь захочешь вернуться в Воу, я всегда рад играть с тобой."));
      }
      Err(kerr) => {
        return
          Err(anyhow!("Failed to import bin {:?}", kerr));
      }
    }
    Ok(())
  }
}
