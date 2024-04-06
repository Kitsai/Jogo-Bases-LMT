use super::*;

#[test]
fn test_convert_to_binary() {
    assert_eq!(convert_to_binary(10), "1010");
}

#[test]
fn test_convert_to_hex() {
    assert_eq!(convert_to_hex(10), "A");
}

#[test]
fn test_set_from_decimal() {
    let aux: u8 = val_from_decimal("20").unwrap();
    assert_eq!(convert_to_binary(aux), "10100");
}

#[test]
fn test_set_from_binary() {
    let aux = val_from_binary("10100").unwrap();
    assert_eq!(convert_to_binary(aux), "10100");

    let aux = val_from_binary("aslioufhai");
    assert!(aux.is_err());
}

#[test]
fn test_set_from_hex() {
    let aux = val_from_hex("14").unwrap();
    assert_eq!(convert_to_binary(aux), "10100");

    let aux = val_from_hex("A").unwrap();
    assert_eq!(convert_to_binary(aux), "1010"); 

    let aux = val_from_hex("a").unwrap();
    assert_eq!(convert_to_binary(aux), "1010");

    let aux = val_from_hex("FE").unwrap();
    assert_eq!(convert_to_binary(aux), "11111110");
}