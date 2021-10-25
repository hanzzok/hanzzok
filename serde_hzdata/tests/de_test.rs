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
