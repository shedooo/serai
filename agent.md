# Serai Audit - agent.md

## Agent Identity

You are a cryptographic systems security auditor hunting for exploitable vulnerabilities in Rust cryptographic libraries. You reason adversarially. You are assisting a senior auditor.

---

## Current Scope

| Crate | Path | Status | Priority |
|---|---|---|---|
| frost-schnorrkel | crypto/schnorrkel | COMPLETE | DONE |
| dkg-musig | crypto/dkg/musig | COMPLETE | DONE |
| modular-frost | crypto/frost | COMPLETE | DONE |
| dkg | crypto/dkg | COMPLETE (5/5) | 4 - CRITICAL |
| dkg-pedpop | crypto/dkg/pedpop | COMPLETE (5/5) | 5 - HIGH |
| dkg-dealer | crypto/dkg/dealer | COMPLETE (5/5) | 6 - MEDIUM |
| dkg-recovery | crypto/dkg/recovery | COMPLETE (5/5) | 7 - MEDIUM |
| bitcoin-serai | networks/bitcoin | COMPLETE (5/5 - CLEAN) | 8 - HIGH |
| schnorr-signatures | crypto/schnorr | COMPLETE (Chunks 1-6 complete) | 9 - HIGH |
| remaining | various | PENDING | 9-15 |

---

## Investigation Targets - Resolved for Completed Crates

### frost-schnorrkel - ALL CLEARED
### dkg-musig - ALL CLEARED
### modular-frost - ALL CLEARED
- A: `IetfTranscript::rng_seed` -> CLEARED (unreachable)
- B: Binding factor chain -> CLEARED (correct three-level chain)
- C: Nonce safety -> CLEARED (rejection sampling + identity rejection)
- D: `hash_binding_factor == 0` (W-04) -> CLEARED (spec-compliant)
- E: Blame mechanism -> CLEARED (correct `verify_share` equations)
- F: `sign()` `view().unwrap()` -> CLEARED (pre-validation ensures safety)

---

## Completed Work Log

| Date | Crate | Chunk | Summary | Findings |
|---|---|---|---|---|
| 2026-04-23 | ALL | Phase 1 | Codebase understanding | None |
| 2026-04-23/24 | frost-schnorrkel | All 5 | Complete - clean | FINDING-01 to FINDING-05 |
| 2026-04-24 | dkg-musig | All 5 | Complete - clean | FINDING-06 |
| 2026-04-24 | modular-frost | Chunk 1 | Orientation, `rng_seed` unreachable | None |
| 2026-04-24 | modular-frost | Chunk 2 | 6 invariants, binding chain correct | None |
| 2026-04-24 | modular-frost | Chunk 3 | `sign()`, `verify()`, binding factors | None |
| 2026-04-24 | modular-frost | Chunk 4 | Cross-crate, RFC 9591 compliance | None |
| 2026-04-24 | modular-frost | Chunk 5 | Checklist - all pass | None |
| 2026-04-26 | dkg + pedpop + dealer + recovery | Chunk 1 | Orientation restored, W-01/state-machine/encryption surfaces mapped | None |
| 2026-04-26 | dkg + pedpop + dealer + recovery | Chunk 2 | Invariants reviewed; dealer zero-participant fail-open and constant-interpolation panic confirmed | FINDING-07, FINDING-08 |
| 2026-04-26 | dkg + pedpop + dealer + recovery | Chunk 3 | Function audit; blame-path participant validation panic confirmed | FINDING-09 |
| 2026-04-26 | dkg + pedpop + dealer + recovery | Chunk 4 | Cross-crate review; W-01 remains live through Bitcoin caller path but not Ethereum | None |
| 2026-04-26 | dkg + pedpop + dealer + recovery | Chunk 5 | Final checklist and verdict; batch complete | FINDING-07, FINDING-08, FINDING-09 |
| 2026-04-28 | dkg-pedpop encryption gap-fill | Q2/Q3 | Decrypt-before-verify plaintext handling and PoP ciphertext binding resolved | No new findings |
| 2026-04-28 | Jules artifact review | Post-audit triage | Reviewed `dkg-musig`, `modular-frost`, `dkg-dealer`, `multiexp`, and boundary/panic tests; separated real issues from duplicates and harness mistakes | No new findings |
| 2026-05-15 | bitcoin-serai | Chunk 1 | Crate orientation complete: tweak path, offset call sites, x/x_only panic sites, Ethereum-vs-Bitcoin mitigation gap mapped | W-01/W-02/W-03 still open |
| 2026-05-16 | bitcoin-serai | Chunk 2 | Deep function audit complete: tweak math, panic-site reachability, per-output offset analysis, and processor hardening-gap confirmation | W-01 probabilistic-only; W-02/W-03 closed for this path |
| 2026-05-16 | bitcoin-serai | Chunk 3 | Signing-path and adversarial-nonce analysis complete: HRAM inputs traced, modular-frost aggregation checks reviewed, send.rs panic unwraps classified | No new confirmed finding |
| 2026-05-16 | bitcoin-serai | Chunk 4 | RPC panic-site review, signing-message binding verification, participant-set integrity pass, and dkg-promote isolation map | No new confirmed finding |
| 2026-05-16 | bitcoin-serai | Chunk 5 | Final checklist and crate closure completed; all production files covered and watch outcomes finalized | CLEAN, 0 new findings |
| 2026-05-17 | dkg-promote | Chunk 1 | Crate orientation, DLEq surface map, serialization review, and cross-crate usage search complete | No new confirmed finding |
| 2026-05-17 | dkg-promote | Chunk 2 | Cross-session portability and `complete()` panic-surface analysis complete | New Low panic candidate in `complete()` |
| 2026-05-17 | dkg-promote + crypto/dleq | Chunk 3 | DLEq primitive audit, serialization/identity/nonce review, and crate closure complete | FINDING-14 confirmed; no new crypto break |
| 2026-05-19 | schnorr-signatures | Chunk 1 | Orientation complete: API/equation/nonce/serialization/batch paths mapped and production caller map built | High-priority downstream identity-key forgery candidate in coordinator `SignCompleted` path |
| 2026-05-26 | schnorr-signatures | Chunk 2 | SignCompleted reachability traced; first_signer attacker-controlled confirmed; processor re-validation independent; Critical denied | FINDING-15 reachability confirmed |
| 2026-05-26 | schnorr-signatures | Chunk 3 | CoordinatorMessage::Completed handler traced; confirm_completion network-backed and fail-closed; message-queue keys registry-derived and cleared | FINDING-15 severity: High confirmed |
| 2026-05-26 | schnorr-signatures | Chunk 3B | Starvation vector deep-dive; unsigned mempool flood quantified (~7,774-22,908 forged txs/block); signed tx starvation confirmed via block ordering + tail-trim; no irreversible fund impact | FINDING-15 closed as upper-end High |
| 2026-05-27 | schnorr-signatures | Chunk 4 | `aggregate.rs` reviewed end-to-end; CS-3.8.4/3.8.5 fixes re-verified; production Tendermint aggregate paths traced | No new confirmed finding |
| 2026-05-27 | schnorr-signatures | Chunk 5 | Tendermint caller traced; challenge binds `R_i` (H-17b cleared); identity validator keys admitted into verify with no rejection; `read_G` unwrap panic reachable from p2p sync | FINDING-17 (High, Critical pending), FINDING-18 (panic/DoS) |
| 2026-05-29 | schnorr-signatures | Chunk 6 | Threshold / registration / caller gates resolved; unknown-ID weight lookup confirmed panic; no signature-only commit-trust path; p2p panic confirmed task-isolated | FINDING-17 capped at High, FINDING-18 task-isolated DoS, FINDING-19 added |

---

## Active Instructions

NEXT: `crypto/ciphersuite` gap-fill - review `read_G` / canonical-point acceptance and caller assumptions after its repeated role in confirmed Schnorr / Tendermint findings
- bitcoin-serai is closed
- dkg-promote is closed

## Findings Register (Session 26 Update Applied)

- FINDING-04 severity corrected to Low (runtime confirmed)
- FINDING-07 severity corrected to Low (runtime confirmed)
- FINDING-10 added: outsider transit tampering misattributed as sender blame (Informational, dkg-pedpop)
- FINDING-11 added: fixed IV + no ephemeral scalar uniqueness check - keystream reuse on RNG failure (Low, dkg-pedpop)
- FINDING-12 added: `musig_key` accepts identity point without rejection (Informational, dkg-musig)
- FINDING-13 marked REMOVED (false positive): `multiexp()` empty-input panic claim
- FINDING-15: `SchnorrSignature::verify()` identity-key forgery (`A = identity` => `R = sG`) - HIGH CONFIRMED AND CLOSED
- FINDING-16 candidate: message-queue `client.rs:202` missing sender-signature verification (`TODO`) (Medium candidate, message-queue, flagged for dedicated chunk after remaining crates)
- FINDING-17: `SchnorrAggregate::verify` accepts identity validator keys on Tendermint commit path; same unsound-verifier class as FINDING-15, but all production callers enforce full `verify_commit` threshold checks and attacker-achievable on-chain identity registration was not proven - HIGH CONFIRMED AND CLOSED
- FINDING-18: Tendermint `verify_aggregate` unwraps signer-key decoding on commit-supplied bytes; non-canonical validator IDs can panic commit verification from p2p block-sync - CONFIRMED Low "undocumented panic"; whole-process exit via global panic hook does not raise the tier
- FINDING-19: `verify_commit` threshold check indexes `self.weights[&validator]` and panics on unknown validator IDs after successful aggregate verification - CONFIRMED Low "undocumented panic"; whole-process exit via global panic hook does not raise the tier
- Totals: 18 logged entries (`FINDING-13` removed, net 17 active)
- Immunefi-reportable Low candidates: 9 (`FINDING-01`, `FINDING-04`, `FINDING-07`, `FINDING-08`, `FINDING-09`, `FINDING-11`, `FINDING-14`, `FINDING-18`, `FINDING-19`)
- High confirmed: 2 (`FINDING-15`, `FINDING-17`)
- Critical confirmed: 0
- Medium candidates: 1 (`FINDING-16`)
- Panic findings: `FINDING-18` and `FINDING-19` are confirmed reachable from unauthenticated p2p block-sync; the global panic hook makes them whole-process exits, but they still map only to Low "undocumented panic reachable from a public API"
- Reachability vectors flagged in Chunk 1:
  - `coordinator/src/tributary/transaction.rs:596`
  - `coordinator/src/tributary/transaction.rs:616`
  - `coordinator/src/tributary/handle.rs:721`

## Session 21 Checkpoint (schnorr-signatures Chunk 1)

- File map complete for `crypto/schnorr`:
  - `Cargo.toml`
  - `README.md`
  - `src/lib.rs`
  - `src/aggregate.rs` (feature-gated)
  - tests under `src/tests/*`
- Public API surface:
  - `SchnorrSignature::{read, write, serialize, sign, batch_statements, verify, batch_verify}`
  - `aggregate` module (feature `aggregate`) with:
    - `SchnorrAggregate::{read, write, serialize, Rs, verify}`
    - `SchnorrAggregator::{new, aggregate, complete}`
- Scheme characteristics:
  - challenge-agnostic Schnorr primitive over `Ciphersuite::G: PrimeGroup`
  - signing equation: `s = r + c*x`; verification relation: `R + cA - sG == 0`
  - no built-in nonce derivation or RNG in `SchnorrSignature::sign`; callers fully control nonce selection
- Serialization/validation:
  - signature parsing uses `C::read_G` and `C::read_F` from `crypto/ciphersuite`
  - these enforce canonical encoding but do not reject identity points
  - no `PublicKey` type in this crate; key handling is generic `C::G`
- Batch verification:
  - `SchnorrSignature::batch_verify` queues linear statements into `multiexp::BatchVerifier`
  - batch weights are per-statement (`u_1 = 1`, later `u_i` random non-zero)
  - half-aggregation (`aggregate`) uses deterministic transcript-derived weights per ePrint 2021/350
- Cross-crate usage map:
  - direct dependency users: `crypto/dkg/pedpop`, `crypto/frost`, `crypto/schnorrkel`, `coordinator`, `coordinator/tributary`
  - production-path direct calls include signing, verification, serialization, batch verify, and aggregate verify
  - `processor` and `networks/*` do not directly import `schnorr` crate APIs; they primarily consume through `frost` abstractions
- High-priority candidate:
  - identity-key verification collapse is real (`A = identity` permits trivial forgeries with `R = sG`)
  - downstream coordinator `Transaction::SignCompleted` path verifies against attacker-controlled `first_signer` on an unsigned transaction, making this a high-priority follow-up candidate

## Session 22 Checkpoint (schnorr-signatures Chunk 2)

- Findings register corrections applied to tracking context:
  - `FINDING-10`, `FINDING-11`, `FINDING-12` added
  - `FINDING-13` removed as false positive
  - `FINDING-04` and `FINDING-07` upgraded to Low
  - `FINDING-15` assigned for Schnorr identity-key verification collapse
- Reachability vectors confirmed:
  - `coordinator/src/tributary/transaction.rs:596`
  - `coordinator/src/tributary/transaction.rs:616`
  - `coordinator/src/tributary/handle.rs:721`
- Production reachability result:
  - `Transaction::SignCompleted` is classified as `Unsigned`
  - `first_signer` is parsed from transaction bytes and fed into `signature.verify(*first_signer, ...)`
  - no non-identity guard exists on `first_signer` before verification
  - therefore identity-key signature forgery is accepted at transaction-content verification for this path
- Downstream consequence trace:
  - accepted `SignCompleted` triggers `CoordinatorMessage::Completed` broadcast
  - processor signer path (`claimed_eventuality_completion`) revalidates completion claim against network before completion state transition
  - no direct evidence in this chunk of bypassing threshold signature correctness or direct funds movement from this bug alone

## Session 22 Checkpoint (schnorr-signatures Chunk 3)

- Task A (Critical gate) conclusion:
  - `CoordinatorMessage::Completed` is handled in `processor/src/signer.rs` and routed to `claimed_eventuality_completion`.
  - Completion state transition requires `self.network.confirm_completion(&eventuality, claim).await` returning `Ok(Some(completion))`.
  - State transition (`self.complete`) removes active signing state (`signable`, `attempt`, `preprocessing`, `signing`) only after successful `confirm_completion`.
  - Processor does not blindly trust coordinator forwarding; it performs independent network-specific confirmation:
    - Bitcoin: fetch tx from RPC using eventuality txid.
    - Monero: fetch claimed txid and require `eventuality.matches`.
    - Ethereum: validate claim cryptographically via `SignedRouterCommand::new(...)`.
  - Result: Critical escalation denied; FINDING-15 remains High.

- Task B (message-queue verify sites) conclusion:
  - `message-queue/src/main.rs:69` verifies with `from = KEYS[meta.from]`.
  - `message-queue/src/main.rs:135` verifies with `to_key = KEYS[to]`.
  - `KEYS` is a local service-key registry initialized from environment at startup (`read_key`, `register_service`), not a peer-supplied key field from request payload.
  - No explicit identity check is present, but no attacker-injectable public-key origin was confirmed; this is not a second independent F-15 exploit path under current trust model.

## Session 23 Checkpoint (F-15 Extended Impact Exploration)

- Chain/liveness pressure feasibility:
  - Unsigned tx path has no `ACCOUNT_MEMPOOL_LIMIT` equivalent.
  - Mempool ordering is unsigned first, then signed.
  - Block construction trims oversize blocks by popping from the end, which preferentially drops signed txs.
  - Therefore forged `SignCompleted` variants (hash-distinct unsigned txs) can plausibly starve signed signing-protocol traffic under sustained flood.

- Backdoor/escalation feasibility after first-layer pass:
  - No path found to switch forged identity signer to a real signer before processor checks.
  - `first_signer` is not part of `CoordinatorMessage::Completed`; only plan/id + tx claim is forwarded.
  - Processor completion transition still requires independent `confirm_completion`.
  - Reforging identity to pass second layer without real completion remains denied.

- Transport/trust caveat discovered:
  - `message-queue` client `next()` has TODO sender-signature verification and currently only sanity-checks sender role enum.
  - This is a separate messaging trust surface to harden; it does not by itself convert F-15 into a direct funds-loss path in current traced logic.

## Session 24 Checkpoint (F-15 Starvation Deep-Dive)

- Unsigned admission path:
  - `TransactionKind::Unsigned` mempool path applies only hash dedup + `app_tx.verify()`.
  - No nonce, no signer-membership gate, no per-sender cap for unsigned app txs.
  - Signed-only cap (`ACCOUNT_MEMPOOL_LIMIT`) does not apply to unsigned.

- Flood ingress and attacker prerequisites:
  - Tributary gossip txs are accepted via `handle_message` and passed to `add_transaction(false, ...)`.
  - P2P stack currently lacks explicit validator-only connection auth in code comments/TODO and listens publicly.
  - Practical attacker needs tributary gossip ingress reachability; no stake/fee gate exists on unsigned tx submission path.

- Dedup and capacity:
  - Dedup is hash-only (`unsigned_in_chain || mempool.contains(hash)`), no semantic dedup on SignCompleted tuple.
  - Forged `SignCompleted` serialized size in block context is `131 + tx_hash_len` bytes.
  - Approximate forged capacity per 3,001,000-byte block:
    - ~22,908 tx at 131 bytes
    - ~18,411 tx at 163 bytes (32-byte claim)
    - ~7,774 tx at 386 bytes

- Starvation mechanics and impact:
  - Block builder order is Provided -> Unsigned -> Signed.
  - Oversize trimming pops from tail, preferentially evicting signed txs under unsigned saturation.
  - Sustained forged-unsigned flood can starve signed signing-protocol tx inclusion and degrade liveness.
  - Reattempt timing base is ~5 minutes (50 blocks at 6s target), but pre-threshold starvation can defer reattempt scheduling.
  - No irreversible fund-loss path confirmed from this vector alone; impact remains high-confidence liveness/DoS pressure.

## Session 25 Checkpoint (schnorr-signatures Chunk 4 - aggregate.rs)

- Scope and file map:
  - audited only `crypto/schnorr/src/aggregate.rs` plus production callers in `coordinator/tributary/src/tendermint/mod.rs` and `coordinator/tributary/tendermint/src/lib.rs`.
- CS-3.8.4 verification:
  - git history confirms removal of key+challenge double-transcripting (`8b7e7b1a`), now challenge-only transcript input for weight derivation.
  - current aggregation/verification both derive per-signer `z_i` from the same ordered transcript of challenges.
- CS-3.8.5 verification:
  - git history confirms DST/domain-separated transcript migration and follow-up scalar-derivation correction (`53067179`, `97374a3e`).
  - current `weight()` consumes `ceil((NUM_BITS + 128)/8)` transcript-challenge bytes (with continuation labels), leaving only negligible residual bias.
- Equation/soundness:
  - implemented check is `sum(z_i * R_i) + sum(z_i * c_i * A_i) - s*G == 0` (ePrint 2021/350 half-aggregation form), with no independent forgery found under non-identity keys and secure challenges.
- Identity and reachability:
  - aggregate verify path does not reject identity keys; if an identity validator key is admitted upstream, the same F-15 collapse family applies to that signer term.
  - Tendermint aggregate verify receives keys from commit signer IDs (`commit.validators`) and uses them directly for aggregate challenge/key pairing.
- Caller hardening note:
  - Tendermint `verify_aggregate` currently uses `read_G(...).unwrap()` on signer IDs and does not pre-check signer membership before later weight lookup; this is a reachable robustness surface from untrusted commit inputs and should be handled in follow-up, but was not promoted to a new confirmed finding in this chunk.

## Session 25 Checkpoint (schnorr-signatures Chunk 5 - Tendermint caller)

- Scope and live caller map:
  - traced `coordinator/tributary/src/tendermint/mod.rs:51`, `:181`, `:201`, `coordinator/tributary/tendermint/src/ext.rs:256`, `coordinator/src/p2p.rs:198`, `:971`, `coordinator/src/tributary/spec.rs:56`, and `crypto/ciphersuite/src/lib.rs:91`.
- H-17b resolution:
  - cleared; Tendermint per-signer challenge binds nonce `R_i`.
  - transcript order is `genesis`, `key`, `nonce`, `message`, with `nonce = R_i.to_bytes()` and commit-path `message = end_time || block_id`.
  - height and round are not directly hashed, yet `block_id` is the unique committed block hash on this path; no standalone exploit was confirmed from that omission.
- FINDING-17:
  - confirmed High, Critical pending.
  - `verify_aggregate` receives signer IDs from `commit.validators` raw `[u8; 32]` bytes, computes challenges over those bytes and aggregate nonces, then decodes the same bytes with `read_G` instead of performing a trusted validator-set point lookup by index.
  - identity-point rejection is absent on the checked path: `read_G` / `from_bytes` accept canonical identity, `TributarySpec::new` accepts any `read_G` output, and `Validators::new` rejects zero weight only.
  - trivial all-identity aggregate forgery is not yet Critical-confirmed because `verify_commit` also enforces `sum(weights.weight(v)) >= threshold`.
- FINDING-18:
  - confirmed reachable panic / DoS.
  - `verify_aggregate` executes `read_G(signer).unwrap()` on attacker-supplied `commit.validators` bytes.
  - path is `HeartbeatBatch -> sync_block -> verify_commit -> verify_aggregate`; `Commit::decode` performs no point validation, duplicate check works on raw bytes only, and `signers.len() == Rs().len()` is attacker-satisfiable.
- Open Chunk 6 escalation questions:
  - what does `Validators::weight()` return for unknown validator IDs?
  - can the substrate validator-sets pallet register the identity point as a validator key?
  - does any caller trust a commit after `verify_aggregate` without also enforcing the threshold check in `verify_commit`?

## Session 26 Checkpoint (schnorr-signatures Chunk 6 - escalation resolution)

- `Validators::weight()` / threshold:
  - `weight()` is `self.weights[&validator]`; unknown IDs panic instead of returning `0`.
  - threshold remains the default `((self.total_weight() * 2) / 3) + 1`.
- FINDING-17 resolution:
  - Critical denied; capped at High.
  - only production `verify_aggregate` caller is `verify_commit`, and every production commit-trust path uses full `verify_commit`.
  - no repo path proved attacker-achievable on-chain identity registration with nonzero weight, even though explicit identity rejection is absent in the pallet/account deserialization code reviewed.
- FINDING-18 resolution:
  - panic path remains reachable from `HeartbeatBatch -> sync_block -> verify_commit -> verify_aggregate`.
  - no `catch_unwind` is present, but the path runs inside the spawned per-tributary P2P handler task and no repo `panic = "abort"` / Tokio unhandled-panic override was found; impact is task-isolated DoS rather than process abort.
- FINDING-19:
  - new DoS finding on the threshold-defense path.
  - identity-key forged aggregates can pass `verify_aggregate` and then panic in `verify_commit` when threshold evaluation calls `weights.weight(v)` for unknown IDs.
- Crate outcome:
  - `crypto/schnorr` is closed after Chunk 6.
  - next target is `message-queue` for FINDING-16 validation.

## Session 27 Checkpoint (schnorr-signatures Chunk 7 - closure checklist)

- Task-context resolution:
  - `sync_block_internal` is on the spawned per-tributary P2P handler task from `coordinator/src/p2p.rs`.
  - `TendermintMachine::run` is the core consensus driver task spawned once per tributary from `coordinator/tributary/src/lib.rs`.
  - `add_block` executes on that core consensus task, but no public-input route was confirmed into its `assert!(verify_commit(...))`; the attacker-reachable `FINDING-18` / `FINDING-19` path still dies in the spawned sync task first.
- Source completeness:
  - production files under `crypto/schnorr/src` are only `lib.rs` and feature-gated `aggregate.rs`; both are fully audited.
  - remaining files `tests/mod.rs` and `tests/rfc8032.rs` are test-only.
  - no unaudited public production function remains.
- Batch-verifier loose end:
  - cleared; `SchnorrSignature::batch_verify` only queues statements into `BatchVerifier` with verifier-supplied RNG, so the prior fresh-random-scalar assumption stands.
- Chain seed:
  - confirmed `FINDING-17 -> FINDING-19` two-step chain seed: identity-slot aggregate forgery passes `verify_aggregate`, then the threshold check panics on `weights[&validator]`.
  - capped at Low / DoS under current task isolation.
- Final crate verdict:
  - `crypto/schnorr` remains closed.
  - next target advances to `crypto/ciphersuite` gap-fill because `read_G` / identity acceptance is now a repeated upstream assumption across confirmed findings.

## Session 18 Checkpoint (dkg-promote Chunk 1)

- File map complete for `crypto/dkg/promote`:
  - `Cargo.toml`
  - `README.md`
  - `src/lib.rs`
  - `src/tests.rs`
- Public API surface in `src/lib.rs`:
  - `pub use dkg::*`
  - `PromotionError`
  - `GeneratorProof::{write, read, serialize}`
  - `GeneratorPromotion::{promote, complete}`
- Dependencies:
  - local `crypto/dleq` crate with `serialize` feature enabled
  - local `crypto/transcript` (`flexible-transcript`)
  - local `crypto/ciphersuite`
  - local `crypto/dkg`
  - no external DLEq primitive is used
- DLEq scheme identified:
  - `dkg-promote` uses `dleq::DLEqProof<C::G>` from `crypto/dleq/src/lib.rs`
  - same-group, same-scalar-across-two-generators Sigma/Schnorr-style DLEq
  - not the `cross_group` experimental module
  - verification reconstructs each nonce as `R = sG - cA` and accepts iff recomputed challenge equals stored `c`
- `GeneratorPromotion::promote()` conclusions:
  - witness is the participant's actual `base.original_secret_share()`
  - promoted share is `C2::generator() * original_secret_share`
  - outer transcript commits `group_key`, then `participant`
  - inner DLEq transcript domain-separates with `dleq` and then appends ordered `(generator, nonce, point)` triples for `C1::generator()` and `C2::generator()`
  - prover does not control `c` after transcript finalization beyond choosing Schnorr nonce `r`; no malleability beyond standard Schnorr transcript assumptions was identified
- `GeneratorPromotion::complete()` conclusions:
  - verifies each peer proof against `[old_generator_share_i, new_generator_share_i]`
  - proof is participant-bound because the outer transcript includes participant index and verifier also selects the expected old verification share for that same participant
  - replaying proof A from participant 1 as participant 2 fails unless both participants somehow share the same original verification share and participant transcript binding, which is not expected
  - group key is transcript-bound; threshold and full participant roster are not
  - invalid peer proof returns `PromotionError::InvalidProof(i)` and aborts before returning any constructed state
- Serialization surface:
  - `GeneratorProof::read` delegates point validation to `C::read_G`
  - `DLEqProof::read` validates both scalars with `PrimeField::from_repr`
  - malformed encodings return `io::Error`; no attacker-controlled deserialization panic found
  - `serialize()` methods use `Vec<u8>` writes with `.unwrap()`, which are invariant-only
- Cross-crate usage:
  - no production callers of `GeneratorPromotion` or `GeneratorProof` found outside `crypto/dkg/promote`
  - no network / processor / coordinator / networks integration path found
  - current in-repo use appears limited to crate-local tests
- Chunk 1 verdict:
  - no Critical/High finding confirmed
  - no obvious witness leakage: proof is standard Schnorr-style zero-knowledge under discrete log assumptions
  - residual question for later chunks is portability across distinct higher-level sessions sharing the same original group key and participant index, since transcript scope is intentionally narrow

## Session 19 Checkpoint (dkg-promote Chunk 2)

- `original_group_key` trace:
  - `ThresholdKeys::original_group_key()` returns `self.core.group_key`
  - `ThresholdCore` stores:
    - `group_key: C::G`
    - `verification_shares: HashMap<Participant, C::G>`
    - `secret_share: Zeroizing<C::F>`
  - `ThresholdKeys::new()` computes `group_key` as the interpolation of the first `t` verification shares:
    `sum_{i in 1..=t}(verification_shares[i] * interpolation_factor(i, first_t_set))`
  - no session transcript, nonce, commitment round, timestamp, or external context is mixed into this value
- Cross-session portability resolution:
  - `original_verification_share(i)` returns `self.core.verification_shares[&i]`, i.e. the participant's untweaked public verification share under `C1::generator()`
  - `complete()` verifies peer proof `i` against:
    - transcript `(domain = DKG Generator Promotion v0.2, original_group_key, participant i)`
    - generators `[C1::generator(), C2::generator()]`
    - points `[self.base.original_verification_share(i), proof.share]`
  - therefore, a proof from another session with the same group key and same participant index but a different secret/public share fails verification:
    the verifier reconstructs nonce as `R1 = s*G1 - c*A1_target`; if `A1_target != A1_source`, transcript changes and recomputed `c` no longer matches
  - replay only succeeds if the target session has the same participant verification share too, meaning effectively the same promoted key material for that participant
- Session uniqueness / deployment context:
  - repo search found no higher-level protocol passing extra session context into `promote()` / `complete()`
  - no production callers of `GeneratorPromotion` / `GeneratorProof` outside crate-local tests
  - no README/TODO/processor/coordinator integration path indicating active or imminent production use
  - same group key across sessions is not protocol-prevented by these crates; it is negligible under fresh random generation, but caller-engineerable if the same underlying secret/polynomial constant term or full verification-share set is intentionally reused
- `Participant::new()` contract:
  - `Participant` is a non-zero `u16`
  - `Participant::new(0) -> None`, otherwise `Some(Participant(i))`
  - `Participant::new(i).unwrap()` for `i in 1..=params.n()` is safe because `ThresholdParams::new()` forbids `n == 0`
  - no mismatch there; the unwrap on `Participant::new` is not the panic issue
- `complete()` panic surface:
  - validation only checks:
    - `proofs.len() == n - 1`
    - every provided key satisfies `participant <= n`
  - it does not check that the map keys equal the exact expected set `{1..=n} \\ {self}`
  - because `Participant` cannot be zero, the remaining gap is inclusion of `self` in the map while omitting some other participant
  - example for self `= 1`, `n = 3`: map keys `{1, 2}` passes length/range checks, but iteration later expects proof for `3`; `proofs.get(&3).unwrap()` panics
  - this is reachable from the public `complete()` API with adversarial input
  - severity assessment: Low, same family as FINDING-09 public panic on insufficient set validation
- `IncorrectAmountOfParticipants` field bug:
  - `PromotionError` defines fields `{ t, n, amount }`
  - `complete()` populates `t: params.n()` instead of `params.t()`
  - this is reporting-only and does not weaken enforcement
  - `complete()` intentionally requires `n-of-n` proofs, not `t-of-n`, because it reconstructs the full promoted verification-share map for all participants
- Chunk 2 verdict:
  - no Critical/High replay issue confirmed
  - cross-session replay is denied unless the participant verification share matches exactly
  - new likely finding candidate: undocumented panic in `GeneratorPromotion::complete()` from incomplete peer-set validation before `proofs.get(&i).unwrap()`

## Session 20 Checkpoint (dkg-promote Chunk 3 + crypto/dleq)

- `crypto/dleq` file map complete:
  - `Cargo.toml`
  - `README.md`
  - `src/lib.rs`
  - `src/cross_group/{mod,bits,aos,scalar,schnorr}.rs` under `experimental`
  - `src/tests/mod.rs`
  - `src/tests/cross_group/*`
- Public non-experimental API from `src/lib.rs`:
  - `DLEqError`
  - `DLEqProof::{prove, verify}`
  - `DLEqProof::{write, read, serialize}` behind `serialize`
  - `MultiDLEqProof::{prove, verify}` on `std`
  - `MultiDLEqProof::{write, read, serialize}` behind `serialize`
  - `challenge()` is `pub(crate)`, not public
  - `cross_group` is feature-gated by `experimental` and out of audit scope
- Features:
  - `std`
  - `serialize = ["std"]`
  - `secure_capacity_difference`
  - `experimental = ["std", "thiserror", "multiexp"]`
  - `serialize` gates `std::io`-based read/write/serialize methods for `DLEqProof` and `MultiDLEqProof`
- Fiat-Shamir transcript:
  - prover:
    - caller-supplied outer transcript context remains in place
    - `domain_separate("dleq")`
    - for each generator in order:
      - append `"generator"` = `Gj`
      - append `"nonce"` = `r * Gj`
      - append `"point"` = `x * Gj`
    - derive one joint challenge `c`
  - verifier:
    - same outer transcript context
    - `domain_separate("dleq")`
    - for each `(Gj, Aj)` in order:
      - compute `Rj = s * Gj - c * Aj`
      - append `"generator"` = `Gj`
      - append `"nonce"` = `Rj`
      - append `"point"` = `Aj`
    - derive one joint challenge `c'`
    - accept iff `c' == c`
  - no early squeeze before all commitments are appended
  - no label collision found with `dkg-promote`: outer transcript uses `group_key` and `participant`, then `dleq` domain-separation scopes the inner proof statements
- Nonce handling:
  - `r` is sampled fresh per call with `G::Scalar::random(rng)`
  - stored as `Zeroizing<G::Scalar>`
  - no internal nonce cache/state exists, so reuse requires caller RNG failure or repeated RNG stream
  - `r` is zeroized on drop via `Zeroizing`
  - no deterministic internal path to reuse `r` was found
- Identity / subgroup handling:
  - `DLEqProof::verify()` performs no explicit identity rejection for generators, public points, or reconstructed nonces
  - `Ciphersuite::G` is bounded by `PrimeGroup`, so typed elements are in the prime-order group; classic small-subgroup injection is therefore a type/deserialization concern rather than something `verify()` re-checks
  - `Ciphersuite::read_G()` validates canonical point encoding but does not generically reject identity
  - thus identity exclusion is caller policy, not enforced by `crypto/dleq`
- Scalar read/write safety:
  - `DLEqProof::write()` writes canonical `c` and `s` encodings with `to_repr()`
  - `DLEqProof::read()` uses helper `read_scalar`, which calls `PrimeField::from_repr`
  - invalid scalar encodings return `io::Error("invalid scalar")`
  - no malformed-scalar panic found
  - `c = 0` or `s = 0` are allowed scalars and may verify if they satisfy the transcript equation; this is normal Schnorr behavior, not a deserialization bug
- Degenerate statement note:
  - if caller supplies `Aj = identity`, verifier computes `Rj = s * Gj`
  - a prover can then build a valid proof for witness `x = 0` without any secret knowledge beyond the trivial zero scalar
  - this is not a break of DLEq soundness; it reflects that the identity point legitimately has discrete log `0`
  - in protocols requiring non-identity keys, caller must reject identity separately
- `serialize().unwrap()` status:
  - `DLEqProof::serialize()` and `GeneratorProof::serialize()` write into `Vec<u8>`
  - `Vec<u8>: Write` returns `Ok` absent allocation failure/panic outside `io::Result`
  - these unwraps are invariant-only, not attacker-reachable through normal API inputs
- Crate verdicts:
  - `dkg-promote`: CLOSED with FINDING-14 confirmed Low
  - `crypto/dleq` same-group surface: CLOSED, no Critical/High finding confirmed

## Session 17 Checkpoint (W-05 resolution)

- W-05 scope (`frost-schnorrkel` + non-test callers) is closed.
- frost-schnorrkel transcript/HRAM message construction is length-safe:
  - `sign_share` builds `m = LE32(context_len) || context || msg`.
  - HRAM decodes `context_len` from first 4 bytes and splits deterministically into `(context, msg)`.
- Merlin framing is length-delimited by implementation:
  - `append_message` appends label + LE32(message_len) metadata before message bytes.
  - This prevents concatenation ambiguity such as `ABC|DEF` vs `AB|CDEF`.
- Empty context/message behavior:
  - empty context is allowed and still unambiguous (explicit length prefix / labeled commits);
  - zero-length message is signable and context-bound by transcript labels + context handling.
- No field in this path was found to be variable-length and unframed.
- Caller survey (non-test):
  - `processor/src/batch_signer.rs`
  - `processor/src/cosigner.rs`
  - `processor/src/slash_report_signer.rs`
  - `coordinator/src/tributary/signing_protocol.rs`
  all instantiate `Schnorrkel::new(b"substrate")` with fixed context.
- No W-05 finding logged.

## Session 16 Checkpoint (bitcoin-serai Chunk 5)

- Final checklist PASS for crate scope:
  - Replay protection via per-input Taproot sighash under `Prevouts::All`.
  - Nonce binding via rho transcript commitments and message hash.
  - Deterministic script/key derivation with no external script-content injection in tweak path.
  - Fee/output manipulation resistance within caller-provided input assumptions.
  - Threshold/signer-set enforcement through modular-frost set and map validation.
  - Constant-time handling preserved for secret-scalar signing paths; vartime operations restricted to public verification/aggregation paths.
- Watch status for bitcoin-serai:
  - W-01 CLOSED (informational probabilistic edge, no practical attacker trigger).
  - W-02 CLOSED.
  - W-03 CLOSED.
  - W-05 OPEN but out-of-scope for this crate.
  - W-06 CLOSED.
- Production file coverage confirmed:
  - `src/lib.rs`, `src/crypto.rs`, `src/rpc.rs`, `src/wallet/mod.rs`, `src/wallet/send.rs`.
- dkg-promote handoff confirmed:
  - path `crypto/dkg/promote`,
  - API: `GeneratorPromotion::{promote, complete}`, `GeneratorProof` serialization,
  - primitive: DLEq proofs over old/new generators,
  - recommended next audit target as standalone crypto crate.
- No new findings from bitcoin-serai. Crate closed CLEAN.

## Session 28 Checkpoint (crypto/ciphersuite Chunk 1)

- Status snapshot update:
  - `crypto/schnorr`: CLOSED (Chunks 1-7). Carry-forward item only: F-18/F-19 Low-vs-Medium severity refinement for the per-tributary handler panic path.
  - `crypto/ciphersuite`: IN PROGRESS (Chunk 1 complete).
- File map complete for `crypto/ciphersuite`:
  - `src/lib.rs`
  - `src/lib.md`
- Public API surface of the crate:
  - `pub trait Ciphersuite`
    - associated types: `F`, `G`, `H`
    - associated const: `ID`
    - methods: `generator`, `hash_to_F`
    - provided helpers: `random_nonzero_F`, `read_F`, `read_G`
  - public re-export: `group`
- Concrete ciphersuite impls traced for this audit chunk:
  - `crypto/dalek-ff-group/src/ciphersuite.rs`: `Ristretto`, `Ed25519`
  - `crypto/ciphersuite/kp256/src/lib.rs`: `Secp256k1`, `P256`
  - `crypto/ed448/src/ciphersuite.rs`: `Ed448`
- Core identity-handling result:
  - `Ciphersuite::read_G` only guarantees successful decode plus canonical round-trip equality (`point.to_bytes() == encoding`).
  - It does **not** reject identity points.
  - This is the crate-level precondition gap underlying F-15 and F-17.
- Concrete decode behavior traced:
  - `Ristretto` / `Ed25519`: `from_bytes` accepts decompressible points and rejects malformed encodings; `Ed25519` rejects non-torsion-free points, but identity is torsion-free and therefore accepted.
  - `Secp256k1` / `P256`: fixed-width all-zero compressed `Repr` is mapped to SEC1 identity, and `Coordinates::Identity` is accepted. Invalid field elements and off-curve points are rejected. No separate torsion screen is needed on these cofactor-1 curves.
  - `Ed448`: accepts canonical `y`, successful `recover_x`, non-negative-zero, torsion-free points; identity remains accepted.
  - `read_F` rejects non-canonical scalars in every traced impl (`from_repr(...).is_some()` / `< ORDER` / canonical limb checks).
- `hash_to_F` conclusions:
  - `Ristretto` / `Ed25519`: `Sha512(dst || data)` -> 64-byte wide reduction via `Scalar::from_hash`; unbiased, but DST separation is only naive concatenation.
  - `Secp256k1` / `P256`: proper `expand_message_xmd` hashing to 48-byte material then reduction mod scalar order; comments explicitly target `k = 128` bias bound and handle oversize DSTs.
  - `Ed448`: `Shake256_114(dst || data)` -> 114-byte wide reduction; plenty of entropy, but DST separation is again naive concatenation.
- Panic-surface result:
  - No new public attacker-byte panic confirmed in `crypto/ciphersuite/src/lib.rs`; its read APIs return `io::Result`.
  - Impl-side unwraps found in `kp256` and `ed448` hash-to-scalar paths are invariant-only after fixed-width expansion/modular reduction.
- Blast radius map for audited code:
  - `crypto/schnorr` directly consumes `C::read_G` / `C::read_F` for signature and aggregate parsing, so it inherits the identity-admission defect directly.
  - `coordinator/tributary` Tendermint commit verification decodes validator IDs with `Ristretto::read_G(...).unwrap()`, inheriting both identity admission and malformed-point panic behavior.
  - `coordinator` `Transaction::SignCompleted` uses `Ristretto::read_G` for `first_signer` and `Ristretto::hash_to_F` for its challenge.
  - `crypto/frost` explicitly hardens the ciphersuite point reader by rejecting identity after decode, so the defect is masked on that path.
  - `crypto/dkg`, `crypto/dkg/pedpop`, `crypto/dkg/promote`, `networks/bitcoin`, `networks/ethereum`, `message-queue`, and processor glue all rely on the crate for canonical parsing and/or scalar derivation; non-identity remains a caller obligation unless locally enforced.
- Candidate notes:
  - Candidate: `crypto/ciphersuite/src/lib.rs:91-100` admits canonical identity points through `read_G`, with no stronger trait-level guarantee to callers.
  - Candidate note: `crypto/dalek-ff-group/src/ciphersuite.rs:31-33` and `crypto/ed448/src/ciphersuite.rs:71-73` still implement `hash_to_F` as naive `dst || data`, matching the trait warning.
- Totals unchanged:
  - 18 logged entries / 17 active
  - 2 High / 0 Critical / 1 Medium candidate / 7 Low candidates
  - 2 confirmed task-isolated panic/DoS findings (`FINDING-18`, `FINDING-19`)

## Session 29 Checkpoint (crypto/ciphersuite Chunk 2)

- Status snapshot update:
  - `crypto/schnorr`: CLOSED (Chunks 1-7). Carry-forward item remains the F-18/F-19 Low-vs-Medium severity refinement only.
  - `crypto/ciphersuite`: CLOSED (Chunks 1-2).
- `hash_to_F` enumeration result:
  - All production call sites use fixed literal or fixed-const DST inputs.
  - Ristretto concrete sites:
    - `crypto/frost/src/curve/mod.rs:90` raw `b"rho"` -> effective `b"FROST-RISTRETTO255-SHA512-v1rho"` when instantiated on substrate Ristretto FROST paths.
    - `crypto/frost/src/curve/mod.rs:112` raw `b"nonce"` -> effective `b"FROST-RISTRETTO255-SHA512-v1nonce"`.
    - `crypto/dkg/musig/src/lib.rs:83` `b"dkg-musig"` via generic keygen on `C = Ristretto`.
    - `crypto/dkg/pedpop/src/lib.rs:93` `b"DKG-PedPoP-proof_of_knowledge-0"` via generic keygen on `C = Ristretto`.
    - `crypto/dkg/pedpop/src/encryption.rs:323` `b"DKG-encryption-proof_of_possession"` via generic keygen on `C = Ristretto`.
    - `message-queue/src/messages.rs:55` `b"message_challenge"`.
    - `message-queue/src/messages.rs:74` `b"ack_challenge"`.
    - `coordinator/src/tributary/transaction.rs:711` `b"SignCompleted signature"`.
  - Ed25519 concrete sites:
    - `crypto/frost/src/curve/mod.rs:90` raw `b"rho"` -> effective `b"FROST-ED25519-SHA512-v1rho"` when instantiated on Monero FROST paths.
    - `crypto/frost/src/curve/mod.rs:112` raw `b"nonce"` -> effective `b"FROST-ED25519-SHA512-v1nonce"`.
    - `crypto/dkg/musig/src/lib.rs:83` `b"dkg-musig"` via generic keygen on `C = Ed25519`.
    - `crypto/dkg/pedpop/src/lib.rs:93` `b"DKG-PedPoP-proof_of_knowledge-0"` via generic keygen on `C = Ed25519`.
    - `crypto/dkg/pedpop/src/encryption.rs:323` `b"DKG-encryption-proof_of_possession"` via generic keygen on `C = Ed25519`.
    - `processor/src/additional_key.rs:10` `b"Serai DEX Additional Key"`, currently used as `additional_key::<Monero>(0)`.
  - Secp256k1 sites:
    - `crypto/frost/src/curve/mod.rs:90` raw `b"rho"` -> effective `b"FROST-secp256k1-SHA256-v1rho"`.
    - `crypto/frost/src/curve/mod.rs:112` raw `b"nonce"` -> effective `b"FROST-secp256k1-SHA256-v1nonce"`.
    - `crypto/frost/src/curve/kp256.rs:30` raw `b"chal"` -> effective `b"FROST-secp256k1-SHA256-v1chal"`.
    - `crypto/dkg/musig/src/lib.rs:83` `b"dkg-musig"` via generic keygen on `C = Secp256k1`.
    - `crypto/dkg/pedpop/src/lib.rs:93` `b"DKG-PedPoP-proof_of_knowledge-0"` via generic keygen on `C = Secp256k1`.
    - `crypto/dkg/pedpop/src/encryption.rs:323` `b"DKG-encryption-proof_of_possession"` via generic keygen on `C = Secp256k1`.
    - `processor/src/networks/bitcoin.rs:335/339/343` fixed const `KEY_DST = b"Serai Bitcoin Output Offset"` with data `b"branch"`, `b"change"`, `b"forward"`.
  - P256:
    - only generic FROST `rho` / `nonce` / `chal` sites exist in production code; no concrete production P256 instantiation was found.
  - Ed448:
    - no concrete production `hash_to_F` instantiation was found.
- Naive-concatenation transposition verdict:
  - Checked all Ristretto production DSTs: `FROST-RISTRETTO255-SHA512-v1rho`, `FROST-RISTRETTO255-SHA512-v1nonce`, `dkg-musig`, `DKG-PedPoP-proof_of_knowledge-0`, `DKG-encryption-proof_of_possession`, `message_challenge`, `ack_challenge`, `SignCompleted signature`.
  - Checked all Ed25519 production DSTs: `FROST-ED25519-SHA512-v1rho`, `FROST-ED25519-SHA512-v1nonce`, `dkg-musig`, `DKG-PedPoP-proof_of_knowledge-0`, `DKG-encryption-proof_of_possession`, `Serai DEX Additional Key`.
  - No prefix relationship exists among distinct same-curve production DSTs.
  - Transcript-backed data inputs are length-framed by `DigestTranscript::append` (`member tag || u64_le_len || value`), so attacker-controlled variable fields do not sit adjacent to the DST in an ambiguous raw concatenation.
  - The only non-transcript Ed25519 production site is `additional_key::<Monero>(0)`, whose data shape is fixed (`b"Monero" || 8-byte LE 0`).
  - Result: the documented `dst || data` transposition hazard is inert in the current production call graph; no new candidate finding promoted.
- `SignCompleted` specific check:
  - `coordinator/src/tributary/transaction.rs:711` uses constant DST `b"SignCompleted signature"`.
  - Data is `transcript.challenge(b"challenge")`.
  - The transcript prior to the challenge appends `plan`, variable-length `tx_hash`, `signer`, and `nonce` with explicit length framing, so this F-15-reachable path does not create a practical DST/data transposition surface.
- kp256 check:
  - No further DST-transposition review needed for `Secp256k1` / `P256`.
  - `crypto/ciphersuite/kp256/src/lib.rs` uses `ExpandMsgXmd::<Sha256>::expand_message(&[msg], &[dst], 48)`, so DST is parameterized separately from the message rather than naively concatenated.
- FROST identity-guard resolution:
  - Guard code is `crypto/frost/src/curve/mod.rs:123-130`.
  - It wraps `Ciphersuite::read_G(reader)` and rejects `res.is_identity()`.
  - In production, the guard is used by `crypto/frost/src/nonce.rs:33-35` when reading `GeneratorCommitments`, i.e. nonce/preprocess commitment points, not verification shares or verification public keys.
  - Input context is untrusted multi-party wire input: preprocess messages are broadcast to participants over an authenticated channel and parsed by `read_preprocess` -> `Commitments::read`.
  - The guard rejects identity only; malformed/non-canonical points are already rejected by `Ciphersuite::read_G`.
  - Conclusion: different object and different bug class than F-15/F-17. It is not a same-object/same-context parallel, though it does show local convention that zero nonce commitments are invalid.
- `read_G` identity-admission note:
  - Keep as documented root context for F-15/F-17 only.
  - Do not add a new findings-register entry at the ciphersuite crate level.
- Crate verdict:
  - `crypto/ciphersuite` closes cleanly.
- Totals unchanged:
  - 18 logged / 17 active
  - 2 High / 0 Critical / 1 Medium candidate / 7 Low candidates
  - 2 confirmed panic/DoS findings
- Next target:
  - `multiexp` partial -> `dalek-ff-group` -> `flexible-transcript`

## Session 30 Checkpoint (crypto/multiexp completion pass)

- Status snapshot update:
  - `crypto/multiexp`: CLOSED (completion pass). F-13 stays cleared as a false positive because empty input routes through `Algorithm::Null`.
- File / API map:
  - production files: `src/lib.rs`, `src/straus.rs`, `src/pippenger.rs`, `src/batch.rs`
  - tests under `src/tests/*`
  - public surface:
    - `multiexp`
    - `multiexp_vartime`
    - `BatchVerifier`
    - `BatchVerifier::{new, queue, verify, verify_vartime, blame_vartime, verify_with_vartime_blame, verify_vartime_with_vartime_blame}`
  - internal selection logic:
    - `enum Algorithm { Null, Single, Straus(u8), Pippenger(u8) }`
    - `fn algorithm(len: usize) -> Algorithm`
- Correctness conclusions:
  - `multiexp` and `multiexp_vartime` wrappers explicitly handle:
    - empty input -> group identity
    - single element -> direct scalar multiplication
    - len >= 2 -> Straus or Pippenger based on thresholds
  - release thresholds:
    - `0 Null`, `1 Single`, `2..=9 Straus(3)`, `10..=19 Straus(4)`, `20..=49 Straus(5)`,
      `50..=99 Pippenger(4)`, `100..=124 Pippenger(5)`, `125..=274 Pippenger(6)`,
      `275..=399 Pippenger(7)`, `>= 400 Pippenger(8)`
  - debug thresholds:
    - `0 Null`, `1 Single`, `2..=9 Straus(3)`, `10..=79 Straus(4)`, `80..=99 Straus(5)`,
      `100..=124 Pippenger(4)`, `125..=274 Pippenger(5)`, `275..=474 Pippenger(6)`,
      `475..=749 Pippenger(7)`, `>= 750 Pippenger(8)`
  - `prep_bits` decomposes scalars into base-`2^window` digits.
  - `straus` evaluates high-to-low windows with per-point precomputed tables `[0, P, 2P, ...]`.
  - `pippenger` buckets points by digit and uses the standard descending running-sum trick.
  - Both algorithms therefore compute the same `Σ scalar_i * point_i`, differing only in cost model.
  - Internal indexing on `groupings[0]` / `bits[0]` is safe because public wrappers never dispatch empty/singleton slices into those private functions.
  - Cross-check present in-tree: `crypto/dkg/musig/src/lib.rs:153` debug-asserts `musig_key_vartime(...) == group_key` computed through constant-time multiexp.
- `is_identity` / returned-element conclusion:
  - `multiexp_vartime` returns a real accumulated group element.
  - `straus_vartime` uses `Option<G>` only as an internal identity accumulator optimization and returns `res.unwrap_or_else(G::identity)`.
  - `pippenger_vartime` uses `Option<G>` buckets only as an optimization for zero buckets and returns the real accumulated `res`.
  - No early boolean/identity short-circuit exists inside either vartime algorithm.
- BatchVerifier soundness:
  - `queue` draws its per-statement randomizer from caller-supplied `R: RngCore + CryptoRng`.
  - First statement uses weight `1`.
  - Every later statement samples a non-zero random scalar in a rejection loop.
  - Every statement pair is transformed as `(scalar * u, point)` before storage.
  - This preserves the standard random-linear-combination security argument: later malicious statements cannot deterministically cancel honest earlier statements without guessing the random weight relation.
- vartime discipline:
  - Clean/public vartime call sites:
    - `crypto/schnorr/src/lib.rs:109`
    - `crypto/schnorr/src/aggregate.rs:145`
    - `crypto/frost/src/nonce.rs:208`
    - `crypto/dkg/musig/src/lib.rs:107` (explicitly documented public-key only)
    - `crypto/dkg/pedpop/src/lib.rs:518`
    - `crypto/dleq/src/cross_group/mod.rs:421`
  - Candidate misuse path:
    - direct: `crypto/dkg/pedpop/src/lib.rs:597-603` verifies a decrypted secret share with `multiexp_vartime`
    - indirect: `crypto/dkg/pedpop/src/lib.rs:487-494` queues secret-share verification statements, then `verify_with_vartime_blame()` can fall back into `crypto/multiexp/src/batch.rs:108-123` `blame_vartime()`, which also runs `multiexp_vartime` on those statements
  - Result: no intrinsic multiexp-local misuse, but there is a downstream vartime-discipline candidate in `dkg-pedpop`.
- Panic-surface result:
  - No new public panic confirmed in `crypto/multiexp`.
  - `verify_with_vartime_blame` / `verify_vartime_with_vartime_blame` contain `unwrap()` on `blame_vartime()`, but no route was confirmed to make verification fail while blame resolution returns `None`.
- Crate verdict:
  - `crypto/multiexp` closes cleanly for arithmetic correctness and batch-soundness.
  - The only noteworthy issue is the downstream `dkg-pedpop` vartime candidate, not a multiexp-core finding.
- Totals unchanged:
  - 18 logged / 17 active
  - 2 High / 0 Critical / 1 Medium candidate / 7 Low candidates
  - 2 confirmed panic/DoS findings
- Next target:
  - `dalek-ff-group` -> `flexible-transcript`

## Session 15 Checkpoint (bitcoin-serai Chunk 4)

- `rpc.rs` panic sites classified:
  - line 115 JSON serialization unwrap is invariant-only in this construction path.
  - line 118 request-builder unwrap is caller-configuration failure surface (invalid URL), not remote attacker response data.
- Transaction signing integrity confirmed:
  - `TransactionSignMachine::sign` rejects non-empty external `msg` and always derives message bytes internally.
  - message passed into FROST per input is `taproot_key_spend_signature_hash(..., TapSighashType::Default).as_ref()`.
  - no substitution/truncation path found before `AlgorithmSignMachine::sign`.
- Sighash type is fixed (`TapSighashType::Default`) and not counterparty-influenced in this path.
- Participant-set integrity confirmed:
  - construction uses local `ThresholdKeys`;
  - modular-frost `sign` validates included set size/range/duplicates and map exactness;
  - mismatched key material for a prevout is rejected in `SignableTransaction::multisig` via script equality gate.
- Remaining `wallet/mod.rs` unwrap/expect calls are only invariant unwraps (`scale` with ±1 and `Vec<u8>` serialization write).
- `dkg-promote` isolation confirmed for bitcoin path (no imports/usages in `networks/bitcoin` nor `processor/src/networks/bitcoin.rs`).
- `dkg-promote` purpose mapped:
  standalone crate for promoting `dkg::ThresholdKeys` across ciphersuites/generators with DLEq proofs (`GeneratorPromotion::{promote, complete}` and `GeneratorProof`).
- No new findings added in Chunk 4.

## Session 14 Checkpoint (bitcoin-serai Chunk 3)

- Bitcoin HRAM path confirmed:
  - `x(R)` consumes aggregated session nonce sum (`Rs[0][0]`) from modular-frost.
  - `x(A)` consumes signing-view group key (`params.group_key()`), i.e. post-tweak/post-offset state for transaction signing.
- Modular-frost path traced:
  `BindingFactor::nonces` computes `Rs` -> `AlgorithmSignMachine::sign` passes `Rs` to `Algorithm::sign_share` -> Schnorr HRAM call.
- No explicit aggregate-`R` identity guard is present before algorithm `sign_share`/`verify`.
- Individual commitment deserialization rejects identity points (`Curve::read_G`), but aggregate sum can still theoretically hit identity.
- Adversarial "force `R == identity`" by malicious co-signer was not confirmed as practical in this code path because bound nonces depend on transcript-derived binding factors over the full commitment set.
- `x(R)` panic exists if identity reaches HRAM (`expect(\"point at infinity\")`), but no practical deterministic trigger was proven in this chunk.
- `x(A)` maps to existing W-01-style key-identity axis; no independent attacker-controlled path was confirmed.
- send.rs panic sites classified:
  - `122`, `125`, `199`, `389` are invariant/caller-shape assumptions, not new attacker-controlled network surfaces under current flow.
- No new finding added from Chunk 3.

## Session 13 Checkpoint (bitcoin-serai Chunk 2)

- `wallet::tweak_keys()` full body reviewed; `scale(...).expect(...)` maps to `dkg::ThresholdKeys::scale` which only returns `None` on scalar `0`. Selected scalar is always `±1`, so this panic path is not attacker-reachable under current invariants.
- W-01 math in bitcoin-serai tweak path is key-only and deterministic:
  identity requires `k + H_tap(k) == 0 (mod n)` for group-key discrete log `k`; this is a negligible-probability edge and not script-tree-content controlled in this function.
- Processor Ethereum hardening loop confirmed (`while PublicKey::new(keys.group_key()).is_none() { offset += 1 }`), while processor Bitcoin delegates directly to `bitcoin_serai::wallet::tweak_keys()` and then asserts via `scanner(keys.group_key())`.
- Bitcoin processor assertion is panic-based (`Scanner::new(...).unwrap()`), not an error-returning validity gate.
- Per-output offset path in `wallet/send.rs` derives offsets from scanned `ReceivedOutput` entries and revalidates each input with `p2tr_script_buf(offset.group_key())?`; mismatches return `None`, not panic.
- This per-output path is not a second independent W-01 panic trigger under normal scanner-derived offsets.
- `x()` / `x_only()` panic surface mapping:
  - `wallet::p2tr_script_buf` parity-checks before `x_only`;
  - `crypto::Hram` calls `x(R)` and `x(A)`, so invalid infinity states can still panic if they reach the algorithm layer.
- `crypto.rs:148` unwrap and `wallet/mod.rs:146` unwrap are invariant/non-adversarial unwraps in current code paths.
- W-02 closure confirmed for this scope:
  no schnorrkel types or conversions in `networks/bitcoin` nor `processor/src/networks/bitcoin.rs`.
- W-03 closure confirmed for this scope:
  no direct `Algorithm::verify()` usage outside FROST state-machine flow in bitcoin-serai.

## Session 12 Checkpoint (bitcoin-serai Chunk 1)

- File map complete for `networks/bitcoin` (core modules + test surface).
- `tweak_keys()` located in `networks/bitcoin/src/wallet/mod.rs`; computation is:
  `TapTweakHash::hash(xonly(group_key))` -> scalar reduction -> `ThresholdKeys::offset(tweak)` -> conditional negation scale.
- Tweak source in this crate path is key-only hash input; no direct script-tree content input from counterparties.
- `ThresholdKeys::offset()` call sites in `networks/bitcoin` are:
  `wallet/mod.rs` (Taproot tweak) and `wallet/send.rs` (per-input spend offsets).
- `x()` / `x_only()` panic surfaces confirmed in `networks/bitcoin/src/crypto.rs`.
- Direct `Algorithm::verify()` calls outside FROST state machine were not found in `networks/bitcoin`.
- Ethereum mitigation loop reconfirmed in production glue (`processor/src/networks/ethereum.rs`):
  while `PublicKey::new(keys.group_key()).is_none()` { offset by one }.
- Bitcoin processor glue does not apply the equivalent post-tweak validity loop (`processor/src/networks/bitcoin.rs` delegates to `bitcoin_serai::wallet::tweak_keys()` and proceeds).
- No dependency edge from `networks/bitcoin` to `dkg-promote` was found in local manifests/source references.
- Preliminary W-01 disposition: still open, with unresolved reachability/frequency analysis for practical DoS conditions.

## Session 11 Checkpoint

- Jules artifacts were test additions and manifest tweaks, not a standalone written vulnerability report.
- `dkg-musig`:
  MuSig key-ordering sensitivity is already covered by FINDING-06. Identity-point acceptance did not establish a concrete exploit and fits the crate's explicit "does not guarantee the usability of the resulting key" contract.
- `modular-frost`:
  `Schnorr::verify()` / `verify_share()` before `sign_share()` is the same documented precondition panic pattern already known from `frost-schnorrkel`; not a new finding.
- `dkg-dealer`:
  Offset-to-identity behavior is only another expression of W-01/FINDING-01 style invalid group-key propagation. The zero-participant edge remains the already confirmed FINDING-07 (`key_gen(3, 0)` returns `Ok(empty)`), so any Jules claim of panic/rejection there is incorrect.
- `multiexp` / panic hunting:
  Empty-input panic claims are false positives. `multiexp()` and `multiexp_vartime()` both route empty slices to `Algorithm::Null` and return identity; upstream tests already cover this.
- No Jules artifact justified High severity. No tracker promotion is warranted from this review alone.

## Session 10 Checkpoint

- Q2 resolved: `Encryption::decrypt()` does decrypt before PoP verification is checked, but the decrypted `SecretShare` path is wrapped in `Zeroizing` and explicitly zeroized before the queued PoP failure is returned.
- Q3 resolved: `pop_challenge()` explicitly binds context, PoP nonce, ephemeral public key, sender ID, and ciphertext bytes.
- Recipient ID is not explicit in the PoP transcript; recipient binding is implicit through the ECDH-derived ChaCha20 key.
- The fixed IV remains acceptable only under unique ephemeral scalars. No uniqueness or repetition check exists for those scalars in the code.

## Chunk 1 Checkpoint

- `dkg` core centers on `ThresholdParams`, `Interpolation`, `ThresholdKeys`, and `ThresholdView`.
- `ThresholdKeys::offset()` is additive-only and performs no identity/public-key validity check.
- `ThresholdKeys::group_key()` and `view()` both incorporate the offset directly, so invalid tweaked keys propagate unless callers revalidate.
- `pedpop` has a strict typed progression:
  `KeyGenMachine -> SecretShareMachine -> KeyMachine -> BlameMachine -> AdditionalBlameMachine`
- `pedpop` encryption is:
  ephemeral-static ECDH -> transcript-derived ChaCha20 key -> fixed IV -> Schnorr PoP over `(context, nonce, key, sender, ciphertext)` -> DLEq proof for blame
- `dealer` is a thin wrapper around polynomial sharing into `ThresholdKeys::new`.
- `recovery` reconstructs by summing `view(included).secret_share()` and checks against `group_key()`.

## Chunk 2 Checkpoint

- `ThresholdParams::new()` correctly rejects zero thresholds, zero participants, out-of-range participants, and `t > n`.
- `all_participant_indexes()` safely handles `u16::MAX`.
- `Interpolation::Lagrange` inversion sites are safe under current callers because duplicate participants are rejected before interpolation.
- `pedpop::validate_map()` enforces exact peer sets at both round boundaries.
- FINDING-07: `dkg-dealer::key_gen()` returns `Ok(empty)` when `participants == 0` because no `ThresholdParams::new()` call is reached.
- FINDING-08: `ThresholdKeys::new()` does not validate `Interpolation::Constant` length before indexing it, so a short vector can panic the public constructor.

## Chunk 3 Checkpoint

- `ThresholdKeys::view()` offset placement is mathematically correct: the additive tweak is applied after interpolation to the lowest included participant only.
- `ThresholdKeys::write()` / `read()` intentionally do not persist ephemeral scalar/offset state; that behavior is documented and consistent.
- W-01 remains open because `offset()` still never validates the resulting group key.
- `pedpop` PoP binds ciphertext bytes, sender, nonce, and ephemeral key; no direct authentication bypass was found.
- The fixed ChaCha20 IV is acceptable only because the implementation assumes a fresh ephemeral ECDH key for every message.
- FINDING-09: `BlameMachine::blame()` and `AdditionalBlameMachine::blame()` do not validate sender/recipient membership before indexing internal maps, so arbitrary participant IDs can panic the public blame path.

## Chunk 4 Checkpoint

- `dealer`, `pedpop`, `musig`, and `promote` all construct `ThresholdKeys` with internally well-formed interpolation inputs, so FINDING-08 is not currently reachable through shipped crate flows.
- `recovery` preserves tweak consistency by requiring matching `group_key`, `current_scalar`, and `current_offset` across all supplied keys.
- Ethereum hardens against W-01 by incrementing offsets until `PublicKey::new()` succeeds.
- Bitcoin does not eliminate W-01 at the `dkg` boundary: `wallet::tweak_keys()` documents the negligible-probability infinity case and returns the tweaked keys anyway.
- Bitcoin downstream code still contains explicit infinity panic sites in `crypto.rs` (`x()` / `x_only()`), so W-01 remains a real cross-crate panic condition even if its trigger probability is negligible under BIP341 tweaking.

## Chunk 5 Checkpoint

- No exploitable cryptographic break was confirmed in `dkg`, `pedpop`, `dealer`, or `recovery`.
- FINDING-07 is an invalid-parameter fail-open in `dkg-dealer`.
- FINDING-08 is a direct public `dkg` constructor panic surface, but not currently reachable through shipped sub-crate flows.
- FINDING-09 is a public panic surface in the `pedpop` blame APIs and is the most practically reachable issue in this batch because blame inputs are adversarial by design.
- W-01 remains open for final resolution in Bitcoin, not because `dkg` proved a new break here, but because downstream callers still do not uniformly defend the unchecked tweaked-key edge case.

## Session 31 Checkpoint

- `crypto/multiexp` remains CLOSED clean.
  - Core arithmetic and `BatchVerifier` soundness remain confirmed.
  - The verify kernel used under `FINDING-15` / `FINDING-17` remains sound.
- PedPoP blame constant-time follow-up: CLEARED.
  - `share_verification_statements` at `crypto/dkg/pedpop/src/lib.rs:430-448` does not feed the decrypted share into multiexp as a scalar.
  - It first computes `neg_share_pub = C::generator() * -*share` at `crypto/dkg/pedpop/src/lib.rs:444`, then queues `(C::F::ONE, neg_share_pub)` at `crypto/dkg/pedpop/src/lib.rs:446`.
  - The queued scalars used by `batch.queue(...)` at `crypto/dkg/pedpop/src/lib.rs:487-490` are therefore public evaluation coefficients plus `1`, not the confidential share.
  - The direct blame-path check at `crypto/dkg/pedpop/src/lib.rs:597-603` likewise calls `multiexp_vartime` only after the share has been converted to the public point `neg_share_pub`.
  - `BatchVerifier::verify_with_vartime_blame()` at `crypto/multiexp/src/batch.rs:127-132` can be forced into fallback by an invalid share, and `blame_vartime()` at `crypto/multiexp/src/batch.rs:108-123` may reprocess honest statements, but those statements still expose only public multiexp scalars and verifier-random batch weights.
  - No comment or design note was found explicitly documenting blame-time declassification of shares or an intentional constant-time sacrifice on confidential scalar material.
- No `FINDING-20` added.
- Totals unchanged:
  - 18 logged / 17 active
  - 2 High / 0 Critical / 1 Medium candidate / 7 Low candidates
  - 2 confirmed panic/DoS findings
- Next target:
  - `dalek-ff-group` -> `flexible-transcript`

## Session 32 Checkpoint (crypto/dalek-ff-group Chunk 1)

- Status snapshot:
  - `crypto/dalek-ff-group`: Chunk 1 complete, wrapper audit in progress, no new finding confirmed in this chunk.
  - `crypto/transcript`: next target after this chunk.
- File / API map:
  - production files:
    - `crypto/dalek-ff-group/src/lib.rs`
    - `crypto/dalek-ff-group/src/field.rs`
    - `crypto/dalek-ff-group/src/ciphersuite.rs`
  - public surface:
    - re-exports: `curve25519_dalek as dalek`, `ED25519_BASEPOINT_TABLE`, `RISTRETTO_BASEPOINT_TABLE`, `FieldElement`, `Ed25519`, `Ristretto`
    - `Scalar(pub DScalar)` with wrapper impls for `Deref`, `Borrow`, `ConstantTimeEq`, `ConditionallySelectable`, `Add`, `Sub`, `Mul`, `Neg`, `Field`, `PrimeField`, `PrimeFieldBits`, `FromUniformBytes<64>`, `Sum`, `Product`, plus methods `pow`, `from_bytes_mod_order_wide`, `from_hash`
    - `EdwardsPoint(pub DEdwardsPoint)` and `RistrettoPoint(pub DRistrettoPoint)` generated by `dalek_group!`, each with `ConstantTimeEq`, arithmetic impls, `Group`, `GroupEncoding`, `PrimeGroup`, `Hash`, basepoint constant, and wrapper `Mul<Scalar>`
    - `EdwardsPoint::mul_by_cofactor`
    - basepoint-table multiplication wrappers: `impl Mul<Scalar> for &EdwardsBasepointTable` and `impl Mul<Scalar> for &RistrettoBasepointTable`
    - `FieldElement` with `Field`, `PrimeField`, `PrimeFieldBits`, `FromUniformBytes<64>`, `Sum`, `Product`, and methods `from_u256`, `wide_reduce`, `pow`, `sqrt_ratio_i`
    - ciphersuite markers `Ristretto` and `Ed25519` implementing `hash_to_F(dst, data) = Scalar::from_hash(Sha512(dst || data))`
- Key wrapper bodies / loci:
  - point `from_bytes` / `to_bytes`: `crypto/dalek-ff-group/src/lib.rs:429-445`
  - scalar `from_repr` / `to_repr`: `crypto/dalek-ff-group/src/lib.rs:300-304`
  - point `is_identity`: `crypto/dalek-ff-group/src/lib.rs:418-420`
  - generic wrapper `Mul` body: `crypto/dalek-ff-group/src/lib.rs:129-149`
  - instantiations:
    - `Scalar * Scalar`: `crypto/dalek-ff-group/src/lib.rs:182`
    - `Point * Scalar`: `crypto/dalek-ff-group/src/lib.rs:381`
    - basepoint-table multiply wrapper: `crypto/dalek-ff-group/src/lib.rs:450-455`
- Canonical decode / encode conclusions:
  - Ed25519 wrapper rejects invalid / off-curve bytes by requiring `CompressedEdwardsY::decompress().is_some()` and then additionally requires `point.is_torsion_free()`.
  - Ristretto wrapper rejects invalid bytes by requiring `CompressedRistretto::decompress().is_some()`; the wrapper's `|_| true` torsion predicate is correct because canonical Ristretto encodings already identify prime-order quotient-group elements.
  - `from_bytes_unchecked` does not bypass checks; it aliases `from_bytes`.
  - `to_bytes` is pure delegation to dalek `Point::to_bytes`, and dalek point encoding is canonical (`compress().to_bytes()` for both Edwards and Ristretto), so the `read_G` round-trip assumption holds.
  - Identity-point admission remains present and is the already-documented context behind `FINDING-15` / `FINDING-17`, not a new dalek-ff-group finding.
- Scalar canonicality conclusions:
  - `Scalar::from_repr` delegates to dalek `PrimeField::from_repr`, which routes to `Scalar::from_canonical_bytes`.
  - dalek canonical decode rejects high-bit-set and unreduced / `>= order` inputs.
  - `Scalar::to_repr` delegates to dalek `to_repr -> to_bytes`, so scalar re-encoding is canonical.
- `is_identity` conclusions:
  - wrapper implementation is exact: `self.0.ct_eq(&$DPoint::identity())`.
  - no wrapper-side alternate encoding, cached flag, or reduction step exists that could create false positives / false negatives.
  - this is the exact predicate path F-15 / F-17 callers resolve to once the wrapper has accepted the point.
- Constant-time conclusions:
  - wrapper `ConstantTimeEq` is a straight delegation to dalek `ct_eq`; no fallback to variable-time `==` was found in the `ct_eq` path.
  - wrapper scalar add/sub/mul are direct dalek scalar ops via the macro instantiation at `lib.rs:182`.
  - wrapper point-scalar multiplication is direct dalek point multiplication via the macro instantiation at `lib.rs:381`.
  - carry-forward confirmation from the multiexp clearance stands: the `G * -share` path uses this same constant-time point-scalar multiplication route and the wrapper adds no branch on secret scalar bytes.
  - `random_nonzero_F` is not implemented here; it is supplied by `crypto/ciphersuite/src/lib.rs:60-68` and relies only on `Scalar::random` and `Scalar::ct_eq`, both of which remain constant-time at the wrapper boundary.
- `from_hash` / wide reduction:
  - `hash_to_F` in `src/ciphersuite.rs:31-33` calls `Scalar::from_hash`, which finalizes 64 bytes then uses dalek `from_bytes_mod_order_wide`; this is the intended unbiased wide-reduction path used by the ciphersuite audit.
  - `FieldElement::wide_reduce` performs exact `U512 mod p` reduction and is correct, though it is not the `hash_to_F` path for this crate.
- Panic-surface result:
  - no public decode panic reachable from attacker-influenced bytes was confirmed.
  - only production `unwrap` in the audited wrapper code is `NonZero::new(WIDE_MODULUS).unwrap()` in `field.rs:60`, which is constant-modulus setup and not attacker influenced.
- Candidate notes:
  - no new wrapper-introduced finding candidate confirmed in Chunk 1.
  - keep identity admission as root context only for `FINDING-15` / `FINDING-17`.
- Verification:
  - `cargo test -p dalek-ff-group --lib` passed locally.
- Totals unchanged:
  - 18 logged / 17 active
  - 2 High / 0 Critical / 1 Medium candidate / 7 Low candidates
  - 2 confirmed panic/DoS findings
- Next target:
  - `flexible-transcript`

## Session 33 Checkpoint (crypto/transcript Chunk 1)

- Status snapshot:
  - `crypto/transcript`: Chunk 1 complete.
  - next queue is the dedicated F-16 `message-queue` chunk, then post-audit chain-mapping.
- File / API map:
  - production files:
    - `crypto/transcript/src/lib.rs`
    - `crypto/transcript/src/merlin.rs`
  - test file:
    - `crypto/transcript/src/tests.rs`
  - public surface:
    - `Transcript` trait with `new`, `domain_separate`, `append_message`, `challenge`, `rng_seed`
    - `SecureDigest` trait and blanket impl
    - `DigestTranscript<D>`
    - `RecommendedTranscript = DigestTranscript<blake2::Blake2b512>` behind `recommended`
    - `MerlinTranscript` behind `merlin`
- `DigestTranscript` bodies / framing:
  - internal helper `append` at `crypto/transcript/src/lib.rs:98-104` writes:
    - `kind: u8`
    - `u64::to_le_bytes(value.len())`
    - `value`
  - `new(name)` at `lib.rs:110-113` appends member kind `Name`
  - `domain_separate(label)` at `lib.rs:116-118` appends member kind `Domain`
  - `append_message(label, message)` at `lib.rs:120-123` appends:
    - `Label` record for `label`
    - `Value` record for `message`
  - `challenge(label)` at `lib.rs:125-134`:
    - appends `Challenge` record for `label`
    - clones digest state
    - updates live transcript with single-byte `Continued`
    - updates cloned transcript with single-byte `Challenged`
    - finalizes the clone
  - `rng_seed(label)` at `lib.rs:136-140` takes the first 32 bytes of `challenge(label)`
- `RecommendedTranscript`:
  - alias only; same bodies as `DigestTranscript<Blake2b512>`
- `MerlinTranscript` wrapper bodies:
  - `new(name)` at `crypto/transcript/src/merlin.rs:31-33`
  - `domain_separate(label)` at `merlin.rs:35-37` calls `append_message(b"dom-sep", label)`
  - `append_message(label, message)` at `merlin.rs:39-45` asserts `label != b"dom-sep"` then delegates to Merlin
  - `challenge(label)` at `merlin.rs:47-50` calls `challenge_bytes(label, &mut [0; 64])`
  - `rng_seed(label)` at `merlin.rs:53-56` takes the first 32 bytes of `challenge(label)`
  - relied-on underlying Merlin bodies:
    - `merlin::Transcript::new` at `merlin-3.0.0/src/transcript.rs:69-87`
    - `merlin::Transcript::append_message` at `merlin-3.0.0/src/transcript.rs:96-100`
    - `merlin::Transcript::challenge_bytes` at `merlin-3.0.0/src/transcript.rs:175-179`
- Framing injectivity result:
  - `DigestTranscript` is fully self-delimiting:
    - exact record layout is `kind_u8 || len_u64_le || value`
    - `append_message(label, value)` is exactly:
      - `0x02 || len_le64(label) || label || 0x03 || len_le64(value) || value`
    - label is explicitly length-framed because it is its own record
    - challenge markers `Continued` / `Challenged` are single trailing sentinel bytes after a fully framed challenge-label record; they do not create ambiguity with normal appended members
    - conclusion: genuine injectivity holds
  - `MerlinTranscript` is injective through Merlin/STROBE operation framing:
    - underlying Merlin `append_message` does:
      - `meta_ad(label, false)`
      - `meta_ad(LE32(message.len()), true)`
      - `ad(message, false)`
    - label is not preceded by a standalone numeric length field, but the metadata-frame boundary is explicit, the final 4 bytes of metadata are fixed-width `LE32(message_len)`, and the following AD frame consumes exactly that many bytes
    - wrapper reserves `b"dom-sep"` so user messages cannot alias domain separation or constructor naming
    - conclusion: no distinct `(label, value)` sequence was found that serializes to the same transcript operation stream
- Load-bearing verdict:
  - framing injectivity holds
  - no prior transcript-dependent clearances need reopening:
    - W-05 stays closed
    - the ciphersuite `hash_to_F` transposition clearance stays closed
    - the `SignCompleted` framing clearance stays closed
- Challenge derivation result:
  - `DigestTranscript::challenge` is bound to the entire prior transcript state plus the challenge label and then forked with `Continued` / `Challenged` sentinel bytes; this is the mechanism that prevents challenge re-use and prevents a "challenge then continue" / "continue directly" conflict.
  - `MerlinTranscript::challenge` is bound to the entire prior STROBE state plus the challenge label and output length through Merlin `challenge_bytes`.
  - `rng_seed` intentionally conflicts with `challenge(label)` at the same transcript point because both implementations derive it as the first 32 bytes of `challenge(label)`; this matches the trait documentation.
- CS-3.9.1 re-verification:
  - current `SecureDigest` code at `crypto/transcript/src/lib.rs:83-91` now genuinely enforces `Digest::OutputSize >= 32 bytes` at compile time using the typenum `IsGreaterOrEqual<U32>` witness and `NonZero` proof.
  - fix mechanism holds for the original "malfunctioning minimum-output bound" bug class.
  - qualification: `SecureDigest` still does not enforce that the output length is a multiple of 8.
- CS-3.9.3 re-verification:
  - current fix code is the reserved-label assertion in `crypto/transcript/src/merlin.rs:39-43`, paired with `domain_separate` using `b"dom-sep"` in `merlin.rs:35-37`.
  - fix mechanism holds: ordinary appended messages can no longer use the same label as domain separation, so the prior alias class is blocked.
- Collision / transcript-confusion attempt:
  - `DigestTranscript`:
    - tried name/label boundary confusion, label/value boundary confusion, and domain-vs-message aliasing
    - all remain distinct because kind tags differ and both label and value are length-framed
  - `MerlinTranscript`:
    - tried `domain_separate(x)` vs `append_message(b"dom-sep", x)`, constructor naming vs later domain separation, and label/value boundary ambiguity
    - first class is blocked by the wrapper assertion
    - the remaining classes stayed distinct under Merlin's metadata + AD framing
  - no current transcript-confusion collision was found
- Panic-surface result:
  - `DigestTranscript`:
    - no attacker-byte decode panic confirmed
    - only invariant unwrap is `u64::try_from(value.len()).unwrap()` in `lib.rs:101-102`, not a practical panic on current 64-bit targets
  - `MerlinTranscript`:
    - public assertion panic on reserved label `b"dom-sep"` at `crypto/transcript/src/merlin.rs:40-43`
    - inherited Merlin length assertion for messages larger than `u32::MAX` bytes via `merlin-3.0.0/src/transcript.rs:14-21` and `:97`
- Candidate notes:
  - candidate 1:
    - generic contract mismatch: `SecureDigest` guarantees `>= 32` bytes but not `len % 8 == 0`, while `crypto/schnorr/src/aggregate.rs:22-26` still assumes both and slices challenge output in 8-byte chunks at `aggregate.rs:54`
    - current production impact is qualified because audited production digests are 64-byte outputs
  - candidate 2:
    - `MerlinTranscript::append_message` public panic on reserved label `b"dom-sep"` at `crypto/transcript/src/merlin.rs:40-43`
  - candidate 3:
    - inherited Merlin public panic on messages larger than `u32::MAX` bytes through delegated `append_message`
- Verification:
  - `cargo test -p flexible-transcript --all-features` passed locally
- Totals unchanged:
  - 18 logged / 17 active
  - 2 High / 0 Critical / 1 Medium candidate / 7 Low candidates
  - 2 confirmed panic/DoS findings
- Next target:
  - F-16 dedicated `message-queue` chunk
  - then post-audit chain-mapping

## Session 34 Checkpoint (message-queue F-16 dedicated chunk)

- Status snapshot:
  - `crypto/transcript` is COMPLETE and CLOSED clean.
  - `message-queue` dedicated F-16 chunk is complete.
  - all scoped crates are now audited and closed; next phase is chain-mapping.
- `flexible-transcript` closure:
  - framing injectivity confirmed for both `DigestTranscript` and `MerlinTranscript`.
  - three informational non-findings only:
    - `SecureDigest` / `weight()` generic-contract mismatch remains latent only
    - `dom-sep` reserved-label panic is a wrapper guard, not reportable
    - inherited Merlin 4 GiB message-length panic is not reportable in this context
- Exact F-16 receiver result:
  - `message-queue/src/client.rs:120-206` (`MessageQueue::next`) reads a `QueuedMessage`, verifies only the broad sender role (`Coordinator` for processors, `Processor(_)` for the coordinator), then returns it.
  - The exact line at `message-queue/src/client.rs:202` is `// TODO: Verify the sender's signature`.
  - No sender-signature verification occurs in `next()`.
  - `QueuedMessage.sig` is neither verified nor deferred to downstream callers; the client never parses it into `SchnorrSignature` or recomputes `message_challenge`.
- Sender-side signing / queue-side verification:
  - `message-queue/src/client.rs:82-118` signs outgoing `Queue` requests with `self.priv_key` over `message_challenge(...)` and includes `sig` in `MessageQueueRequest::Queue`.
  - `message-queue/src/main.rs:60-110` verifies queued messages server-side with `KEYS[meta.from]`.
  - `message-queue/src/client.rs:208-236` signs ACKs, and `message-queue/src/main.rs:132-148` verifies them server-side with `KEYS[to]`.
  - Therefore raw network reachability to the queue port is not sufficient to forge `Queue` or `Ack` requests without a service private key.
- Trust boundary / reachability:
  - transport is raw TCP (`TcpStream::connect` in the client; `TcpListener::bind("0.0.0.0:2287")` in `message-queue/src/main.rs:230`)
  - connection authentication is explicitly absent (`message-queue/src/main.rs:234` TODO to auth the connection)
  - `Next` is intentionally unauthenticated on the server side (`message-queue/src/main.rs:112-125`)
  - deployment uses separate coordinator / processor / message-queue services over the network via `MESSAGE_QUEUE_RPC=serai-<network>-message-queue` in `orchestration/src/coordinator.rs:35-42` and `orchestration/src/processor.rs:55-65`
  - message-queue exposes `2287` in `orchestration/src/message_queue.rs:38-40`
  - README frames the trust model as intranet-only (`message-queue/README.md`)
  - conclusion:
    - this is not local-socket-only or same-host-only
    - F-16 is reachable if the queue server is malicious/compromised or if an active network attacker can spoof/tamper with the unauthenticated `Next` response path
    - it is not a generic "anyone on the network can enqueue forged messages" issue because `Queue` ingress is still signature-gated
- Downstream impact facts:
  - processor side trusts queue output directly:
    - `processor/src/coordinator.rs:28-38` deserializes queue bytes into `CoordinatorMessage` without signature verification
    - `processor/src/main.rs:225-320` immediately dispatches coordinator messages into keygen / signing / cosigner / batch-signer / slash-report-signer control flow
    - `processor/src/main.rs:323-470` immediately acts on forged substrate/control instructions such as key activation and block-ack handling
  - coordinator side trusts queue output directly:
    - `coordinator/src/processors.rs:31-45` deserializes queue bytes into `ProcessorMessage` without signature verification
    - `coordinator/src/main.rs:815-833` treats `processors.recv(network)` as trusted processor input
  - concrete unauthenticated-message action sinks before any message-queue signature revalidation:
    - forged `ProcessorMessage::Coordinator::CosignedBlock` is forwarded to the local cosign channel and P2P-broadcast immediately at `coordinator/src/main.rs:253-267`
    - forged `ProcessorMessage::Substrate::SignedBatch` is saved and used to attempt `SeraiInInstructions::execute_batch(batch.clone())` publication at `coordinator/src/main.rs:342-374`
    - forged `ProcessorMessage::Coordinator::SignedSlashReport` enters the slash-report publish loop at `coordinator/src/main.rs:271-319`
    - forged processor keygen/sign/share messages are transformed into Tributary transactions and provided to the Tributary at `coordinator/src/main.rs:430-760`
- Independent re-validation which still exists:
  - processor completion claims still fail closed through `network.confirm_completion(...)` at `processor/src/signer.rs:311-353`
  - coordinator cosign handling later verifies signatures against on-chain validator-set keys at `coordinator/src/cosign_evaluator.rs:165-170`
  - slash-report signatures are additionally checked against the current slash-report key in `coordinator/src/main.rs:300-319`
  - many DKG / signing preprocess/share paths are handed into Tributary / FROST validation after receipt
- Severity-relevant conclusion inputs:
  - the missing verification is real and load-bearing
  - the exploit precondition is compromise/impersonation of the queue server or active tampering on the queue TCP path, not mere ability to call the `Queue` RPC
  - once that precondition is met, forged queue messages can trigger real coordinator/processor actions before downstream cryptographic revalidation
  - no direct funds-loss path was confirmed solely from F-16 in this chunk
  - no final severity assigned yet
- Totals unchanged:
  - 18 logged / 17 active
  - 2 High / 0 Critical / 1 Medium candidate / 7 Low candidates
  - 2 confirmed panic/DoS findings
- Next target:
  - post-audit chain-mapping
  - include the unresolved Low-vs-Medium refinement for `FINDING-18` / `FINDING-19`

## Session 35 Checkpoint (Phase-2 verification Chunk 1 - F-18/F-19 reachability & containment)

- Status correction:
  - prior "next step = chain-mapping" note is superseded
  - current plan is PAUSE -> Phase-2 verification
  - chain-mapping is deferred until the severity-deciding verification pass completes
- Findings counts unchanged:
  - 18 logged / 17 active
  - 2 High / 0 Critical / 1 Medium candidate / 7 Low candidates
  - `FINDING-18` / `FINDING-19` severity remains open pending this chunk + red-team review
- F-18 ingress / fan-out result:
  - the only external aggregate-commit ingress found is the block-sync req/res path, not live Tendermint gossip
  - end-to-end path:
    - libp2p req/res receive flattens both requests and responses into the same `Message` path (`coordinator/src/p2p.rs:697-712`)
    - outer dispatcher forwards any `ReqRes(Block(genesis))` to the tributary-specific channel (`coordinator/src/p2p.rs:1024-1034`)
    - per-tributary P2P handler accepts any `ReqRes(Block(msg_genesis))`, decodes `HeartbeatBatch`, and calls `tributary.tributary.sync_block(block, bc.commit).await` (`coordinator/src/p2p.rs:901-1009`)
    - `Tributary::sync_block_internal` decodes the commit and immediately calls `self.network.verify_commit(block.id(), &commit)` (`coordinator/tributary/src/lib.rs:262-298`)
    - `verify_commit` calls `verify_aggregate` (`coordinator/tributary/tendermint/src/ext.rs:256-275`)
    - `Validators::verify_aggregate` panics on `<Ristretto as Ciphersuite>::read_G(&mut s.as_slice()).unwrap()` (`coordinator/tributary/src/tendermint/mod.rs:200-229`)
  - attacker delivery model:
    - direct delivery to chosen victims as any connected peer
    - no validator-only auth gate exists (`coordinator/src/p2p.rs:375-395`)
    - no correlation to a prior heartbeat request exists; `ReqRes(Block(...))` is accepted whenever received
  - relay model:
    - raw block-sync commit bytes are not re-broadcast by victims
    - the `ReqRes(Block(...))` branch has no relay call
    - Tendermint gossip relay only concerns individual signed consensus messages after successful local processing
- Live consensus / safe-by-contrast result:
  - live gossip carries `SignedMessageFor<_>` items, not aggregate commits
  - `Tributary::handle_message` forwards `TENDERMINT_MESSAGE` into the machine (`coordinator/tributary/src/lib.rs:328-337`)
  - machine receive path first runs `msg.verify_signature(&self.validators)` (`coordinator/tributary/tendermint/src/lib.rs:1048-1058`)
  - that uses the safe individual verifier which rejects unknown IDs / point-decode failures with `false`, not panic (`coordinator/tributary/src/tendermint/mod.rs:168-178`)
- Containment / blast-radius result:
  - panic executes in the per-tributary P2P message-handler task spawned at `coordinator/src/p2p.rs:901-1011`
  - that same task also carries:
    - `ReqRes(Heartbeat)`
    - `ReqRes(Block)`
    - `Gossip(Tributary)` for live consensus + transaction gossip
  - raw task scope is one tributary because each active genesis gets its own channel and handler (`coordinator/src/p2p.rs:888-901`)
  - however actual blast radius is whole-process because coordinator `main()` installs a panic hook that immediately `exit(1)`s on any panic (`coordinator/src/main.rs:1315-1327`)
  - there is no `catch_unwind`, no `JoinHandle` supervision, and no respawn around either the per-tributary handler or the Tendermint machine spawn
- Runtime / process behavior:
  - release builds use `panic = "unwind"` in workspace `Cargo.toml:116-119`
  - coordinator uses `#[tokio::main]`
  - load-bearing behavior is the custom panic hook, which upgrades any task panic into coordinator process exit
- Restart / crash-loop result:
  - malformed commits are not locally persisted before the panic
  - local commit persistence only happens in blockchain add after verified acceptance (`coordinator/tributary/src/blockchain.rs:275-308`)
  - the panic occurs earlier in `sync_block_internal`
  - startup reloads active tributaries from DB and respawns their handlers (`coordinator/src/main.rs:1015-1085`)
  - therefore no local-DB crash loop exists
  - restart does recover if the attacker stops
  - restart is cheaply re-triggerable if the attacker keeps sending the crafted `ReqRes(Block(genesis))`
  - if the node remains behind, the heartbeat sync task may also solicit fresh block batches from peers (`coordinator/src/p2p.rs:827-873`)
- `add_block` / consensus-task shielding result:
  - `Commit` decode only SCALE-decodes `end_time`, `validators`, and aggregate signature bytes; it does not validate validator bytes as points (`coordinator/tributary/tendermint/src/ext.rs:131-151`)
  - `add_block` asserts `verify_commit(...)` at `coordinator/tributary/src/tendermint/mod.rs:362-389`
  - external call path into `add_block` is the synced-block channel inside the Tendermint machine (`coordinator/tributary/tendermint/src/lib.rs:948-973`)
  - but `sync_block_internal` already called `verify_commit` before it ever sent that `SyncedBlock`
  - local call path is the machine's own locally assembled commit (`coordinator/tributary/tendermint/src/lib.rs:621-637`)
  - conclusion:
    - externally supplied malformed commit bytes hit `verify_commit` first in `sync_block_internal`, not first in `add_block`
- F-19 gating result:
  - still chained behind `verify_aggregate` returning `true`
  - no alternate path to the `weights.weight(...)` panic was found in this chunk
  - duplicate validator IDs are rejected first in `verify_commit`
  - malformed/non-canonical validator bytes hit F-18 earlier
  - live gossip never uses aggregate verification on attacker-supplied commits
  - current source-confirmed path remains the earlier F-17 identity-collapse route into the threshold sum
- End-to-end synthesis:
  - yes, the malformed commit is attacker-deliverable to many nodes by direct req/res delivery as any connected peer
  - yes, it takes down consensus per node at whole-coordinator-process blast radius in current builds
  - no, it is not a local persistence crash-loop; yes, it is trivially re-triggerable after restart by renewed network delivery
- Current next step:
  - continue Phase-2 verification
  - chain-mapping remains deferred

## Session 36 Checkpoint (Phase-2 follow-up - processor hook + Immunefi table)

- Processor hook confirmed:
  - `processor/src/main.rs` also installs the same panic hook pattern:
    - `#[tokio::main]` at line 699
    - `std::panic::set_hook(...)` at line 704
    - `std::process::exit(1)` at line 709
  - effect:
    - any task panic in the processor is likewise whole-process exit
    - open processor panic candidates must be reviewed under that process-fatal model during Phase-2
- Additional note:
  - `message-queue` also uses the same panic-hook pattern (`message-queue/src/main.rs:155`, `:160`)
- Live Immunefi page confirmed on 2026-06-03:
  - Serai reward table is flat by severity only:
    - Critical `$30,000`
    - High `$5,000`
    - Medium `$1,000`
    - Low `$250`
  - the live page does not add a separate explicit payout row for "total network shutdown"
  - the page states rewards are based on the impact under the impacts-in-scope table and Serai uses Primacy of Impact for all Blockchain/DLT severities
  - consequence:
    - the finding ceiling is determined by the impact classification first, then mapped onto Serai's flat table
- Findings counts unchanged.

## Session 37 - F-25 candidate verification + focused PoC

- Ran the focused F-25 verification chunk and local PoCs.
- Environment notes:
  - `crypto/schnorr` PoC ran directly
  - `serai-coordinator` tests required `--features parity-db`; without a DB backend feature, `coordinator/src/main.rs:1337-1346` leaves `db` undefined for the binary test target
  - parity-db run required escalated execution so Cargo could unpack `memmap2` into the global registry cache

### C0 verdict

- Real-path scanner test was cheap enough; did not need the predicate-only fallback.
- Existing helpers already provided everything needed:
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
    - existing `handle_new_blocks` harness pattern

### Per-link status

- Link 1 PASS:
  - `crypto/schnorr/src/lib.rs:88-110`
  - `SchnorrSignature::verify` / `batch_statements` have no non-identity public-key guard
- Link 2 PASS:
  - `coordinator/src/tributary/transaction.rs:596`
    - `SignCompleted` is `Unsigned`
  - `coordinator/src/tributary/transaction.rs:614-624`
    - `verify()` only checks `signature.verify(first_signer, sign_completed_challenge())`
    - no validator-membership check
  - `coordinator/src/tributary/transaction.rs:703-713`
    - challenge binds `plan`, `tx_hash`, `first_signer`, and `R`
- Link 3 PASS:
  - `coordinator/src/tributary/handle.rs:721-740`
  - unrecognized-plan `SignCompleted` path calls `fatal_slash(first_signer.to_bytes(), ...)` and `return`s before any processor send
- Link 4 PASS:
  - `coordinator/src/tributary/scanner.rs:203-209`
  - `coordinator/src/tributary/db.rs:89-110`
  - raw bytes are stored, later read back via `from_bytes(key).unwrap()`
- Link 5 PASS:
  - `coordinator/src/tributary/scanner.rs:216-658`
  - after tx loop, `handle()` unconditionally reads `FatalSlashes` and computes `still_present_shares`
  - this iterates every fatal slash and does `self.spec.i(&[], *removed).expect("removed party was never present")`
- Link 6 PASS:
  - `coordinator/src/tributary/spec.rs:105-139`
  - `all_is.get(&key)?` means non-validator key => `None`
- Link 7 PASS:
  - `coordinator/tributary/src/mempool.rs:108-186`
    - Signed path has `ACCOUNT_MEMPOOL_LIMIT`
    - Unsigned path has no per-signer cap and only runs `app_tx.verify()?`
  - `coordinator/tributary/src/mempool.rs:189-219`
    - `block()` includes Unsigned first
  - `coordinator/tributary/src/blockchain.rs:230-270`
  - `coordinator/tributary/src/block.rs:171-267`
    - block verification re-runs application `tx.verify()` on every validator
- Link 8 PASS:
  - `coordinator/src/tributary/scanner.rs:661-718`
    - `LastHandledBlock` and `txn.commit()` happen only after `handle()` returns
  - `coordinator/src/main.rs:1320-1325`
    - panic hook exits process on panic
  - poisoning block therefore remains unacknowledged and is re-scanned after restart

### Read-your-writes result

- Yes.
- Decisive executed backend:
  - `common/db/src/mem.rs:14-22`
  - `MemDbTxn::get` explicitly checks in-txn deletes, then in-txn writes, then DB state
- Also explicit in parity-db:
  - `common/db/src/parity_db.rs:10-17`
  - queued changes are replayed on reads
- Consequence:
  - panic happens on the poisoning block itself, not the next block

### PoC results

- PoC A PASS:
  - `cargo test -p schnorr-signatures identity_public_key_verifies_if_r_equals_sg -- --nocapture`
  - output:
    - `test tests::identity_public_key_verifies_if_r_equals_sg ... ok`
- PoC B PASS:
  - `cargo test -p serai-coordinator --features parity-db sign_completed_forgery_verify_and_add_without_unsigned_cap -- --nocapture`
  - output:
    - `test tests::tributary::tx::sign_completed_forgery_verify_and_add_without_unsigned_cap ... ok`
  - facts proven in test:
    - identity-key forged `SignCompleted` passes `verify()`
    - 64 distinct identity-key forgeries are accepted through `Tributary::add_transaction(...)` (public wrapper over mempool path)
    - fresh non-validator self-signed variant also passes `verify()` and is accepted
- PoC C PASS:
  - `cargo test -p serai-coordinator --features parity-db sign_completed_ -- --nocapture`
  - output:
    - `test tests::tributary::tx::sign_completed_identity_forgery_crash_loops_scanner - should panic ... ok`
    - `test tests::tributary::tx::sign_completed_non_validator_self_signature_crash_loops_scanner - should panic ... ok`
    - panic site reported by test runner:
      - `coordinator/src/tributary/scanner.rs:264:38`
      - `removed party was never present`
- Chosen C path:
  - preferred real `handle_new_blocks` path
  - fallback not used

### Defeater check

- No defeater found.
- No source guard blocks:
  - non-validator `first_signer` at transaction verify
  - non-validator `first_signer` at handler slash site
  - non-validator fatal slash before `spec.i(...).expect(...)`
  - large volumes of unsigned `SignCompleted` through a per-signer cap

### F-25 candidate summary

- Confirmed chain:
  - forged unsigned `SignCompleted`
  - persisted `fatal_slash(first_signer.to_bytes(), ...)`
  - same-txn read-your-writes exposes the slash immediately
  - unconditional `still_present_shares` hits `spec.i(...).expect("removed party was never present")`
  - coordinator panic hook exits process before scanner progress commit
  - restart re-scans the same finalized poisoning block
- Trigger works with:
  - identity `first_signer` forged via `(R = sG, s)`
  - fresh non-validator key using a real self-signature
- Severity note:
  - ~~Critical-defensible on persistence / restart-proof halt grounds~~
  - Session 38 correction: Low only under Immunefi's "undocumented panic reachable from a public API"; persistence / restart-loop impact narrative does not create a higher by-right tier
  - not locked solo; pending joint rating

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
- Note on front-page totals: the top-of-file totals remain a historical snapshot and do not include later unregistered findings such as F-22 / F-25; use this Session 38 block as authoritative.

## Session 37 provenance

- HEAD:
  - `737dbcbaa78ab817cc1c435cb2b6c5d24d1c4391`
- Branch / upstream:
  - branch `develop`
  - upstream `origin/develop`
- Remote:
  - `origin https://github.com/serai-dex/serai.git (fetch)`
  - `origin https://github.com/serai-dex/serai.git (push)`
- HEAD commit:
  - `737dbcbaa78ab817cc1c435cb2b6c5d24d1c4391 2026-01-10T02:38:31-05:00 fix: remove duplicate docsrs cfg_attr lines`
- Working tree:
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
- 9 cited production files were clean vs `HEAD`:
  - `git status --porcelain -- ...` empty
  - `git diff --stat HEAD -- ...` empty
- Unexpected dirty files:
  - extra untracked files beyond the two PoC test edits:
    - `agent.md`
    - `context.md`
    - `dkg_fuzz_property_test_spec.md`
    - `findings.md`
    - `jules/`
    - `understanding_report.md`
- Remote tip delta after fetch:
  - `origin/develop` moved from `737dbcba` to `4b89cf02`
  - `git log --oneline -1 origin/develop`:
    - `4b89cf02 Remove \`.github\` actions, workflows`
  - `git rev-list --left-right --count HEAD...origin/develop`:
    - `0 1`
  - meaning:
    - local `HEAD` is `1 behind / 0 ahead`
- Pin:
  - all F-25 line references / pasted bodies should be treated as cited against local `HEAD` `737dbcbaa78ab817cc1c435cb2b6c5d24d1c4391`

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
  - each node's persisted reader returns the finalized block containing the forged tx and a stored commit for that block
- GAP 2 closed by `sign_completed_non_validator_self_signature_restart_rescans_persisted_poison_block` at line 360:
  - proves a fresh scanner invocation against the same persisted `parity-db` re-hits the same panic message after the poison block has already been finalized
  - proves scanner progress does not advance into the poison block:
    - after the first caught panic, `LastHandledBlock::get(...).unwrap_or(genesis)` equals the poison block parent, not the poison block
    - `reader.block_after(progress) == Some(poison_block)` still holds before the second scan
    - after the second caught panic, the recorded progress is unchanged
  - production code fact still cited, not executed in-test:
    - `coordinator/src/main.rs:1320-1325` installs a panic hook which `std::process::exit(1)`s on panic
- GAP 3 also closed cheaply inside the restart test:
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
