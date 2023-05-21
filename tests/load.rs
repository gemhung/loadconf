#![allow(unused)]

#[test]
fn load_file() -> Result<(), anyhow::Error> {
    #[derive(Debug, PartialEq, loadconf_derive::LoadConf)]
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
            token1: 999,
            token2: 3.14159,
            token3: true,
            token4: vec![
                "str-1".to_string(),
                "str-2".to_string(),
                "str-123ABC!@$%^&*()".to_string(),
            ],
            token5: vec![1, 2, 3, 4, 5],
        }
    );
    Ok(())
}

#[test]
fn load_raw() -> Result<(), anyhow::Error> {
    #[derive(Debug, PartialEq, loadconf_derive::LoadConf)]
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
