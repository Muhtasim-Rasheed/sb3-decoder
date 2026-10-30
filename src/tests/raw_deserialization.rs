use crate::raw::*;

const JSON: &str = include_str!("fixtures/raw_deserialization.json");

#[test]
fn raw_deserialization() {
    let deserialized = serde_json::from_str::<Project>(JSON).unwrap();
    insta::assert_debug_snapshot!(deserialized);
}
