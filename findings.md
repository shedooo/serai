# Serai Audit - Findings Register

All findings (confirmed, unconfirmed, informational) are tracked here.  
Severity may be **upgraded** as cross-crate analysis reveals broader patterns.

---

## STATUS KEY
- **Unconfirmed** - Suspected but not yet proven exploitable
- **Confirmed** - Proven with PoC or code trace
- **Cleared** - Investigated and determined non-exploitable
- **Escalated** - Severity upgraded due to broader context
- **Reported** - Submitted to Immunefi

---

## TRIAGE NOTES (2026-05-03)

- Jules artifact review (`jules/`) produced **no new confirmed finding** for `dkg-musig`, `modular-frost`, `dkg-dealer`, or panic/boundary tests.
- `dkg-musig` key-ordering behavior is already tracked as **FINDING-06** (duplicate).
- MuSig identity-point acceptance did not establish an attacker-reachable exploit and is treated as caller-policy / non-guaranteed key usability, not a standalone vulnerability.
- `modular-frost` `Schnorr::verify()` / `verify_share()` before `sign_share()` remains the documented precondition panic pattern (same class as **FINDING-04**), not a new finding.
- `dkg-dealer` zero-participant edge remains **FINDING-07** (`key_gen(3, 0)` returns `Ok(empty)`); any claim of panic/rejection on that exact input is a false positive.
- `multiexp()` empty-input panic claims are false positives; public `multiexp()` / `multiexp_vartime()` dispatch to `Algorithm::Null` and return identity on empty input.
- No Jules claim justified a High-severity upgrade, and **W-01 remains open** pending concrete attacker-reachable path in downstream caller analysis.

---

## FINDING-01
**Title:** `PublicKey::from_bytes().unwrap()` panic on identity group key in frost-schnorrkel verify()  
**Severity:** Low (Informational pending broader context)  
**Crate:** frost-schnorrkel (`crypto/schnorrkel`)  
**Location:** `lib.rs:141` - `Schnorrkel::verify()`  
**Status:** Confirmed - but trigger is computationally infeasible in isolation

**Description:**  
`verify()` calls `PublicKey::from_bytes(&group_key.to_bytes()).unwrap()`. schnorrkel's `PublicKey::from_bytes` rejects the identity point (returns `None`), causing `unwrap()` to panic. The group key comes from `ThresholdView::group_key()` which is determined during DKG. If the DKG produces an identity group key, or if a key offset moves the group key to identity, this function panics instead of returning `None` (verification failure).

**Cryptographic Impact:**  
Denial of service. No key recovery or forgery, but the signing session panics instead of cleanly erroring.

**Attack Scenario:**  
An attacker would need to cause the DKG to produce an identity group key. In PedPoP, this requires all participants' commitments to sum to identity - computationally infeasible with honest participants. In MuSig, this requires `sum(a_i * X_i) = 0` - also infeasible given binding factors derived from hash.

However: `ThresholdKeys::offset()` applies an arbitrary scalar offset to the group key. If an offset equals the negation of the current group key's discrete log, the resulting group key is identity. **This becomes relevant if any caller computes offsets from external/untrusted data.**

**Broader Pattern Watch:**  
- Does `bitcoin-serai::tweak_keys()` ever produce an identity group key after tweak+negation?
- Does any DKG sub-crate allow callers to inject offsets from untrusted sources?
- Does `ThresholdKeys::offset()` check for identity result?

**Proof of Concept:**
1. Construct `ThresholdKeys` where `group_key = G` (generator)
2. Call `keys.offset(-secret)` where `secret` is the discrete log - group key becomes identity
3. Use these keys in a FROST signing session with `Schnorrkel` algorithm
4. When `complete()` calls `verify()`, `PublicKey::from_bytes(&identity.to_bytes()).unwrap()` panics

**Recommendation:**  
Replace `unwrap()` with `ok()?` or return `None` on conversion failure:
```rust
let pk = PublicKey::from_bytes(&group_key.to_bytes()).ok()?;
```

**Immunefi Category:** Low - undocumented panic reachable from public API (contingent on offset/DKG producing identity key)

---

## FINDING-02
**Title:** `Signature::from_bytes().unwrap()` panic in frost-schnorrkel verify()  
**Severity:** Informational  
**Crate:** frost-schnorrkel (`crypto/schnorrkel`)  
**Location:** `lib.rs:140` - `Schnorrkel::verify()`  
**Status:** Confirmed safe - trigger conditions cannot occur

**Description:**  
`verify()` calls `Signature::from_bytes(&sig).unwrap()` after setting `sig[63] |= 1 << 7`. schnorrkel's `Signature::from_bytes` checks that byte 63 has the high bit set (which we just ensured) and performs minimal other validation. This unwrap is safe because:
- `R` bytes (0-31) are a valid compressed Ristretto point from nonce sum
- `s` bytes (32-63) are a valid reduced scalar with high bit artificially set
- schnorrkel masks off the high bit during deserialization

**Broader Pattern Watch:**  
If schnorrkel's `Signature::from_bytes` validation becomes stricter in a future version, this unwrap could break.

**Recommendation:**  
Defensive: replace with `.ok()?` for future-proofing.

---

## FINDING-03
**Title:** `sign_share` panics on context >= 2^32 bytes  
**Severity:** Informational  
**Crate:** frost-schnorrkel (`crypto/schnorrkel`)  
**Location:** `lib.rs:123` - `Schnorrkel::sign_share()`  
**Status:** Confirmed - practically unreachable

**Description:**  
`u32::try_from(self.context.len()).expect("context exceeded 2^32 bytes")` panics if the context byte slice is >= 4GB. Since `context` is `&'static [u8]`, this requires a 4GB+ static byte array, which is unrealistic on any production platform.

**Broader Pattern Watch:**  
If `Schnorrkel::new()` is ever changed to accept `&[u8]` instead of `&'static [u8]`, this becomes more reachable.

**Recommendation:**  
Return an error from `new()` instead of panicking in `sign_share()`:
```rust
pub fn new(context: &'static [u8]) -> Result<Schnorrkel, Error> {
    u32::try_from(context.len()).map_err(|_| Error::ContextTooLarge)?;
    // ...
}
```

---

## FINDING-04
**Title:** `self.msg.as_ref().unwrap()` panic if verify() called before sign_share()  
**Severity:** Informational  
**Crate:** frost-schnorrkel (`crypto/schnorrkel`)  
**Location:** `lib.rs:143` - `Schnorrkel::verify()`  
**Status:** Confirmed - documented precondition in Algorithm trait

**Description:**  
`verify()` accesses `self.msg` which is set to `Some(...)` in `sign_share()`. If `verify()` is called without a prior `sign_share()`, `self.msg` is `None` and `unwrap()` panics. This is a documented precondition: the `Algorithm` trait states "verify, verify_share MUST be called after sign_share is called."

The FROST state machine (`AlgorithmSignatureMachine::complete()`) always calls `sign_share` before `verify`, so this is not reachable through the normal FROST flow.

**Broader Pattern Watch:**  
If any code path calls `Algorithm::verify()` directly without going through the FROST state machine, this panics.

**Recommendation:**  
Replace with `self.msg.as_ref()?` to return `None` instead of panicking.

---

## FINDING-05
**Title:** Identity nonce sum would panic in frost-schnorrkel but is computationally infeasible  
**Severity:** Informational (Cleared)  
**Crate:** frost-schnorrkel (`crypto/schnorrkel`) + modular-frost (`crypto/frost`)  
**Location:** `lib.rs:49-50` (frost-schnorrkel) + `nonce.rs:194-211` (modular-frost)  
**Status:** Cleared - computationally infeasible

**Description:**  
If the aggregate nonce `R = sum(D_i + b_i * E_i)` across all participants equals the identity point, then `PublicKey::from_bytes` in `SchnorrkelHram::hram()` (line 49) would panic. However, the binding factors `b_i` are derived from a hash of all commitments (including `D_i` and `E_i`), making it computationally infeasible for a malicious participant to craft commitments that cause the sum to be identity. Individual commitments are validated as non-identity by `Curve::read_G()`.

**Broader Pattern Watch:**  
This is the same "negligible probability" argument used by the IETF FROST spec for `hash_binding_factor` returning zero. Both are ~2^-252 probability.

---

## FINDING-06
**Title:** Key ordering sensitivity in musig is undocumented  
**Severity:** Informational  
**Crate:** dkg-musig (`crypto/dkg/musig`)  
**Location:** `lib.rs:65-84` - `binding_factor_transcript()` + `binding_factor()`  
**Status:** Confirmed - caller responsibility

**Description:**  
The binding factor transcript concatenates keys in the order provided by the caller: `context || keys_len || X_1 || X_2 || ... || X_n || i`. If two participants call `musig()` with the same keys in a different order, they compute different binding factors and different group keys. The protocol silently produces incompatible key sets.

**Cryptographic Impact:**  
None directly. This is a correctness issue, not a security issue. Participants with different orderings simply cannot produce valid threshold signatures together.

**Broader Pattern Watch:**  
In Serai, the validator-sets pallet provides keys in a deterministic order (by validator set). If any other caller uses `musig()` without ensuring ordering, they'll get incompatible keys.

**Recommendation:**  
Document in the API that keys must be provided in the same order by all participants. Optionally, sort keys by canonical encoding before computing binding factors (as BIP-327 does).

---

## FINDING-07
**Title:** `dkg-dealer::key_gen()` accepts `participants == 0` and returns `Ok(empty)`  
**Severity:** Informational  
**Crate:** dkg-dealer (`crypto/dkg/dealer`)  
**Location:** `lib.rs:18-67` - `key_gen()`  
**Status:** Confirmed

**Description:**  
`key_gen()` never validates `participants` up front. If called with `participants == 0`, both participant loops are skipped and the function returns `Ok(HashMap::new())`. This bypasses the `ThresholdParams::new()` validation which would normally reject `n == 0`.

**Cryptographic Impact:**  
Fail-open on invalid parameters. No direct key compromise, but callers can receive a nominally successful result for an impossible configuration and may treat it as valid setup.

**Attack Scenario:**  
Any caller which forwards unvalidated `(threshold, participants)` values into `dkg_dealer::key_gen()` can be induced to accept an invalid zero-participant DKG result. In practice this is more likely to cause logic confusion or unhandled empty-map behavior than a direct exploit.

**Broader Pattern Watch:**  
- Are any higher-level ceremony constructors assuming `Ok(...)` implies `participants >= 1`?
- Do any integration layers special-case empty key maps incorrectly?

**Proof of Concept:**
```rust
let res = dkg_dealer::key_gen::<_, Ristretto>(&mut rng, 0, 0);
assert_eq!(res, Ok(HashMap::new()));
```

**Recommendation:**  
Reject invalid participant counts at the top of `key_gen()` before generating coefficients.

**Immunefi Category:** Informational - invalid-parameter fail-open

---

## FINDING-08
**Title:** `ThresholdKeys::new()` can panic on malformed `Interpolation::Constant` input  
**Severity:** Low  
**Crate:** dkg (`crypto/dkg`)  
**Location:** `src/lib.rs:224-249`, `src/lib.rs:347-390` - `Interpolation::interpolation_factor()`, `ThresholdKeys::new()`  
**Status:** Confirmed

**Description:**  
`ThresholdKeys::new()` accepts `Interpolation::Constant(Vec<F>)` without validating the vector length against `params.n()`. During group-key derivation it calls `interpolation.interpolation_factor(*i, &t)`, which indexes `c[participant - 1]`. If the vector is shorter than `n`, the public constructor panics with an out-of-bounds access instead of returning `Err`.

**Cryptographic Impact:**  
Denial of service via malformed public API input. No cryptographic break, but a caller can crash the process by constructing `ThresholdKeys` with a short constant-coefficient vector.

**Attack Scenario:**  
Any application exposing `ThresholdKeys::new()` across an FFI, RPC, plugin boundary, or deserialization layer could be crashed by attacker-controlled constant interpolation data. This is not currently reachable through Serai's shipped `dealer`, `pedpop`, `musig`, or `promote` flows, all of which build well-formed interpolation data internally.

**Broader Pattern Watch:**  
- `ThresholdKeys::read()` also reconstructs `Interpolation::Constant` from serialized data and inherits the same unchecked-length assumption.
- `dkg-musig` and `dkg-promote` use `Interpolation::Constant`, so regressions there could widen reachability.

**Proof of Concept:**
```rust
let params = ThresholdParams::new(3, 3, Participant::new(1).unwrap()).unwrap();
let shares = valid_verification_shares();
let _ = ThresholdKeys::<Ristretto>::new(
    params,
    Interpolation::Constant(vec![Scalar::ONE]),
    Zeroizing::new(Scalar::ONE),
    shares,
); // panics
```

**Recommendation:**  
Validate `Interpolation::Constant(c)` before use and return `Err` on length mismatch.

**Immunefi Category:** Low - public API panic on malformed input

---

## FINDING-09
**Title:** `pedpop` blame APIs can panic on out-of-set sender or recipient IDs  
**Severity:** Low  
**Crate:** dkg-pedpop (`crypto/dkg/pedpop`)  
**Location:** `src/lib.rs:575-609`, `src/lib.rs:623-681`; `src/encryption.rs:381-390` - `blame_internal()`, `BlameMachine::blame()`, `AdditionalBlameMachine::blame()`, `decrypt_with_proof()`  
**Status:** Confirmed

**Description:**  
`BlameMachine::blame()` and `AdditionalBlameMachine::blame()` accept arbitrary `sender` / `recipient` `Participant` values, but `blame_internal()` later indexes internal maps with those values:
- `self.commitments[&sender]`
- `self.enc_keys[&decryptor]`

There is no membership check before these lookups. Supplying an out-of-set participant can panic the public blame path instead of returning a structured error.

**Cryptographic Impact:**  
Denial of service in the protocol's fault-adjudication path. This does not let an attacker forge blame outcomes, but it can crash a service processing adversarial blame claims.

**Attack Scenario:**  
Blame inputs are adversarial by design: the API is meant to evaluate accusations made after a failed DKG. A remote peer or integration layer can submit a blame request referencing a non-member participant ID and trigger a panic if the caller forwards it directly.

**Broader Pattern Watch:**  
- `AdditionalBlameMachine::new()` explicitly documents invalid commitment input as undefined behavior; the blame methods have a similar trust assumption but do not document the participant-membership precondition.
- Any networking layer exposing blame as a message handler must validate member IDs before calling the library.

**Proof of Concept:**
1. Complete PedPoP until a `BlameMachine` or `AdditionalBlameMachine` is constructed.
2. Call `blame(sender = Participant(999), recipient = honest_member, ...)`.
3. The code reaches `self.commitments[&sender]` and panics.

The dual case also exists for an out-of-set `recipient`, which panics in `decrypt_with_proof()` via `self.enc_keys[&decryptor]`.

**Recommendation:**  
Validate both `sender` and `recipient` against the known participant set and return a structured error on invalid IDs before indexing internal maps.

**Immunefi Category:** Low - public API panic on adversarial blame input

---

## CROSS-CRATE WATCH LIST

These are patterns to track as we review additional crates. A finding in one crate may elevate severity of findings in another.

| Watch ID | Pattern | Relevant Findings | Crates to Check | Status |
|---|---|---|---|---|
| W-01 | Identity group key after offset/tweak | FINDING-01 | bitcoin-serai (tweak_keys), dkg (offset) | dkg offset() allows it; Ethereum mitigates; Bitcoin still propagates negligible-probability infinity into downstream panic sites |
| W-02 | unwrap() on schnorrkel type conversions | FINDING-01, FINDING-02 | Any crate using schnorrkel types | Open |
| W-03 | Algorithm::verify() called outside FROST state machine | FINDING-04 | bitcoin-serai (Schnorr algorithm) | Open |
| W-04 | Negligible probability identity results treated as impossible | FINDING-05 | All FROST/DKG crates | Open |
| W-05 | Context/message length assumptions | FINDING-03 | frost-schnorrkel callers | Open |
