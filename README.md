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

# Task1
Create a program that loads a file with the same syntax as Linux's sysctl.conf and stores it in data types in a programming language
```
See my parser (https://github.com/gemhung/loadconf/blob/main/src/lib.rs#L87)
```

# Task2
When executing the program you wrote in Task 1, please make sure that the input values can be validated
```
I used derive macro and leverage rust type system as the schema to secure type correctness
See my macro (https://github.com/gemhung/loadconf/blob/main/src/lib.rs#L10)
```

# LoadConf
`LoadConf` is `rust derive macro` to load configuration file based on `sysctl.conf` (https://man7.org/linux/man-pages/man5/sysctl.conf.5.html)

# keywords
```
rust, nom, parser, derive macro, file processing
```

# Use case
```rust
//config.conf
/*
; I'm a comment starts with ';'
# I'm a comment starts with '#'
;token1 =    value1
;token2 invalid   =   value2

token1 =    999   
token2 = 3.14159
token3 = true
token4 = str-1 str-2 str-123ABC!@$%^&*()
token5 = 1 2 3 4 5

*/
fn load_file() -> Result<(), anyhow::Error> {
    #[derive(Debug, PartialEq, Default, loadconf_derive::LoadConf)]
    struct Config {
        pub token1: usize,
        pub token2: f32,
        pub token3: bool,
        pub token4: Vec<String>,
        pub token5: Vec<i64>,
    }
    let demo = Config::load("./config.conf")?;
    assert_eq!(
        demo,
        Config {
            // Support any type that implements `FromStr` trait (https://doc.rust-lang.org/std/str/trait.FromStr.html#implementors)
            token1: 999,
            token2: 3.14159,
            token3: true,
            token4: vec![ // aslo support Vec<_>,
                "str-1".to_string(),
                "str-2".to_string(),
                "str-123ABC!@$%^&*()".to_string(), // support any char except `;` and `#` because they are comment indicators
            ],
            token5: vec![1, 2, 3, 4, 5], // also support Vec<_>
        }
    );
    Ok(())
}
```

# Todo
* Support `token=value ;I'm comment`
* Support types such as `tuple` and `array`
* Add tests to cover more use case
* Better error messages

