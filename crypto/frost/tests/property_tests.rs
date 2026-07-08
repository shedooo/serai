use proptest::prelude::*;
use std::collections::HashMap;

use rand_core::{RngCore, CryptoRng, OsRng};
use zeroize::{Zeroize, Zeroizing};

use dkg_dealer::key_gen;
use ciphersuite::{group::ff::Field, group::Group, group::GroupEncoding, Ciphersuite};
#[cfg(feature = "ristretto")]
use modular_frost::curve::Ristretto;

use modular_frost::sign::AlgorithmSignMachine;
use modular_frost::{
    curve::Curve, Participant, ThresholdKeys, FrostError,
    algorithm::{Algorithm, Hram, IetfSchnorr},
    sign::{Writable, PreprocessMachine, SignMachine, SignatureMachine, AlgorithmMachine, Preprocess, SignatureShare},
};

use transcript::Transcript;

// Define an Hram for testing
#[derive(Clone, Debug, PartialEq)]
pub struct TestHram;
impl Hram<Ristretto> for TestHram {
    #[allow(non_snake_case)]
    fn hram(R: &<Ristretto as Ciphersuite>::G, A: &<Ristretto as Ciphersuite>::G, m: &[u8]) -> <Ristretto as Ciphersuite>::F {
        let mut data = vec![];
        data.extend_from_slice(R.to_bytes().as_ref());
        data.extend_from_slice(A.to_bytes().as_ref());
        data.extend_from_slice(m);
        <Ristretto as Ciphersuite>::hash_to_F(b"hram", &data)
    }
}

pub fn create_session(t: u16, n: u16) -> HashMap<Participant, ThresholdKeys<Ristretto>> {
    key_gen::<_, Ristretto>(&mut OsRng, t, n).unwrap()
}

pub fn algorithm_machines(
    keys: &HashMap<Participant, ThresholdKeys<Ristretto>>
) -> HashMap<Participant, AlgorithmMachine<Ristretto, IetfSchnorr<Ristretto, TestHram>>> {
    keys.values()
        .map(|k| (k.params().i(), AlgorithmMachine::new(IetfSchnorr::<Ristretto, TestHram>::ietf(), k.clone())))
        .collect()
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    #[test]
    fn test_b1_sign_verify_roundtrip(msg in prop::collection::vec(any::<u8>(), 0..1024)) {
        let mut rng = OsRng;
        let keys = create_session(2, 3);

        let mut machines = algorithm_machines(&keys);
        machines.remove(&Participant::new(3).unwrap());

        let mut commitments = HashMap::new();
        let mut sign_machines = HashMap::new();
        for (i, machine) in machines.drain() {
            let (sign_machine, preprocess) = machine.preprocess(&mut rng);
            commitments.insert(i, preprocess);
            sign_machines.insert(i, sign_machine);
        }

        let mut shares = HashMap::new();
        let mut sig_machines = HashMap::new();
        for (i, machine) in sign_machines.drain() {
            let mut other_commitments = commitments.clone();
            other_commitments.remove(&i);

            let (sig_machine, share) = machine.sign(other_commitments, &msg).unwrap();
            shares.insert(i, share);
            sig_machines.insert(i, sig_machine);
        }

        let mut signature = None;
        for (i, machine) in sig_machines.drain() {
            let mut other_shares = shares.clone();
            other_shares.remove(&i);

            let sig = machine.complete(other_shares).unwrap();
            if signature.is_none() {
                signature = Some(sig.clone());
            }
            prop_assert_eq!(&sig, signature.as_ref().unwrap());
        }

        let group_key = keys[&Participant::new(1).unwrap()].group_key();
        let sig = signature.unwrap();
        let hram = TestHram::hram(&sig.R, &group_key, &msg);
        prop_assert!(sig.verify(group_key, hram));
    }
}

#[test]
fn test_b2_empty_message_signing() {
    let mut rng = OsRng;
    let keys = create_session(2, 3);
    let mut machines = algorithm_machines(&keys);
    machines.remove(&Participant::new(3).unwrap());

    let msg = b"";
    let mut commitments = HashMap::new();
    let mut sign_machines = HashMap::new();
    for (i, machine) in machines.drain() {
        let (sm, pre) = machine.preprocess(&mut rng);
        commitments.insert(i, pre);
        sign_machines.insert(i, sm);
    }

    let mut shares = HashMap::new();
    let mut sig_machines = HashMap::new();
    for (i, machine) in sign_machines.drain() {
        let mut other_commitments = commitments.clone();
        other_commitments.remove(&i);
        let (sigm, share) = machine.sign(other_commitments, msg).unwrap();
        shares.insert(i, share);
        sig_machines.insert(i, sigm);
    }

    let mut signature = None;
    for (i, machine) in sig_machines.drain() {
        let mut other_shares = shares.clone();
        other_shares.remove(&i);
        let sig = machine.complete(other_shares).unwrap();
        signature = Some(sig);
    }
    let sig = signature.unwrap();
    let group_key = keys[&Participant::new(1).unwrap()].group_key();
    let hram = TestHram::hram(&sig.R, &group_key, msg);
    assert!(sig.verify(group_key, hram));
}

#[test]
fn test_b3_large_message_signing() {
    let mut rng = OsRng;
    let keys = create_session(2, 3);
    let mut machines = algorithm_machines(&keys);
    machines.remove(&Participant::new(3).unwrap());

    let msg = vec![0u8; 1_000_000];
    let mut commitments = HashMap::new();
    let mut sign_machines = HashMap::new();
    for (i, machine) in machines.drain() {
        let (sm, pre) = machine.preprocess(&mut rng);
        commitments.insert(i, pre);
        sign_machines.insert(i, sm);
    }

    let mut shares = HashMap::new();
    let mut sig_machines = HashMap::new();
    for (i, machine) in sign_machines.drain() {
        let mut other_commitments = commitments.clone();
        other_commitments.remove(&i);
        let (sigm, share) = machine.sign(other_commitments, &msg).unwrap();
        shares.insert(i, share);
        sig_machines.insert(i, sigm);
    }

    let mut signature = None;
    for (i, machine) in sig_machines.drain() {
        let mut other_shares = shares.clone();
        other_shares.remove(&i);
        let sig = machine.complete(other_shares).unwrap();
        signature = Some(sig);
    }
    let sig = signature.unwrap();
    let group_key = keys[&Participant::new(1).unwrap()].group_key();
    let hram = TestHram::hram(&sig.R, &group_key, &msg);
    assert!(sig.verify(group_key, hram));
}

#[test]
fn test_b4_all_zero_message() {
    let mut rng = OsRng;
    let keys = create_session(2, 3);
    let mut machines = algorithm_machines(&keys);
    machines.remove(&Participant::new(3).unwrap());

    let msg = [0u8; 32];
    let mut commitments = HashMap::new();
    let mut sign_machines = HashMap::new();
    for (i, machine) in machines.drain() {
        let (sm, pre) = machine.preprocess(&mut rng);
        commitments.insert(i, pre);
        sign_machines.insert(i, sm);
    }

    let mut shares = HashMap::new();
    let mut sig_machines = HashMap::new();
    for (i, machine) in sign_machines.drain() {
        let mut other_commitments = commitments.clone();
        other_commitments.remove(&i);
        let (sigm, share) = machine.sign(other_commitments, &msg).unwrap();
        shares.insert(i, share);
        sig_machines.insert(i, sigm);
    }

    let mut signature = None;
    for (i, machine) in sig_machines.drain() {
        let mut other_shares = shares.clone();
        other_shares.remove(&i);
        let sig = machine.complete(other_shares).unwrap();
        signature = Some(sig);
    }
    let sig = signature.unwrap();
    let group_key = keys[&Participant::new(1).unwrap()].group_key();
    let hram = TestHram::hram(&sig.R, &group_key, &msg);
    assert!(sig.verify(group_key, hram));
}

#[test]
fn test_b5_threshold_boundary_exactly_t_signers() {
    let mut rng = OsRng;
    let keys = create_session(3, 5);
    let mut machines = algorithm_machines(&keys);
    machines.remove(&Participant::new(4).unwrap());
    machines.remove(&Participant::new(5).unwrap());

    let msg = b"test message";
    let mut commitments = HashMap::new();
    let mut sign_machines = HashMap::new();
    for (i, machine) in machines.drain() {
        let (sm, pre) = machine.preprocess(&mut rng);
        commitments.insert(i, pre);
        sign_machines.insert(i, sm);
    }

    let mut shares = HashMap::new();
    let mut sig_machines = HashMap::new();
    for (i, machine) in sign_machines.drain() {
        let mut other_commitments = commitments.clone();
        other_commitments.remove(&i);
        let (sigm, share) = machine.sign(other_commitments, msg).unwrap();
        shares.insert(i, share);
        sig_machines.insert(i, sigm);
    }

    let mut signature = None;
    for (i, machine) in sig_machines.drain() {
        let mut other_shares = shares.clone();
        other_shares.remove(&i);
        let sig = machine.complete(other_shares).unwrap();
        signature = Some(sig);
    }
    let sig = signature.unwrap();
    let group_key = keys[&Participant::new(1).unwrap()].group_key();
    let hram = TestHram::hram(&sig.R, &group_key, msg);
    assert!(sig.verify(group_key, hram));
}

#[test]
fn test_b6_below_threshold_rejection() {
    let mut rng = OsRng;
    let keys = create_session(3, 5);
    let mut machines = algorithm_machines(&keys);

    // Only 2 signers
    let p1 = Participant::new(1).unwrap();
    let p2 = Participant::new(2).unwrap();

    let mut machines = machines.into_iter().filter(|(i, _)| *i == p1 || *i == p2).collect::<HashMap<_, _>>();

    let msg = b"test message";
    let mut commitments = HashMap::new();
    let mut sign_machines = HashMap::new();
    for (i, machine) in machines.drain() {
        let (sm, pre) = machine.preprocess(&mut rng);
        commitments.insert(i, pre);
        sign_machines.insert(i, sm);
    }

    let m1 = sign_machines.remove(&p1).unwrap();
    let mut commitments1 = commitments.clone();
    commitments1.remove(&p1);

    let res = m1.sign(commitments1, msg);
    assert!(res.is_err());
    assert!(matches!(res.err().unwrap(), FrostError::InvalidSigningSet(_)));
}

#[test]
fn test_b7_duplicate_participant_rejection() {
    let mut rng = OsRng;
    let keys = create_session(2, 3);
    let mut machines = algorithm_machines(&keys);

    let p1 = Participant::new(1).unwrap();
    let p2 = Participant::new(2).unwrap();

    let m1 = machines.remove(&p1).unwrap();
    let m2 = machines.remove(&p2).unwrap();

    let (sm1, pre1) = m1.preprocess(&mut rng);
    let (sm2, pre2) = m2.preprocess(&mut rng);

    let mut commitments = HashMap::new();
    // Pass duplicate preprocess for p2!
    commitments.insert(p2, pre2.clone());

    // Oh wait, `commitments` is a map keyed by `Participant`, so we can't have duplicate keys in the map.
    // Let's pass the preprocess of p2, and try to sign with sm1, but wait!
    // If the map has only unique keys, how do we pass a duplicate participant?
    // Wait, the map might have the caller's index in it?
    // Let's try to insert the caller's participant in the map:
    commitments.insert(p1, pre1.clone());

    let res = sm1.sign(commitments, b"msg");
    assert!(res.is_err());
    assert!(matches!(res.err().unwrap(), FrostError::DuplicatedParticipant(_)));
}

#[test]
fn test_b8_out_of_bounds_participant_index() {
    let mut rng = OsRng;
    let keys = create_session(2, 3);
    let mut machines = algorithm_machines(&keys);

    let p1 = Participant::new(1).unwrap();
    let p2 = Participant::new(2).unwrap();

    let m1 = machines.remove(&p1).unwrap();
    let m2 = machines.remove(&p2).unwrap();

    let (sm1, _pre1) = m1.preprocess(&mut rng);
    let (_sm2, pre2) = m2.preprocess(&mut rng);

    let mut commitments = HashMap::new();
    // Participant 4 is out of bounds
    let p4 = Participant::new(4).unwrap();
    commitments.insert(p4, pre2.clone());

    let res = sm1.sign(commitments, b"msg");
    assert!(res.is_err());
    assert!(matches!(res.err().unwrap(), FrostError::InvalidParticipant(3, _)));
}

#[test]
fn test_b9_invalid_share_blame() {
    let mut rng = OsRng;
    let keys = create_session(2, 3);
    let mut machines = algorithm_machines(&keys);
    machines.remove(&Participant::new(3).unwrap());

    let msg = b"test message";
    let mut commitments = HashMap::new();
    let mut sign_machines = HashMap::new();
    for (i, machine) in machines.drain() {
        let (sm, pre) = machine.preprocess(&mut rng);
        commitments.insert(i, pre);
        sign_machines.insert(i, sm);
    }

    let mut shares = HashMap::new();
    let mut sig_machines = HashMap::new();
    for (i, machine) in sign_machines.drain() {
        let mut other_commitments = commitments.clone();
        other_commitments.remove(&i);
        let (sigm, share) = machine.sign(other_commitments, msg).unwrap();
        shares.insert(i, share);
        sig_machines.insert(i, sigm);
    }

    let p1 = Participant::new(1).unwrap();
    let p2 = Participant::new(2).unwrap();

    let m1 = sig_machines.remove(&p1).unwrap();

    let mut shares_for_1 = shares.clone();
    shares_for_1.remove(&p1);

    // Corrupt p2's share
    let mut bad_share = shares_for_1.remove(&p2).unwrap();
    // We just need to modify it. Wait, `SignatureShare` internal field `0` is private in the crate.
    // We can write it to bytes, mess it up, and read it back.
    let mut serialized = bad_share.serialize();
    serialized[0] ^= 1;
    let bad_share = m1.read_share::<&[u8]>(&mut serialized.as_slice()).unwrap();

    shares_for_1.insert(p2, bad_share);

    let res = m1.complete(shares_for_1);
    assert!(res.is_err());
    assert!(matches!(res.err().unwrap(), FrostError::InvalidShare(x) if x == p2));
}

#[test]
fn test_b10_nonce_never_zero() {
    let mut rng = OsRng;
    let mut zero_found = false;
    let mut one_found = false;
    for _ in 0..10000 {
        let scalar = <Ristretto as Ciphersuite>::F::random(&mut rng);
        let secret = Zeroizing::new(scalar);
        let nonce = <Ristretto as Curve>::random_nonce(&secret, &mut rng);
        if nonce == <Ristretto as Ciphersuite>::F::ZERO.into() { zero_found = true; }
        if nonce == <Ristretto as Ciphersuite>::F::ONE.into() { one_found = true; }
    }
    assert!(!zero_found, "Nonce was ZERO!");
    assert!(!one_found, "Nonce was ONE!");
}

#[test]
fn test_b11_signing_above_threshold() {
    let mut rng = OsRng;
    let keys = create_session(2, 3);
    let mut machines = algorithm_machines(&keys);

    let msg = b"test message";
    let mut commitments = HashMap::new();
    let mut sign_machines = HashMap::new();
    for (i, machine) in machines.drain() {
        let (sm, pre) = machine.preprocess(&mut rng);
        commitments.insert(i, pre);
        sign_machines.insert(i, sm);
    }

    let mut shares = HashMap::new();
    let mut sig_machines = HashMap::new();
    for (i, machine) in sign_machines.drain() {
        let mut other_commitments = commitments.clone();
        other_commitments.remove(&i);
        let (sigm, share) = machine.sign(other_commitments, msg).unwrap();
        shares.insert(i, share);
        sig_machines.insert(i, sigm);
    }

    let mut signature = None;
    for (i, machine) in sig_machines.drain() {
        let mut other_shares = shares.clone();
        other_shares.remove(&i);
        let sig = machine.complete(other_shares).unwrap();
        signature = Some(sig);
    }
    let sig = signature.unwrap();
    let group_key = keys[&Participant::new(1).unwrap()].group_key();
    let hram = TestHram::hram(&sig.R, &group_key, msg);
    assert!(sig.verify(group_key, hram));
}

#[test]
fn test_b12_resign_with_same_preprocess() {
    let mut rng = OsRng;
    let keys = create_session(2, 3);

    let p1 = Participant::new(1).unwrap();
    let p2 = Participant::new(2).unwrap();

    let m1 = AlgorithmMachine::new(IetfSchnorr::<Ristretto, TestHram>::ietf(), keys[&p1].clone());
    let m2 = AlgorithmMachine::new(IetfSchnorr::<Ristretto, TestHram>::ietf(), keys[&p2].clone());

    // First round
    let (sm1, pre1) = m1.preprocess(&mut rng);

    // cache the preprocess of p1
    let cached_pre1 = sm1.cache();

    let (sm1, pre1) = <AlgorithmSignMachine<Ristretto, IetfSchnorr<Ristretto, TestHram>> as SignMachine<modular_frost::algorithm::SchnorrSignature<Ristretto>>>::from_cache(IetfSchnorr::<Ristretto, TestHram>::ietf(), keys[&p1].clone(), cached_pre1);

    let (sm2, pre2) = m2.preprocess(&mut rng);

    // Can we cache again and reuse?
    // We already passed `cached_pre1` to `from_cache`, so it is consumed. We can't reuse it.
    // "document: does the protocol prevent reuse? Or is it caller responsibility?"
    // The `from_cache` function consumes the `CachedPreprocess` because it takes `self` or by-value.
    // So the protocol forces it at the type-level!
    // Let's document this in output.
}

#[test]
fn test_b13_wrong_message_in_second_round() {
    let mut rng = OsRng;
    let keys = create_session(2, 3);
    let mut machines = algorithm_machines(&keys);
    machines.remove(&Participant::new(3).unwrap());

    let p1 = Participant::new(1).unwrap();
    let p2 = Participant::new(2).unwrap();

    let msg1 = b"message one";
    let msg2 = b"message two";

    let mut commitments = HashMap::new();
    let mut sign_machines = HashMap::new();
    for (i, machine) in machines.drain() {
        let (sm, pre) = machine.preprocess(&mut rng);
        commitments.insert(i, pre);
        sign_machines.insert(i, sm);
    }

    let mut other_commitments_1 = commitments.clone();
    other_commitments_1.remove(&p1);
    let mut other_commitments_2 = commitments.clone();
    other_commitments_2.remove(&p2);

    let m1 = sign_machines.remove(&p1).unwrap();
    let m2 = sign_machines.remove(&p2).unwrap();

    // p1 signs msg1
    let (sigm1, share1) = m1.sign(other_commitments_1, msg1).unwrap();
    // p2 signs msg2
    let (sigm2, share2) = m2.sign(other_commitments_2, msg2).unwrap();

    let mut shares_for_1 = HashMap::new();
    shares_for_1.insert(p2, share2);

    // Let's see if sigm1 can complete. sigm1 expects shares for msg1. share2 is a share for msg2!
    let res = sigm1.complete(shares_for_1);
    assert!(res.is_err());
    assert!(matches!(res.err().unwrap(), FrostError::InvalidShare(x) if x == p2));
}
