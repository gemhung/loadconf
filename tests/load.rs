#![allow(unused)]
use loadconf_derive::LoadConf;

#[test]
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

#[test]
fn load_raw() -> Result<(), anyhow::Error> {
    #[derive(Debug, Eq, PartialEq, Default, LoadConf)]
    struct Config {
        pub a: usize,
        pub b_1: usize,
        pub c_2: String,
    }
    let text = r#"
        ; comment 1
        # comment 2
        #; comment 3
            a =             1
            b_1 =   2
            c_2  =      3
        "#;
    let demo = Config::load_with_raw(text)?;
    assert_eq!(
        demo,
        Config {
            a: 1,
            b_1: 2,
            c_2: "3".to_string(),
        }
    );
    Ok(())
}
