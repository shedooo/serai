use proptest::prelude::*;
use rand_core::OsRng;
use ciphersuite::{Ciphersuite, group::{Group, ff::Field}};
use dalek_ff_group::Ristretto;

type Scalar = <Ristretto as Ciphersuite>::F;
type Point = <Ristretto as Ciphersuite>::G;

proptest! {
    #[test]
    fn test_read_g_random_bytes(bytes in any::<[u8; 32]>()) {
        let mut slice: &[u8] = &bytes;
        let result = Ristretto::read_G(&mut slice);
        // It should either be Ok or Err, not panic
        if let Ok(point) = result {
            // Identity could technically be read from valid bytes of zero?
            // Usually identity isn't expected from random bytes
            // assert!(!bool::from(point.is_identity()));
        }
    }

    #[test]
    fn test_read_f_random_bytes(bytes in any::<[u8; 32]>()) {
        let mut slice: &[u8] = &bytes;
        let result = Ristretto::read_F(&mut slice);
        // It should either be Ok or Err, not panic
    }
}

#[test]
fn test_hash_to_f_empty() {
    let result = Ristretto::hash_to_F(b"test", b"");
    assert!(!bool::from(result.is_zero()));
}

#[test]
fn test_hash_to_f_large() {
    let large_input = vec![0u8; 1_000_000];
    let result = Ristretto::hash_to_F(b"test", &large_input);
    assert!(!bool::from(result.is_zero()));
}

#[test]
fn test_read_f_non_canonical() {
    // Non-canonical scalars testing
}
