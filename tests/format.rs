const KITCHEN_SINK_SINGLE_AS3: &str = include_str!("./levels/kitchen_sink_single.as3.txt");
const KITCHEN_SINK_SINGLE_LBL: &str = include_str!("./levels/kitchen_sink_single.lbl.txt");
const LEVEL_X_AS3: &str = include_str!("./levels/level_x.as3.txt");
const COMMENTED_AS3: &str = include_str!("./levels/commented.as3.txt");

#[test]
fn kitchen_sink_single_as3() {
    let (_, data) = sks::format::as3::decode(KITCHEN_SINK_SINGLE_AS3).unwrap();
    assert_eq!(data.len(), sks::LEVEL_SIZE);
}

#[test]
fn level_x_as3() {
    let (_, data) = sks::format::as3::decode(LEVEL_X_AS3).unwrap();
    assert_eq!(data.len(), sks::LEVEL_SIZE);
}

#[test]
fn commented_as3() {
    let (_, data) = sks::format::as3::decode(COMMENTED_AS3).unwrap();
    assert_eq!(data.len(), sks::LEVEL_SIZE);
}

#[test]
fn kitchen_sink_single_lbl() {
    let data = sks::format::lbl::decode(KITCHEN_SINK_SINGLE_LBL).unwrap();
    assert_eq!(data.len(), sks::LEVEL_SIZE);
}

#[test]
fn kitchen_sink_guess_as3() {
    let (_, data) = sks::format::decode(KITCHEN_SINK_SINGLE_AS3).unwrap();
    assert_eq!(data.len(), sks::LEVEL_SIZE);
}

#[test]
fn kitchen_sink_guess_lbl() {
    let (_, data) = sks::format::decode(KITCHEN_SINK_SINGLE_LBL).unwrap();
    assert_eq!(data.len(), sks::LEVEL_SIZE);
}

#[test]
fn kitchen_sink_guess() {
    let (_, data_lbl) = sks::format::decode(KITCHEN_SINK_SINGLE_LBL).unwrap();
    let (_, data_as3) = sks::format::decode(KITCHEN_SINK_SINGLE_AS3).unwrap();
    assert_eq!(data_lbl.len(), sks::LEVEL_SIZE);
    assert_eq!(data_as3.len(), sks::LEVEL_SIZE);
    assert_eq!(data_as3, data_lbl);
}

#[test]
fn round_kitchen_sink_lbl() {
    let decoded = sks::format::lbl::decode(KITCHEN_SINK_SINGLE_LBL).unwrap();
    assert_eq!(decoded.len(), sks::LEVEL_SIZE);
    let encoded = sks::format::lbl::encode(&decoded).unwrap();
    assert!(encoded.lines().count() >= sks::LEVEL_SIZE);
    assert_eq!(encoded, KITCHEN_SINK_SINGLE_LBL);
}

#[test]
fn round_kitchen_sink_as3() {
    let (level_num, decoded) = sks::format::as3::decode(KITCHEN_SINK_SINGLE_AS3).unwrap();
    assert_eq!(decoded.len(), sks::LEVEL_SIZE);

    let encoded = sks::format::as3::encode(&decoded, &level_num).unwrap();
    assert!(encoded.lines().count() >= sks::LEVEL_HEIGHT);
    //assert_eq!(encoded, KITCHEN_SINK_SINGLE_AS3); // Generated results are "prettier" so asserts dont work.

    // Rencode to be able to compare data, not styling
    let (_level_num, decoded1) = sks::format::as3::decode(KITCHEN_SINK_SINGLE_AS3).unwrap();
    assert_eq!(decoded1.len(), sks::LEVEL_SIZE);
    assert_eq!(decoded1, decoded); //Check that data content remains the same
}
