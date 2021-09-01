pub fn capital_first(s: &str) -> String {
  let mut c: std::str::Chars = s.chars();
  match c.next() {
    None => String::new(),
    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
  }
}

#[cfg(test)]
mod utils_tests {
  use super::*;
  #[test]
  fn capital_first_test() {
    assert_eq!("Наруто", capital_first("наруто"));
    assert_eq!("А", capital_first("а"));
  }
}
