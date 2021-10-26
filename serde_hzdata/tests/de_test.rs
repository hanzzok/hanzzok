use std::collections::HashMap;

use serde_hzdata::HzdataValue;

#[test]
pub fn test_de_i8() {
    assert_eq!(127i8, serde_hzdata::from_str::<i8>("  127  ").unwrap());
}

#[test]
pub fn test_de_str() {
    assert_eq!(
        "\'\"\x65\n\r\t\\\0\u{FEFF} WTF HOW IT WORKS",
        serde_hzdata::from_str::<String>(r#" "\'\"\x65\n\r\t\\\0\u{FEFF} WTF HOW IT WORKS" "#)
            .unwrap()
    );
}

#[test]
pub fn test_de_array() {
    assert_eq!(
        [0, 1, 2],
        serde_hzdata::from_str::<[i8; 3]>(r#" [0, 1 ,2 ] "#).unwrap()
    );
}

#[test]
pub fn test_de_struct() {
    #[derive(serde::Deserialize, Debug, PartialEq)]
    struct Test {
        a: Option<String>,
        b: Vec<Option<i8>>,
    }
    assert_eq!(
        Test {
            a: None,
            b: vec![Some(3), Some(5)]
        },
        serde_hzdata::from_str::<Test>(r#" { b = [3, 5] } "#).unwrap()
    );
}

#[test]
pub fn test_de_value() {
    #[derive(serde::Deserialize, Debug, PartialEq)]
    struct Test {
        a: Option<String>,
        b: Vec<Option<i8>>,
    }
    assert_eq!(
        HzdataValue::Object({
            let mut map = HashMap::new();
            map.insert(
                "b".to_owned(),
                HzdataValue::Array(vec![HzdataValue::Integer(3), HzdataValue::Integer(5)]),
            );

            map
        }),
        serde_hzdata::from_str::<HzdataValue>(r#" { "b" = [3,  5 ] } "#).unwrap()
    );
}
