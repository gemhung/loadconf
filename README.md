<h1 align="center">LoadConf derive macro</h1>

<div align="center">
  <!-- CI -->
  <img src="https://github.com/gemhung/loadconf/actions/workflows/clippy.yaml/badge.svg" />
  <a href="https://github.com/rust-secure-code/safety-dance/">
    <img src="https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square"
      alt="Unsafe Rust forbidden" />
  </a>
  <a href="https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html">
    <img src="https://img.shields.io/badge/rustc-1.69.0+-ab6000.svg"
      alt="rustc 1.69.0+" />
  </a>
</div>

# keywords
```
rust, nom, derive macro,
```

# Use case
```rust
use loadconf_derive::LoadConf;
fn load_file() -> Result<(), anyhow::Error> {
    #[derive(Debug, Eq, PartialEq, Default, LoadConf)]
    struct Config {
        pub token1: usize,
        pub token2: Vec<String>,
    }
    let demo = Config::load("./config.conf")?;
    assert_eq!(
        demo,
        Config {
            token1: 999,
            token2: vec![
                "value2-1".to_string(),
                "value2-2".to_string(),
                "@".to_string()
            ]
        }
    );
    Ok(())
}
```

# Todo
