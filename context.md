# Serai Audit - context.md

## Audit Overview

- **Protocol:** Serai - cross-chain DEX
- **Language:** Rust (Cargo workspace)
- **Date Started:** 2026-04-23
- **Repo:** https://github.com/serai-dex/serai (develop branch)
- **Immunefi Program:** https://immunefi.com/bounty/serai/
- **Scope:** 11 crates (Blockchain/DLT category)

| # | Crate Path | Crate Name | Status | Priority |
|---|---|---|---|---|
| 1 | crypto/schnorrkel | frost-schnorrkel | COMPLETE (5/5) | CRITICAL |
| 2 | crypto/dkg/musig | dkg-musig | COMPLETE (5/5) | CRITICAL |
| 3 | crypto/frost | modular-frost | COMPLETE (5/5) | CRITICAL |
| 4 | crypto/dkg | dkg | COMPLETE (5/5) | CRITICAL |
| 5 | crypto/dkg/pedpop | dkg-pedpop | COMPLETE (5/5) | HIGH |
| 6 | crypto/dkg/dealer | dkg-dealer | COMPLETE (5/5) | MEDIUM |
| 7 | crypto/dkg/recovery | dkg-recovery | COMPLETE (5/5) | MEDIUM |
| 8 | crypto/dkg/promote | dkg-promote | COMPLETE (3/3) | MEDIUM |
| 9 | networks/bitcoin | bitcoin-serai | COMPLETE (5/5 - CLEAN) | HIGH |
| 10 | crypto/schnorr | schnorr-signatures | COMPLETE (Chunks 1-6 complete) | HIGH |
| 11-15 | various | remaining | PENDING | MEDIUM |

---

## Audit Progress

- [x] Phase 1: Codebase understanding and report
- [x] Phase 3: frost-schnorrkel (5/5 - clean)
- [x] Phase 3: dkg-musig (5/5 - clean)
- [x] Phase 3: modular-frost (5/5 - clean)
- [x] Phase 3: dkg + pedpop + dealer + recovery (5/5)
- [x] Phase 3: bitcoin-serai (Chunks 1-5 - clean)
- [ ] Phase 3: Remaining crates
- [x] Phase 3: dkg-promote (closed)
- [x] Phase 3: schnorr-signatures Chunks 1-6

---

## Findings Log

**Full write-ups:** `C:\Users\MADAKI\.gemini\antigravity\brain\bdba6169-760b-48e5-8502-1cd70fc3ba5a\findings.md`

| # | Title | Severity | Crate | Status |
|---|---|---|---|---|
| FINDING-01 | PublicKey unwrap panic on identity group key | Low (pending W-01) | frost-schnorrkel | Confirmed |
| FINDING-02 | Signature::from_bytes unwrap in verify | Informational | frost-schnorrkel | Confirmed safe |
| FINDING-03 | sign_share panics on context >= 4GB | Informational | frost-schnorrkel | Confirmed unreachable |
| FINDING-04 | verify panics if called before sign_share | Low | frost-schnorrkel | Runtime confirmed |
| FINDING-05 | Identity nonce sum panic | Informational | frost-schnorrkel + frost | Cleared (infeasible) |
| FINDING-06 | Key ordering sensitivity in musig | Informational | dkg-musig | Caller responsibility |
| FINDING-07 | `dkg-dealer::key_gen` accepts `participants == 0` and returns an empty key map | Low | dkg-dealer | Runtime confirmed |
| FINDING-08 | `ThresholdKeys::new` can panic on short `Interpolation::Constant` vectors | Low | dkg | Confirmed |
| FINDING-09 | `pedpop` blame APIs can panic on out-of-set sender/recipient IDs | Low | dkg-pedpop | Confirmed |
| FINDING-10 | Outsider transit tampering misattributed as sender blame | Informational | dkg-pedpop | Confirmed |
| FINDING-11 | Fixed IV + no ephemeral scalar uniqueness check - keystream reuse on RNG failure | Low | dkg-pedpop | Confirmed |
| FINDING-12 | `musig_key` accepts identity point without rejection | Informational | dkg-musig | Confirmed |
| FINDING-13 | `multiexp()` panics on empty input | REMOVED - false positive | multiexp | Removed |
| FINDING-14 | `GeneratorPromotion::complete()` can panic on sparse peer-proof maps because it validates length/range but not exact participant coverage before `proofs.get(&i).unwrap()` | Low | dkg-promote | Confirmed |
| FINDING-15 | `SchnorrSignature::verify()` accepts identity public key; equation collapses to `R = sG` under `A = identity`, enabling trivial forgery | High | schnorr-signatures | HIGH CONFIRMED AND CLOSED - see full record in findings log |
| FINDING-16 | message-queue `client.rs:202` missing sender-signature verification (TODO in `next()` receiver) | Medium candidate | message-queue | Flagged - needs dedicated chunk |
| FINDING-17 | `SchnorrAggregate::verify` accepts identity validator keys on Tendermint commit path (`A = identity` collapses signer term; same class as F-15) | High | schnorr-signatures / coordinator-tributary | CONFIRMED - capped at High; all production callers enforce `verify_commit` threshold and on-chain identity registrability was not proven |
| FINDING-18 | `verify_aggregate` `read_G(signer).unwrap()` panics on non-canonical validator-ID bytes from p2p block-sync | Low | coordinator-tributary | CONFIRMED reachable; whole-process exit via global panic hook does not raise the tier above Low "undocumented panic" |
| FINDING-19 | `verify_commit` threshold check panics on unknown validator IDs via `self.weights[&validator]` after successful aggregate verification | Low | coordinator-tributary | CONFIRMED reachable with identity-key forged aggregate on p2p block-sync; whole-process exit via global panic hook does not raise the tier above Low "undocumented panic" |

**Finding totals (Session 26 update):**
- Total findings logged: 18 (`FINDING-13` removed, net 17 active)
- Immunefi-reportable Low candidates: 9 (`FINDING-01`, `FINDING-04`, `FINDING-07`, `FINDING-08`, `FINDING-09`, `FINDING-11`, `FINDING-14`, `FINDING-18`, `FINDING-19`)
- High confirmed: 2 (`FINDING-15`, `FINDING-17`)
- Critical confirmed: 0
- Medium candidates: 1 (`FINDING-16`)
- Panic findings: `FINDING-18` and `FINDING-19` are confirmed reachable from unauthenticated p2p block-sync; the global panic hook makes them whole-process exits, but they still map only to Low "undocumented panic reachable from a public API"
- Reachability vectors under investigation: `coordinator/src/tributary/transaction.rs:596`, `coordinator/src/tributary/transaction.rs:616`, `coordinator/src/tributary/handle.rs:721`

### Cross-Crate Watch List

| ID | Pattern | Check In | Status |
|---|---|---|---|
| W-01 | Identity group key after offset/tweak | bitcoin-serai, dkg | Closed for bitcoin-serai (informational probabilistic edge; no practical attacker-controlled trigger confirmed). Keep as cross-crate caller-hardening note |
| W-02 | unwrap on schnorrkel conversions | Any schnorrkel consumer | Closed for bitcoin-serai and processor Bitcoin path (no schnorrkel type-conversion surface found) |
| W-03 | Algorithm::verify() outside state machine | bitcoin-serai | Closed for bitcoin-serai (no direct external verify-call path found) |
| W-04 | Negligible probability identity = impossible | All FROST/DKG | Addressed - spec-compliant per RFC 9591 |
| W-05 | Context/message length assumptions | frost-schnorrkel callers | Closed - transcript fields are length-delimited and role-labeled (Merlin `append_message` framing plus explicit context-length prefix in frost-schnorrkel HRAM preimage) |
| W-06 | Adversarial nonce forcing aggregate identity | bitcoin-serai + modular-frost integration | Closed for bitcoin-serai scope (no practical forcing path confirmed) |

---

## modular-frost Audit Summary

### Chunks Completed: 5/5

**Chunk 1 (Orientation):** ~1150 lines across 8 production files. `IetfTranscript::rng_seed` is unreachable in production. `frost/sign.rs` uses only `domain_separate`, `append_message`, and `challenge`.

**Chunk 2 (Invariants):** 6 invariants mapped. Binding factor chain is correct: algorithm -> rho -> per-participant transcript commits all required values. Double-hash pattern is safe. Nonce reuse prevention is a documented contract. Participant validation is correct.

**Chunk 3 (Functions):** `sign()` `view().unwrap()` is safe after pre-validation. `Schnorr::sign_share` computes the challenge correctly. `verify()` uses the same documented precondition as FINDING-04. `calculate_binding_factors` is correct for standard Schnorr.

**Chunk 4 (Cross-crate):** `verify_share` equations are correct and use individual bound nonces. The Algorithm trust boundary is local only. All 5 curve context strings match RFC 9591. All `Hram` implementations match spec. W-01 update: FROST does not produce or modify the group key.

**Chunk 5 (Checklist):** All items pass. RFC 9591 compliant. Constant-time scalar operations. Zeroize on all secrets. No public API panics. Domain separation is correct.

**Verdict:** modular-frost is cryptographically sound. RFC 9591 compliant. No exploitable vulnerabilities.

---

## dkg + Sub-Crates Audit Summary

### Chunks Completed: 5/5

**Chunk 1 (Orientation):** Scope restored across:
- `crypto/dkg/src/lib.rs`
- `crypto/dkg/pedpop/src/lib.rs`
- `crypto/dkg/pedpop/src/encryption.rs`
- `crypto/dkg/dealer/src/lib.rs`
- `crypto/dkg/recovery/src/lib.rs`

Key orientation results:
- `dkg` core centers on `ThresholdParams`, `Interpolation`, `ThresholdKeys`, and `ThresholdView`.
- `ThresholdKeys::offset()` is additive-only and performs no identity or public-key validity check; `group_key()` and `view()` both consume the offseted state directly.
- `pedpop` uses a strict typed state machine: `KeyGenMachine -> SecretShareMachine -> KeyMachine -> BlameMachine -> AdditionalBlameMachine`.
- `pedpop::validate_map()` is the main exactness gate at both round boundaries.
- `pedpop` encryption is ephemeral-static ECDH with transcript-derived ChaCha20, a fixed IV, Schnorr PoP over `(context, nonce, key, sender, ciphertext)`, and DLEq proofs for blame adjudication.
- `dealer` is a thin wrapper that constructs `ThresholdKeys` from dealer-generated polynomial shares.
- `recovery` reconstructs by summing `view(included).secret_share()` and checking against `group_key()`.

No exploit confirmed in Chunk 1. Highest-risk follow-up items remain W-01, pedpop state-machine edge cases, and encryption misuse resistance.

**Chunk 2 (Invariants):** `ThresholdParams::new()` and `all_participant_indexes()` enforce the nonzero participant domain correctly, and `pedpop::validate_map()` correctly forces exact participant sets at both round boundaries. `Interpolation::Lagrange` unwraps are safe under current callers because duplicate participants are rejected before interpolation. Two invariant bugs were confirmed: `dkg-dealer::key_gen()` can return `Ok(empty)` when `participants == 0` because the invalid configuration bypasses `ThresholdParams::new()`, and `ThresholdKeys::new()` does not validate `Interpolation::Constant` vector length before indexing it, so a short vector can panic the public constructor.

**Chunk 3 (Functions):** `ThresholdKeys::view()` applies offsets correctly after interpolation, and `write()` / `read()` consistently drop ephemeral scalar/offset state as documented. W-01 remains unresolved because `offset()` still performs no post-tweak public-key validity check. `pedpop` encryption function review found no direct ciphertext-authentication break: the PoP binds `(context, nonce, ephemeral key, sender, ciphertext)` and blame DLEq proofs bind the recipient's registered decryption key. The fixed-IV ChaCha20 design is therefore safe only under the code's fresh-ephemeral-key assumption. A new low-severity issue was confirmed: `BlameMachine::blame()` / `AdditionalBlameMachine::blame()` accept arbitrary `Participant` inputs but later index `commitments` and `enc_keys` without validation, so out-of-set sender/recipient IDs can panic the public blame path.

**Chunk 4 (Cross-crate):** `dealer` and `pedpop` both feed internally well-formed `Interpolation::Lagrange` inputs into `ThresholdKeys::new()`, so FINDING-08 is not reachable through shipped sub-crates and remains a direct public-API bug. `recovery` correctly refuses mixed `(t, n, group_key, current_scalar, current_offset)` sets and therefore propagates tweaks consistently rather than silently normalizing them. Downstream W-01 review split callers into two classes: Ethereum hardens against invalid tweaked keys by incrementing the offset until `PublicKey::new()` succeeds, while Bitcoin's `tweak_keys()` explicitly documents the negligible-probability infinity case and returns the tweaked `ThresholdKeys` anyway. That Bitcoin path later reaches `x_only()`, which panics on infinity, so W-01 remains a live cross-crate panic condition even if its deterministic trigger probability is negligible for the BIP341 tweak.

**Chunk 5 (Checklist):**
- `dkg` core math is sound for shipped callers.
- `pedpop` state progression is structurally strong and its encryption/authentication layering is coherent.
- `recovery` is conservative and rejects mixed tweaked sessions correctly.
- No direct cryptographic break was found in the audited batch.
- Confirmed issues are all API-hardening / fail-open / panic-surface problems:
  - FINDING-07: zero-participant dealer success
  - FINDING-08: unchecked `Interpolation::Constant` length panic
  - FINDING-09: blame-path participant validation panic
- W-01 remains an open cross-crate watch item rather than a newly isolated `dkg`-only finding.

**Session 10 Gap-Fill (Q2/Q3):** The targeted follow-up on `crypto/dkg/pedpop/src/encryption.rs` resolved the remaining encryption questions. Q2: decrypt-before-verify is real in `Encryption::decrypt()`, but the decrypted secret share is wrapped in `Zeroizing`, explicitly zeroized on the caller failure path, and no confirmed plaintext-retention leak was found on PoP failure. Q3: the PoP challenge does bind ciphertext bytes, sender ID, context, and ephemeral public key; recipient ID is not explicit in the PoP, but recipient binding is implicit through the ECDH-derived ChaCha20 key. The fixed-IV design remains dependent on fresh ephemeral scalars, and no uniqueness check exists for repeated ephemeral keys.

**Verdict:** `dkg`, `dkg-pedpop`, `dkg-dealer`, and `dkg-recovery` are complete for this audit batch. No exploitable cryptographic break was confirmed. Three low-severity API robustness issues were identified, and W-01 remains open because downstream Bitcoin callers can still propagate the unchecked tweaked-key edge case into panic sites.

---

## bitcoin-serai Audit Summary

### Chunks Completed: 5/5

**Chunk 1 (Crate Orientation):** Scope restored across:
- `networks/bitcoin/src/lib.rs`
- `networks/bitcoin/src/crypto.rs`
- `networks/bitcoin/src/rpc.rs`
- `networks/bitcoin/src/wallet/mod.rs`
- `networks/bitcoin/src/wallet/send.rs`
- `networks/bitcoin/src/tests/mod.rs`
- `networks/bitcoin/src/tests/crypto.rs`
- integration tests under `networks/bitcoin/tests/*`

Key orientation results:
- `tweak_keys()` is in `networks/bitcoin/src/wallet/mod.rs`; tweak is `TapTweakHash::hash(&keys.group_key().to_bytes()[1..])`, reduced to a scalar and applied via `ThresholdKeys::offset()`.
- The tweak source is key-only BIP341-style hashing in this path (no caller-provided script tree bytes).
- Bitcoin key-path signing integrates FROST via `wallet/send.rs` (`AlgorithmMachine`, `AlgorithmSignMachine`, `AlgorithmSignatureMachine`) and custom Schnorr logic in `crypto.rs`.
- `ThresholdKeys::offset()` call sites in `networks/bitcoin` are concentrated in `wallet/mod.rs` (`tweak_keys`) and `wallet/send.rs` (per-input spend offsets).
- Infinity panic surfaces remain in `crypto.rs` (`x()` and `x_only()` expectations) and are downstream-reachable if an invalid tweaked key propagates.
- Ethereum hardening loop exists in production processor code (`processor/src/networks/ethereum.rs`: increment by one until `PublicKey::new(keys.group_key())` succeeds); bitcoin processor path calls `bitcoin_serai::wallet::tweak_keys()` directly and does not add this loop.
- No `dkg-promote` dependency edge was found from `networks/bitcoin` manifests or source references.

Preliminary W-01 stance after Chunk 1:
- Attacker-controlled script-tree content is not used in bitcoin-serai's tweak computation itself.
- The unchecked post-tweak validity condition remains present on the Bitcoin path, so W-01 remains open pending deeper reachability analysis in Chunk 2+.

**Chunk 2 (Deep Function Audit):**
- `wallet::tweak_keys()` was fully traced and its `scale(...).expect(...)` panic condition was resolved to `ThresholdKeys::scale` returning `None` only on zero scalar; the selected scalar is always `±1`, so this expect is not attacker-reachable.
- W-01 math path was resolved as probabilistic-only in this crate path: identity after tweak requires `k + H_tap(k) == 0 mod n` for the unknown discrete log `k` of the group key. This is a negligible-probability edge for random keys and does not accept script-tree content as attacker input in this function.
- Bitcoin processor integration remains unguarded versus Ethereum's increment-until-valid loop: Ethereum iterates until `PublicKey::new(group_key)` succeeds, while Bitcoin directly accepts `bitcoin_serai::wallet::tweak_keys` and proceeds.
- Processor Bitcoin does immediately call `scanner(keys.group_key())`; this is a panic-based assertion (`Scanner::new(...).unwrap()`), not a graceful validity check.
- Per-output offset path (`wallet/send.rs` offsets from scanned `ReceivedOutput`) is not an independent W-01 panic trigger: offsets are pre-filtered through `p2tr_script_buf`, and mismatches in `multisig()` return `None` rather than panic.
- `x()` / `x_only()` panic surfaces were mapped:
  - `wallet::p2tr_script_buf` guards parity before calling `x_only()`;
  - `crypto::Hram` uses `x(R)` and `x(A)` and can still panic on infinity if invalid key/nonce state reaches the algorithm.
- `crypto.rs:148` unwrap (`[u8; 64] -> [u8; 63]` slice conversion) and `wallet/mod.rs:146` unwrap (`Vec<u8>` writer) are non-adversarial/invariant unwraps under current code.
- W-02 closure confirmed for this path: no schnorrkel types or conversions in `networks/bitcoin` nor `processor/src/networks/bitcoin.rs`.

Chunk 2 verdict:
- W-01 is **probabilistic** (negligible-probability DoS edge), not practically attacker-reachable via script-tree tweak control in bitcoin-serai.
- W-02 is closed for bitcoin-serai and processor Bitcoin.
- W-03 is closed for bitcoin-serai.

**Chunk 3 (Signing Path + Adversarial Nonce Analysis):**
- Confirmed HRAM inputs in bitcoin-serai:
  - `R` at `crypto.rs:65` is the aggregated session nonce sum `Rs[0][0]` built by modular-frost (`BindingFactor::nonces`).
  - `A` at `crypto.rs:66` is `params.group_key()` from the signing view, i.e. the post-tweak key state passed into `AlgorithmMachine` (including per-output offset in the transaction path).
- Modular-frost does not explicitly validate aggregate `R != identity` before calling algorithm `sign_share` / `verify`; `Rs` is computed and passed through directly.
- Identity-point rejection exists for *individual deserialized commitment points* (`Curve::read_G` rejects identity), but this does not imply aggregate-`R` non-identity.
- Adversarial nonce forcing assessment:
  - Direct "negate all honest nonces" is not straightforward in FROST because bound nonces include participant-specific hash-derived binding factors over the full commitment set.
  - No practical deterministic forcing path was confirmed from code alone; resulting `R == identity` remains a probabilistic edge case.
- Failure mode remains panic if `R == identity` reaches bitcoin HRAM:
  - `x(R)` uses `encoded.x().expect("point at infinity")`.
- `x(A)` at `crypto.rs:66` is on the same key-validity axis as W-01 (post-tweak/post-offset key). No independent attacker-controlled `A` path was identified beyond previously tracked probabilistic key-identity edge conditions.
- `wallet/send.rs` panic sites classified:
  - line 122 / 125: integer conversion unwraps, invariant-only under realistic tx size bounds.
  - line 199: `PushBytesBuf::try_from(data).expect(...)`, guarded by `data.len() <= 80`, invariant-only.
  - line 389: taproot sighash `expect`, internal-index invariant with aligned prevouts.
- No new confirmed finding added in Chunk 3.

Chunk 3 verdict:
- `crypto.rs:65` (`x(R)`) remains a panic sink but no practical new exploit path was confirmed; treated as cleared for new-finding purposes in this chunk.
- `crypto.rs:66` (`x(A)`) is tied to the existing W-01-style key-identity axis, not a new independent path.
- Adversarial nonce "force `R` identity" was denied as a practical trigger (not confirmed).

**Chunk 4 (RPC Layer + Signing Integrity + Final Surfaces):**
- RPC panic sites in `rpc_call` were classified:
  - `rpc.rs:115` (`serde_json::to_vec(...).unwrap()`): invariant-only for current `serde_json::Value` payload construction.
  - `rpc.rs:118` (`hyper::Request::post(...).body(...).unwrap()`): caller-configuration failure surface (invalid URL), not network-attacker controlled response data.
- Transaction signing message binding was confirmed:
  - signing machine rejects external `msg` input unless empty;
  - per-input message is locally derived with `taproot_key_spend_signature_hash(..., TapSighashType::Default)`;
  - bytes passed to FROST are the full taproot key-spend sighash output for that input.
- Sighash type is fixed in code (`TapSighashType::Default`) and not counterparty-selectable.
- Participant-set integrity at construction and signing was confirmed:
  - `AlgorithmMachine::new` is instantiated from locally held `ThresholdKeys`;
  - signing set is validated in modular-frost (`included` checks + `validate_map`);
  - mismatched keys for a prevout fail early in `SignableTransaction::multisig` via script-pubkey equality check.
- Remaining `wallet/mod.rs` unwrap/expect sites are only:
  - line 74 (scale by ±1) invariant-only;
  - line 146 (serialize to `Vec<u8>`) invariant-only.
- `dkg-promote` isolation confirmed:
  - no imports/usages in `networks/bitcoin` or `processor/src/networks/bitcoin.rs`;
  - crate purpose: generator/ciphersuite promotion of `dkg::ThresholdKeys` using DLEq proofs (`GeneratorPromotion::promote`, `GeneratorPromotion::complete`, `GeneratorProof`).
  - remains a standalone pending audit crate.
- No new finding added in Chunk 4.

Chunk 4 verdict:
- RPC panic sites: classified and not attacker-reachable through remote response content.
- Transaction message binding and participant-set integrity: confirmed correct for this path.
- dkg-promote is isolated and should remain audited as its own crate.

**Chunk 5 (Final Checklist + Crate Verdict):**
- Final checklist status:
  - Replay protection: PASS. Signing message is per-input Taproot key-spend sighash (`taproot_key_spend_signature_hash` with `Prevouts::All` and `TapSighashType::Default`), binding to transaction/input context.
  - Nonce binding: PASS with caller contract. FROST binding factors commit to group key, message hash, and preprocess transcript; preprocess replay is explicitly forbidden by API contract.
  - Script enforcement: PASS. Taproot script/key derivation path is deterministic from group key + internal offsets; no external script-content injection in `tweak_keys`.
  - Fee manipulation: PASS within crate assumptions. Construction validates funds/fee/output constraints and signs the exact built transaction; malformed inputs are caller responsibility.
  - Threshold enforcement: PASS. modular-frost enforces `included >= t`, participant range/uniqueness, and signing/share map exactness before completion.
  - Constant-time handling: PASS for secret-scalar operations in signing path; vartime operations observed are on public commitment/share verification paths.
- Watch outcomes for bitcoin-serai scope:
  - W-01 CLOSED (informational/probabilistic).
  - W-02 CLOSED.
  - W-03 CLOSED.
  - W-05 remains OPEN but out-of-scope for this crate.
  - W-06 CLOSED.
- Source coverage confirmed for all production files:
  - `src/lib.rs`, `src/crypto.rs`, `src/rpc.rs`, `src/wallet/mod.rs`, `src/wallet/send.rs`.
- New findings in bitcoin-serai: none.

Final bitcoin-serai verdict:
- CLEAN (0 new findings). Crate formally closed.
- Deferred work: `dkg-promote` remains standalone pending crate and should be audited next.

---

## Key Observations

1. Three CRITICAL crates are complete and clean: `frost-schnorrkel`, `dkg-musig`, `modular-frost`.
2. W-01 is still open. Identity group key validity is not enforced inside `dkg::ThresholdKeys::offset()`.
3. Two issues are now confirmed: `dkg-dealer::key_gen()` fails open on `participants == 0`, and `ThresholdKeys::new()` can panic on malformed constant interpolation input.
4. `pedpop` encryption appears internally coherent, but its security depends on unique per-message ECDH keys because the ChaCha20 IV is fixed.
5. `pedpop` blame entry points are not hardened against arbitrary participant IDs and currently expose a public panic surface.
6. FINDING-08 is currently a direct `dkg` constructor issue, not a reachable bug through `dealer`, `pedpop`, `musig`, or `promote`.
7. W-01 remains cross-crate relevant because Bitcoin's tweak path preserves the local `dkg` invalid-key risk while Ethereum masks it.
8. This crate batch is complete. Remaining immediate follow-up is `bitcoin-serai` for final W-01 disposition, plus `dkg-promote` as a separate pending crate.
9. The pedpop gap-fill closed the remaining ambiguity: ciphertext tampering is caught at the PoP layer, while nonce-reuse risk is still purely RNG-dependent because the IV is fixed and ephemeral-key reuse is unchecked.
10. Jules artifact review did not produce any new confirmed finding. The substantive dispositions were: MuSig identity acceptance is caller-policy / non-guaranteed-usability rather than a proven exploit, `Schnorr::verify` before `sign_share` in `modular-frost` is the known documented precondition panic, `dkg-dealer` zero-participant behavior remains the already-recorded `Ok(empty)` fail-open (not a panic), and `multiexp()` empty-input panic claims are false positives because the public dispatcher returns identity on empty input.
11. `bitcoin-serai` Chunk 1 is complete: tweak/offest/x-only call graphs are mapped, and Ethereum-vs-Bitcoin mitigation divergence is reconfirmed in processor network glue.
12. `bitcoin-serai` Chunk 2 is complete: W-01 is narrowed to probabilistic-only reachability, per-output offset path does not introduce a second independent panic trigger, and W-02/W-03 are closed for this crate path.
13. `bitcoin-serai` Chunk 3 is complete: signing-path aggregation into HRAM is fully traced, aggregate-nonce identity is not explicitly checked in modular-frost, but no practical adversarial forcing route was confirmed and no new finding was logged.
14. `bitcoin-serai` Chunk 4 is complete: RPC unwraps are classified, transaction sighash binding is fixed and correct, participant-set integrity checks are confirmed, and `dkg-promote` is verified isolated from Bitcoin path.
15. `bitcoin-serai` Chunk 5 is complete: final checklist passes, all bitcoin-serai-scope watches are resolved/closed except out-of-scope W-05, and crate closes clean with zero new findings.
16. W-05 is fully resolved and closed: frost-schnorrkel context/message transcript inputs are length-delimited and domain-labeled, so concatenation-style collisions were not found.
17. `dkg-promote` Chunk 1 is complete: the crate is a small standalone same-group generator-promotion utility built on local `crypto/dleq`, and no production callers were found elsewhere in the repo.
18. `GeneratorPromotion::promote()` proves equality of discrete log between `(C1::generator(), original_verification_share_i)` and `(C2::generator(), promoted_share_i)` using the participant's actual `original_secret_share()` as the witness.
19. Promotion proofs are bound in the outer transcript only to `(original_group_key, participant)` and in the inner DLEq transcript to ordered `(generator, reconstructed nonce, point)` tuples for both generators. They are not explicitly bound to threshold `t`, participant set contents, or any higher-level session identifier.
20. `GeneratorPromotion::complete()` verifies every peer proof against the expected old-generator verification share for that participant before constructing the new `ThresholdKeys`; invalid proofs abort cleanly with `PromotionError::InvalidProof` and no partial state is returned.
21. `GeneratorProof::{read,write,serialize}` rely on `Ciphersuite::read_G` and `DLEqProof::read`; malformed points/scalars are returned as `io::Error`, and no attacker-controlled panic was found in deserialization.
22. No immediate Critical candidates were identified in Chunk 1. The main residual concern for Chunk 2+ is proof portability across sessions sharing the same original group key and participant index, since the transcript omits any extra domain such as threshold, participant roster, or explicit application context.
23. `dkg-promote` Chunk 2 resolved the replay question: proof portability across sessions is blocked unless the target session has the same original verification share for that participant, not merely the same group key and participant index. `complete()` verifies against `self.base.original_verification_share(i)`, so a reused proof from a different share set fails the DLEq equation.
24. `original_group_key` is just the raw untweaked group public key stored in `ThresholdCore.group_key`, computed in `ThresholdKeys::new()` by interpolating the first `t` verification shares. It is not bound to any session-unique transcript or nonce material.
25. Two independent sessions can share the same `original_group_key` either by negligible random collision or by intentionally reusing the same underlying secret/public polynomial constant term. This is not protocol-prevented inside `dkg-promote`, `dkg`, or `dkg-dealer`.
26. A new Low-severity public panic surface exists in `GeneratorPromotion::complete()`: the pre-check only enforces `proofs.len() == n - 1` and `participant <= n`, but it does not enforce exact peer-set membership before `proofs.get(&i).unwrap()`. A caller can supply duplicate/self-substituted participant coverage that passes validation yet omits some expected peer, causing panic.
27. The `PromotionError::IncorrectAmountOfParticipants` construction in `complete()` reports `t: params.n()` instead of `params.t()`. This is a reporting bug only; `complete()` intentionally enforces `n-of-n` promotion inputs because it needs every participant's promoted verification share.
28. `crypto/dleq` Chunk 3 is complete: the non-experimental crate surface is limited to same-group Schnorr-style `DLEqProof` and `MultiDLEqProof`, with `cross_group` gated behind the out-of-scope `experimental` feature.
29. The Fiat-Shamir transcript for `DLEqProof` is complete for the intended statement: after `domain_separate("dleq")`, it appends ordered `(generator, nonce, point)` triples for every generator, where `nonce = rG` in proving and `nonce = sG - cA` in verifying, and only then derives the challenge.
30. Nonce reuse is not prevented beyond the RNG contract: `prove()` samples `r` fresh with `G::Scalar::random(rng)` and stores it in `Zeroizing`, so accidental reuse requires a faulty/repeated RNG stream rather than an internal state bug.
31. `DLEqProof::read()` validates both `c` and `s` with `PrimeField::from_repr`; malformed scalar encodings return `io::Error` and were not found to panic.
32. `DLEqProof::verify()` does not itself reject identity public points or identity generators. This is acceptable for standard prime-order group semantics because the statement may legitimately have witness `x = 0`, but it means identity-exclusion is caller policy, not an invariant enforced by `crypto/dleq`.
33. No Critical/High issue was confirmed in `crypto/dleq`. The implemented verification equation is the standard joint-challenge Schnorr DLEq check, and no witness-forgery path was identified for non-degenerate caller-supplied generators/points.
34. `crypto/schnorr` Chunk 1 is complete: the crate is a challenge-agnostic Schnorr primitive (`s = r + cx`, verify via `R + cA - sG == 0`) over `PrimeGroup` ciphersuites; it does not generate nonces internally and relies entirely on caller-supplied `nonce` and `challenge`.
35. Identity public keys are accepted by `SchnorrSignature::verify` (no identity check in signature verify path, and `Ciphersuite::read_G` enforces canonicality but not non-identity). This causes equation collapse under `A = identity`, enabling trivial forgeries of the form `R = sG`.
36. High-priority downstream candidate identified: `coordinator` `Transaction::SignCompleted` verifies a Schnorr signature against attacker-controlled `first_signer` and treats the transaction as `Unsigned`; if `first_signer = identity`, signatures can be forged without key knowledge and still pass `verify()`, potentially spoofing completion events.
37. Session 22 Chunk 2 confirms this path is reachable at admission: `SignCompleted` is unsigned (`kind() -> Unsigned`), decoded `first_signer` is untrusted tx content, and `verify()` performs no identity-key rejection before `signature.verify(*first_signer, sign_completed_challenge)`.
38. Downstream impact is constrained by processor-side claim confirmation: forged `SignCompleted` still forwards `CoordinatorMessage::Completed`, but processor `claimed_eventuality_completion` validates claim against network completion before stopping signing; no direct threshold-signature bypass or funds-move primitive was confirmed from this bug alone.
39. Session 22 Chunk 3 critical gate result: processor validation is independent of coordinator forwarding. `CoordinatorMessage::Completed` enters `claimed_eventuality_completion`, which calls network-specific `confirm_completion(eventuality, claim)` and only transitions completion state on `Ok(Some(completion))`.
40. `confirm_completion` uses chain/network checks per backend: Bitcoin fetches transaction by eventuality txid (`rpc.get_transaction(eventuality.0)`), Monero fetches claimed txid then checks `eventuality.matches`, Ethereum validates the claim as a `SignedRouterCommand::new(&eventuality_pubkey, command, claim.signature)`. This denies blind trust of coordinator forwarding.
41. Message-queue `verify()` call sites at `message-queue/src/main.rs:69` and `:135` use service keys from in-process `KEYS` registry loaded from environment (`read_key` + `register_service`), not peer-supplied public keys; no second independent attacker-injectable F-15 instance was confirmed there.
42. Forged `SignCompleted` can be weaponized for liveness pressure: unsigned txs have no per-signer mempool cap, are ordered before signed txs, and block-size trimming pops from the tail. This preferentially evicts signed txs under unsigned flood (`Block::new` ordering + `while serialize().len() > BLOCK_SIZE_LIMIT { pop() }`).
43. This flood does not directly bypass processor completion validation, but it can delay/pressure signing protocol traffic carried as signed txs (while provided txs retain placement precedence). Observed effect is potential starvation/degradation, not proven full chain halt.
44. No code path was found to "upgrade" forged identity `first_signer` into a real signer before processor checks: `first_signer` is not forwarded in `CoordinatorMessage::Completed` (only `plan` + `tx_hash` claim), and second-layer confirmation ignores this field.
45. Reforging to pass second layer without a real completion remains blocked by `confirm_completion`; however, if an attacker can claim a genuine completion tx hash, the message may pass as a valid report (consistent with unsigned reporter design, not a unique identity-forgery escalation).
46. Additional operational caveat: message-queue client `next()` currently has TODO sender-signature verification and only sanity-checks `msg.from` enum. This is a separate trust-surface concern (transport/server trust), not a direct expansion of F-15.
47. Starvation deep-dive: unsigned tx admission for `TransactionKind::Unsigned` only performs hash dedup and `app_tx.verify()`; it does not enforce signer membership, nonce progression, per-sender quota, fee, or stake checks in mempool path.
48. P2P ingress remains non-validator-authenticated in code comments and network setup (`listen_on 0.0.0.0`, TODO for custom validator-only auth). Tributary gossip messages are accepted and fed into `handle_message -> add_transaction(false, ...)` if deserialization and tx checks pass.
49. Global unsigned mempool cap is absent. Dedup is hash-only (`unsigned_in_chain(hash) || self.txs.contains_key(&hash)`), with no semantic keying on `(plan, first_signer)` for `SignCompleted`.
50. Forged `SignCompleted` size in block context is `131 + tx_hash_len` bytes (outer application envelope + inner variant + fields), so roughly:
    - minimum (`tx_hash_len = 0`): 131 bytes
    - typical 32-byte claim: 163 bytes
    - maximum (`tx_hash_len = 255`): 386 bytes
51. With `BLOCK_SIZE_LIMIT = 3,001,000`, approximate per-block forged capacity is:
    - ~22,908 tx/block at 131 bytes
    - ~18,411 tx/block at 163 bytes
    - ~7,774 tx/block at 386 bytes
52. Block construction order (`Provided` then unsigned then signed) plus oversize trimming via tail `pop()` means signed txs are preferentially discarded when unsigned flood saturates block space; a single saturated block can starve signed inclusion for that block.
53. Signing-liveness effect: `Sign` / `SubstrateSign` protocol traffic is signed tx based, so sustained unsigned flood can delay preprocess/share inclusion and degrade or stall attempt progress. Reattempt scheduling is block-time based (`~5 min` base = 50 blocks at 6s target) and only triggered once preprocess participation threshold logic is reached; starvation before participation thresholds can delay reattempt triggers as well.
54. No irreversible custody loss path was confirmed from this starvation vector alone. Impact is strong liveness degradation / operational DoS while flood is sustained; recoverability is expected once flood pressure stops and signed backlog is mined.

---

## schnorr-signatures Chunk 4 Checkpoint (aggregate.rs)

- Scope restriction held: only `crypto/schnorr/src/aggregate.rs` and production callers in Tributary Tendermint were reviewed.
- CS-3.8.4 re-verification: fix holds. Weights are now transcripted from challenges only (not key+challenge double-accumulation), matching the intended half-aggregation construction.
- CS-3.8.5 re-verification: fix holds. Weight derivation now consumes `ceil((NUM_BITS + 128)/8)` bytes via transcript challenge chaining (`aggregation_weight` / `aggregation_weight_continued`), reducing modulo group order with residual bias bounded below the 128-bit target.
- Aggregate equation check: `sum(z_i * R_i) + sum(z_i * c_i * A_i) - s*G == 0` is implemented correctly for ePrint 2021/350 half-aggregation, with matching prover/verifier transcript order.
- Identity handling: aggregate path itself does not reject identity keys; if an identity validator key is admitted upstream, the same `A = identity` collapse family as FINDING-15 applies to that signer term.
- Caller hardening note (outside `aggregate.rs`): Tendermint `verify_aggregate` currently unwraps signer key decoding and does not pre-check signer membership before weight lookup. This is tracked as follow-up review surface, not yet promoted to a new confirmed finding in this chunk.

---

## schnorr-signatures Chunk 5 Checkpoint (Tendermint caller)

- Scope expansion covered the live Tendermint caller path in `coordinator/tributary/src/tendermint/mod.rs:51`, `:181`, `:201`, `coordinator/tributary/tendermint/src/ext.rs:256`, `coordinator/src/p2p.rs:198`, `:971`, `coordinator/src/tributary/spec.rs:56`, and `crypto/ciphersuite/src/lib.rs:91`.
- H-17b cleared: Tendermint per-signer challenge binds `R_i`. Transcript order in `challenge()` is `genesis`, `key`, `nonce`, `message`, where `nonce = R_i.to_bytes()` and the commit-path `message = end_time || block_id`.
- Minor note on challenge domain: height and round are not directly hashed, yet `block_id` is the unique committed block hash on this path; no standalone exploit was confirmed from the absence of explicit height/round bytes.
- FINDING-17 confirmed as High with Critical escalation pending: `verify_aggregate` is fed signer IDs from received `commit.validators` raw `[u8; 32]` bytes, computes challenges over those bytes and aggregate nonces, and then decodes the same bytes into curve points with `read_G` instead of performing a trusted validator-set point lookup by index.
- Identity admission remains live through every checked layer: `read_G` / `from_bytes` accept canonical identity points, `TributarySpec::new` accepts any point returned by `read_G`, and `Validators::new` only rejects zero weight, not identity keys.
- Consensus-path severity gate: the trivial all-identity aggregate forgery is not currently confirmed as Critical because `verify_commit` separately requires `sum(weights.weight(v)) >= threshold`, so severity now hinges on what `Validators::weight()` does for unknown IDs and whether identity can enter the trusted weight map.
- FINDING-18 confirmed as reachable panic / remote DoS: `verify_aggregate` executes `read_G(signer).unwrap()` on attacker-supplied `commit.validators` bytes. `Commit::decode` performs no point validation, `signers.len() == aggregate.Rs().len()` is attacker-satisfiable, and one non-canonical signer byte string can panic a node verifying a synced commit from `HeartbeatBatch -> sync_block -> verify_commit -> verify_aggregate`.
- Lower-priority honest-path note: `aggregate()` / `complete().unwrap()` also retain panic behavior on the local commit-builder path, but the remotely reachable `verify_aggregate` unwrap is the priority issue.
- Open Critical-escalation questions for Chunk 6:
  - What does `Validators::weight()` return for unknown validator IDs during `verify_commit`?
  - Can the substrate validator-sets pallet register the identity point as a validator key?
  - Does any caller trust a commit after `verify_aggregate` without also enforcing the threshold check in `verify_commit`?

---

## schnorr-signatures Chunk 6 Checkpoint (F-17 escalation + F-18 containment)

- `Validators::weight()` resolution: the exact body is `self.weights[&validator]`, so unknown validator IDs do not return `0`; they panic through the `HashMap` index path in `coordinator/tributary/src/tendermint/mod.rs:238`.
- `Weights::threshold()` resolution: threshold itself is the default `((self.total_weight() * 2) / 3) + 1` in `coordinator/tributary/tendermint/src/ext.rs:163`.
- FINDING-17 escalation outcome: capped at High, not Critical.
  - No production caller was found that trusts a commit after signature-only `verify_aggregate`; the only production use of `verify_aggregate` is inside `verify_commit` in `coordinator/tributary/tendermint/src/ext.rs:256`, and all production commit-trust call sites invoke full `verify_commit`.
  - The on-chain `serai.participants()` path is `SeraiValidatorSets::participants()` storage read -> `validator-sets` pallet `Participants` storage -> `new_set()` population from `SortedAllocationsIter` over validator `Public` account IDs. No explicit identity rejection exists in `new_set()`, `BuildGenesisConfig`, `allocate`, `set_allocation`, or `Public` deserialization in repo code.
  - Despite that absence, attacker-achievable identity registration with nonzero weight was not proven from audited code alone: active-set membership is populated from genesis participants and signed-origin allocator accounts, not from the external-network `set_keys` `KeyPair`, and no code path here proves an attacker can originate a signed account with the identity public key.
- FINDING-18 containment outcome: task-isolated, not process-abort under current repo configuration.
  - Reachability remains confirmed: `HeartbeatBatch` carries raw `commit: Vec<u8>` into `sync_block`, `Commit::decode` performs no point validation on `commit.validators`, and `verify_aggregate` still does `read_G(signer).unwrap()`.
  - Containment result: `sync_block` runs inside the per-tributary P2P handler spawned with `tokio::spawn` in `coordinator/src/p2p.rs`, no `catch_unwind` is present, and no `panic = "abort"` / Tokio unhandled-panic override was found in repo config. Under that call structure, the panic kills the spawned per-tributary handler task rather than the whole node process.
- FINDING-19 confirmed as a second panic / DoS on the threshold-defense path.
  - Because identity keys are accepted by `verify_aggregate`, an attacker can craft a forged identity-key aggregate that passes signature verification and then panics in `verify_commit` when the threshold check evaluates `weights.weight(v)` for unknown IDs.
  - This means the threshold defense that caps FINDING-17 at High is not fail-closed for unknown IDs; it degrades into a task-killing DoS via `self.weights[&validator]`.
- Production caller inventory:
  - `verify_aggregate`: only production caller is `verify_commit` in `coordinator/tributary/tendermint/src/ext.rs:265`.
  - `verify_commit`: production uses are `coordinator/tributary/src/lib.rs:290` (`sync_block_internal` on p2p-synced commits), `coordinator/tributary/tendermint/src/lib.rs:960` (machine handling externally synced blocks), and `coordinator/tributary/src/tendermint/mod.rs:377` / `coordinator/tributary/tendermint/src/lib.rs:628` as internal assert/debug-assert sanity checks.

---

## schnorr-signatures Chunk 7 Checkpoint (closure checklist)

- Task-context result:
  - `sync_block_internal` in `coordinator/tributary/src/lib.rs:290` runs on the spawned per-tributary P2P handler task created under `coordinator/src/p2p.rs:901`; this is the confirmed public-input path for `FINDING-18` and `FINDING-19`.
  - `TendermintMachine::run` in `coordinator/tributary/tendermint/src/lib.rs:937` is the core consensus driver task spawned once per tributary from `coordinator/tributary/src/lib.rs:201`.
  - `add_block` in `coordinator/tributary/src/tendermint/mod.rs:362` executes on that core `machine.run()` task, reached from `upon_successful_precommits` / synced-block handling in `coordinator/tributary/tendermint/src/lib.rs:637` and `:969`.
  - No public-input path was confirmed from p2p ingress into the core-task `add_block` assert or the core-task synced-block `verify_commit`, because external synced commits are filtered through `sync_block_internal` first; the confirmed attacker-triggerable panics remain isolated to the spawned sync task.
- Source-file completeness:
  - `crypto/schnorr/src/lib.rs` fully covered: `read`, `write`, `serialize`, `sign`, `batch_statements`, `verify`, `batch_verify`.
  - `crypto/schnorr/src/aggregate.rs` fully covered: `SchnorrAggregate::{read, write, serialize, Rs, verify}` and `SchnorrAggregator::{new, aggregate, complete}`.
  - Remaining files under `crypto/schnorr/src` are tests only: `tests/mod.rs` and `tests/rfc8032.rs`.
  - No unaudited public production function remains in the crate.
- Batch verification loose end:
  - Cleared. The public batch path in `lib.rs` delegates to `BatchVerifier::queue(rng, ...)`; there is no deterministic scalar reuse or cross-signature weight mixing introduced by this crate, and the prior batch-verify mix-and-match hypothesis remains cleared under fresh verifier RNG.
- Confirmed finding set for `crypto/schnorr`:
  - `FINDING-15`, `FINDING-17`, `FINDING-18`, and `FINDING-19`.
- Post-audit chain seed:
  - Confirmed 2-step chain seed `FINDING-17 -> FINDING-19`: identity-slot aggregate forgery can pass `verify_aggregate`, which then unlocks the threshold-path panic at `weights[&validator]`.
  - This chain is currently capped at Low / DoS because the attacker-reachable panic point is isolated to the spawned sync task, not the process or the core consensus task.
- Crate verdict:
  - `crypto/schnorr` closes cleanly after Chunk 7. No further unresolved public API or caller-path questions remain in-scope for this crate.

---

## Prior Audit Intelligence

| CS Finding | Crate | Issue | Fix Status |
|---|---|---|---|
| CS-3.3.3 | dkg | Polynomial eval at zero | Fixed |
| CS-3.3.4 | dkg | Incomplete session domain separation | Fixed |
| CS-3.6.7 | frost | IetfTranscript public | Fixed (`pub(crate)`) |
| CS-3.8.4 | schnorr | Incorrect half-aggregation | Fixed (re-verified Session 25) |
| CS-3.8.5 | schnorr | Non-uniform hash sampling | Fixed (re-verified Session 25) |
| CS-3.9.1 | transcript | Malfunctioning digest bound | Fixed |
| CS-3.7.4 | multiexp | Variable-time ops | Fixed |
| CS-3.4.4 | dleq | Prover input consistency | By design |
| CS-3.9.3 | transcript | Merlin domain-separation conflict | Fixed |

---

## Next Actions

| Priority | Crate | Chunk | Action | Why |
|---|---|---|---|---|
| 1 | crypto/ciphersuite | Gap-fill | review `read_G` / canonical-point acceptance and caller assumptions after its repeated role in identity-admission paths | next queue item with direct relevance to confirmed Schnorr / Tendermint findings |
| 2 | remaining | Future | Scheduling | continue bounty queue |

---

## Session History

- **Session 1 (2026-04-23):** Phase 1 complete. All 9 Cypher Stack fixes verified.
- **Session 2 (2026-04-23/24):** frost-schnorrkel 5/5 complete. 5 informational findings.
- **Session 3 (2026-04-24):** dkg-musig 5/5 complete. FINDING-06 added.
- **Session 4 (2026-04-24):** modular-frost 5/5 complete. `IetfTranscript::rng_seed` unreachable confirmed. RFC 9591 compliant. No new findings.
- **Session 5 (2026-04-26):** dkg + sub-crates Chunk 1 complete. Orientation restored; W-01 still open in `dkg::ThresholdKeys::offset()`. `pedpop` state machine and encryption architecture mapped for deeper review.
- **Session 6 (2026-04-26):** dkg + sub-crates Chunk 2 complete. Confirmed FINDING-07 (`dkg-dealer::key_gen` returns `Ok(empty)` for `participants == 0`) and FINDING-08 (`ThresholdKeys::new` can panic on short `Interpolation::Constant` input).
- **Session 7 (2026-04-26):** dkg + sub-crates Chunk 3 complete. Encryption review found no direct authenticity break, but confirmed FINDING-09: `pedpop` blame APIs can panic on out-of-set sender/recipient IDs.
- **Session 8 (2026-04-26):** dkg + sub-crates Chunk 4 complete. Cross-crate review confirmed Ethereum mitigates W-01, while Bitcoin still propagates the negligible-probability infinity case into downstream `x_only()` panic sites.
- **Session 9 (2026-04-26):** dkg + sub-crates Chunk 5 complete. Batch closed with three confirmed low/informational issues and no confirmed cryptographic break.
- **Session 10 (2026-04-28):** Targeted pedpop encryption gap-fill complete. Confirmed Q2 is clean for plaintext-retention on PoP failure, Q3 PoP does bind ciphertext bytes, and the remaining fixed-IV risk is repeated ephemeral-scalar reuse without any built-in uniqueness check.
- **Session 11 (2026-04-28):** Jules artifact review complete for `dkg-musig`, `modular-frost`, `dkg-dealer`, `multiexp`, and auxiliary boundary tests. No new findings accepted. Confirmed duplicates/false positives only: FINDING-06 duplication, documented `verify`/`sign_share` precondition panic, FINDING-07 already-known zero-participant fail-open, and false-positive `multiexp()` empty panic suspicion.
- **Session 12 (2026-05-15):** bitcoin-serai Chunk 1 (orientation) complete. Located `tweak_keys()` computation and all local `ThresholdKeys::offset()` / `x()` / `x_only()` sites, mapped FROST integration points, reconfirmed Ethereum mitigation loop in processor glue versus no equivalent post-tweak validity loop on Bitcoin path, and kept W-01/W-02/W-03 open for Chunk 2 resolution.
- **Session 13 (2026-05-16):** bitcoin-serai Chunk 2 (deep function audit) complete. Resolved `tweak_keys()` and per-output-offset reachability details, classified W-01 as probabilistic-only in this path, denied a second independent trigger through scanned offsets, and closed W-02/W-03 for bitcoin-serai plus processor Bitcoin path.
- **Session 14 (2026-05-16):** bitcoin-serai Chunk 3 (signing path + adversarial nonce analysis) complete. Traced `R` and `A` into HRAM, confirmed no explicit aggregate-`R` identity check before algorithm invocation, classified send.rs panic sites, and did not confirm a practical malicious-co-signer route forcing `R == identity`.
- **Session 15 (2026-05-16):** bitcoin-serai Chunk 4 (RPC + signing integrity + final surfaces) complete. Classified rpc unwraps, verified fixed Taproot sighash message binding and participant-set validation, confirmed only invariant unwraps remain in wallet module, and validated `dkg-promote` isolation plus standalone-scope status.
- **Session 16 (2026-05-16):** bitcoin-serai Chunk 5 (final checklist + crate verdict) complete. Replay/message binding, threshold enforcement, script determinism, and constant-time handling were all confirmed for crate scope. No new findings were logged; bitcoin-serai closed CLEAN and next crate is dkg-promote.
- **Session 17 (2026-05-16):** W-05 resolution completed for frost-schnorrkel and callers. Verified Merlin transcript framing includes per-field length metadata and labels; verified frost-schnorrkel prehash includes explicit 32-bit context length prefix before context/message split; surveyed non-test callers and confirmed fixed Schnorrkel context (`b\"substrate\"`) with no transcript-collision finding.
- **Session 18 (2026-05-17):** `dkg-promote` Chunk 1 complete. Mapped full crate surface and dependencies, traced `GeneratorPromotion::{promote, complete}` and `GeneratorProof` serialization, identified the DLEq primitive as the local same-group Schnorr-style proof from `crypto/dleq`, confirmed proof binding to `(original_group_key, participant)` plus ordered generator/nonce/point tuples, and confirmed no production callers or network integration paths elsewhere in the repo. No Critical/High finding confirmed.
- **Session 19 (2026-05-17):** `dkg-promote` Chunk 2 complete. Traced `original_group_key` and `original_verification_share` back to `ThresholdKeys::new()`, denied the cross-session replay concern unless the participant verification share also matches exactly, confirmed no higher-level session context is layered around promotion in the repo, and identified a new Low-severity panic surface in `GeneratorPromotion::complete()` due to incomplete peer-set validation before `proofs.get(&i).unwrap()`.
- **Session 20 (2026-05-17):** `dkg-promote` Chunk 3 and `crypto/dleq` audit complete. Confirmed the DLEq Fiat-Shamir transcript commits all intended `(generator, nonce, point)` tuples before challenge derivation, nonce freshness depends only on caller RNG and is zeroized, scalar deserialization is canonical and error-returning, and the verification equation is standard Schnorr-style same-group DLEq. No Critical/High issue was found in `crypto/dleq`; `dkg-promote` is now closed with FINDING-14 confirmed as Low.
- **Session 21 (2026-05-19):** `crypto/schnorr` Chunk 1 complete. Mapped file/API/dependency/feature surfaces; confirmed the signature primitive is challenge-agnostic and nonce-externalized; traced serialization and verification equation details; and mapped batch verification paths (`batch_verify` via `multiexp::BatchVerifier` and half-aggregation via `aggregate`). Cross-crate usage review identified a high-priority candidate in coordinator where identity-key verification collapse may permit forged `SignCompleted` signatures on an unsigned transaction path.
- **Session 22 (2026-05-26):** `crypto/schnorr` Chunk 2 complete. Applied findings-register corrections (F-10/11/12 added, F-13 removed, F-04 and F-07 severity upgrades, F-15 assigned), confirmed `SignCompleted` admission path is forge-reachable via identity `first_signer`, and traced downstream processor handling to show completion claims are independently validated before signer state completion.
- **Session 22 (2026-05-26):** `crypto/schnorr` Chunk 3 complete. Traced processor handling of `CoordinatorMessage::Completed` and confirmed independent network-backed `confirm_completion` checks gate completion-state transition, so Critical escalation is denied and FINDING-15 remains High. Reviewed message-queue Schnorr verify call sites and classified their public keys as trusted registry-derived (env-configured service keys), not peer-injectable.
- **Session 23 (2026-05-26):** Extended F-15 impact exploration complete. Confirmed no first-layer-to-second-layer signer-ID upgrade path and no processor bypass from identity forgery alone, but identified realistic unsigned-flood starvation pressure against signed tx inclusion due to mempool ordering and block tail-trimming policy. Classified this as liveness/DoS pressure surface pending stress-test quantification.
- **Session 24 (2026-05-26):** F-15 starvation vector deep-dive complete. Quantified forged `SignCompleted` block fill capacity, confirmed unsigned admission lacks per-sender/global cap and uses hash-only dedup, traced non-validator-authenticated gossip ingress assumptions, and confirmed sustained flood can starve signed signing-protocol tx inclusion (upper-end High liveness degradation) without confirmed irreversible fund impact.
- **Session 25 (2026-05-27):** `crypto/schnorr` Chunk 4 (`aggregate.rs`) complete. Re-verified CS-3.8.4 and CS-3.8.5 fixes as holding in current code, confirmed half-aggregation equation correctness against ePrint 2021/350, and traced production use to Tributary Tendermint commit verification/aggregation paths. No new confirmed finding was added in this chunk.
- **Session 25 (2026-05-27):** `crypto/schnorr` Chunk 5 (Tendermint caller) complete. Cleared H-17b by confirming Tendermint challenge binding over `genesis`, validator key bytes, nonce `R_i`, and `end_time || block_id`; confirmed FINDING-17 as High with Critical escalation still gated by threshold enforcement and validator-key admission; confirmed FINDING-18 as a reachable panic/DoS on the p2p block-sync commit-verification path. Crate remains open for Chunk 6 escalation.
- **Session 26 (2026-05-29):** `crypto/schnorr` Chunk 6 complete. Confirmed `Validators::weight()` panics on unknown IDs, adding FINDING-19 as a second task-isolated p2p sync DoS; capped FINDING-17 at High because every production commit-trust path enforces full `verify_commit` threshold checks and attacker-achievable on-chain identity registration was not proven; confirmed FINDING-18 as task-isolated rather than process-aborting under current Tokio/repo panic configuration. `crypto/schnorr` is now closed.
- **Session 27 (2026-05-29):** `crypto/schnorr` Chunk 7 closure checklist complete. Confirmed the attacker-reachable `FINDING-18` / `FINDING-19` panic path is isolated to the spawned per-tributary sync task, while the `add_block` assert lives on the core consensus task without a confirmed public-input route; completed the final source-file inventory and confirmed no unaudited public production function remains; recorded the `FINDING-17 -> FINDING-19` chain seed for post-audit chaining. Crate closure stands and next target is `crypto/ciphersuite`.

---

## Session 28 Checkpoint (crypto/ciphersuite Chunk 1 - orientation + identity sweep)

### Crate Status Snapshot (Session 28)

| Crate Path | Crate Name | Status | Note |
|---|---|---|---|
| crypto/schnorr | schnorr-signatures | CLOSED (Chunks 1-7) | closure stands; only carry-forward item is F-18/F-19 Low-vs-Medium severity refinement |
| crypto/ciphersuite | ciphersuite | IN PROGRESS (Chunk 1 complete) | orientation + identity-handling sweep complete |

### File map

- `crypto/ciphersuite/src/lib.rs`
- `crypto/ciphersuite/src/lib.md`

### Public API surface

- `pub trait Ciphersuite`
  - associated types: `F`, `G`, `H`
  - associated const: `ID`
  - required methods: `generator()`, `hash_to_F(dst, msg)`
  - provided methods: `random_nonzero_F(rng)`, `read_F(reader)`, `read_G(reader)` behind `alloc/std`
- Crate-local public re-export: `pub use group`
- Concrete repo ciphersuite impls traced for this chunk:
  - `crypto/dalek-ff-group/src/ciphersuite.rs`: `Ristretto`, `Ed25519`
  - `crypto/ciphersuite/kp256/src/lib.rs`: `Secp256k1`, `P256`
  - `crypto/ed448/src/ciphersuite.rs`: `Ed448`

### Identity / canonicality checkpoint

- Root defect confirmed at crate boundary: [`crypto/ciphersuite/src/lib.rs:91`](C:/Users/MADAKI/Documents/serai/crypto/ciphersuite/src/lib.rs:91) `read_G` enforces successful decode plus round-trip canonicality (`point.to_bytes() == encoding`) but does **not** reject `point.is_identity()`.
- Concrete acceptance behavior traced:
  - `Ristretto` / `Ed25519`: [`crypto/dalek-ff-group/src/lib.rs:429`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:429) accepts any decompressible point with `choice(decompressed.is_some()) & choice($torsion_free(point))`; no identity rejection is present in `from_bytes`.
  - `Ed25519`: torsion rejected by `point.is_torsion_free()` at [`crypto/dalek-ff-group/src/lib.rs:473`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:473), but identity is torsion-free and therefore accepted.
  - `Ristretto`: torsion predicate is hardcoded `|_| true` at [`crypto/dalek-ff-group/src/lib.rs:489`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:489); identity is accepted.
  - `Secp256k1`: SEC1 identity is explicitly mapped from all-zero fixed-width `Repr` at [`k256 affine.rs:220-225`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/k256-0.13.4/src/arithmetic/affine.rs:220), then returned from `from_encoded_point` on `Coordinates::Identity` at [`k256 affine.rs:247-250`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/k256-0.13.4/src/arithmetic/affine.rs:247).
  - `P256`: same fixed-width SEC1 identity admission at [`primeorder affine.rs:255-265`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/primeorder-0.13.6/src/affine.rs:255) and `Coordinates::Identity` acceptance at [`primeorder affine.rs:177-180`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/primeorder-0.13.6/src/affine.rs:177).
  - `Ed448`: [`crypto/ed448/src/point.rs:303`](C:/Users/MADAKI/Documents/serai/crypto/ed448/src/point.rs:303) accepts points whose `y` is canonical, recoverable on-curve, not negative zero, and torsion-free; no explicit identity rejection exists, so canonical identity is accepted.
- Canonicality behavior traced:
  - `read_F` rejects non-canonical scalars through `PrimeField::from_repr(...) -> None` at [`crypto/ciphersuite/src/lib.rs:74-82`](C:/Users/MADAKI/Documents/serai/crypto/ciphersuite/src/lib.rs:74).
  - `read_G` rejects malformed / non-canonical point encodings by requiring both successful `from_bytes` and byte-for-byte round-trip equality at [`crypto/ciphersuite/src/lib.rs:91-100`](C:/Users/MADAKI/Documents/serai/crypto/ciphersuite/src/lib.rs:91).
  - `Ed25519` / `Ristretto`: malformed decompressions rejected; non-canonical encodings are caught by the trait-level round-trip check.
  - `Secp256k1` / `P256`: invalid field coordinates and off-curve points are rejected during SEC1 decode; identity remains valid because these are cofactor-1 prime-order curves, not because of a separate subgroup screen.
  - `Ed448`: `FieldElement::from_repr` requires `res < MODULUS` and `bytes[56] == 0` after clearing the sign bit; `from_bytes` additionally rejects non-curve, torsion, and negative-zero encodings.

### hash_to_F checkpoint

- `Ristretto` / `Ed25519`: [`crypto/dalek-ff-group/src/ciphersuite.rs:31-33`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/ciphersuite.rs:31) uses `Scalar::from_hash(Sha512::new_with_prefix(&[dst, data].concat()))`; uniformity comes from 64-byte wide reduction in [`crypto/dalek-ff-group/src/lib.rs:243-249`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:243), but DST separation is only naive `dst || data`.
- `Secp256k1` / `P256`: [`crypto/ciphersuite/kp256/src/lib.rs:35-91`](C:/Users/MADAKI/Documents/serai/crypto/ciphersuite/kp256/src/lib.rs:35) implements hash-to-field style expansion with `ExpandMsgXmd::<Sha256>::expand_message(&[msg], &[dst], 48)` and reduces a 384-bit value mod the scalar order; comments explicitly target `k = 128`, so residual bias is bounded to about `2^-128`.
- `Ed448`: [`crypto/ed448/src/ciphersuite.rs:71-73`](C:/Users/MADAKI/Documents/serai/crypto/ed448/src/ciphersuite.rs:71) digests `dst || data` with fixed-output `Shake256_114` and reduces 114 bytes in [`crypto/ed448/src/scalar.rs:55-63`](C:/Users/MADAKI/Documents/serai/crypto/ed448/src/scalar.rs:55); entropy is ample, but DST separation is again naive concatenation.

### Panic-surface checkpoint

- No new public attacker-byte panic was confirmed in `crypto/ciphersuite/src/lib.rs`; `read_F` and `read_G` are error-returning.
- Reviewed impl unwraps are invariant-only under current code:
  - [`crypto/ciphersuite/kp256/src/lib.rs:70`](C:/Users/MADAKI/Documents/serai/crypto/ciphersuite/kp256/src/lib.rs:70) `checked_add(...).unwrap()` after widening a 32-byte modulus into `U384`
  - [`crypto/ciphersuite/kp256/src/lib.rs:75-76`](C:/Users/MADAKI/Documents/serai/crypto/ciphersuite/kp256/src/lib.rs:75) `expand_message(...).unwrap()` with fixed 48-byte request
  - [`crypto/ciphersuite/kp256/src/lib.rs:85`](C:/Users/MADAKI/Documents/serai/crypto/ciphersuite/kp256/src/lib.rs:85) `Scalar::from_repr(array).unwrap()` after modular reduction
  - [`crypto/ed448/src/ciphersuite.rs:72`](C:/Users/MADAKI/Documents/serai/crypto/ed448/src/ciphersuite.rs:72) `try_into().unwrap()` on fixed 114-byte digest output

### Cross-crate blast radius snapshot

- `crypto/schnorr`: directly inherits `C::read_G` / `C::read_F` for signature and aggregate parsing at [`crypto/schnorr/src/lib.rs:51-52`](C:/Users/MADAKI/Documents/serai/crypto/schnorr/src/lib.rs:51) and [`crypto/schnorr/src/aggregate.rs:84-87`](C:/Users/MADAKI/Documents/serai/crypto/schnorr/src/aggregate.rs:84); this is the direct root of F-15 and F-17.
- `coordinator/tributary` Tendermint: directly decodes commit validator IDs with `Ristretto::read_G(...).unwrap()` at [`coordinator/tributary/src/tendermint/mod.rs:225`](C:/Users/MADAKI/Documents/serai/coordinator/tributary/src/tendermint/mod.rs:225), inheriting both identity admission and malformed-point panic behavior.
- `coordinator` transaction parsing: `SignCompleted` deserializes `first_signer` with `Ristretto::read_G` at [`coordinator/src/tributary/transaction.rs:418`](C:/Users/MADAKI/Documents/serai/coordinator/src/tributary/transaction.rs:418), then uses `Ristretto::hash_to_F` for the challenge at [`coordinator/src/tributary/transaction.rs:711`](C:/Users/MADAKI/Documents/serai/coordinator/src/tributary/transaction.rs:711).
- `crypto/frost`: masks the identity-admission defect for its own point readers by wrapping ciphersuite decoding with an explicit `is_identity()` rejection at [`crypto/frost/src/curve/mod.rs:123-130`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/curve/mod.rs:123).
- `crypto/dkg`, `crypto/dkg/pedpop`, `crypto/dkg/promote`, `networks/bitcoin`, `networks/ethereum`, `message-queue`, and processor/network glue all rely on `Ciphersuite` canonical scalar/point parsing and/or `hash_to_F`; identity safety depends on caller-local checks because the crate does not provide that guarantee.

### Candidate notes

- Candidate: `Ciphersuite::read_G` accepts canonical identity points and documents no stronger non-identity guarantee for callers, while audited consumers (`crypto/schnorr`, coordinator Tendermint, coordinator `SignCompleted`) treat successful decode as a usable verification key. Primary locus: [`crypto/ciphersuite/src/lib.rs:91-100`](C:/Users/MADAKI/Documents/serai/crypto/ciphersuite/src/lib.rs:91).
- Candidate note: `hash_to_F` domain separation is not uniform across impls; `Ristretto`, `Ed25519`, and `Ed448` still use naive `dst || data` concatenation, matching the trait warning at [`crypto/ciphersuite/src/lib.rs:49-56`](C:/Users/MADAKI/Documents/serai/crypto/ciphersuite/src/lib.rs:49).

### Totals status

- Totals unchanged after Chunk 1: 18 logged / 17 active / 2 High / 0 Critical / 1 Medium candidate / 7 Low candidates / 2 confirmed panic-DoS findings.

- **Session 28 (2026-05-29):** `crypto/ciphersuite` Chunk 1 complete. Restored the full crate file map and trait surface, traced all repo ciphersuite impls (`Ristretto`, `Ed25519`, `Secp256k1`, `P256`, `Ed448`), confirmed that `Ciphersuite::read_G` enforces decode success plus canonical round-trip but not non-identity, and mapped the exact blast radius into previously audited Schnorr/Tendermint/transaction parsing paths. `crypto/schnorr` remains closed; only the F-18/F-19 severity-refinement item carries forward. Totals unchanged.

---

## Session 29 Checkpoint (crypto/ciphersuite Chunk 2 - DST sweep + frost-guard check + close-out)

### Crate Status Snapshot (Session 29)

| Crate Path | Crate Name | Status | Note |
|---|---|---|---|
| crypto/schnorr | schnorr-signatures | CLOSED (Chunks 1-7) | carry-forward item still limited to F-18/F-19 Low-vs-Medium severity refinement |
| crypto/ciphersuite | ciphersuite | CLOSED (Chunks 1-2) | no new register entry; `read_G` identity admission remains documented as the root context for F-15/F-17 |

### `hash_to_F` production-call enumeration

#### Ristretto

- [`crypto/frost/src/curve/mod.rs:90`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/curve/mod.rs:90)
  - raw DST argument: `b"rho"`
  - fixed literal
  - effective ciphersuite DST on the Ristretto FROST path: `b"FROST-RISTRETTO255-SHA512-v1rho"` via [`crypto/frost/src/curve/mod.rs:56-58`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/curve/mod.rs:56)
  - production instantiation evidence: substrate Ristretto FROST signing/keygen paths in `processor/src/{batch_signer,cosigner,slash_report_signer,key_gen}.rs`
- [`crypto/frost/src/curve/mod.rs:112`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/curve/mod.rs:112)
  - raw DST argument: `b"nonce"`
  - fixed literal
  - effective ciphersuite DST on the Ristretto FROST path: `b"FROST-RISTRETTO255-SHA512-v1nonce"`
  - production instantiation evidence: same Ristretto FROST paths
- [`crypto/dkg/musig/src/lib.rs:83`](C:/Users/MADAKI/Documents/serai/crypto/dkg/musig/src/lib.rs:83)
  - DST argument: `b"dkg-musig"`
  - fixed literal
  - production instantiation evidence: generic keygen pipeline instantiates `C = Ristretto` in [`processor/src/key_gen.rs:132-145`](C:/Users/MADAKI/Documents/serai/processor/src/key_gen.rs:132)
- [`crypto/dkg/pedpop/src/lib.rs:93`](C:/Users/MADAKI/Documents/serai/crypto/dkg/pedpop/src/lib.rs:93)
  - DST argument: `b"DKG-PedPoP-proof_of_knowledge-0"`
  - fixed literal
  - production instantiation evidence: generic keygen pipeline instantiates `C = Ristretto` in [`processor/src/key_gen.rs:241-250`](C:/Users/MADAKI/Documents/serai/processor/src/key_gen.rs:241)
- [`crypto/dkg/pedpop/src/encryption.rs:323`](C:/Users/MADAKI/Documents/serai/crypto/dkg/pedpop/src/encryption.rs:323)
  - DST argument: `b"DKG-encryption-proof_of_possession"`
  - fixed literal
  - production instantiation evidence: same generic keygen pipeline
- [`message-queue/src/messages.rs:55`](C:/Users/MADAKI/Documents/serai/message-queue/src/messages.rs:55)
  - DST argument: `b"message_challenge"`
  - fixed literal
- [`message-queue/src/messages.rs:74`](C:/Users/MADAKI/Documents/serai/message-queue/src/messages.rs:74)
  - DST argument: `b"ack_challenge"`
  - fixed literal
- [`coordinator/src/tributary/transaction.rs:711`](C:/Users/MADAKI/Documents/serai/coordinator/src/tributary/transaction.rs:711)
  - DST argument: `b"SignCompleted signature"`
  - fixed literal

#### Ed25519

- [`crypto/frost/src/curve/mod.rs:90`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/curve/mod.rs:90)
  - raw DST argument: `b"rho"`
  - fixed literal
  - effective ciphersuite DST on the Ed25519 FROST path: `b"FROST-ED25519-SHA512-v1rho"`
  - production instantiation evidence: Monero uses `type Curve = Ed25519` at [`processor/src/networks/monero.rs:461-463`](C:/Users/MADAKI/Documents/serai/processor/src/networks/monero.rs:461)
- [`crypto/frost/src/curve/mod.rs:112`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/curve/mod.rs:112)
  - raw DST argument: `b"nonce"`
  - fixed literal
  - effective ciphersuite DST on the Ed25519 FROST path: `b"FROST-ED25519-SHA512-v1nonce"`
  - production instantiation evidence: same Monero FROST path
- [`crypto/dkg/musig/src/lib.rs:83`](C:/Users/MADAKI/Documents/serai/crypto/dkg/musig/src/lib.rs:83)
  - DST argument: `b"dkg-musig"`
  - fixed literal
  - production instantiation evidence: generic keygen pipeline instantiates `C = N::Curve`, and `N::Curve = Ed25519` for Monero
- [`crypto/dkg/pedpop/src/lib.rs:93`](C:/Users/MADAKI/Documents/serai/crypto/dkg/pedpop/src/lib.rs:93)
  - DST argument: `b"DKG-PedPoP-proof_of_knowledge-0"`
  - fixed literal
  - production instantiation evidence: same Monero keygen path
- [`crypto/dkg/pedpop/src/encryption.rs:323`](C:/Users/MADAKI/Documents/serai/crypto/dkg/pedpop/src/encryption.rs:323)
  - DST argument: `b"DKG-encryption-proof_of_possession"`
  - fixed literal
  - production instantiation evidence: same Monero keygen path
- [`processor/src/additional_key.rs:10-12`](C:/Users/MADAKI/Documents/serai/processor/src/additional_key.rs:10)
  - DST argument: `b"Serai DEX Additional Key"`
  - fixed literal
  - current production use: `additional_key::<Monero>(0)` at [`processor/src/networks/monero.rs:264-265`](C:/Users/MADAKI/Documents/serai/processor/src/networks/monero.rs:264)

#### Secp256k1

- [`crypto/frost/src/curve/mod.rs:90`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/curve/mod.rs:90)
  - raw DST argument: `b"rho"`
  - fixed literal
  - effective ciphersuite DST on the Secp256k1 FROST path: `b"FROST-secp256k1-SHA256-v1rho"`
  - production instantiation evidence: Bitcoin/Ethereum use `type Curve = Secp256k1` at [`processor/src/networks/bitcoin.rs:587`](C:/Users/MADAKI/Documents/serai/processor/src/networks/bitcoin.rs:587) and [`processor/src/networks/ethereum.rs:386`](C:/Users/MADAKI/Documents/serai/processor/src/networks/ethereum.rs:386)
- [`crypto/frost/src/curve/mod.rs:112`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/curve/mod.rs:112)
  - raw DST argument: `b"nonce"`
  - fixed literal
  - effective ciphersuite DST on the Secp256k1 FROST path: `b"FROST-secp256k1-SHA256-v1nonce"`
- [`crypto/frost/src/curve/kp256.rs:30-33`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/curve/kp256.rs:30)
  - raw DST argument: `b"chal"`
  - fixed literal
  - effective ciphersuite DST on the Secp256k1 FROST path: `b"FROST-secp256k1-SHA256-v1chal"`
- [`crypto/dkg/musig/src/lib.rs:83`](C:/Users/MADAKI/Documents/serai/crypto/dkg/musig/src/lib.rs:83)
  - DST argument: `b"dkg-musig"`
  - fixed literal
  - production instantiation evidence: generic keygen pipeline instantiates `C = N::Curve`, and `N::Curve = Secp256k1` for Bitcoin/Ethereum
- [`crypto/dkg/pedpop/src/lib.rs:93`](C:/Users/MADAKI/Documents/serai/crypto/dkg/pedpop/src/lib.rs:93)
  - DST argument: `b"DKG-PedPoP-proof_of_knowledge-0"`
  - fixed literal
- [`crypto/dkg/pedpop/src/encryption.rs:323`](C:/Users/MADAKI/Documents/serai/crypto/dkg/pedpop/src/encryption.rs:323)
  - DST argument: `b"DKG-encryption-proof_of_possession"`
  - fixed literal
- [`processor/src/networks/bitcoin.rs:335`](C:/Users/MADAKI/Documents/serai/processor/src/networks/bitcoin.rs:335)
  - DST argument: `KEY_DST`
  - fixed const `b"Serai Bitcoin Output Offset"` from [`processor/src/networks/bitcoin.rs:308`](C:/Users/MADAKI/Documents/serai/processor/src/networks/bitcoin.rs:308)
  - data: `b"branch"`
- [`processor/src/networks/bitcoin.rs:339`](C:/Users/MADAKI/Documents/serai/processor/src/networks/bitcoin.rs:339)
  - DST argument: `KEY_DST`
  - fixed const
  - data: `b"change"`
- [`processor/src/networks/bitcoin.rs:343`](C:/Users/MADAKI/Documents/serai/processor/src/networks/bitcoin.rs:343)
  - DST argument: `KEY_DST`
  - fixed const
  - data: `b"forward"`

#### P256

- [`crypto/frost/src/curve/mod.rs:90`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/curve/mod.rs:90)
  - raw DST argument: `b"rho"`
  - fixed literal
  - effective ciphersuite DST: `b"FROST-P256-SHA256-v1rho"`
- [`crypto/frost/src/curve/mod.rs:112`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/curve/mod.rs:112)
  - raw DST argument: `b"nonce"`
  - fixed literal
  - effective ciphersuite DST: `b"FROST-P256-SHA256-v1nonce"`
- [`crypto/frost/src/curve/kp256.rs:30-33`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/curve/kp256.rs:30)
  - raw DST argument: `b"chal"`
  - fixed literal
  - effective ciphersuite DST: `b"FROST-P256-SHA256-v1chal"`
- No concrete production P256 instantiation was found elsewhere in the repo.

#### Ed448

- No concrete production `hash_to_F` instantiation on Ed448 was found elsewhere in the repo.
- If the FROST Ed448 module were instantiated, the generic FROST calls above would use fixed effective DSTs `b"FROST-ED448-SHAKE256-v1rho"` and `b"FROST-ED448-SHAKE256-v1nonce"` via [`crypto/frost/src/curve/mod.rs:56-58`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/curve/mod.rs:56) and [`crypto/frost/src/curve/ed448.rs:9-12`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/curve/ed448.rs:9).

### DST-transposition result

- Every production `hash_to_F` call site uses a fixed literal or fixed const DST. No variable/computed attacker-controlled DST argument was found.
- For the naive-concatenation curves actually instantiated in production:
  - Ristretto: checked `FROST-RISTRETTO255-SHA512-v1rho`, `FROST-RISTRETTO255-SHA512-v1nonce`, `dkg-musig`, `DKG-PedPoP-proof_of_knowledge-0`, `DKG-encryption-proof_of_possession`, `message_challenge`, `ack_challenge`, and `SignCompleted signature`. No prefix relationship was found.
  - Ed25519: checked `FROST-ED25519-SHA512-v1rho`, `FROST-ED25519-SHA512-v1nonce`, `dkg-musig`, `DKG-PedPoP-proof_of_knowledge-0`, `DKG-encryption-proof_of_possession`, and `Serai DEX Additional Key`. No prefix relationship was found.
  - Ed448: no production instantiation found, so no reachable same-curve production pair to collide.
- Data-shape review on the naive-concatenation curves:
  - Transcript-based sites feed `transcript.challenge(...)`, not raw concatenated attacker strings.
  - `DigestTranscript::append` length-frames every field with an 8-byte LE length before bytes at [`crypto/transcript/src/lib.rs:99-103`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:99), and `append_message` emits separate label/value frames at [`crypto/transcript/src/lib.rs:120-123`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:120).
  - The only non-transcript direct Ed25519 site in production is `additional_key::<Monero>(0)`, whose data is fixed-shape `b"Monero" || 8-byte little-endian 0`.
- Verdict: the documented `dst || data` transposition hazard is **inert in the current production call graph**. No reachable collision candidate was promoted.

### `SignCompleted` challenge check

- Code path: [`coordinator/src/tributary/transaction.rs:703-711`](C:/Users/MADAKI/Documents/serai/coordinator/src/tributary/transaction.rs:703).
- DST is constant: `b"SignCompleted signature"`.
- Data is `&transcript.challenge(b"challenge")`.
- The transcript is length-framed:
  - `plan`, variable-length `tx_hash`, `signer`, and `nonce` are each appended with `append_message(...)`.
  - `DigestTranscript::append` writes `(member_tag || u64_le_len || value)` for every component at [`crypto/transcript/src/lib.rs:99-103`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:99).
- Verdict for this path: no DST-transposition collision was found on the F-15 attacker-reachable `SignCompleted` challenge.

### kp256 check

- `Secp256k1` / `P256` need no further DST-transposition review for this crate.
- Reason: [`crypto/ciphersuite/kp256/src/lib.rs:75-80`](C:/Users/MADAKI/Documents/serai/crypto/ciphersuite/kp256/src/lib.rs:75) passes DST as a separate parameter to `ExpandMsgXmd::<Sha256>::expand_message(&[msg], &[dst], 48)`, so it does not use naive `dst || data` concatenation.

### FROST identity-guard result

- Guard body: [`crypto/frost/src/curve/mod.rs:123-130`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/curve/mod.rs:123)
  - `/// Read a point from a reader, rejecting identity.`
  - `let res = <Self as Ciphersuite>::read_G(reader)?;`
  - `if res.is_identity().into() { Err(io::Error::other("identity point"))?; }`
- Object/context resolution:
  - In production, this helper is used for `GeneratorCommitments::read` only, at [`crypto/frost/src/nonce.rs:33-35`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/nonce.rs:33).
  - That object is a pair of nonce-commitment points (`commitment_D`, `commitment_E`) inside a preprocess message, not a verification share or verification public key.
  - The input context is untrusted multi-party wire input: preprocess messages are broadcast to other participants over an authenticated channel per [`crypto/frost/src/sign.rs:103-107`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/sign.rs:103), then parsed by `read_preprocess` via `Commitments::read` at [`crypto/frost/src/sign.rs:276-278`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/sign.rs:276).
  - The guard rejects identity only; malformed/non-canonical points are already rejected by `Ciphersuite::read_G` before the `is_identity()` check.
  - Evident reason: zero nonces would yield zero commitments and are explicitly treated as unsafe in the nonce-generation comments at [`crypto/frost/src/curve/mod.rs:103-108`](C:/Users/MADAKI/Documents/serai/crypto/frost/src/curve/mod.rs:103).
- Verdict: this is **not** a same-object / same-context parallel to F-15/F-17. It is a nonce/commitment guard on untrusted preprocess input, not a verification-public-key identity check.

### Crate verdict

- `crypto/ciphersuite` closes cleanly after Chunk 2.
- No new register entry was added from the `hash_to_F` DST review.
- `read_G` identity admission remains recorded as the documented root context behind already-confirmed `FINDING-15` and `FINDING-17`, not as a separate new finding.

### Totals status

- Totals unchanged after Session 29: 18 logged / 17 active / 2 High / 0 Critical / 1 Medium candidate / 7 Low candidates / 2 confirmed panic-DoS findings.

### Next target

- Next queue: `multiexp` partial -> `dalek-ff-group` -> `flexible-transcript`.

- **Session 29 (2026-05-29):** `crypto/ciphersuite` Chunk 2 complete and crate closed. Enumerated every production `hash_to_F` call site, resolved the naive-concatenation DST set on Ristretto/Ed25519, and found no reachable transposition collision because all production DSTs are fixed/distinct and transcript-backed inputs are length-framed. Separately confirmed the FROST `read_G` identity guard is on nonce-commitment preprocess points, not verification public keys, so it is not a direct parallel to F-15/F-17. `read_G` identity admission remains documented as root context only. Totals unchanged; next target is `multiexp` partial -> `dalek-ff-group` -> `flexible-transcript`.

---

## Session 30 Checkpoint (crypto/multiexp completion pass)

### Crate Status Snapshot (Session 30)

| Crate Path | Crate Name | Status | Note |
|---|---|---|---|
| crypto/ciphersuite | ciphersuite | CLOSED (Chunks 1-2) | closed cleanly; `read_G` identity admission remains root context only |
| crypto/multiexp | multiexp | CLOSED (completion pass) | F-13 empty-input panic remains cleared as false positive |

### File / API map

- Production files:
  - `crypto/multiexp/src/lib.rs`
  - `crypto/multiexp/src/straus.rs`
  - `crypto/multiexp/src/pippenger.rs`
  - `crypto/multiexp/src/batch.rs`
- Test file/folder:
  - `crypto/multiexp/src/tests/*`
- Public surface:
  - [`crypto/multiexp/src/lib.rs:180`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/lib.rs:180) `pub fn multiexp`
  - [`crypto/multiexp/src/lib.rs:194`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/lib.rs:194) `pub fn multiexp_vartime`
  - [`crypto/multiexp/src/batch.rs:24`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/batch.rs:24) `pub struct BatchVerifier`
  - `BatchVerifier::{new, queue, verify, verify_vartime, blame_vartime, verify_with_vartime_blame, verify_vartime_with_vartime_blame}`
- Internal algorithm-selection logic:
  - [`crypto/multiexp/src/lib.rs:76-82`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/lib.rs:76) `enum Algorithm { Null, Single, Straus(u8), Pippenger(u8) }`
  - [`crypto/multiexp/src/lib.rs:129-175`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/lib.rs:129) `fn algorithm(len: usize) -> Algorithm`

### Correctness result

- Wrapper correctness:
  - [`crypto/multiexp/src/lib.rs:183-189`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/lib.rs:183) handles the public API cases explicitly:
    - `len == 0` -> `Group::identity()`
    - `len == 1` -> `pairs[0].1 * pairs[0].0`
    - `len >= 2` -> Straus or Pippenger by threshold
  - `multiexp_vartime` mirrors the same wrapper logic at [`crypto/multiexp/src/lib.rs:195-200`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/lib.rs:195).
- Selection boundaries:
  - release:
    - `0 -> Null`
    - `1 -> Single`
    - `2..=9 -> Straus(3)`
    - `10..=19 -> Straus(4)`
    - `20..=49 -> Straus(5)`
    - `50..=99 -> Pippenger(4)`
    - `100..=124 -> Pippenger(5)`
    - `125..=274 -> Pippenger(6)`
    - `275..=399 -> Pippenger(7)`
    - `>= 400 -> Pippenger(8)`
  - debug:
    - `0 -> Null`
    - `1 -> Single`
    - `2..=9 -> Straus(3)`
    - `10..=79 -> Straus(4)`
    - `80..=99 -> Straus(5)`
    - `100..=124 -> Pippenger(4)`
    - `125..=274 -> Pippenger(5)`
    - `275..=474 -> Pippenger(6)`
    - `475..=749 -> Pippenger(7)`
    - `>= 750 -> Pippenger(8)`
- Core arithmetic:
  - `prep_bits` in [`crypto/multiexp/src/lib.rs:54-74`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/lib.rs:54) decomposes each scalar into base-`2^window` little-endian digits.
  - `straus` in [`crypto/multiexp/src/straus.rs:27-50`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/straus.rs:27) precomputes `[0, P, 2P, ...]` per point and then evaluates windows high-to-low, doubling by `window` and adding the per-point digit multiple. This is standard Straus evaluation of `Σ s_i P_i`.
  - `pippenger` in [`crypto/multiexp/src/pippenger.rs:10-40`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/pippenger.rs:10) buckets points by each digit, then walks buckets descending with the standard running-sum trick so bucket `k` contributes `k * bucket[k]`. This is standard Pippenger evaluation of `Σ s_i P_i`.
  - Both algorithms consume the same `prep_bits` representation and therefore implement the same sum over different cost models.
  - Empty-case false positive remains cleared: the public wrappers return identity before the internal algorithms touch `groupings[0]` / `bits[0]`.
  - Single-element case is direct scalar multiplication and bypasses both algorithms.
  - In-tree agreement check exists at [`crypto/dkg/musig/src/lib.rs:153`](C:/Users/MADAKI/Documents/serai/crypto/dkg/musig/src/lib.rs:153): `debug_assert_eq!(musig_key_vartime::<C>(context, keys), Ok(group_key));`, comparing constant-time and vartime results on the same multiexp input.

### `is_identity` / returned-element result

- `multiexp_vartime` always returns an actual `G`, not a boolean or shortcut marker.
- `straus_vartime` uses `Option<G>` only as an internal identity accumulator optimization and returns `res.unwrap_or_else(G::identity)` at [`crypto/multiexp/src/straus.rs:78`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/straus.rs:78).
- `pippenger_vartime` uses `Option<G>` buckets only as an internal optimization for zero buckets and returns the accumulated `res: G` at [`crypto/multiexp/src/pippenger.rs:48-85`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/pippenger.rs:48).
- There is no early `is_identity()` check or short-circuit in either algorithm; downstream `is_identity()` queries act on the real accumulated group element.

### vartime-discipline result

- Clean public-data vartime sites:
  - `crypto/schnorr/src/lib.rs:109` signature verification over public `(R, A, c)`.
  - `crypto/schnorr/src/aggregate.rs:145` aggregate verification over public nonces/keys/challenges.
  - `crypto/frost/src/nonce.rs:208` aggregation of public nonce commitments with public binding factors.
  - `crypto/dkg/musig/src/lib.rs:107` `musig_key_vartime`, explicitly documented as public-key only.
  - `crypto/dkg/pedpop/src/lib.rs:518` verification-share reconstruction from public stripe commitments and public participant indices.
  - `crypto/dleq/src/cross_group/mod.rs:421` `BatchVerifier::verify_vartime()` over public proof statements, public generators, and reconstructed public keys.
- Candidate vartime-discipline issue:
  - Direct secret-scalar vartime use: [`crypto/dkg/pedpop/src/lib.rs:597-603`](C:/Users/MADAKI/Documents/serai/crypto/dkg/pedpop/src/lib.rs:597) calls `multiexp_vartime` on `share_verification_statements(..., Zeroizing::new(share))`, where `share` is the decrypted secret share from the sender.
  - Indirect secret-scalar vartime path through `BatchVerifier` blame:
    - secret share statements are queued at [`crypto/dkg/pedpop/src/lib.rs:487-490`](C:/Users/MADAKI/Documents/serai/crypto/dkg/pedpop/src/lib.rs:487),
    - then [`crypto/dkg/pedpop/src/lib.rs:493`](C:/Users/MADAKI/Documents/serai/crypto/dkg/pedpop/src/lib.rs:493) calls `verify_with_vartime_blame()`,
    - whose fallback reaches [`crypto/multiexp/src/batch.rs:108-123`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/batch.rs:108) `blame_vartime()` and executes `multiexp_vartime` on those queued statements if batch verification fails.
- No intrinsic secret-scalar misuse was found inside `crypto/multiexp`; the misuse candidate is downstream call-site discipline in `dkg-pedpop`.

### BatchVerifier soundness result

- Random weight source:
  - [`crypto/multiexp/src/batch.rs:39-44`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/batch.rs:39) takes caller-supplied `rng: &mut R` with `R: RngCore + CryptoRng`.
  - [`crypto/multiexp/src/batch.rs:49-53`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/batch.rs:49) draws `weight = G::Scalar::random(&mut *rng)`.
  - [`crypto/multiexp/src/batch.rs:82-84`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/batch.rs:82) rejects zero weights.
- Application:
  - first statement uses `u = 1` at [`crypto/multiexp/src/batch.rs:46-48`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/batch.rs:46),
  - every later statement gets its own independent non-zero `u`,
  - each pair is transformed to `(scalar * u, point)` at [`crypto/multiexp/src/batch.rs:88`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/batch.rs:88).
- Soundness implication:
  - this linearizes each statement under an independent random scalar factor, preventing a malicious later statement from deterministically cancelling an honest earlier one unless it guesses the random weight relation.
  - This matches the batch-soundness assumption relied on in the Schnorr batch-clearance work.

### Panic-surface result

- No new public attacker-length panic was confirmed in `multiexp` itself.
- `multiexp` / `multiexp_vartime` safely handle empty and single-element inputs at the public wrapper layer.
- `straus{,_vartime}` and `pippenger{,_vartime}` index `groupings[0]` / `bits[0]`, but these functions are crate-private and only reached through the wrapper routing that excludes `len < 2`.
- `BatchVerifier::{verify_with_vartime_blame, verify_vartime_with_vartime_blame}` contain `unwrap()` on `blame_vartime()` at [`crypto/multiexp/src/batch.rs:131`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/batch.rs:131) and [`crypto/multiexp/src/batch.rs:141`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/batch.rs:141), but no attacker-controlled public-API route was confirmed to make `verify* == false` while `blame_vartime() == None` under the current arithmetic.

### Candidate notes

- Candidate: secret scalar flows into the vartime path in `dkg-pedpop` blame handling, both directly and indirectly through `BatchVerifier` fallback.
  - direct locus: [`crypto/dkg/pedpop/src/lib.rs:597-603`](C:/Users/MADAKI/Documents/serai/crypto/dkg/pedpop/src/lib.rs:597)
  - indirect locus: [`crypto/dkg/pedpop/src/lib.rs:487-494`](C:/Users/MADAKI/Documents/serai/crypto/dkg/pedpop/src/lib.rs:487) plus [`crypto/multiexp/src/batch.rs:108-123`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/batch.rs:108)

### Crate verdict

- `crypto/multiexp` closes cleanly for arithmetic correctness and batch-soundness.
- No intrinsic arithmetic or panic issue was confirmed in the crate.
- The only notable result is a downstream vartime-discipline candidate in `dkg-pedpop`, not a new `multiexp`-local finding.

### Totals status

- Totals unchanged after Session 30: 18 logged / 17 active / 2 High / 0 Critical / 1 Medium candidate / 7 Low candidates / 2 confirmed panic-DoS findings.

### Next target

- Next queue: `dalek-ff-group` -> `flexible-transcript`.

- **Session 30 (2026-05-29):** `crypto/multiexp` completion pass complete and crate closed. Re-verified that empty input safely returns identity through `Algorithm::Null`, mapped the exact Straus/Pippenger selection thresholds, confirmed both constant-time and vartime wrappers compute the same `Σ scalar_i * point_i` semantics, and re-confirmed `BatchVerifier` soundness from caller-supplied non-zero random weights. No arithmetic or public panic issue was confirmed in the crate. One downstream candidate was recorded: `dkg-pedpop` sends decrypted secret-share scalars into the vartime path directly and through `BatchVerifier` blame fallback. Totals unchanged; next target is `dalek-ff-group` -> `flexible-transcript`.

---

## Session 31 Checkpoint (PedPoP blame constant-time follow-up)

### Crate Status Snapshot (Session 31)

| Crate Path | Crate Name | Status | Note |
|---|---|---|---|
| crypto/multiexp | multiexp | CLOSED (completion pass) | core arithmetic and `BatchVerifier` soundness confirmed; verify kernel under F-15/F-17 remains sound |
| crypto/dkg/pedpop | dkg-pedpop | CLOSED (follow-up only) | prior vartime candidate on blame path cleared; no FINDING-20 added |

### PedPoP blame vartime check

- Queue call resolution:
  - [`crypto/dkg/pedpop/src/lib.rs:487-490`](C:/Users/MADAKI/Documents/serai/crypto/dkg/pedpop/src/lib.rs:487) queues:
    - `share_verification_statements::<C>(self.params.i(), &self.commitments[&l], share)`
  - The returned pairs are built in [`crypto/dkg/pedpop/src/lib.rs:430-448`](C:/Users/MADAKI/Documents/serai/crypto/dkg/pedpop/src/lib.rs:430):
    - `exponential::<C>(target, commitments)` contributes `(exp, commitment_point)` pairs where `exp` is derived from the public participant index `target`
    - the decrypted `share` is converted outside the multiexp into `neg_share_pub = C::generator() * -*share`
    - the final queued pair is `(C::F::ONE, neg_share_pub)`
  - Result: the queued multiexp scalars are public values (`i^k` evaluation coefficients and `1`), not the decrypted secret share.

- Direct blame-path `multiexp_vartime` resolution:
  - [`crypto/dkg/pedpop/src/lib.rs:597-603`](C:/Users/MADAKI/Documents/serai/crypto/dkg/pedpop/src/lib.rs:597) calls:
    - `multiexp_vartime(&share_verification_statements::<C>(recipient, &self.commitments[&sender], Zeroizing::new(share)))`
  - The share at that point is still locally decrypted from the accusation message inside `blame_internal`; it is not published by the API as a scalar.
  - However, `share_verification_statements` again converts it to the public point `neg_share_pub = G * (-share)` before the vartime multiexp is entered.
  - Result: the direct vartime call still receives only public scalars (`recipient` powers and `1`), not the confidential share scalar bits.

- `verify_with_vartime_blame()` fallback scope:
  - [`crypto/multiexp/src/batch.rs:108-123`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/batch.rs:108) binary-searches over slices:
    - `if multiexp_vartime(&flat(&slice[.. split])).is_identity().into() { slice = &slice[split ..]; } else { slice = &slice[.. split]; }`
    - then checks the final single statement with `multiexp_vartime(value)`
  - Result: yes, fallback reprocesses halves that may contain honest queued statements, not only the single failing statement.
  - Yet those queued statements still use public multiexp scalars, merely reweighted by the verifier's random `u`.

- Reachability:
  - A single malformed/invalid share can force the constant-time batch to fail:
    - queued at [`crypto/dkg/pedpop/src/lib.rs:487-490`](C:/Users/MADAKI/Documents/serai/crypto/dkg/pedpop/src/lib.rs:487)
    - failure is checked by [`crypto/dkg/pedpop/src/lib.rs:493`](C:/Users/MADAKI/Documents/serai/crypto/dkg/pedpop/src/lib.rs:493) `batch.verify_with_vartime_blame()`
    - fallback then runs [`crypto/multiexp/src/batch.rs:127-132`](C:/Users/MADAKI/Documents/serai/crypto/multiexp/src/batch.rs:127) `if self.verify() { ... } else { Err(self.blame_vartime().unwrap()) }`
  - So attacker-forceable fallback is real.
  - But the fallback does not expose decrypted secret shares as vartime scalars; it reprocesses statements whose scalars are public coefficients/random batch weights.

- Design-note check:
  - No comment or design note was found stating that blame-time secret shares are considered public/disclosed or that blame intentionally sacrifices constant-time on confidential scalar material.
  - The code comments around blame focus on authenticated channels and identifying the faulty party, not on intentional declassification of share values.

### Verdict

- Cleared.
- Reason: the attacker can force the vartime fallback, and that fallback may include honest statements, but the multiexp scalars in both the queued path and the direct `blame_internal` path are public coefficients (`i^k`, `1`) plus verifier-random batch weights, not the decrypted secret share itself.
- No FINDING-20 added.

### Totals status

- Totals unchanged after Session 31: 18 logged / 17 active / 2 High / 0 Critical / 1 Medium candidate / 7 Low candidates / 2 confirmed panic-DoS findings.

### Next target

- Next queue: `dalek-ff-group` -> `flexible-transcript`.

- **Session 31 (2026-05-30):** Targeted PedPoP blame constant-time follow-up complete. Confirmed that a malformed share can force `verify_with_vartime_blame()` into the binary-search fallback, and that fallback can reprocess honest statements, but cleared the multiexp-vartime candidate because neither the queued statements nor the direct blame-path `multiexp_vartime` call receives the decrypted secret share as a scalar. The share is converted to a public point before entering multiexp, while the actual multiexp scalars remain public evaluation coefficients and batch weights. `crypto/multiexp` remains CLOSED clean. Totals unchanged; next target remains `dalek-ff-group` -> `flexible-transcript`.

---

## Session 32 Checkpoint (crypto/dalek-ff-group Chunk 1 - wrapper correctness + constant-time)

### Crate Status Snapshot (Session 32)

| Crate Path | Crate Name | Status | Note |
|---|---|---|---|
| crypto/dalek-ff-group | dalek-ff-group | Chunk 1 complete | wrapper audit in progress; no new finding confirmed in Chunk 1 |
| crypto/transcript | flexible-transcript | NEXT | next target after dalek-ff-group Chunk 1 |

### File / API map

- Production files:
  - `crypto/dalek-ff-group/src/lib.rs`
  - `crypto/dalek-ff-group/src/field.rs`
  - `crypto/dalek-ff-group/src/ciphersuite.rs`
- Public surface:
  - Re-exports:
    - `curve25519_dalek as dalek`
    - `ED25519_BASEPOINT_TABLE`
    - `RISTRETTO_BASEPOINT_TABLE`
    - `FieldElement`
    - ciphersuite markers `Ed25519`, `Ristretto`
  - `Scalar(pub DScalar)`:
    - derives `Clone`, `Copy`, `PartialEq`, `Eq`, `Default`, `Debug`, `Zeroize`
    - trait impls via wrapper macros: `Deref`, `Borrow`, `ConstantTimeEq`, `ConditionallySelectable`, `Add`, `AddAssign`, `Sub`, `SubAssign`, `Mul`, `MulAssign`, `Neg`
    - integer conversions: `From<u8>`, `From<u16>`, `From<u32>`, `From<u64>`, `From<u128>`
    - methods: `pow`, `from_bytes_mod_order_wide`, `from_hash`
    - trait impls: `Field`, `PrimeField`, `PrimeFieldBits`, `FromUniformBytes<64>`, `Sum`, `Product`
  - group wrappers generated by `dalek_group!`:
    - `EdwardsPoint(pub DEdwardsPoint)`
    - `RistrettoPoint(pub DRistrettoPoint)`
    - both derive `Clone`, `Copy`, `PartialEq`, `Eq`, `Debug`, `Zeroize`
    - both implement `Deref`, `Borrow`, `ConstantTimeEq`, `ConditionallySelectable`, `Add`, `AddAssign`, `Sub`, `SubAssign`, `Mul<Scalar>`, `MulAssign<Scalar>`, `Neg`, `Sum`, `Group`, `GroupEncoding`, `PrimeGroup`, `Hash`
    - constants: `ED25519_BASEPOINT_POINT`, `RISTRETTO_BASEPOINT_POINT`
    - extra method: `EdwardsPoint::mul_by_cofactor`
    - basepoint-table multiplication wrappers: `impl Mul<Scalar> for &EdwardsBasepointTable` and `impl Mul<Scalar> for &RistrettoBasepointTable`
  - `FieldElement`:
    - derives `Clone`, `Copy`, `PartialEq`, `Eq`, `Default`, `Debug`, `Zeroize`
    - trait impls: `ConstantTimeEq`, `ConditionallySelectable`, `Add`, `AddAssign`, `Sub`, `SubAssign`, `Mul`, `MulAssign`, `Neg`, `Field`, `PrimeField`, `PrimeFieldBits`, `FromUniformBytes<64>`, `Sum`, `Product`
    - methods: `from_u256`, `wide_reduce`, `pow`, `sqrt_ratio_i`
  - ciphersuite marker structs:
    - `Ristretto`, `Ed25519`
    - both implement `ciphersuite::Ciphersuite<F = Scalar, G = Point, H = Sha512>` with `hash_to_F(dst, data) = Scalar::from_hash(Sha512(dst || data))`

### Wrapper-body results

- Point decode/encode wrappers:
  - [`crypto/dalek-ff-group/src/lib.rs:429-445`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:429) performs:
    - `decompress()`
    - `CtOption::new($Point(point), decompressed.is_some() & torsion_free(point))`
    - `from_bytes_unchecked == from_bytes`
    - `to_bytes == self.0.to_bytes()`
- Scalar canonical wrappers:
  - [`crypto/dalek-ff-group/src/lib.rs:300-304`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:300) delegates `PrimeField::{from_repr,to_repr}` directly to dalek `Scalar`
- Identity predicate:
  - [`crypto/dalek-ff-group/src/lib.rs:418-420`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:418) uses `self.0.ct_eq(&$DPoint::identity())`
- Scalar multiplication entry points:
  - generic wrapper body at [`crypto/dalek-ff-group/src/lib.rs:129-149`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:129), instantiated for:
    - `Scalar * Scalar` at [`crypto/dalek-ff-group/src/lib.rs:182`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:182)
    - `Point * Scalar` at [`crypto/dalek-ff-group/src/lib.rs:381`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:381)
  - basepoint-table wrapper body at [`crypto/dalek-ff-group/src/lib.rs:450-455`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:450)

### Canonical decode / F-15-F-17 root-property result

- Off-curve / invalid encodings are rejected:
  - Ed25519 wrapper uses `CompressedEdwardsY::decompress()` and rejects `None` at [`crypto/dalek-ff-group/src/lib.rs:429-437`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:429); dalek `decompress()` returns `None` when the input is not a valid curve `y`-coordinate at [`curve25519-dalek-4.1.3/src/edwards.rs:190-201`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/curve25519-dalek-4.1.3/src/edwards.rs:190).
  - Ristretto wrapper uses `CompressedRistretto::decompress()` and rejects `None`; dalek explicitly documents this as accepting only canonical encodings of a point at [`curve25519-dalek-4.1.3/src/ristretto.rs:248-269`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/curve25519-dalek-4.1.3/src/ristretto.rs:248).
- Prime-order filtering is wrapper-correct:
  - Ed25519 passes `|point| point.is_torsion_free()` at [`crypto/dalek-ff-group/src/lib.rs:470-478`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:470), and dalek implements that as `[l]P == identity` at [`curve25519-dalek-4.1.3/src/edwards.rs:1257-1259`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/curve25519-dalek-4.1.3/src/edwards.rs:1257).
  - Ristretto passes `|_| true` at [`crypto/dalek-ff-group/src/lib.rs:486-494`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:486); this is correct because canonical Ristretto encodings already represent elements of the prime-order quotient group, so there is no extra torsion filter to apply at the wrapper.
- Canonical round-trip holds:
  - wrapper `to_bytes` is just inner `self.0.to_bytes()` at [`crypto/dalek-ff-group/src/lib.rs:443-444`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:443)
  - dalek `EdwardsPoint::to_bytes()` is `self.compress().to_bytes()` at [`curve25519-dalek-4.1.3/src/edwards.rs:1330-1332`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/curve25519-dalek-4.1.3/src/edwards.rs:1330)
  - dalek `RistrettoPoint::to_bytes()` is `self.compress().to_bytes()` at [`curve25519-dalek-4.1.3/src/ristretto.rs:1228-1229`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/curve25519-dalek-4.1.3/src/ristretto.rs:1228)
- Identity admission remains the already-documented F-15/F-17 context:
  - canonical identity encodings pass `from_bytes` because the wrapper rejects invalid / non-prime-order points, not the identity itself
  - this is context only, not a new finding in this crate

### Scalar canonicality result

- Scalar decode is canonical and rejects unreduced inputs:
  - wrapper `Scalar::from_repr` directly delegates to dalek at [`crypto/dalek-ff-group/src/lib.rs:300-301`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:300)
  - dalek `PrimeField::from_repr` routes to `from_canonical_bytes` at [`curve25519-dalek-4.1.3/src/scalar.rs:1258-1260`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/curve25519-dalek-4.1.3/src/scalar.rs:1258)
  - dalek `from_canonical_bytes` rejects high-bit-set and non-canonical / `>= order` representatives at [`curve25519-dalek-4.1.3/src/scalar.rs:254-265`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/curve25519-dalek-4.1.3/src/scalar.rs:254)
- Scalar re-encoding is canonical:
  - wrapper `to_repr` is `self.0.to_repr()` at [`crypto/dalek-ff-group/src/lib.rs:303-304`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:303)
  - dalek `to_repr` is `to_bytes()` at [`curve25519-dalek-4.1.3/src/scalar.rs:1277-1278`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/curve25519-dalek-4.1.3/src/scalar.rs:1277), and `to_bytes()` returns the canonical internal byte encoding at [`curve25519-dalek-4.1.3/src/scalar.rs:680-692`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/curve25519-dalek-4.1.3/src/scalar.rs:680)

### `is_identity` soundness result

- Wrapper implementation is exact:
  - [`crypto/dalek-ff-group/src/lib.rs:418-420`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:418) checks inner-point equality against dalek identity
  - no wrapper-side alternate representation or cached flag exists that could drift from the true group element
- Backing dalek equality is constant-time group equality, not encoding equality:
  - EdwardsPoint uses projective cross-multiplication at [`curve25519-dalek-4.1.3/src/edwards.rs:482-491`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/curve25519-dalek-4.1.3/src/edwards.rs:482)
  - RistrettoPoint uses its quotient-aware constant-time equality relation at [`curve25519-dalek-4.1.3/src/ristretto.rs:828-841`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/curve25519-dalek-4.1.3/src/ristretto.rs:828)
- Conclusion:
  - no wrapper-introduced false positive / false negative was found for `is_identity`

### Constant-time result

- Wrapper `ConstantTimeEq` is a direct delegation, not a fallback to variable-time `==`:
  - [`crypto/dalek-ff-group/src/lib.rs:104-113`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:104)
  - Scalar inner `ct_eq` compares bytes in constant time at [`curve25519-dalek-4.1.3/src/scalar.rs:301-302`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/curve25519-dalek-4.1.3/src/scalar.rs:301)
- Secret-scalar arithmetic stays on constant-time paths:
  - wrapper `Scalar` add/sub/mul are direct dalek scalar ops via macro instantiation at [`crypto/dalek-ff-group/src/lib.rs:182`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:182)
  - wrapper `Point * Scalar` is direct dalek point multiplication via macro instantiation at [`crypto/dalek-ff-group/src/lib.rs:381`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:381)
  - same wrapper path covers the previously cleared `G * -share` use: the share is negated as a scalar, then multiplied through dalek point-scalar multiplication with no wrapper branch on scalar bytes
  - dalek variable-base Edwards multiplication goes through `backend::variable_base_mul` at [`curve25519-dalek-4.1.3/src/edwards.rs:726-727`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/curve25519-dalek-4.1.3/src/edwards.rs:726); Ristretto delegates to underlying Edwards multiplication at [`curve25519-dalek-4.1.3/src/ristretto.rs:930-935`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/curve25519-dalek-4.1.3/src/ristretto.rs:930)
- Nonzero sampling is not reimplemented here:
  - `random_nonzero_F` is provided by the ciphersuite trait at [`crypto/ciphersuite/src/lib.rs:60-68`](C:/Users/MADAKI/Documents/serai/crypto/ciphersuite/src/lib.rs:60)
  - it loops on `F::random()` plus `ct_eq(&ZERO)`, so the only dalek-ff-group responsibility is that `Scalar::random` and `Scalar::ct_eq` stay constant-time, which they do
- No wrapper-introduced data-dependent branch on secret scalar bytes was found in the audited paths

### `from_hash` / wide reduction result

- `hash_to_F` path:
  - [`crypto/dalek-ff-group/src/ciphersuite.rs:31-33`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/ciphersuite.rs:31) calls `Scalar::from_hash(Sha512(dst || data))`
  - [`crypto/dalek-ff-group/src/lib.rs:244-249`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/lib.rs:244) finalizes 64 bytes and calls `DScalar::from_bytes_mod_order_wide`
  - dalek `from_bytes_mod_order_wide` is the canonical unbiased wide reduction entry at [`curve25519-dalek-4.1.3/src/scalar.rs:250-252`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/curve25519-dalek-4.1.3/src/scalar.rs:250)
- `FieldElement::wide_reduce`:
  - exact 512-bit modular reduction at [`crypto/dalek-ff-group/src/field.rs:58-62`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/field.rs:58) and [`crypto/dalek-ff-group/src/field.rs:230-231`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/field.rs:230)
  - not used by `hash_to_F` in this crate, but the reduction logic itself is correct for uniform 64-byte inputs

### Panic-surface result

- No attacker-byte public decode panic was confirmed in the wrapper surface.
- Relevant decode paths (`GroupEncoding::from_bytes`, `PrimeField::from_repr`, `Scalar::from_hash`, `FieldElement::wide_reduce`) contain no attacker-controlled `unwrap` / `expect`.
- Only `unwrap` in production code is the constant-modulus construction in [`crypto/dalek-ff-group/src/field.rs:60`](C:/Users/MADAKI/Documents/serai/crypto/dalek-ff-group/src/field.rs:60): `NonZero::new(WIDE_MODULUS).unwrap()`.
  - This is compile-time-constant and not attacker influenced.

### Candidate notes

- No new wrapper-introduced finding candidate was confirmed in Chunk 1.
- Context carried forward only:
  - canonical identity point admission remains the already-recorded root property behind `FINDING-15` / `FINDING-17`
  - `from_bytes_unchecked` does not bypass checks; it aliases `from_bytes`

### Verification

- `cargo test -p dalek-ff-group --lib` passed locally

### Totals status

- Totals unchanged after Session 32: 18 logged / 17 active / 2 High / 0 Critical / 1 Medium candidate / 7 Low candidates / 2 confirmed panic-DoS findings.

### Next target

- Next queue: `flexible-transcript`.

- **Session 32 (2026-05-30):** `crypto/dalek-ff-group` Chunk 1 complete. Mapped the full public wrapper surface, confirmed point decode delegates safely to dalek plus the wrapper's intended prime-order gate (`is_torsion_free` for Ed25519, no extra gate for canonical Ristretto), re-confirmed scalar canonical decode/re-encode through dalek `from_canonical_bytes`, and verified that wrapper `is_identity` is a direct constant-time equality check against dalek identity with no wrapper-side ambiguity. Constant-time review found no wrapper-introduced branch on secret scalar bytes; the same point-scalar multiplication path covers the earlier `G * -share` clearance. No new candidate added. Totals unchanged; next target is `flexible-transcript`.

---

## Session 33 Checkpoint (crypto/transcript Chunk 1 - framing integrity + collision resistance)

### Crate Status Snapshot (Session 33)

| Crate Path | Crate Name | Status | Note |
|---|---|---|---|
| crypto/transcript | flexible-transcript | Chunk 1 complete | framing injectivity confirmed; no reopening of prior transcript-dependent clearances |
| message-queue | message-queue | NEXT | dedicated F-16 chunk next |

### File / API map

- Production files:
  - `crypto/transcript/src/lib.rs`
  - `crypto/transcript/src/merlin.rs`
- Test file:
  - `crypto/transcript/src/tests.rs`
- Public surface:
  - `Transcript` trait:
    - `type Challenge`
    - `new`
    - `domain_separate`
    - `append_message`
    - `challenge`
    - `rng_seed`
  - `SecureDigest` trait and blanket impl
  - `DigestTranscript<D>`
  - `RecommendedTranscript = DigestTranscript<blake2::Blake2b512>` behind `recommended`
  - `MerlinTranscript` behind `merlin`

### Transcript bodies

- `DigestTranscript`:
  - internal framing helper at [`crypto/transcript/src/lib.rs:98-104`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:98):
    - `kind: u8`
    - `u64::to_le_bytes(value.len())`
    - raw `value`
  - constructor / APIs at [`crypto/transcript/src/lib.rs:110-140`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:110)
- `RecommendedTranscript`:
  - alias only; exact bodies are the same as `DigestTranscript<Blake2b512>` at [`crypto/transcript/src/lib.rs:175-177`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:175)
- `MerlinTranscript` wrapper:
  - constructor / APIs at [`crypto/transcript/src/merlin.rs:31-57`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/merlin.rs:31)
  - underlying Merlin framing bodies relied on by the wrapper:
    - `merlin::Transcript::new` at [`merlin-3.0.0/src/transcript.rs:69-87`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/merlin-3.0.0/src/transcript.rs:69)
    - `merlin::Transcript::append_message` at [`merlin-3.0.0/src/transcript.rs:96-100`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/merlin-3.0.0/src/transcript.rs:96)
    - `merlin::Transcript::challenge_bytes` at [`merlin-3.0.0/src/transcript.rs:175-179`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/merlin-3.0.0/src/transcript.rs:175)

### Framing integrity result

- `DigestTranscript` byte layout is fully self-delimiting and injective.
  - Member tags:
    - `Name = 0`
    - `Domain = 1`
    - `Label = 2`
    - `Value = 3`
    - `Challenge = 4`
    - `Continued = 5`
    - `Challenged = 6`
    - source: [`crypto/transcript/src/lib.rs:56-79`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:56)
  - `append(kind, value)` serializes exactly:
    - `kind_u8 || len_u64_le || value`
    - source: [`crypto/transcript/src/lib.rs:99-103`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:99)
  - `append_message(label, message)` serializes exactly:
    - `0x02 || len_le64(label) || label || 0x03 || len_le64(message) || message`
    - source: [`crypto/transcript/src/lib.rs:120-123`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:120)
  - `domain_separate(label)` serializes:
    - `0x01 || len_le64(label) || label`
    - source: [`crypto/transcript/src/lib.rs:116-118`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:116)
  - transcript name serializes:
    - `0x00 || len_le64(name) || name`
    - source: [`crypto/transcript/src/lib.rs:110-113`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:110)
  - label is explicitly length-framed because it is its own `append` record.
  - injectivity conclusion:
    - yes, genuine injectivity holds for distinct `(kind, value)` sequences because each record begins with a 1-byte tag and a fixed-width 8-byte little-endian length, so the stream parses uniquely left-to-right.
- `DigestTranscript` challenge framing:
  - `challenge(label)` first appends:
    - `0x04 || len_le64(label) || label`
  - then forks transcript state with:
    - live transcript gets naked trailing byte `0x05` (`Continued`)
    - finalized challenge clone gets naked trailing byte `0x06` (`Challenged`)
  - source: [`crypto/transcript/src/lib.rs:125-134`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:125)
  - these sentinel bytes are not ambiguous with a normal appended member because normal members always include an additional 8-byte length after their tag; the sentinel appears only after a fully framed challenge-label record.
- `MerlinTranscript` framing is operation-delimited and injective under the wrapper's reserved-label rule.
  - wrapper `domain_separate(label)` calls `append_message(b"dom-sep", label)` at [`crypto/transcript/src/merlin.rs:35-37`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/merlin.rs:35)
  - wrapper `append_message` forbids user label `b"dom-sep"` via assertion at [`crypto/transcript/src/merlin.rs:39-45`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/merlin.rs:39)
  - underlying Merlin `append_message(label, message)` does:
    - `meta_ad(label, false)`
    - `meta_ad(LE32(message.len()), true)`
    - `ad(message, false)`
    - source: [`merlin-3.0.0/src/transcript.rs:96-100`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/merlin-3.0.0/src/transcript.rs:96)
  - effective framing for a single appended message is:
    - metadata record `label || len_u32_le(message)` (same meta-AD frame, via `more = true`)
    - followed by an AD frame of exactly `message.len()` bytes
  - label is not prefixed by its own numeric length field in Merlin, but it is still unambiguous because:
    - the metadata frame boundary is explicit,
    - the final 4 bytes of the metadata frame are fixed-width little-endian message length,
    - the following AD frame consumes exactly that many bytes.
  - injectivity conclusion:
    - yes, for both `new/domain_separate` and `append_message`, no distinct `(label, value)` sequences were found that serialize to the same transcript operation stream under Merlin's framing.

### Load-bearing injectivity verdict

- The length/framing injectivity property holds.
- No prior transcript-dependent clearance needs to be reopened.
  - W-05 remains closed.
  - the ciphersuite `hash_to_F` transposition clearance remains closed.
  - the `SignCompleted` framing clearance remains closed.

### Challenge derivation result

- `DigestTranscript::challenge` is bound to the entire prior transcript state plus the challenge label.
  - it first appends the framed challenge label, then clones the digest state, then diverges live vs finalized states with `Continued` / `Challenged` sentinel bytes at [`crypto/transcript/src/lib.rs:125-134`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:125)
  - fix mechanism / security effect:
    - the challenge output is `Digest(prefix || framed_challenge_label || 0x06)`
    - the live transcript continues as `Digest(prefix || framed_challenge_label || 0x05 || ...)`
    - this explicitly prevents challenge re-use and prevents a length-extension style collision between "challenge then continue" and "continue without challenge".
- `MerlinTranscript::challenge` is bound to the entire prior Merlin/STROBE state plus the challenge label and requested output length.
  - wrapper calls `challenge_bytes(label, &mut [0; 64])` at [`crypto/transcript/src/merlin.rs:47-50`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/merlin.rs:47)
  - underlying Merlin does:
    - `meta_ad(label, false)`
    - `meta_ad(LE32(dest.len()), true)`
    - `prf(dest, false)`
    - source: [`merlin-3.0.0/src/transcript.rs:175-179`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/merlin-3.0.0/src/transcript.rs:175)
  - this both domain-separates the challenge label and advances transcript state through the PRF operation.
- `rng_seed` for both transcript implementations is the first 32 bytes of `challenge(label)`:
  - `DigestTranscript`: [`crypto/transcript/src/lib.rs:136-140`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:136)
  - `MerlinTranscript`: [`crypto/transcript/src/merlin.rs:53-56`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/merlin.rs:53)
  - this matches the trait's documented allowance for `rng_seed(label)` to conflict with `challenge(label)` at the same transcript point.

### `SecureDigest` bound result

- Current code:
  - [`crypto/transcript/src/lib.rs:83-91`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:83)
  - blanket impl requires only `OutputSize >= 32 bytes`
- CS-3.9.1 re-verification:
  - fixed mechanism holds for the original bug class: the bound now genuinely enforces a minimum output size of 32 bytes at compile time via typenum comparison (`IsGreaterOrEqual<U32>` plus `NonZero` witness).
- Qualification:
  - `SecureDigest` does **not** guarantee output length is a multiple of 8.
  - this matters because `crypto/schnorr/src/aggregate.rs:24-26` still comments "This should be guaranteed thanks to SecureDigest" right after `debug_assert_eq!(bytes.len() % 8, 0);`
  - see Candidate Notes below.

### CS-3.9.3 re-verification

- Current fix code:
  - wrapper `domain_separate` uses reserved label `b"dom-sep"` at [`crypto/transcript/src/merlin.rs:35-37`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/merlin.rs:35)
  - wrapper `append_message` rejects user label `b"dom-sep"` with a hard assertion at [`crypto/transcript/src/merlin.rs:39-43`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/merlin.rs:39)
- Fix mechanism:
  - the prior conflict class was "domain separation uses the same Merlin label as ordinary appended messages".
  - the wrapper now reserves that label globally, so a caller cannot encode an ordinary message that aliases a domain-separation event.
- Re-verification result:
  - the conflict is fixed as implemented.
  - `MerlinTranscript::new(name)` also uses Merlin's own `append_message(b"dom-sep", name)` internally at [`merlin-3.0.0/src/transcript.rs:82-86`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/merlin-3.0.0/src/transcript.rs:82); the wrapper's reserved-label assertion prevents user `append_message` calls from colliding with either constructor naming or explicit `domain_separate`.

### Collision / transcript-confusion attempt

- `DigestTranscript`:
  - attempted ambiguities:
    - `new(b"12"); challenge(b"c")` vs `new(b"1"); challenge(b"2c")`
    - `append_message(b"a", b"bc")` vs `append_message(b"ab", b"c")`
    - `domain_separate(b"x")` vs `append_message(b"x", b"")`
  - result:
    - all remain distinct because name/domain/label/value/challenge occupy different member tags and every payload is length-framed with `u64_le`.
- `MerlinTranscript`:
  - attempted ambiguities:
    - `domain_separate(b"x")` vs `append_message(b"dom-sep", b"x")`
    - `new(b"ab")` vs `new(b"a"); domain_separate(b"b")`
    - `append_message(b"a", b"bc")` vs `append_message(b"ab", b"c")`
  - result:
    - first class is blocked by the reserved-label assertion
    - remaining classes stay distinct because each append is a separate STROBE operation sequence with fixed `LE32(message_len)` framing in metadata plus an AD frame of exact length
    - no current unframed boundary was found that makes two distinct append sequences yield the same challenge output

### Panic-surface result

- `DigestTranscript`:
  - no attacker-byte decode panic confirmed
  - only theoretical invariant unwrap is `u64::try_from(value.len()).unwrap()` at [`crypto/transcript/src/lib.rs:101-102`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:101); on current 64-bit targets this is not a practical public-API panic surface
- `MerlinTranscript`:
  - reserved-label panic:
    - [`crypto/transcript/src/merlin.rs:40-43`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/merlin.rs:40) panics if caller uses label `b"dom-sep"`
  - oversized-message panic inherited from Merlin:
    - Merlin encodes lengths as `u32` with assertion `x <= u32::MAX` at [`merlin-3.0.0/src/transcript.rs:14-21`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/merlin-3.0.0/src/transcript.rs:14)
    - wrapper `append_message` passes arbitrary `message.as_ref()` through, so a >4 GiB message can panic inside Merlin length encoding

### Candidate notes

- Candidate 1:
  - `SecureDigest` does not enforce `challenge().len() % 8 == 0`, yet `schnorr::aggregate::weight()` assumes it
  - loci:
    - [`crypto/transcript/src/lib.rs:83-91`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/lib.rs:83)
    - [`crypto/schnorr/src/aggregate.rs:22-26`](C:/Users/MADAKI/Documents/serai/crypto/schnorr/src/aggregate.rs:22)
    - slice/word load depending on that assumption at [`crypto/schnorr/src/aggregate.rs:54`](C:/Users/MADAKI/Documents/serai/crypto/schnorr/src/aggregate.rs:54)
  - current production impact appears qualified because existing digest choices in audited production paths are 64-byte outputs, but the generic contract mismatch is real.
- Candidate 2:
  - `MerlinTranscript::append_message` panics on reserved label `b"dom-sep"`
  - locus:
    - [`crypto/transcript/src/merlin.rs:40-43`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/merlin.rs:40)
- Candidate 3:
  - Merlin inherited public panic on message lengths > `u32::MAX`
  - loci:
    - [`crypto/transcript/src/merlin.rs:44`](C:/Users/MADAKI/Documents/serai/crypto/transcript/src/merlin.rs:44)
    - [`merlin-3.0.0/src/transcript.rs:14-21`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/merlin-3.0.0/src/transcript.rs:14)
    - [`merlin-3.0.0/src/transcript.rs:97`](C:/Users/MADAKI/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/merlin-3.0.0/src/transcript.rs:97)

### Verification

- `cargo test -p flexible-transcript --all-features` passed locally

### Totals status

- Totals unchanged after Session 33: 18 logged / 17 active / 2 High / 0 Critical / 1 Medium candidate / 7 Low candidates / 2 confirmed panic-DoS findings.

### Next target

- Next queue:
  - F-16 dedicated `message-queue` chunk
  - then post-audit chain-mapping

- **Session 33 (2026-05-31):** `crypto/transcript` Chunk 1 complete. Re-verified both transcript implementations directly. `DigestTranscript` framing is fully self-delimiting as `kind_u8 || len_u64_le || value`, and `append_message` is concretely `Label-record || Value-record`, so label and value are both framed and the encoding is injective. `MerlinTranscript` remains injective through Merlin's STROBE operation framing plus fixed `LE32(message_len)` metadata, with the prior `dom-sep` alias bug prevented by reserving that label in the wrapper. No prior W-05 / `hash_to_F` / `SignCompleted` clearances need reopening. One qualified generic contract gap was noted: `SecureDigest` enforces `>= 32` bytes but not a multiple of 8, while `schnorr::aggregate::weight()` still assumes both. Totals unchanged; next step is the F-16 dedicated `message-queue` chunk, then chain-mapping.

## Session 34 Checkpoint (message-queue F-16 dedicated chunk)

### Crate Status Snapshot (Session 34)

| Crate Path | Crate Name | Status | Note |
|---|---|---|---|
| crypto/transcript | flexible-transcript | COMPLETE - CLOSED CLEAN | framing injectivity confirmed; 3 informational non-findings only |
| message-queue | message-queue | Chunk complete | F-16 reachability / trust-boundary facts established |
| all scoped crates | audit scope | COMPLETE | all scoped crates are now audited and closed; next phase is chain-mapping |

### flexible-transcript closure note

- `flexible-transcript` is now CLOSED clean.
- Final non-reportable notes from Session 33:
  - `SecureDigest` enforces `>= 32` bytes but not `len % 8 == 0`; this is a latent generic-contract mismatch against `crypto/schnorr/src/aggregate.rs::weight()`, yet no audited production digest violates the assumption.
  - `MerlinTranscript` reserves `b"dom-sep"` and panics on that static label; this is a wrapper guard, not a reportable vulnerability.
  - Merlin inherited `u32::MAX` / 4 GiB message-length panic remains purely informational in this context.

### F-16 exact receiver result

- `message-queue/src/client.rs:120-206` (`MessageQueue::next`) deserializes a `QueuedMessage`, checks only the coarse sender role, then returns it:
  - if `self.service` is a processor, it asserts `msg.from == Service::Coordinator`
  - else it asserts `matches!(msg.from, Service::Processor(_))`
  - immediately after that it contains the exact TODO at `message-queue/src/client.rs:202`:
    - `// TODO: Verify the sender's signature`
- No sender-signature verification happens in `next()`.
- The signature is not "checked but ignored" and it is not deferred to a later helper in `message-queue`, `processor`, or `coordinator`.
- `QueuedMessage.sig` is returned by the server and deserialized by the client, but the client never parses it back into `SchnorrSignature`, never recomputes `message_challenge`, and never calls `verify`.

### Sender-side signing and server-side queue verification

- Outgoing `Queue` requests do carry a Schnorr signature:
  - `message-queue/src/client.rs:82-118` signs `message_challenge(metadata.from, self.pub_key, metadata.to, &metadata.intent, &msg, nonce_pub)` with `self.priv_key` and sends `MessageQueueRequest::Queue { meta, msg, sig }`.
- `self.pub_key` is derived from the env-configured private key in `MessageQueue::new` at `message-queue/src/client.rs:31-44`.
- The server verifies queued messages before storing them:
  - `message-queue/src/main.rs:60-110` loads `from = KEYS[meta.from]` from the in-process service-key registry and asserts `sig.verify(from, message_challenge(...))`.
- ACKs are also signed and server-verified:
  - client signing at `message-queue/src/client.rs:208-236`
  - server verification at `message-queue/src/main.rs:132-148`
- Accordingly, a caller who only has network access to the queue port cannot forge a valid `Queue` or `Ack` request without a service private key.

### Trust boundary / transport / reachability facts

- Transport is raw TCP:
  - clients connect with `TcpStream::connect(&self.url)` in `message-queue/src/client.rs:109`, `:130`, and `:227`
  - server binds `TcpListener::bind("0.0.0.0:2287")` in `message-queue/src/main.rs:230`
- Connection authentication is explicitly absent:
  - `message-queue/src/main.rs:234` TODO: `Add a magic value with a key at the start of the connection to make this authed`
- `Next` is intentionally unauthenticated on the server side:
  - `message-queue/src/main.rs:112-125` comment states this RPC is "not authenticated"
- The README explicitly frames this as an intranet-trust service:
  - `message-queue/README.md:10` says error cases are treated as unreachable "given its intranet status"
- Deployment wiring uses separate services/containers over the network, not a local socket:
  - coordinator gets `MESSAGE_QUEUE_RPC=serai-<network>-message-queue` in `orchestration/src/coordinator.rs:35-42`
  - processors get `MESSAGE_QUEUE_RPC=serai-<network>-message-queue` in `orchestration/src/processor.rs:55-65`
  - the message-queue Dockerfile exposes port `2287` in `orchestration/src/message_queue.rs:38-40`
- Test harnesses further confirm network reachability by publishing port `2287` externally in tests:
  - `tests/message-queue/src/lib.rs:59`
  - host-port lookup at `tests/message-queue/src/lib.rs:81-82`
- Reachability conclusion:
  - this is not a Unix socket / same-process / localhost-only trust boundary
  - it is an intra-service TCP service intended for a trusted network segment
  - the precise missing-verification exploit requires a malicious/compromised message-queue server or an active network attacker able to spoof/tamper with the unauthenticated `Next` response path
  - mere ability to connect to port `2287` is not enough to enqueue forged messages, because `Queue` ingress is signature-checked against trusted service keys

### Why F-16 is real despite signed queue ingress

- The authenticity guarantee on queued messages currently lives at the queue server, not at the receiving client.
- Because `Next` is unauthenticated and `next()` ignores `QueuedMessage.sig`, the receiver trusts:
  - that it connected to the real queue server
  - that the response was not modified in transit
  - that the server/DB was not tampered with
- If any of those assumptions break, the recipient accepts attacker-chosen message bytes so long as `msg.from` has the expected role (`Coordinator` for processors, `Processor(_)` for the coordinator).

### Downstream impact map

- Processor consumers trust `MessageQueue::next()` output directly:
  - `processor/src/coordinator.rs:28-38` deserializes `QueuedMessage.msg` into `CoordinatorMessage` with no signature check
  - `processor/src/main.rs:609-639` accepts `coordinator.recv()` output as ordered input
  - `processor/src/main.rs:225-239` immediately dispatches forged `CoordinatorMessage::KeyGen` / `Sign` into local keygen/signer state machines
  - `processor/src/main.rs:242-320` immediately dispatches forged coordinator-control messages into cosigner / batch signer / slash-report signer flows
  - `processor/src/main.rs:323-470` immediately acts on forged substrate instructions, including key activation and block-ack handling
- Coordinator consumers also trust `MessageQueue::next()` output directly:
  - `coordinator/src/processors.rs:31-45` deserializes `QueuedMessage.msg` into `ProcessorMessage` with no signature check
  - `coordinator/src/main.rs:815-833` treats `processors.recv(network)` as trusted processor input
- Concrete coordinator-side immediate-action sinks without message-queue signature revalidation:
  - forged `ProcessorMessage::Coordinator::CosignedBlock` is forwarded to the local cosign channel and rebroadcast to P2P immediately at `coordinator/src/main.rs:253-267`
  - forged `ProcessorMessage::Substrate::SignedBatch` is persisted and immediately used to build/publish `SeraiInInstructions::execute_batch(batch.clone())` at `coordinator/src/main.rs:342-374`
  - forged `ProcessorMessage::Coordinator::SignedSlashReport` is immediately wrapped into a slash-report extrinsic publish loop at `coordinator/src/main.rs:271-319`
  - forged processor keygen/sign/share messages are transformed into Tributary transactions and provided to the Tributary at `coordinator/src/main.rs:430-760`

### Independent re-validation that does exist downstream

- Some high-impact paths do independently fail closed later:
  - processor sign-completion claims still go through `network.confirm_completion(...)` in `processor/src/signer.rs:311-353`
  - coordinator cosign handling eventually verifies the signature against on-chain validator-set keys in `coordinator/src/cosign_evaluator.rs:165-170`
  - invalid slash-report signatures can fail at Serai publication time, and the coordinator additionally checks the current slash-report key in `coordinator/src/main.rs:300-319`
- Other paths are primarily local-work / state-machine triggers first, with correctness then delegated to downstream protocol validation:
  - DKG and signing preprocess/share flows are handed into Tributary / FROST logic and may later reject malformed contents
  - signed batch publication is attempted immediately by the coordinator without local signature verification; any ultimate rejection would occur in the Serai/Substrate execution path, not at message-queue receipt time

### Severity-relevant conclusion inputs for F-16

- Missing verification is real and load-bearing:
  - the receiver never verifies `QueuedMessage.sig`
  - trust is delegated entirely to an unauthenticated TCP channel and to the queue server's integrity
- This is not equivalent to "remote attacker can forge Queue requests":
  - `Queue` and `Ack` ingress remain signature-gated by trusted service keys on the server
- The actual exploit precondition is stronger and more specific:
  - compromise / impersonation of the message-queue server, or active tampering on the coordinator<->queue / processor<->queue TCP path
- Once that precondition is met, there are real unauthenticated-message sinks that act before any downstream cryptographic revalidation, including:
  - processor task creation / signing-flow initiation (`processor/src/main.rs:225-320`)
  - coordinator P2P cosign rebroadcast (`coordinator/src/main.rs:253-267`)
  - coordinator batch publication attempts to Serai (`coordinator/src/main.rs:342-374`)
- No funds-loss path was confirmed solely from F-16 in this chunk.
- No final severity assigned in this checkpoint.

### Totals status

- Totals unchanged after Session 34:
  - 18 logged / 17 active
  - 2 High / 0 Critical / 1 Medium candidate / 7 Low candidates
  - 2 confirmed panic-DoS findings
- All scoped crates are now audited and closed.

### Next target

- Next phase:
  - post-audit chain-mapping
  - include the outstanding Low-vs-Medium refinement for `FINDING-18` / `FINDING-19`

- **Session 34 (2026-06-02):** `message-queue` dedicated F-16 chunk complete. Confirmed that `MessageQueue::next()` never verifies `QueuedMessage.sig`; it only checks the broad sender role and then returns the message. Outbound `Queue` / `Ack` requests are signed by the sender and verified by the queue server against env-loaded service keys, so simple network access to port `2287` is not enough to enqueue forged messages. The actual trust gap is on the receive side: `Next` is an intentionally unauthenticated raw-TCP RPC, the server binds `0.0.0.0:2287`, and client/server connection auth is explicitly TODO. Deployment uses separate coordinator/processor/message-queue services over the network rather than a local socket. Accordingly, F-16 is a real authenticity gap if the queue server or network path can be spoofed/tampered with, but not a generic "any remote can forge Queue traffic" bug. Downstream impact is non-trivial because both processor and coordinator consume queue output as trusted control-plane input; forged messages can start local signing/keygen work, trigger coordinator P2P cosign rebroadcast, and prompt coordinator Serai batch-publication attempts before any message-queue signature revalidation. Some later sinks still fail closed through independent checks (`confirm_completion`, cosign signature verification, slash-report key verification). Totals unchanged; all scoped crates are now audited; next step is chain-mapping.

## Session 35 Checkpoint (Phase-2 verification Chunk 1 - F-18/F-19 reachability & containment)

### Status correction

- Prior "next step = chain-mapping" entry is superseded.
- Current plan is PAUSE -> Phase-2 verification.
- Chain-mapping is deferred until the severity-deciding verification pass completes.

### Scope

- Targeted only:
  - `FINDING-18`: `Validators::verify_aggregate` panic at `coordinator/tributary/src/tendermint/mod.rs:220-228` via `<Ristretto as Ciphersuite>::read_G(&mut s.as_slice()).unwrap()`
  - `FINDING-19`: threshold-weight panic at `coordinator/tributary/tendermint/src/ext.rs:273-275` via `weights.weight(*v)` -> `self.weights[&validator]`
- Goal of this chunk:
  - resolve actual ingress, fan-out, task containment, restart behavior, and process-level blast radius from source
  - leave findings counts unchanged; severity still open pending joint rating and red-team review

### A. Ingress and fan-out facts

- External aggregate-commit ingress to `verify_aggregate` exists on the block-sync path, not the live Tendermint gossip path.
- End-to-end block-sync chain:
  - libp2p req/res reception flattens both requests and responses into `Message` without origin class checks:
    - `coordinator/src/p2p.rs:697-712`
  - outer P2P dispatcher forwards any `ReqRes(Block(genesis))` to the per-tributary channel:
    - `coordinator/src/p2p.rs:1024-1044`
  - per-tributary P2P handler accepts any `ReqRes(Block(msg_genesis))`, decodes `HeartbeatBatch`, then calls `tributary.tributary.sync_block(block, bc.commit).await` for each element:
    - `coordinator/src/p2p.rs:901-1009`
  - `Tributary::sync_block_internal` SCALE-decodes the commit and immediately calls `self.network.verify_commit(block.id(), &commit)` before sending anything to the Tendermint machine:
    - `coordinator/tributary/src/lib.rs:262-298`
  - `Network::verify_commit` calls `signature_scheme().verify_aggregate(...)` and then sums validator weights:
    - `coordinator/tributary/tendermint/src/ext.rs:256-275`
  - `Validators::verify_aggregate` is the `read_G(...).unwrap()` panic site:
    - `coordinator/tributary/src/tendermint/mod.rs:200-229`
- Live-consensus/gossip path does not feed aggregate commits from the wire into `verify_aggregate`.
  - gossip `TENDERMINT_MESSAGE` bytes are decoded as `SignedMessageFor<_>` in `Tributary::handle_message` and sent to the Tendermint machine:
    - `coordinator/tributary/src/lib.rs:307-338`
  - Tendermint machine receive path checks `msg.verify_signature(&self.validators)` on each incoming signed message:
    - `coordinator/tributary/tendermint/src/lib.rs:1048-1058`
  - that uses the safe individual verifier:
    - `coordinator/tributary/tendermint/src/lib.rs:117-123`
    - `coordinator/tributary/src/tendermint/mod.rs:168-178`
  - the only aggregate commit used on the live path is locally assembled from validated precommits inside the machine and then `debug_assert!(self.network.verify_commit(...))`:
    - `coordinator/tributary/tendermint/src/lib.rs:621-637`
- Delivery model:
  - primary delivery is direct request/response, not relay
  - any connected peer can send a raw `Block(genesis)` req/res payload to a chosen victim; there is no check that the message corresponds to a prior heartbeat request
  - this follows because:
    - `RrEvent::Message` collapses both `Request` and `Response` into the same `Message` path at `coordinator/src/p2p.rs:697-712`
    - the per-tributary handler accepts `ReqRes(Block(msg_genesis))` unconditionally and never correlates it to prior state at `coordinator/src/p2p.rs:971-997`
- Connection / peer-auth facts:
  - libp2p listens publicly on `0.0.0.0:30563`:
    - `coordinator/src/p2p.rs:375-395`
  - the file explicitly says validator-only connection auth is still TODO:
    - `coordinator/src/p2p.rs:375-378`
  - therefore the attacker prerequisite is "connected peer", not "selected sync source" and not "validator"
- Relay ordering:
  - raw block-sync commit bytes are not re-gossiped or relayed by the victim after receipt
  - `ReqRes(Block(...))` branch only decodes and calls `sync_block`; there is no `broadcast` call:
    - `coordinator/src/p2p.rs:971-997`
  - tributary gossip relay occurs only for `handle_message(...) == true`, and Tendermint consensus-message branch returns `false` after queueing into the machine:
    - `coordinator/src/p2p.rs:999-1004`
    - `coordinator/tributary/src/lib.rs:328-337`
  - Tendermint machine later re-broadcasts only individual signed consensus messages, and only after local processing succeeds:
    - `coordinator/tributary/tendermint/src/lib.rs:1066-1090`

### B. Containment, blast radius, recovery

- The panicking call executes in the per-tributary P2P message-handler task, not in the Tendermint machine.
  - per-tributary handler spawn:
    - `coordinator/src/p2p.rs:900-1011`
  - panic stack for both findings is:
    - `recv.recv()` -> `ReqRes(Block)` branch -> `tributary.sync_block(...)` -> `sync_block_internal` -> `verify_commit` -> `verify_aggregate`
- That same per-tributary handler also carries the tributary's live consensus gossip and transaction gossip.
  - full handled kinds in the same `match`:
    - `KeepAlive`
    - `ReqRes(Heartbeat)`
    - `ReqRes(Block)`
    - `Gossip(Tributary)`
    - `Gossip(CosignedBlock)` unreachable
  - source:
    - `coordinator/src/p2p.rs:909-1008`
- Raw task scope is one tributary:
  - each `TributaryEvent::NewTributary` creates a dedicated `(send, recv)` channel inserted under that tributary genesis
  - then spawns one per-tributary handler bound to captured `genesis`, `spec_set`, and `tributary`
  - source:
    - `coordinator/src/p2p.rs:888-901`
- There is no `catch_unwind`, no `JoinHandle` supervision, and no respawn logic around either:
  - the per-tributary P2P handler spawn
  - the Tendermint machine spawn
  - relevant spawns:
    - `coordinator/src/p2p.rs:881-1022`
    - `coordinator/tributary/src/lib.rs:191-201`
- Actual blast radius is process-wide, not task-local, because coordinator `main()` installs a panic hook which immediately exits the process on any panic:
  - `coordinator/src/main.rs:1315-1327`
- Process-level consequence:
  - one delivered malformed commit kills the entire coordinator process for that node
  - that halts all tributaries, all processor-message handling, and the node's main-chain participation until restart
  - this overrides the earlier task-isolation-only containment claim
- Restart behavior:
  - malformed commits are not persisted locally before the panic
  - local commit persistence only happens inside blockchain add after a verified block is accepted:
    - `coordinator/tributary/src/blockchain.rs:275-308`
  - `sync_block_internal` panics before `synced_block.send(...)` and long before any commit `txn.put(...)`:
    - `coordinator/tributary/src/lib.rs:278-296`
  - on restart, active tributaries are reloaded from `ActiveTributaryDb` and P2P handlers are respawned:
    - `coordinator/src/main.rs:1015-1085`
  - therefore there is no local-DB crash loop
  - however the crash is trivially re-triggerable by the attacker after restart by sending the same crafted `Block(genesis)` again as a connected peer
  - if the node remains behind, the heartbeat sync task can also solicit new `Block` replies from peers:
    - `coordinator/src/p2p.rs:827-873`

### C. Consensus-task shielding and `add_block`

- `Commit::decode` does not validate validator-ID bytes as points; it only SCALE-decodes `end_time`, `validators: Vec<ValidatorId>`, and aggregate signature bytes:
  - `coordinator/tributary/tendermint/src/ext.rs:131-151`
- The consensus task independently re-verifies commits only on the synced-block channel:
  - `coordinator/tributary/tendermint/src/lib.rs:948-973`
- `add_block` contains `assert!(self.verify_commit(serialized_block.id(), &commit));`:
  - `coordinator/tributary/src/tendermint/mod.rs:362-389`
- `add_block` call sites are:
  - locally produced commit:
    - `coordinator/tributary/tendermint/src/lib.rs:621-637`
  - externally synced block after `sync_block_internal` already called `verify_commit`:
    - `coordinator/tributary/tendermint/src/lib.rs:948-973`
    - `coordinator/tributary/src/lib.rs:262-298`
- Result:
  - externally supplied malformed commit bytes do not first enter `add_block`; they hit `verify_commit` in `sync_block_internal`
  - the `add_block` assert is not an alternate first-hit ingress for F-18/F-19
- Safe-by-contrast individual verify path:
  - `coordinator/tributary/src/tendermint/mod.rs:168-178`
  - this rejects unknown IDs and point-decode failure with `false`, not panic

### D. Process-level panic behavior

- Release profile is unwind, not abort:
  - workspace `Cargo.toml:116-119`
- Runtime construction:
  - `#[tokio::main]` in `coordinator/src/main.rs:1315`
- Load-bearing override:
  - the custom panic hook in `coordinator/src/main.rs:1317-1326` calls `std::process::exit(1)`
- Meaning:
  - even though spawned Tokio tasks are detached, any panic in them terminates the whole coordinator process immediately in current builds

### E. Pre-verify gates on the sync path

- Between socket ingress and the `read_G(...).unwrap()` panic, the only checks are:
  - req/res kind-prefix parse:
    - `coordinator/src/p2p.rs:705-712`
  - outer-genesis dispatch into the per-tributary channel:
    - `coordinator/src/p2p.rs:1024-1034`
  - `assert_eq!(msg_genesis, genesis)` in the per-tributary handler:
    - `coordinator/src/p2p.rs:971-972`
  - `HeartbeatBatch::decode(...)`:
    - `coordinator/src/p2p.rs:973-979`
  - block `ReadWrite` decode:
    - `coordinator/src/p2p.rs:985-988`
  - commit SCALE decode plus trailing-bytes check:
    - `coordinator/tributary/src/lib.rs:279-289`
- There is no peer-authentication gate restricting this path to validators.
- There is no semantic validation of validator IDs before `verify_commit` enters `verify_aggregate`.

### F-19-specific gating

- `FINDING-19` remains chained behind `verify_aggregate` returning `true`.
- The only confirmed path in source remains the previously established F-17 identity-key collapse:
  - `verify_commit` first calls `verify_aggregate`, then sums weights:
    - `coordinator/tributary/tendermint/src/ext.rs:256-275`
  - `Validators::weight` is the indexing panic:
    - `coordinator/tributary/src/tendermint/mod.rs:232-240`
- No alternate path was found in this chunk for an unregistered validator to reach `weights.weight(...)`:
  - duplicate IDs are rejected first by `HashSet` dedup check in `verify_commit`
  - malformed/non-canonical validator bytes hit F-18 earlier in `read_G(...).unwrap()`
  - live gossip uses the safe individual verifier, not aggregate verification
  - no separate deregistration state exists on this path beyond "not present in `weights`"

### End-to-end synthesis

- Malformed-bytes-on-the-wire chain for F-18:
  - unauthenticated peer connects to public libp2p listener
  - sends `ReqRes(Block(genesis))` payload directly to victim
  - outer receive loop forwards by genesis to that tributary's per-handler task
  - per-handler decodes `HeartbeatBatch`, then calls `tributary.sync_block(...)`
  - `sync_block_internal` decodes `Commit` and calls `verify_commit`
  - `verify_commit` enters `verify_aggregate`
  - `verify_aggregate` executes `read_G(...).unwrap()` on attacker-controlled validator bytes and panics
  - coordinator panic hook exits the whole process
- Fan-out answer:
  - yes, the attacker can fan this out to many nodes by direct delivery, not by relay
  - the mechanism is "connect as a peer to each victim and send crafted `ReqRes(Block(genesis))`", with no requirement to be selected as a sync source and no validator-only auth gate
  - relay is not needed and raw commit bytes are not further re-broadcast by victims
- Consensus / blast-radius answer:
  - per delivered malformed commit, the node loses the entire coordinator process
  - that takes down all tributaries and coordinator participation on that node until restart
- Recovery answer:
  - not a local persistence crash-loop
  - restart recovers if the attacker stops
  - restart re-crashes cheaply if the attacker continues delivering the crafted block-sync message

### Totals status

- Findings-register counts unchanged after this Phase-2 verification chunk:
  - 18 logged / 17 active
  - 2 High / 0 Critical / 1 Medium candidate / 7 Low candidates
  - 2 panic-DoS findings with severity still open
- `FINDING-18` / `FINDING-19` severity remains open pending this chunk plus red-team review.

### Current next step

- Continue Phase-2 verification.
- Chain-mapping remains deferred.

## Session 36 Checkpoint (Phase-2 follow-up - processor hook + Immunefi table)

### Processor panic-hook confirmation

- `processor` installs the same global panic hook pattern as `coordinator`.
- Relevant loci:
  - `processor/src/main.rs:699` `#[tokio::main]`
  - `processor/src/main.rs:704` `std::panic::set_hook(...)`
  - `processor/src/main.rs:709` `std::process::exit(1);`
- Effect:
  - any task panic in the processor also becomes whole-process exit
  - therefore externally reachable processor panics must be treated as process-fatal during Phase-2, not as local task failures
- `message-queue` likewise has the same pattern:
  - `message-queue/src/main.rs:155`
  - `message-queue/src/main.rs:160`

### Immunefi program-page confirmation

- Verified against the live Serai Immunefi page on 2026-06-03:
  - rewards table is flat by threat level only:
    - Critical `$30,000`
    - High `$5,000`
    - Medium `$1,000`
    - Low `$250`
  - source:
    - `https://immunefi.com/bug-bounty/serai/information/`
- The Serai page does **not** add a bespoke payout row explicitly naming "total network shutdown" or otherwise auto-mapping that impact to Critical on the rewards table itself.
- The page says rewards are distributed based on the impact according to the "Impacts in Scope" table further below, and the program is under Primacy of Impact for all Blockchain/DLT severities.
- Therefore the ceiling question remains:
  - classify the sustained network-wide halt under the Immunefi impact taxonomy first
  - then map that severity to Serai's flat reward table
- This follow-up does not change the finding counts.

## Session 37 - F-25 candidate verification + focused PoC

### Scope and environment note

- Executed the focused F-25 verification chunk and PoC locally.
- `crypto/schnorr` PoC ran directly with:
  - `cargo test -p schnorr-signatures identity_public_key_verifies_if_r_equals_sg -- --nocapture`
- `serai-coordinator` tests required a DB backend feature for the binary test target:
  - without features, `coordinator/src/main.rs:1337-1346` leaves `db` undefined and test compilation fails
  - reran with `--features parity-db`
- The parity-db run also required elevated execution so Cargo could unpack `memmap2` into the global registry cache.

### C0 scaffolding-cost verdict

- Preferred real-path scanner test was cheap enough; no fallback was needed.
- Existing helpers already covered the needed surface:
  - `coordinator/src/tests/tributary/chain.rs`
    - `new_keys`
    - `new_spec`
    - `new_tributaries`
    - `run_tributaries`
    - `wait_for_tx_inclusion`
  - `coordinator/src/tests/mod.rs`
    - `MemProcessors`
    - `LocalP2p`
  - `coordinator/src/tests/tributary/dkg.rs`
    - existing `handle_new_blocks::<...>()` harness usage with no-op / panic-on-use closures
- This made the preferred path viable:
  - commit a real forged `Transaction::SignCompleted` onto a local tributary
  - call real `handle_new_blocks`
  - observe the exact `scanner.rs` `.expect("removed party was never present")` panic

### Per-link verification

- Link 1 PASS - crypto root
  - `crypto/schnorr/src/lib.rs:88-110`
  - `SchnorrSignature::batch_statements` / `verify` have no non-identity public-key guard.
  - Verification is pure multiexp on `(R, cA, -sG)`.
- Link 2 PASS - transaction framing
  - `coordinator/src/tributary/transaction.rs:596`
    - `Transaction::SignCompleted` is `TransactionKind::Unsigned`
  - `coordinator/src/tributary/transaction.rs:614-624`
    - `verify()` only checks `signature.verify(*first_signer, self.sign_completed_challenge())`
    - no validator-membership check on `first_signer`
  - `coordinator/src/tributary/transaction.rs:703-713`
    - challenge binds `plan`, `tx_hash`, `first_signer`, and signature nonce `R`
- Link 3 PASS - handler
  - `coordinator/src/tributary/handle.rs:721-740`
  - Unrecognized plan path does:
    - `self.fatal_slash(first_signer.to_bytes(), "claimed an unrecognized plan was completed");`
    - `return;`
  - No validator-membership check on `first_signer`
  - Return occurs before any `processors.send(...)`
- Link 4 PASS - slash persistence
  - `coordinator/src/tributary/scanner.rs:203-209`
    - `fatal_slash` stores raw `[u8; 32]` via `FatallySlashed::set_fatally_slashed(...)`
  - `coordinator/src/tributary/db.rs:89-95`
    - `FatalSlashes::get_as_keys` reads bytes back with `<Ristretto as Ciphersuite>::G::from_bytes(key).unwrap()`
  - `coordinator/src/tributary/db.rs:99-110`
    - `set_fatally_slashed` persists the raw key bytes and appends to `FatalSlashes`
- Link 5 PASS - panic site
  - `coordinator/src/tributary/scanner.rs:216-658`
  - After the transaction loop, `handle()` unconditionally executes:
    - `let current_fatal_slashes = FatalSlashes::get_as_keys(self.txn, genesis);`
    - the `still_present_shares` block
  - In that block it iterates all fatal slashes and calls:
    - `self.spec.i(&[], *removed).expect("removed party was never present")`
  - No filter exists for non-validator keys before the `expect`
- Link 6 PASS - `spec.i`
  - `coordinator/src/tributary/spec.rs:105-139`
  - `let original_i = all_is.get(&key)?.clone();`
  - Non-validator key returns `None`
- Link 7 PASS - inclusion and revalidation
  - `coordinator/tributary/src/mempool.rs:108-186`
    - Signed path enforces `ACCOUNT_MEMPOOL_LIMIT`
    - Unsigned path only checks duplicate hash / included-on-chain and then runs `app_tx.verify()?`
    - no per-signer cap on Unsigned
  - `coordinator/tributary/src/mempool.rs:189-219`
    - `block()` includes Unsigned transactions ahead of Signed
  - `coordinator/tributary/src/blockchain.rs:230-270`
    - `verify_block()` re-runs `block.verify::<...>()`
  - `coordinator/tributary/src/block.rs:171-267`
    - `Block::verify()` re-enters `verify_transaction(tx, genesis, ...)`
    - for `Transaction::Application(tx)`, `verify_transaction()` calls `tx.verify()?`
  - Therefore a forged `SignCompleted` accepted into the mempool is re-verified again during block verification on every validator
- Link 8 PASS - unrecoverability
  - `coordinator/src/tributary/scanner.rs:661-718`
  - `handle_new_blocks` opens a txn, runs `TributaryBlockHandler { ... }.handle().await`, and only then:
    - `LastHandledBlock::set(&mut txn, genesis, &next);`
    - `txn.commit();`
  - `coordinator/src/main.rs:1320-1325`
    - panic hook exits process on panic
  - Therefore a panic in `handle()` exits before `LastHandledBlock` advancement / commit
  - Restart re-scans the same already-finalized poisoning block

### Read-your-writes answer

- Yes; the decisive executed path is same-txn read-your-writes.
- `common/db/src/mem.rs:14-22`
  - `MemDbTxn::get` first checks in-txn deletes, then in-txn writes, then backing DB
  - this is explicit read-your-writes
- `common/db/src/parity_db.rs:10-17`
  - transaction `get` starts from DB state and then replays queued changes in-order
  - this is also explicit read-your-writes
- `common/db/src/rocks.rs:16-18`
  - transaction `get` delegates to `RocksTransaction::get(...)`
- Runtime consequence for the executed PoC:
  - the poisoning `fatal_slash(...)` write is visible to the later `FatalSlashes::get_as_keys(self.txn, genesis)` in the same `handle()` call
  - the panic fires on the poisoning block itself, not the next block

### PoC results

- PoC A PASS
  - Added `crypto/schnorr/src/tests/mod.rs:126-135`
  - Test:
    - `tests::identity_public_key_verifies_if_r_equals_sg`
  - Result:
    - `test tests::identity_public_key_verifies_if_r_equals_sg ... ok`
- PoC B PASS
  - Added coordinator test helpers / test in `coordinator/src/tests/tributary/tx.rs`
  - Identity variant:
    - built forged `Transaction::SignCompleted` with `first_signer = identity`
    - used `signature = { R = sG, s }`
    - `forged.verify().is_ok()` held
    - 64 distinct forgeries were accepted by `Tributary::add_transaction(...)`, which exercises `Blockchain::add_transaction -> Mempool::add`
  - Own-key non-validator variant:
    - built a fresh non-validator keypair
    - produced a real self-signature for `sign_completed_challenge()`
    - `verify().is_ok()` held
    - `add_transaction(...)` accepted it
  - Test result:
    - `test tests::tributary::tx::sign_completed_forgery_verify_and_add_without_unsigned_cap ... ok`
- PoC C PASS (preferred real path)
  - Added real scanner-path harness in `coordinator/src/tests/tributary/tx.rs`
  - Preferred path taken; no fallback needed
  - For both variants:
    - forged `SignCompleted` was committed to a real local tributary block
    - `handle_new_blocks::<...>()` was invoked against that committed block stream
    - execution panicked at the real scanner site:
      - `coordinator/src/tributary/scanner.rs:264:38`
      - message: `removed party was never present`
  - Test results:
    - `test tests::tributary::tx::sign_completed_identity_forgery_crash_loops_scanner - should panic ... ok`
    - `test tests::tributary::tx::sign_completed_non_validator_self_signature_crash_loops_scanner - should panic ... ok`

### Defeaters / red-team result

- No source guard was found that breaks the chain.
- In particular, no guard was found for:
  - validator-membership of `first_signer` in `Transaction::verify()`
  - validator-membership of `first_signer` in the `SignCompleted` handler arm
  - filtering non-validator fatal slashes before `spec.i(...).expect(...)`
  - suppressing unsigned `SignCompleted` accumulation via a per-signer mempool cap
- The only non-hypothesis issue encountered while executing was environmental:
  - coordinator tests must be run with a DB backend feature such as `parity-db`
  - this does not affect the verified vulnerability chain

### F-25 candidate statement

- F-25 candidate confirmed from source + focused PoC:
  - forged unsigned `SignCompleted`
  - accepted by transaction verification and unsigned mempool path
  - included in a real block and re-verified during block verification
  - unrecognized plan path persists `fatal_slash(first_signer.to_bytes(), ...)`
  - same-txn read-your-writes makes that slash visible immediately
  - unconditional post-loop `still_present_shares` executes `spec.i(&[], removed).expect("removed party was never present")`
  - panic hook exits the coordinator process before `LastHandledBlock` / txn commit
  - restart re-scans the same finalized poisoning block
- Trigger variants confirmed:
  - identity `first_signer` with forged `(R = sG, s)` signature
  - fresh non-validator key with a real self-signature
- Severity note:
  - ~~Critical-defensible on persistence / restart-proof network halt grounds~~
  - Session 38 correction: Low only under Immunefi's "undocumented panic reachable from a public API"; persistence / restart-loop impact narrative does not create a higher by-right tier
  - not solo-locked here; joint rating pending

## Session 38 - Impact-Table Correction

- Authoritative correction: the Session 37 live-scope severity recalibration was based on a truncated/off-by-one read of the Immunefi impacts table and is superseded.
- Verified impact table (Last Updated 2026-04-24):
  - Critical: Signing of unintended messages
  - Critical: Ability to forge proofs
  - Critical: Unintended/undocumented recovery of private spend keys (or shares)
  - Critical: Reportedly received funds which weren't actually received/spendable
  - High: Incorrect/incomplete crypto formula in a VERIFIER's callstack
  - Medium: Undocumented transcript collision
  - Low: Undocumented panic reachable from a public API
  - Low: Non-constant-time implementation w.r.t. secret data
  - Low: Incorrect/incomplete crypto formula in a PROVER's callstack
- There is no availability / liveness / network-shutdown / DoS tier. Whole-process exit from a panic still maps only to the Low "undocumented panic" line.
- Corrected severities overriding conflicting earlier notes:
  - F-22 = Critical (received-funds-not-spendable). Still an unverified candidate; rewardability may be reduced or denied under the pre-mainnet active-development caveat.
  - F-15 = High. Critical ceiling remains gated on a protocol-used-key identity-acceptance forge (DKG-PoP recon pending explicit GO).
  - F-17 = High. Critical denied.
  - F-25 = Low. PoC-confirmed on both triggers, but permanence / network-wide restart looping does not raise the tier.
  - F-18 / F-19 = Low. The global panic-hook containment refutation is a correct fact, but it does not raise the tier.
- Locked remap:
  - F-01 / F-04 / F-08 = Low panic.
  - F-09 / F-14 = Low panic only if the relevant precondition is undocumented; documented panic contracts are out of scope.
  - F-11 = Low prover-callstack formula issue.
  - F-07 = Out of scope.
  - F-16 = Out-of-scope lean only if all downstream signing / fund-movement sinks independently re-validate; see gate result below.
  - F-21 / F-23 / F-24 = Low panic if prod-reachable; likely one combined same-class report.
- F-09 gate quote:
  - `crypto/dkg/pedpop/src/lib.rs:615-617`: "The message should be a copy of the encrypted secret share from the accused sender to the accusing recipient. This message must have been authenticated as actually having come from the sender in question."
- F-14 gate quote:
  - `crypto/dkg/promote/src/lib.rs:119`: "Complete promotion by taking in the proofs from all other participants."
  - Implementation still only checks `proofs.len() == n - 1` plus range before `proofs.get(&i).unwrap()`, so exact participant coverage is not documented at the rustdoc/contract level.
- F-16 gate result:
  - `message-queue/src/client.rs:202`: `// TODO: Verify the sender's signature`
  - `coordinator/src/processors.rs:31-40` deserializes `ProcessorMessage` directly from `next()` output.
  - `coordinator/src/main.rs:334-373` immediately stores the received `SignedBatch` and attempts `SeraiInInstructions::execute_batch(batch.clone())` publication without local verification of `batch.signature`.
  - Independent signature verification exists only later in `substrate/in-instructions/pallet/src/lib.rs:307-317` during on-chain execution, so the "all sinks re-validate before acting" premise is false for this fund-movement publication path.
- Note on front-page totals: the top-of-file totals remain a historical Session 26 snapshot and do not include later unregistered findings such as F-22 / F-25; use this Session 38 block as authoritative.

## Session 37 provenance

- HEAD:
  - `737dbcbaa78ab817cc1c435cb2b6c5d24d1c4391`
- Branch / upstream:
  - branch: `develop`
  - upstream: `origin/develop`
- Remote:
  - `origin https://github.com/serai-dex/serai.git (fetch)`
  - `origin https://github.com/serai-dex/serai.git (push)`
- HEAD commit:
  - `737dbcbaa78ab817cc1c435cb2b6c5d24d1c4391 2026-01-10T02:38:31-05:00 fix: remove duplicate docsrs cfg_attr lines`
- Working tree status at provenance check:
  - modified:
    - `coordinator/src/tests/tributary/tx.rs`
    - `crypto/schnorr/src/tests/mod.rs`
  - untracked:
    - `agent.md`
    - `context.md`
    - `dkg_fuzz_property_test_spec.md`
    - `findings.md`
    - `jules/`
    - `understanding_report.md`
- 9 cited production files cleanliness vs HEAD:
  - `crypto/schnorr/src/lib.rs`
  - `coordinator/src/tributary/transaction.rs`
  - `coordinator/src/tributary/handle.rs`
  - `coordinator/src/tributary/scanner.rs`
  - `coordinator/src/tributary/db.rs`
  - `coordinator/src/tributary/spec.rs`
  - `coordinator/tributary/src/mempool.rs`
  - `coordinator/tributary/src/blockchain.rs`
  - `coordinator/src/main.rs`
  - result:
    - `git status --porcelain -- ...` returned empty
    - `git diff --stat HEAD -- ...` returned empty
    - all 9 cited production files were clean / unmodified vs `HEAD`
- Unexpected dirty files:
  - relative to the earlier expectation that only two PoC test files would be dirty, there were additional untracked files/directories:
    - `agent.md`
    - `context.md`
    - `dkg_fuzz_property_test_spec.md`
    - `findings.md`
    - `jules/`
    - `understanding_report.md`
- Remote tip delta:
  - `git fetch` updated `origin/develop` from `737dbcba` to `4b89cf02`
  - `git log --oneline -1 origin/develop`:
    - `4b89cf02 Remove \`.github\` actions, workflows`
  - `git rev-list --left-right --count HEAD...origin/develop`:
    - `0 1`
  - interpretation:
    - `HEAD` is `0 ahead / 1 behind` `origin/develop`
- Citation pin:
  - all F-25 line numbers / pasted bodies in the report are against local `HEAD` `737dbcbaa78ab817cc1c435cb2b6c5d24d1c4391`

## Session 39 - F-25 own-key PoC hardening

- Scope:
  - own-key variant only; no new identity-path or `crypto/schnorr` work
  - all new proof work lives in `coordinator/src/tests/tributary/tx.rs`
  - run command:
    - `cargo test -p serai-coordinator --features parity-db sign_completed_non_validator_self_signature -- --nocapture`
- Green results:
  - `sign_completed_non_validator_self_signature_verify_and_add_without_unsigned_cap ... ok`
  - `sign_completed_non_validator_self_signature_crash_loops_scanner - should panic ... ok`
  - `sign_completed_non_validator_self_signature_finalizes_and_persists ... ok`
  - `sign_completed_non_validator_self_signature_restart_rescans_persisted_poison_block ... ok`
- New parity-db helpers added in `coordinator/src/tests/tributary/tx.rs`:
  - `wait_for_parity_tx_inclusion` at line 146
  - `scan_poison_block_and_capture_panic` at line 202
- GAP 1 closed by `sign_completed_non_validator_self_signature_finalizes_and_persists` at line 325:
  - proves the forged own-key `SignCompleted` survives real mempool admission, real consensus inclusion, real block finalization, and real tributary DB persistence under `parity-db`
  - proof artifact:
    - each node's persisted reader returns the finalized block containing `TributaryTransaction::Application(tx.clone())`
    - each node's persisted reader returns a stored commit for that block
  - this replaces the earlier "follows by construction" inference that an honest proposer/finalization path would carry the tx through `build_block` / `verify_block` / `add_block`
- GAP 2 closed by `sign_completed_non_validator_self_signature_restart_rescans_persisted_poison_block` at line 360:
  - proves a fresh scanner invocation against the same persisted `parity-db` re-hits the same panic message after the poison block has already been finalized
  - proves scanner progress does not advance into the poison block:
    - after the first caught panic, `LastHandledBlock::get(...).unwrap_or(genesis)` equals the poison block parent, not the poison block
    - `reader.block_after(progress) == Some(poison_block)` still holds before the second scan
    - after the second caught panic, the recorded progress is unchanged
  - production code fact still cited, not executed in-test:
    - `coordinator/src/main.rs:1320-1325` installs a panic hook which `std::process::exit(1)`s on panic
- GAP 3 was also closed cheaply inside the restart test:
  - scanning a second node's separate persisted DB for the same finalized block/spec also panics with the same message
  - this is a node-independence proxy showing the failure is a function of shared finalized history + shared spec, not a one-node local-db artifact
- Proven vs still inferred after hardening:
  - proven:
    - own-key forged `SignCompleted` passes `verify()`
    - it enters the unsigned mempool without a per-signer cap
    - it is included, finalized, and persisted via the real production path
    - scanning the persisted poison block panics
    - scanner progress is not committed past the poison block
    - a fresh scan of the same persisted state re-hits the same panic
    - a second node with separate persisted state for the same finalized block also panics
  - still inferred / code-cited rather than directly executed in-test:
    - the production global panic hook turns that panic into whole-process exit
    - operational recovery under normal restart requires patching or state repair rather than self-healing replay
