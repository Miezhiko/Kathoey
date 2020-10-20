# Kathoey
Rust library for text feminization

 - currently using Russian from http://opencorpora.org
 - using xmlparser for [perfomance](https://github.com/RazrFalcon/roxmltree#performance)
 - exporting parsed Kathoey to optimized rudano format
 - use from_rudano for speed up!
 - two modes (extreme and regular)

# Usage

Serialization from xml and export to rudano
(tho, export is really optional, export from xml is enough fast)

```rust
fn from_csv() -> eyre::Result<()> {
  match Kathoey::from_xml("dict.opcorpora.xml") {
    Ok(k) => {
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
```

Few test and import from Rudano:

```rust
fn from_rudano() -> eyre::Result<()> {
  match Kathoey::from_rs("dict.rs") {
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
    }
    Err(kerr) => {
      return
        Err(eyre!("Failed to import rs {:?}", kerr));
    }
  }
  Ok(())
}
```
