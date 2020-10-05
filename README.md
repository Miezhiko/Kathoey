# Kathoey
Rust library for text feminization

 - currently using Russian from http://opencorpora.org
 - using xmlparser for [perfomance](https://github.com/RazrFalcon/roxmltree#performance)
 - exporting processed xmlparser map to rudano format
 - use from_rudano for speed up (10s vs 55s)
 - two modes (extreme and regular)

# Usage

```rust
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
```
