use std::collections::HashMap;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct NewType<T>(T);

#[test]
fn deserialize_newtype_i32() {
    let result = vec![("field".to_owned(), NewType(11))];

    assert_eq!(comma_serde_urlencoded::from_str("field=11"), Ok(result));
}

#[test]
fn deserialize_bytes() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(
        comma_serde_urlencoded::from_bytes(b"first=23&last=42"),
        Ok(result)
    );
}

#[test]
fn deserialize_str() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(comma_serde_urlencoded::from_str("first=23&last=42"), Ok(result));
}

#[test]
fn deserialize_borrowed_str() {
    let result = vec![("first", 23), ("last", 42)];

    assert_eq!(comma_serde_urlencoded::from_str("first=23&last=42"), Ok(result));
}

#[test]
fn deserialize_reader() {
    let result = vec![("first".to_owned(), 23), ("last".to_owned(), 42)];

    assert_eq!(
        comma_serde_urlencoded::from_reader(b"first=23&last=42" as &[_]),
        Ok(result)
    );
}

#[test]
fn deserialize_option() {
    let result = vec![
        ("first".to_owned(), Some(23)),
        ("last".to_owned(), Some(42)),
    ];
    assert_eq!(comma_serde_urlencoded::from_str("first=23&last=42"), Ok(result));
}

#[test]
fn deserialize_unit() {
    assert_eq!(comma_serde_urlencoded::from_str(""), Ok(()));
    assert_eq!(comma_serde_urlencoded::from_str("&"), Ok(()));
    assert_eq!(comma_serde_urlencoded::from_str("&&"), Ok(()));
    assert!(comma_serde_urlencoded::from_str::<()>("first=23").is_err());
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
enum X {
    A,
    B,
    C,
}

#[test]
fn deserialize_unit_enum() {
    let result = vec![
        ("one".to_owned(), X::A),
        ("two".to_owned(), X::B),
        ("three".to_owned(), X::C),
    ];

    assert_eq!(
        comma_serde_urlencoded::from_str("one=A&two=B&three=C"),
        Ok(result)
    );
}

#[test]
fn deserialize_unit_type() {
    assert_eq!(comma_serde_urlencoded::from_str(""), Ok(()));
}

#[test]
fn deserialize() {
    let a: Result<HashMap<String, Vec<i32>>, comma_serde_urlencoded::de::Error> = comma_serde_urlencoded::from_str("one=1,2");

    println!("{:?}", a);
}
