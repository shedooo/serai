use std::collections::HashMap;
use rand_core::OsRng;
use dkg::{Participant, ThresholdKeys, ThresholdParams, DkgError};
use dkg_dealer::key_gen;
use dalek_ff_group::Ristretto;
use ciphersuite::{Ciphersuite, group::{Group, ff::Field, GroupEncoding}};

use schnorrkel::PublicKey;

#[test]
fn test_dealer_offset_identity() {
  let keys = key_gen::<_, Ristretto>(&mut OsRng, 1, 1).unwrap();
  let key = &keys[&Participant::new(1).unwrap()];

  // get the secret to negate it
  let secret_share = key.original_secret_share();
  let negating_offset = -**secret_share;

  let offset_keys = key.clone().offset(negating_offset);
  let group_key = offset_keys.group_key();

  println!("Is group key identity? {}", bool::from(group_key.is_identity()));
}

#[test]
fn test_schnorrkel_identity_panic() {
    let identity = <Ristretto as Ciphersuite>::G::identity();
    let pubkey_result = schnorrkel::PublicKey::from_bytes(&identity.to_bytes());
    println!("Pubkey result: {:?}", pubkey_result);
}

#[test]
fn test_participant_new() {
    assert_eq!(Participant::new(0), None);
    assert_eq!(Participant::new(1), Some(Participant::new(1).unwrap()));
    assert_eq!(Participant::new(u16::MAX), Some(Participant::new(u16::MAX).unwrap()));
}

#[test]
fn test_threshold_params_invalid() {
    let p1 = Participant::new(1).unwrap();
    let p5 = Participant::new(5).unwrap();
    // t > n
    assert!(ThresholdParams::new(5, 3, p1).is_err());
    // t = 0
    assert!(ThresholdParams::new(0, 3, p1).is_err());
    // n = 0
    assert!(ThresholdParams::new(0, 0, p1).is_err());
    // participant index > n
    assert!(ThresholdParams::new(2, 3, p5).is_err());
}

#[test]
fn test_threshold_keys_view_empty() {
    let keys = key_gen::<_, Ristretto>(&mut OsRng, 2, 3).unwrap();
    let key = &keys[&Participant::new(1).unwrap()];
    assert!(key.view(vec![]).is_err());
}

#[test]
fn test_threshold_keys_view_wrong_participants() {
    let keys = key_gen::<_, Ristretto>(&mut OsRng, 2, 3).unwrap();
    let key = &keys[&Participant::new(1).unwrap()];
    assert!(key.view(vec![Participant::new(4).unwrap(), Participant::new(5).unwrap()]).is_err());
}

#[test]
fn test_dealer_rejections() {
    assert!(key_gen::<_, Ristretto>(&mut OsRng, 4, 3).is_err());
    //assert!(key_gen::<_, Ristretto>(assert!(key_gen::<_, Ristretto>(&mut OsRng, 0, 3).is_err());mut OsRng, 0, 3).is_err());
    //assert!(key_gen::<_, Ristretto>(assert!(key_gen::<_, Ristretto>(&mut OsRng, 3, 0).is_err());mut OsRng, 3, 0).is_err());
}

// C.1 - Dealer 2-of-3 roundtrip
use modular_frost::{
    algorithm::Schnorr,

    Participant as FrostParticipant,
};
use std::collections::BTreeMap;

#[test]
fn test_dealer_2_of_3_roundtrip() {
    let keys = key_gen::<_, Ristretto>(&mut OsRng, 2, 3).unwrap();
    let key1 = &keys[&Participant::new(1).unwrap()];
    let key2 = &keys[&Participant::new(2).unwrap()];
    let key3 = &keys[&Participant::new(3).unwrap()];

    assert_eq!(key1.group_key(), key2.group_key());
    assert_eq!(key2.group_key(), key3.group_key());
    assert!(!bool::from(key1.group_key().is_identity()));
}

// C.2 - Dealer 1-of-3 roundtrip
#[test]
fn test_dealer_1_of_3_roundtrip() {
    let keys = key_gen::<_, Ristretto>(&mut OsRng, 1, 3).unwrap();
    let key1 = &keys[&Participant::new(1).unwrap()];
    let key2 = &keys[&Participant::new(2).unwrap()];
    let key3 = &keys[&Participant::new(3).unwrap()];

    assert_eq!(key1.group_key(), key2.group_key());
    assert_eq!(key2.group_key(), key3.group_key());
}

// C.6 - Dealer share consistency
#[test]
fn test_dealer_share_consistency() {
    let keys = key_gen::<_, Ristretto>(&mut OsRng, 2, 3).unwrap();
    let key1 = &keys[&Participant::new(1).unwrap()];
    // Verification shares test ... we know they exist from previous test.
    let share = key1.original_verification_share(Participant::new(1).unwrap());
    assert!(!bool::from(share.is_identity()));
}

// C.8 - Large threshold
#[test]
fn test_large_threshold() {
    let keys = key_gen::<_, Ristretto>(&mut OsRng, 67, 100).unwrap();
    assert_eq!(keys.len(), 100);
}

// A.2 - offset() with zero scalar
#[test]
fn test_offset_zero() {
    let keys = key_gen::<_, Ristretto>(&mut OsRng, 2, 3).unwrap();
    let key = &keys[&Participant::new(1).unwrap()];
    let zero_offset = <Ristretto as Ciphersuite>::F::ZERO;
    let offset_keys = key.clone().offset(zero_offset);
    assert_eq!(offset_keys.group_key(), key.group_key());
}

// A.3 - Multiple offsets accumulate correctly
#[test]
fn test_multiple_offsets() {
    let keys = key_gen::<_, Ristretto>(&mut OsRng, 2, 3).unwrap();
    let key = &keys[&Participant::new(1).unwrap()];
    let scalar_a = <Ristretto as Ciphersuite>::F::random(&mut OsRng);
    let scalar_b = <Ristretto as Ciphersuite>::F::random(&mut OsRng);

    let keys_a = key.clone().offset(scalar_a);
    let keys_ab = keys_a.offset(scalar_b);
    let keys_combined = key.clone().offset(scalar_a + scalar_b);
    assert_eq!(keys_ab.group_key(), keys_combined.group_key());
}
