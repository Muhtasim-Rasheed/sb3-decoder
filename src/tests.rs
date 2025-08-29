use crate::prelude::*;

#[test]
fn sprite_decode_test() {
    let mut decoder = DecoderBuilder::new()
        .use_bytes(include_bytes!("test_assets/project.sb3"))
        .build()
        .unwrap();
    let project = decoder.decode().unwrap();
    assert_eq!(project.sprites.len(), 1);
    assert_eq!(project.sprites[0].name, "Cat");
}

#[test]
fn variable_decode_test() {
    let mut decoder = DecoderBuilder::new()
        .use_bytes(include_bytes!("test_assets/project.sb3"))
        .build()
        .unwrap();
    let project = decoder.decode().unwrap();
    let sprite = project.get("Cat").unwrap();

    // Check sprite variables
    assert_eq!(sprite.variables.len(), 2);
    assert_eq!(
        sprite.variables.get("stringing").unwrap().0,
        Value::String("hello world".to_string())
    );
    assert_eq!(
        sprite.variables.get("numbering").unwrap().0,
        Value::Number(3.0)
    );

    // Check global variables
    assert_eq!(project.global_variables().len(), 1);
    assert_eq!(
        project.global_variables().get("my variable").unwrap().0,
        Value::Number(0.0)
    );
}

#[cfg(feature = "costume_png")]
#[test]
fn costume_decode_test() {
    use image::DynamicImage;

    let mut decoder = DecoderBuilder::new()
        .use_bytes(include_bytes!("test_assets/project.sb3"))
        .build()
        .unwrap();
    let project = decoder.decode().unwrap();
    let sprite = project.get("Cat").unwrap();

    // Check sprite costumes
    assert_eq!(sprite.costume_names().len(), 2);
    assert_eq!(sprite.current_costume, 0);
    assert!(matches!(
        sprite.get_costume("costume1").unwrap().0,
        DynamicImage::ImageRgba8(_)
    ));
}

#[cfg(feature = "costume_svg")]
#[test]
fn costume_decode_test() {
    use usvg::Tree;

    let mut decoder = DecoderBuilder::new()
        .use_bytes(include_bytes!("test_assets/project.sb3"))
        .build()
        .unwrap();
    let project = decoder.decode().unwrap();
    let sprite = project.get("Cat").unwrap();

    // Check sprite costumes
    assert_eq!(sprite.costume_names().len(), 2);
    assert_eq!(sprite.current_costume, 0);
    assert!(matches!(
        sprite.get_costume("costume1").unwrap().0,
        Tree { .. }
    ));
}

#[test]
fn sound_decode_test() {
    let mut decoder = DecoderBuilder::new()
        .use_bytes(include_bytes!("test_assets/project.sb3"))
        .build()
        .unwrap();
    let project = decoder.decode().unwrap();
    let sprite = project.get("Cat").unwrap();

    // Check sprite sounds
    assert_eq!(sprite.sounds.len(), 1);
    assert_eq!(sprite.sounds.get("Meow").unwrap().rate, 22050);
    assert_eq!(sprite.sounds.get("Meow").unwrap().data.len(), 18688);
}

#[test]
fn broadcast_decode_test() {
    let mut decoder = DecoderBuilder::new()
        .use_bytes(include_bytes!("test_assets/project.sb3"))
        .build()
        .unwrap();
    let project = decoder.decode().unwrap();

    // Check broadcasts
    let broadcasts = project.broadcasts();
    assert_eq!(broadcasts.len(), 1);
    assert!(broadcasts.iter().any(|b| b.0 == "wiggly wobbly"));
}

#[test]
fn list_decode_test() {
    let mut decoder = DecoderBuilder::new()
        .use_bytes(include_bytes!("test_assets/project.sb3"))
        .build()
        .unwrap();
    let project = decoder.decode().unwrap();
    let sprite = project.get("Cat").unwrap();

    // Check sprite lists
    assert_eq!(sprite.lists.len(), 1);
    let list = sprite.lists.get("listing").unwrap();
    assert_eq!(
        list.0,
        vec![9.0, 5.0, 8.0, 10.0]
            .iter()
            .map(|n| Value::Number(*n))
            .collect::<Vec<_>>()
    );
}

#[test]
fn opcode_test() {
    use std::str::FromStr;

    // Test valid opcodes
    let motion_opcode = Opcode::from_str("motion_movesteps").unwrap();
    assert_eq!(motion_opcode, Opcode::Motion(MotionOpcode::MoveSteps));

    let looks_opcode = Opcode::from_str("looks_sayforsecs").unwrap();
    assert_eq!(looks_opcode, Opcode::Looks(LooksOpcode::SayForSeconds));

    let sound_opcode = Opcode::from_str("sound_play").unwrap();
    assert_eq!(sound_opcode, Opcode::Sound(SoundOpcode::StartSound));

    let events_opcode = Opcode::from_str("event_whenflagclicked").unwrap();
    assert_eq!(events_opcode, Opcode::Events(EventsOpcode::WhenFlagClicked));

    let control_opcode = Opcode::from_str("control_if").unwrap();
    assert_eq!(control_opcode, Opcode::Control(ControlOpcode::If));

    let sensing_opcode = Opcode::from_str("sensing_touchingcolor").unwrap();
    assert_eq!(
        sensing_opcode,
        Opcode::Sensing(SensingOpcode::TouchingColor)
    );

    let operators_opcode = Opcode::from_str("operator_add").unwrap();
    assert_eq!(operators_opcode, Opcode::Operators(OperatorsOpcode::Add));

    let data_opcode = Opcode::from_str("data_setvariableto").unwrap();
    assert_eq!(data_opcode, Opcode::Data(DataOpcode::SetVariableTo));

    let procedures_opcode = Opcode::from_str("procedures_definition").unwrap();
    assert_eq!(
        procedures_opcode,
        Opcode::Procedures(ProceduresOpcode::Define)
    );
}

#[test]
fn block_decode_test() {
    let mut decoder = DecoderBuilder::new()
        .use_bytes(include_bytes!("test_assets/project.sb3"))
        .build()
        .unwrap();
    let project = decoder.decode().unwrap();
    let sprite = project.get("Cat").unwrap();

    // Decode scripts
    assert_eq!(sprite.scripts.len(), 2);
    assert_eq!(sprite.scripts[0].len(), 4);
    assert_eq!(sprite.scripts[0][0].opcode, Opcode::Events(EventsOpcode::WhenFlagClicked));
    assert_eq!(sprite.scripts[0][1].opcode, Opcode::Data(DataOpcode::AddToList));
    assert_eq!(sprite.scripts[0][2].opcode, Opcode::Motion(MotionOpcode::MoveSteps));
    assert_eq!(sprite.scripts[0][3].opcode, Opcode::Events(EventsOpcode::Broadcast));
    assert!( matches!( sprite.scripts[0][1].inputs["ITEM"].value, InputValue::Block( _ ) ) );
    let sub_block = match sprite.scripts[0][1].inputs["ITEM"].value {
        InputValue::Block(ref b) => *b.clone(),
        _ => panic!("Expected a block"),
    };
    assert!( matches!( sub_block.opcode, Opcode::Operators(OperatorsOpcode::Random ) ) );
}
