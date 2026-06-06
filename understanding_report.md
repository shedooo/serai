# Serai Bug Bounty Audit — Codebase Understanding Report

**Date:** 2026-04-23  
**Auditor:** Senior Cryptographic Systems Auditor  
**Repo:** serai-dex/serai (develop branch)  
**Program:** Immunefi Bug Bounty

---

## 1. PROTOCOL SUMMARY

**Serai** is a cross-chain DEX for Bitcoin, Ethereum, and Monero. The in-scope crypto stack provides threshold signing and key generation primitives allowing a distributed validator set to collectively control funds on external chains.

The library stack implements: **DKG** (PedPoP, dealer, MuSig variants), **FROST** threshold signing, and **protocol bridges** (BIP-340/Taproot, Schnorrkel/Substrate).

**Security guarantees:** (1) No single party below threshold can sign or recover the group key, (2) Signatures are indistinguishable from single-signer, (3) Malicious participants are detectable via blame, (4) Transcript integrity prevents cross-protocol attacks, (5) Secret material is zeroized.

**Trust model:** Libraries assume authenticated channels. DKG assumes authenticated broadcast. FROST assumes ≤ t-1 malicious participants. MuSig is n-of-n.

---

## 2. CRATE ARCHITECTURE MAP

### `flexible-transcript` (crypto/transcript)
- **Purpose:** Unified transcript abstraction for Fiat-Shamir transforms
- **Public API:** `Transcript` trait, `DigestTranscript<D>`, `MerlinTranscript`, `RecommendedTranscript`
- **Invariants:** Domain separation prevents collisions; Merlin wrapper asserts `label != "dom-sep"`
- **Prior fix CS-3.9.3:** `dom-sep` assertion — **verified holding**
- **no_std:** Yes

### `ciphersuite` (crypto/ciphersuite)
- **Purpose:** Unified elliptic curve trait (F, G, H, generator, hash_to_F, read_F/G)
- **Invariants:** `read_G` rejects non-canonical points; `read_F` rejects non-canonical scalars
- **no_std:** Yes

### `dalek-ff-group` (crypto/dalek-ff-group)
- **Purpose:** Wraps curve25519-dalek to implement ff/group traits for Ristretto/Ed25519
- **no_std:** Yes with std-shims

### `multiexp` (crypto/multiexp)
- **Purpose:** Multi-scalar multiplication (Straus/Pippenger) + `BatchVerifier`
- **Invariants:** `multiexp` is constant-time; `multiexp_vartime` is separate; BatchVerifier weights are non-zero
- **Prior fix CS-3.7.4:** Uses `black_box` + `u8_from_bool` — **verified holding**

### `schnorr-signatures` (crypto/schnorr)
- **Purpose:** Schnorr signatures (R, s) + half-aggregation per [ePrint 2021/350]
- **Invariants:** Challenge must bind R, A, m (caller responsibility); aggregation weights are unbiased
- **Prior fixes CS-3.8.4/CS-3.8.5:** `weight()` uses wide reduction with `(NUM_BITS + 128)/8` bytes — **verified holding**

### `dleq` (crypto/dleq)
- **Purpose:** Discrete Log Equality proofs across generators
- **Prior CS-3.4.4:** Prover doesn't check input consistency — by design (prover's own data)

### `dkg` (crypto/dkg)
- **Purpose:** Core types: `Participant` (non-zero u16), `ThresholdParams`, `ThresholdKeys`, `ThresholdView`
- **Prior fix CS-3.3.3:** `Participant::new(0)` returns `None` — **verified holding**
- **Prior fix CS-3.3.4:** FROST session domain separation now in sign.rs — **verified holding**
- **no_std:** Yes

### `dkg-dealer` (crypto/dkg/dealer) — no_std
- **Purpose:** Single-dealer key generation
- **Invariant:** Polynomial evaluation uses `Participant` (never zero)

### `dkg-recovery` (crypto/dkg/recovery) — no_std
- **Purpose:** Recover full private key from t threshold shares
- **Invariant:** Verifies recovered key against group key

### `dkg-musig` (crypto/dkg/musig) — **NEVER AUDITED**
- **Purpose:** MuSig n-of-n key aggregation with rogue-key prevention
- **Binding factor:** `hash_to_F("dkg-musig", context || keys_len || all_keys || i)`
- **Invariants:** All keys unique; binding factors commit to ALL keys + index

### `dkg-promote` (crypto/dkg/promote)
- **Purpose:** Promote threshold keys to a different generator via DLEq proofs

### `dkg-pedpop` (crypto/dkg/pedpop) — Complex, ~700 lines + 16KB encryption
- **Purpose:** Interactive DKG with PedPoP + encrypted share transport
- **State machine:** `KeyGenMachine` → `SecretShareMachine` → `KeyMachine` → `BlameMachine`

### `modular-frost` (crypto/frost)
- **Purpose:** Core FROST threshold signing protocol
- **Spec:** IETF FROST draft with extensions (generalized nonces, key offsets, Algorithm trait)
- **Binding factor:** `rho_transcript = new("FROST_rho") || group_key || H(msg) || H(H(preprocesses))`
- **IetfTranscript:** `pub(crate)` sealed — CS-3.6.7 fix **verified holding**

### `frost-schnorrkel` (crypto/schnorrkel) — **NEVER AUDITED, HIGHEST PRIORITY**
- **Purpose:** Bridge modular-frost ↔ external schnorrkel=0.11 for Substrate
- **Message format:** `[context_len_le32 || context || msg]`
- **Challenge:** Via schnorrkel's `signing_context().bytes()` then `challenge_scalar`

### `bitcoin-serai` (networks/bitcoin)
- **Purpose:** Bitcoin Taproot threshold signing + UTXO scanning
- **BIP-340 Hram:** Tagged hash, nonce negation for even R
- **Dust:** 546 sats; fee checking against `DEFAULT_MIN_RELAY_TX_FEE`

---

## 3. DEPENDENCY GRAPH

```
transcript → ciphersuite → multiexp → schnorr → dkg → frost
                                          ↓         ↓      ↓
                                        dleq    dkg-*    frost-schnorrkel
                                                         bitcoin-serai
```

**Highest-risk integration:** `frost-schnorrkel` bridges FROST (MerlinTranscript) ↔ schnorrkel (SigningTranscript). Two distinct transcript systems must produce compatible challenges.

---

## 4. PANIC SURFACE MAP (Non-test, public API reachable)

| Crate | File:Line | Source | Trigger |
|---|---|---|---|
| frost-schnorrkel | lib.rs:43 | `expect("malformed message")` | msg < 4 bytes |
| frost-schnorrkel | lib.rs:49 | `PublicKey::from_bytes().unwrap()` | identity point |
| frost-schnorrkel | lib.rs:52 | `Scalar::from_repr().unwrap()` | invalid repr |
| frost-schnorrkel | lib.rs:140 | `Signature::from_bytes().unwrap()` | invalid encoding |
| frost-schnorrkel | lib.rs:123 | `expect("context exceeded 2^32")` | context ≥ 4GB |
| frost | algorithm.rs:216,227 | `self.c.unwrap()` | verify before sign_share |
| bitcoin | crypto.rs:15,22 | `expect("point at infinity")` | identity key |
| bitcoin | send.rs:334,345 | `unimplemented!()` | cache/from_cache called |

---

## 5. INITIAL RISK SURFACE

### Highest suspicion
1. **frost-schnorrkel SchnorrkelHram** — Hand-rolled challenge bridge between two transcript systems. Multiple unwrap() calls. Message parsing with unchecked slicing.
2. **frost-schnorrkel verify** — `sig[63] |= 1 << 7` bit manipulation for Ristretto→schnorrkel format conversion
3. **dkg-musig binding factor transcript** — Raw concatenation of keys without per-key length prefix (safe only if group encodings are fixed-length)
4. **pedpop encryption.rs** — 16KB of encryption code, never reviewed here

### Novel/unaudited code
- `dkg-musig`, `frost-schnorrkel` (entire crates)
- `dkg-pedpop`, `dkg-promote`, `dkg-recovery` (new sub-crates)

### Silent failures
- `Curve::hash_binding_factor` can return zero (negligible probability, not handled per IETF spec)
- `IetfTranscript::rng_seed` has `unimplemented!()` — must verify no internal path reaches it
