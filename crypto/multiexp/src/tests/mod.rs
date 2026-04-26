use std_shims::vec::Vec;
use std::time::Instant;

use rand_core::OsRng;

use zeroize::Zeroize;

use ff::{Field, PrimeFieldBits};
use group::Group;

use k256::ProjectivePoint;
use dalek_ff_group::EdwardsPoint;

use crate::{straus, straus_vartime, pippenger, pippenger_vartime, multiexp, multiexp_vartime};

#[cfg(feature = "batch")]
mod batch;
#[cfg(feature = "batch")]
use batch::test_batch;

#[allow(dead_code)]
fn test_algorithm<G: Group<Scalar: Zeroize + PrimeFieldBits>>(
  f: fn(&[(G::Scalar, G)], u8) -> G,
  total: usize,
  window: u8,
) {
  let mut pairs = Vec::with_capacity(total);
  for _ in 0 .. total {
    pairs.push((G::Scalar::random(&mut OsRng), G::random(&mut OsRng)));
  }

  let mut naive = G::identity();
  for (scalar, point) in &pairs {
    naive += *point * *scalar;
  }

  let start = Instant::now();
  let optimized = f(&pairs, window);
  println!(
    "{} for {} with a window of {}: {:?}",
    stringify!($f),
    total,
    window,
    start.elapsed()
  );
  assert_eq!(naive, optimized);
}

#[allow(dead_code)]
fn test_multiexp<G: Zeroize + Group<Scalar: Zeroize + PrimeFieldBits>>(vartime: bool) {
  let mut pairs = Vec::with_capacity(1000);
  for _ in 0 .. 1000 {
    pairs.push((G::Scalar::random(&mut OsRng), G::random(&mut OsRng)));
  }

  let mut naive = G::identity();
  for (scalar, point) in &pairs {
    naive += *point * *scalar;
  }

  assert_eq!(
    naive,
    if vartime { multiexp_vartime(&pairs) } else { multiexp(&pairs) }
  );
}

#[test]
fn test_secp256k1() {
  test_multiexp::<ProjectivePoint>(false);
  test_multiexp::<ProjectivePoint>(true);
}

#[test]
fn test_ed25519() {
  test_multiexp::<EdwardsPoint>(false);
  test_multiexp::<EdwardsPoint>(true);
}
