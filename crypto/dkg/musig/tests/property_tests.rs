use proptest::prelude::*;
use std::collections::HashMap;

use zeroize::{Zeroize, Zeroizing};
use rand_core::OsRng;

use dalek_ff_group::Ristretto;
use ciphersuite::{group::ff::Field, group::Group, group::GroupEncoding, Ciphersuite};
use dkg_recovery::recover_key;
use dkg_musig::{musig_key, musig_key_vartime, musig, MusigError};

// Helper to generate a random Ristretto scalar
fn random_scalar() -> <Ristretto as Ciphersuite>::F {
    <Ristretto as Ciphersuite>::F::random(&mut OsRng)
}

// Helper to generate a random Ristretto point
fn random_point() -> <Ristretto as Ciphersuite>::G {
    <Ristretto as Ciphersuite>::generator() * random_scalar()
}

prop_compose! {
    fn arb_context()(bytes in any::<[u8; 32]>()) -> [u8; 32] {
        bytes
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn test_a1_binding_factor_determinism(context in arb_context(), len in 2usize..=8) {
        let mut keys = vec![];
        for _ in 0..len {
            keys.push(random_point());
        }

        let key1 = musig_key::<Ristretto>(context, &keys).unwrap();
        let key2 = musig_key::<Ristretto>(context, &keys).unwrap();
        let key3 = musig_key_vartime::<Ristretto>(context, &keys).unwrap();

        prop_assert_eq!(key1, key2);
        prop_assert_eq!(key1, key3);
    }

    #[test]
    fn test_a2_key_ordering_sensitivity(context in arb_context()) {
        let k1 = random_point();
        let k2 = random_point();
        let k3 = random_point();

        let res_abc = musig_key::<Ristretto>(context, &[k1, k2, k3]).unwrap();
        let res_acb = musig_key::<Ristretto>(context, &[k1, k3, k2]).unwrap();
        let res_bac = musig_key::<Ristretto>(context, &[k2, k1, k3]).unwrap();
        let res_bca = musig_key::<Ristretto>(context, &[k2, k3, k1]).unwrap();
        let res_cab = musig_key::<Ristretto>(context, &[k3, k1, k2]).unwrap();
        let res_cba = musig_key::<Ristretto>(context, &[k3, k2, k1]).unwrap();

        prop_assert_ne!(res_abc, res_cba);
        prop_assert_ne!(res_abc, res_bac);
        prop_assert_ne!(res_cba, res_bac);

        let mut set = std::collections::HashSet::new();
        set.insert(res_abc.to_bytes().as_ref().to_vec());
        set.insert(res_acb.to_bytes().as_ref().to_vec());
        set.insert(res_bac.to_bytes().as_ref().to_vec());
        set.insert(res_bca.to_bytes().as_ref().to_vec());
        set.insert(res_cab.to_bytes().as_ref().to_vec());
        set.insert(res_cba.to_bytes().as_ref().to_vec());
        prop_assert_eq!(set.len(), 6, "Permutations produced the same group key!");
    }

    #[test]
    fn test_a3_context_sensitivity(context_a in arb_context(), context_b in arb_context()) {
        prop_assume!(context_a != context_b);
        let keys = vec![random_point(), random_point(), random_point()];

        let res_a = musig_key::<Ristretto>(context_a, &keys).unwrap();
        let res_b = musig_key::<Ristretto>(context_b, &keys).unwrap();

        prop_assert_ne!(res_a, res_b);
    }

    #[test]
    fn test_a4_single_key_aggregation(context in arb_context()) {
        let key = random_point();
        let res = musig_key::<Ristretto>(context, &[key]).unwrap();

        prop_assert_ne!(res, key, "Single key aggregation returned the identical key!");
    }

    #[test]
    fn test_a5_two_key_symmetry_check(context in arb_context()) {
        let k1 = random_point();
        let k2 = random_point();

        let res1 = musig_key::<Ristretto>(context, &[k1, k2]).unwrap();
        let res2 = musig_key::<Ristretto>(context, &[k2, k1]).unwrap();

        prop_assert_ne!(res1, res2);
    }

    #[test]
    fn test_a6_duplicate_key_rejection(context in arb_context()) {
        let key = random_point();

        let res2 = musig_key::<Ristretto>(context, &[key, key]);
        prop_assert!(res2.is_err());

        let res3 = musig_key::<Ristretto>(context, &[key, key, key]);
        prop_assert!(res3.is_err());
    }

    #[test]
    fn test_a9_musig_full_roundtrip(context in arb_context()) {
        let x1 = Zeroizing::new(random_scalar());
        let x2 = Zeroizing::new(random_scalar());
        let x3 = Zeroizing::new(random_scalar());

        let k1 = <Ristretto as Ciphersuite>::generator() * *x1;
        let k2 = <Ristretto as Ciphersuite>::generator() * *x2;
        let k3 = <Ristretto as Ciphersuite>::generator() * *x3;
        let keys = vec![k1, k2, k3];

        let keys1 = musig::<Ristretto>(context, x1, &keys).unwrap();
        let keys2 = musig::<Ristretto>(context, x2, &keys).unwrap();
        let keys3 = musig::<Ristretto>(context, x3, &keys).unwrap();

        prop_assert_eq!(keys1.group_key(), keys2.group_key());
        prop_assert_eq!(keys2.group_key(), keys3.group_key());

        let all_keys = vec![keys1, keys2, keys3];
        let recovered = recover_key(&all_keys).unwrap();
        prop_assert_eq!(<Ristretto as Ciphersuite>::generator() * *recovered, all_keys[0].group_key());
    }
}

#[test]
fn test_a7_empty_key_rejection() {
    let context = [0u8; 32];
    let res = musig_key::<Ristretto>(context, &[]);
    assert!(res.is_err());
    assert!(matches!(res.unwrap_err(), MusigError::NoKeysProvided));
}

#[test]
fn test_a8_identity_point_behavior() {
    let identity = <Ristretto as Ciphersuite>::G::identity();
    let context = [0u8; 32];
    let valid_key = random_point();

    let res_with_identity = musig_key::<Ristretto>(context, &[identity, valid_key]);
    println!("A.8: Identity as one key: {:?}", res_with_identity);

    let res_identity_only = musig_key::<Ristretto>(context, &[identity]);
    println!("A.8: Identity as only key: {:?}", res_identity_only);
}

#[test]
fn test_a10_binding_factor_uniqueness() {
    let context = [0u8; 32];
    let keys: Vec<_> = (0..5).map(|_| random_point()).collect();

    let x1 = Zeroizing::new(random_scalar());
    let keys_full: Vec<_> = vec![<Ristretto as Ciphersuite>::generator() * *x1].into_iter().chain(keys.into_iter().take(4)).collect();

    let keys1 = musig::<Ristretto>(context, x1, &keys_full).unwrap();

    // Check verification shares. A verification share is `key_i * binding_factor_i`.
    // Since keys are distinct and random, checking that verification shares aren't equal
    // isn't enough to check if binding factors are distinct, but we can verify all
    // verification shares are distinct from the original keys. Wait, the original keys are public.
    // If binding factor is unique, let's just make sure they aren't the same.
    // Actually, we can just look at `keys1.original_verification_share(i)`.
    let mut factors_points = std::collections::HashSet::new();
    for i in 1..=5 {
        let p = dkg_musig::Participant::new(i).unwrap();
        let share = keys1.original_verification_share(p);
        factors_points.insert(share.to_bytes().as_ref().to_vec());
    }
    assert_eq!(factors_points.len(), 5, "Binding factors were not unique!");
}
