use modular_frost::{Participant, ThresholdParams};

#[test]
fn test_participant_u16_max() {
    let p = Participant::new(1).unwrap();
    let result = ThresholdParams::new(1, u16::MAX, p);
    println!("ThresholdParams result: {:?}", result);
}
