use multiexp::{multiexp, multiexp_vartime, BatchVerifier};
use dalek_ff_group::Ristretto;
use ciphersuite::{Ciphersuite, group::{Group, ff::Field}};
use rand_core::OsRng;
use std::time::Instant;

type Scalar = <Ristretto as Ciphersuite>::F;
type Point = <Ristretto as Ciphersuite>::G;

#[test]
fn test_multiexp_zero_scalar() {
    let generator = Ristretto::generator();
    let pairs = vec![(Scalar::ZERO, generator)];
    let result = multiexp(&pairs);
    assert!(bool::from(result.is_identity()));
}

#[test]
fn test_multiexp_identity_point() {
    let identity_point = Point::identity();
    let pairs = vec![(Scalar::ONE, identity_point)];
    let result = multiexp(&pairs);
    assert!(bool::from(result.is_identity()));
}

#[test]
fn test_multiexp_empty() {
    let pairs: Vec<(Scalar, Point)> = vec![];
    let result = std::panic::catch_unwind(|| {
        multiexp::<Point>(&pairs)
    });

    match result {
        Ok(r) => println!("multiexp empty Ok: is_identity: {}", bool::from(r.is_identity())),
        Err(_) => println!("multiexp empty Panicked!"),
    }
}

#[test]
fn test_multiexp_vartime_large() {
    let generator = Ristretto::generator();
    let pairs: Vec<_> = (0..10_000)
        .map(|_| (Scalar::random(&mut OsRng), generator))
        .collect();
    let start = Instant::now();
    let result = multiexp_vartime(&pairs);
    let elapsed = start.elapsed();
    println!("multiexp_vartime_large elapsed: {:?}", elapsed);
}

#[test]
fn test_batch_verifier_empty() {
    let batch = BatchVerifier::<u32, Point>::new(0);
    let result = batch.verify_vartime();
    assert!(result);
}
