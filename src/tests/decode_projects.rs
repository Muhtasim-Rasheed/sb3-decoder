use crate::prelude::*;

#[test]
fn decode_empty_project() {
    let decoded = Decoder::from_bytes(include_bytes!("fixtures/empty.sb3"))
        .unwrap()
        .decode()
        .unwrap();
    insta::assert_debug_snapshot!(decoded);
}

#[test]
fn decode_procedures() {
    let decoded = Decoder::from_bytes(include_bytes!("fixtures/procedures.sb3"))
        .unwrap()
        .decode()
        .unwrap();
    insta::assert_debug_snapshot!(decoded);
}

#[test]
fn decode_primitives_and_ids() {
    let decoded = Decoder::from_bytes(include_bytes!("fixtures/primitives_and_ids.sb3"))
        .unwrap()
        .decode()
        .unwrap();
    insta::assert_debug_snapshot!(decoded);
}
