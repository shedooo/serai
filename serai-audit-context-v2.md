# SERAI AUDIT — CONTEXT v2 (clean rebuild)
> Re-upload at the start of every session to restore context. Supersedes the legacy `claude-context-final.md` (40-session file, kept only as an archive). Append-only from here; keep it tight.
> **Built fresh at Session 41.** Old finding-by-finding sprawl is intentionally NOT carried over — only current truth, hard scope, lessons, coverage, and the active strategy.

---

## 0. CURRENT STATUS (read first)
- **F-25 was SUBMITTED → CLOSED ineligible** (network out of scope, pre-mainnet, Primacy of Impact, "rewritten on private staging"). It is the proof that **the whole non-crypto network is currently unrewardable.**
- **Strategy pivot:** stop mining the network/findings backlog. **Hunt the post-audit delta** — code that changed *after* the Cypher Stack audit commit (`669d2db`, Mar 2023), which is professionally unaudited. That delta, weighted by payoff, is where a real unfound in-scope bug most likely lives.
- **Live confirmed findings:** F-15 / F-17 — two Highs, same root cause (schnorr verify accepts identity). Ceiling proven to be **High, not Critical** (recon closed it, §8). Contestable as a documented design choice.
- **Open external item:** a scope-clarification message is drafted and pending — does the program entertain *library-intrinsic* crypto findings pre-mainnet? Everything below is gated on that "yes," but the bitcoin/FROST signing targets are the cleanest possible in-scope class regardless.

---

## 1. HARD SCOPE BOUNDARY ⚠ (the #1 anti-drift rule)
**ELIGIBLE** = a defect *inside a listed crypto-library asset* (see §2) whose impact is a **property of the library**, demonstrable with the crate's own public API + test harness, **with no deployed network required.**

**INELIGIBLE (pre-mainnet)** = anything whose impact is only realized through the undeployed **coordinator / processor / substrate runtime ("the network")**, reached via Primacy of Impact. *The network is NOT in scope at this time.*

- **By-asset (eligible)** ≠ **Primacy-of-Impact-on-the-network (dead pre-mainnet).** Always classify a candidate on this axis first.
- **Primacy-of-Impact caveat (verbatim intent):** code that is historical / visibly under active development / never deployed / not slated for deployment = *no real impact = ineligible.* This killed F-25 and gates all network findings.
- **Reward reality:** Critical = 10% of funds-at-risk capped $30k → **pre-mainnet funds-at-risk ≈ 0**, so a "funds" Critical computes to ~nothing anyway. Sign/forge/key-recovery Criticals are valued as *library* defects, not by funds.

---

## 2. IN-SCOPE ASSETS (the only rewardable surface)
| Crate | Path |
|---|---|
| schnorr-signatures | `crypto/schnorr` |
| frost-schnorrkel | `crypto/schnorrkel` |
| modular-frost | `crypto/frost` |
| dkg (+ musig, pedpop, promote) | `crypto/dkg`, `crypto/dkg/{musig,pedpop,promote}` |
| ciphersuite | `crypto/ciphersuite` |
| multiexp | `crypto/multiexp` |
| dalek-ff-group | `crypto/dalek-ff-group` |
| flexible-transcript | `crypto/transcript` |
| dleq | `crypto/dleq` |
| bitcoin-serai | `networks/bitcoin` |

**Explicitly OUT:** coordinator, processor, substrate pallets/runtime, message-queue, the running network; BFT/51%/Sybil/centralization; experimental cross-group DLEq; test-only/unsafe-only.

---

## 3. CANONICAL IMPACT TABLE (9 impacts — VERIFIED against the live page)
| Severity | Impact |
|---|---|
| CRITICAL | Signing of unintended messages |
| CRITICAL | Ability to forge proofs |
| CRITICAL | Unintended/undocumented recovery of private spend keys or key shares |
| CRITICAL | Reportedly received funds that weren't actually received/spendable |
| HIGH | Incorrect/incomplete cryptographic formulae within a **verifier's** callstack |
| MEDIUM | Undocumented transcript collision |
| LOW | Undocumented panic reachable from a public API |
| LOW | Non-constant-time implementation w.r.t. secret data |
| LOW | Incorrect/incomplete cryptographic formulae within a **prover's** callstack |

- **THERE IS NO availability / liveness / network-shutdown / DoS tier.** A process-exit panic is **Low** ("panic from a public API") regardless of blast radius. (Three prior session-generations burned time over-rating panics via a phantom tier — never again.)
- **Rewards:** Critical = 10% funds capped $30k; High = $5k; Medium = $1k; Low = $250 (flat).
- **PoC required** for any Critical and for the Low "panic from a public API."

---

## 4. RULES OF ENGAGEMENT (non-negotiable)
1. **Classify on the scope axis first** (§1): library-intrinsic vs network-routed. Network-routed = stop, it's dead pre-mainnet.
2. **Severity gated against the §3 table only.** No invented tiers. No "permanence/blast-radius raises it."
3. **Re-derive math/logic from source** (read it), never trust an executor verdict or this file's prose. This file records *conclusions*; verification lives in the code.
4. **Hunt the unaudited delta first** (post-`669d2db`), not the whole stack blindly.
5. One finding = one root cause = one fix.
6. **Pin commit `737dbcba`** in every report. Submit via Immunefi **"Submit a Bug"** (never a GitHub Security Advisory).
7. Don't attach whole test files that expose held/other findings — paste only the relevant test fns + harness.
8. **Every Codex executor chunk ends** with an explicit instruction to update BOTH `context.md` and `agent.md` (append-only), naming what to record.

---

## 5. PROVENANCE / PINS
- **Repo:** `github.com/serai-dex/serai` (canonical). Default/working branch `develop`.
- **Audit pin (cite this):** `737dbcbaa78ab817cc1c435cb2b6c5d24d1c4391` (develop, 2026-01-10). Work on local branch `audit-737dbcba`.
- **`develop` tip** = `4b89cf02` ("Remove `.github` actions") = pin + a benign `.github`-only commit. So the pin == current public code for all in-scope crates.
- **Cypher Stack pro-audit commit:** `669d2db` (Mar 2023). schnorr README: everything after is **unaudited**. → the delta `669d2db..737dbcba` is the hunting map (§7).
- **No public `staging` branch** exists; the team's "staging" (cited in the F-25 close) is private/unpublished and unreachable to us. F-15/F-17 verify defects confirmed live on every public branch (develop/next/testnet-2/coordinator-quic/custom-generator).
- **Toolchain:** Rust **1.89 stable**, target `wasm32v1-none` (per `rust-toolchain.toml`). `cargo check -p schnorr-signatures` builds clean.

---

## 6. WORKFLOW
- **Claude** = strategy/analysis + prompt-crafting + direct source re-derivation. **Codex (VS Code)** = executor, holds the repo clone. **User** relays between them.
- **Codex I/O contract:** pastes full function bodies with `file:line` (never summaries); maintains append-only local `context.md` + `agent.md`. *(Those local files were lost when the clone was deleted/re-cloned — RESEED them from this file on the next Codex chunk.)*
- Claude can also read source directly from `raw.githubusercontent.com/serai-dex/serai/<commit>/<path>` for independent cross-checks.

---

## 7. THE HUNT — POST-AUDIT DELTA (active strategy)
**Thesis:** a bug survives in proportion to how few competent eyes hit it. The Cypher Stack pro-audit was at `669d2db`; the delta `669d2db..737dbcba` is **professionally unaudited** and has had only **one single-reviewer pass** (§8). So this is an *adversarial RE-look at the least-scrutinized in-scope code* — not virgin territory. Odds are modest; we target the gaps the pass left (the §8 "wedge"), not its covered ground.

**Map:** `git diff 669d2db..737dbcba -- crypto networks/bitcoin` → 138 files, ~6.2k insertions. The high-signal surface:

**A. Whole crate is post-audit (professionally unaudited; only the single-reviewer pass)**
- **`networks/bitcoin` — entire crate**, ~1,100 new lines: `wallet/send.rs` (+429, tx construction + signing), `rpc.rs` (+226), `wallet/mod.rs` (+228, Taproot tweak/offset), `crypto.rs` (+163, BIP-340 adapter over FROST). A signing/composition defect = "sign unintended" / "forge proofs" Critical, library-demonstrable. **See §8 for what's already traced — hunt the offset/parity *composition*, not the basics.**
- **`crypto/schnorrkel` — entire crate** (`lib.rs` +156): substrate signing context.

**B. Pro-audited THEN rewritten (the changed lines are unaudited)**
- **`crypto/frost/src/nonce.rs`** (115 changed) + **`sign.rs`** (123): nonce + preprocess lifecycle = the FROST **key-share-recovery** surface. (Re-aim the old "cached-preprocess reuse" lead at the *library internals* here — its processor framing was the out-of-scope part.)
- **`crypto/transcript/src/lib.rs`** (58): if framing changed, the injectivity several clearances rest on may reopen (collision = Medium, or worse if it breaks a binding factor).
- **`crypto/multiexp`** (lib 53 / batch 34 / pippenger 20 / straus 23): the verify **kernel** — bug = every verification wrong (high payoff, low prob, well-tested).
- `crypto/frost/algorithm.rs` (29), `curve/mod.rs` (35); `crypto/schnorr/{aggregate.rs 25, lib.rs 19}` = F-15/F-17 home (known).

**Ranked targets:** (1) `networks/bitcoin` signing/tweak → (2) `frost/nonce.rs` + `sign.rs` → (3) `transcript/lib.rs` delta → (4) `schnorrkel` → (5) `multiexp` kernel.

**Progress so far (Session 41):**
- Read `networks/bitcoin/src/crypto.rs`. **BIP-340 even-Y nonce handling is CONSISTENT** (no bug): `Hram` negates the challenge when aggregate `R` is odd-Y; `verify` negates `s` to match; published `R` is x-only. Both parity cases balance (`s·G == R_even + c·A`).
- **Open seam (the wedge, per §8):** `crypto.rs` *assumes the group key `A` is even-Y* (uses `x(A)`, never negates on `A`'s parity, unlike `R`); group-key parity is handled in the wallet tweak code. The basics (message binding, sighash, participant set, tweak source) are already traced clean — so the unexamined risk is the **composition**: per-input spend **offsets** + the key tweak + the even-Y challenge/`s` negation, under malicious co-signers. → **Next reads:** `wallet/send.rs` (per-input offset path + how `AlgorithmSignMachine` composes offsets with the tweak) and `wallet/mod.rs` (tweak/even-Y), looking for a non-probabilistic path to a degenerate or unintended signature; then `frost/nonce.rs` (post-audit nonce rewrite).

---

## 8. COVERAGE RECORD (so we don't redo it)
**Caveat:** every "CLEAN/CONFIRMED" in the legacy file = *single-reviewer reasoning over pasted code*, NOT proven, NOT PoC'd, NOT adversarially broken. Clean crates were **cleared, not broken** — which is exactly why the §7 delta hunt is worth doing.

- **All in-scope crypto crates received a single-reviewer pass.** Verdicts: CLEAN except `crypto/schnorr` (F-15/F-17) and panic findings. Foundation (`ciphersuite`, `multiexp`, `dalek-ff-group`, `transcript`) confirmed sound *under* F-15/F-17 — no "the primitive is the real bug" escape hatch.
- **Key constructions verified (conclusions; re-derive if load-bearing):** FROST binding factor `H(group_key‖H(msg)‖H(H(preprocesses)))`; dkg-musig binding `hash_to_F("dkg-musig", ctx‖len‖keys‖i)`; schnorr verify `R + cA − sG == 0` via `multiexp_vartime` (NO identity guard on A = F-15); aggregate `Σ z_iR_i + Σ z_ic_iA_i − sG == 0` (NO identity guard = F-17); `ciphersuite::read_G` admits identity by documented design (consumer-guards-identity convention = root of F-15/F-17); transcript framing `kind‖len‖value` injective.
- **Known-issues check done:** GitHub **#295** (crafted ThresholdKeys → FROST panic; team stance = trusted struct, wontfix) covers F-08 and threatens F-01/F-04. F-15/F-17/F-25 are NOT known issues.

**Already traced in the prior single-reviewer pass — do NOT re-tread (go deeper/adversarial, not repeat):**
- **`networks/bitcoin` (5-chunk trace):** signing message is **locally derived** — `taproot_key_spend_signature_hash(..., TapSighashType::Default)`, sighash type fixed in code (not counterparty-selectable), and the machine rejects an external `msg` unless empty ⇒ the basic "sign unintended message" angle was checked. Participant set validated (`AlgorithmMachine` from local `ThresholdKeys`; prevout script-pubkey equality in `SignableTransaction::multisig`). Tweak = key-only BIP341 (`TapTweakHash(group_key[1..])` → `offset()`; no attacker script-tree). `offset()` is additive, no post-tweak validity check ⇒ post-tweak identity is a **probabilistic** edge (`k+H_tap(k)≡0`). Aggregate `R` is NOT checked `!= identity` (only individual commitment points are), but adversarial forcing of `R==identity` was **denied as practical** (bound nonces include per-participant binding factors). `x()`/`x_only()` infinity panics are downstream of those probabilistic edges (Low, and they execute in the **processor = network = OOS**). send.rs panic sites 122/125/199/389 = invariant-only. W-01/W-02/W-03/W-06 closed. Session 41: `crypto.rs` even-Y R-negation re-verified consistent.
- **`crypto/frost` (5-chunk trace):** binding-factor chain correct (`algorithm → rho → per-participant transcript`); nonce-reuse prevention is a **documented caller contract**; `verify_share` equations correct (individual bound nonces); FROST does **not** produce or modify the group key; RFC 9591-compliant; CT scalar ops; secrets zeroized.
- **Wedge (what the prior pass did NOT adversarially test):** the *composition* soundness — per-input spend **offsets** (`send.rs`) + the key tweak + the even-Y challenge/`s` negation, under malicious co-signers; and any non-probabilistic path to a degenerate signature/key. That is where to push, not `tweak_keys`/HRAM/sighash structure.

---

## 9. FINDINGS — CURRENT TRUTH (consolidated)
> **Fresh-clone note (Codex):** the `F-NN` IDs are Claude-side labels, each fully described in this table — this file is the **complete** findings record. Your clone is fresh: there is **no `findings.md`, no prior PoC test files (e.g. in `tx.rs`), no `jules/`** in your tree. Do **not** search for them; nothing is missing. Treat dead/closed rows as "do not resurrect," live rows as "already characterized — don't re-discover."

| ID | What | Status |
|---|---|---|
| **F-15** | `SchnorrSignature::verify` accepts identity key (`crypto/schnorr/src/lib.rs:108`; `batch_statements`). A=identity ⇒ `cA` vanishes ⇒ forge with `R=sG`, no secret. | **HIGH** (verifier-formula). Critical CLOSED (§ recon). Same root/fix as F-17 → file together. Contestable as documented design choice. |
| **F-17** | `SchnorrAggregate::verify` accepts identity key (`crypto/schnorr/src/aggregate.rs:127`/`:139`). | **HIGH**, same class. |
| F-25 | Forged Unsigned SignCompleted → coordinator panic crash-loop. | **SUBMITTED → CLOSED ineligible** (network/pre-mainnet). Dead. |
| F-18 / F-19 | verify_aggregate / verify_commit panics on crafted p2p input. | **DEAD** (network; no availability tier). |
| F-22 | Partial-batch InstructionFailure → funds received-not-credited. | **DEAD** (substrate pallet/processor = network; funds-at-risk ≈ 0 pre-mainnet). Triage NOT run. |
| F-01/04/09/11/14 | Library API panics / prover-formula. | **Low $250-class**, contestable / #295-shadowed. Don't over-invest. |
| F-08 | ThresholdKeys panic on short Interpolation. | **Known (#295).** Dropped. |

**RECON RESULT — DKG-PoP identity-acceptance is INERT (F-15 ceiling = High, source-backed):**
- PoP challenge **binds the commitment key** (`crypto/dkg/pedpop/src/lib.rs:86-93`, absorbs `commitments`) → no rogue-key-via-unbound-challenge.
- PoP verify **accepts identity** (`lib.rs:323-327`, `batch_verify` vs `commitments[0]`, no guard) — attacker *can* forge a PoP for an identity commitment.
- **But group key = `Σ commitments[0]`** (`lib.rs:507`) → an identity contribution is **zero/inert**: can't bias, can't cancel honest contributions (needs `−Σ(others)` with unknown dlog → no forgeable PoP, blocked by the binding challenge), can't force the group key to identity. FROST/promote/schnorrkel inherit this safety.
- ⇒ identity-acceptance is real but does nothing exploitable in-library. **No Critical via this path.** (The recon also hands the team a "no impact in any in-scope caller" line against the F-15/F-17 *High* — weigh that when filing.)

---

## 10. OPEN THREADS / NEXT
1. **Scope clarification (pending send):** does the program entertain library-intrinsic crypto findings pre-mainnet? Minimal message drafted. The hunt's filing-EV is gated on "yes."
2. **Active dive:** `networks/bitcoin/src/wallet/mod.rs` + `send.rs` (TapTweak/parity composition), then `frost/src/nonce.rs`.
3. **If/when "yes":** F-15+F-17 = one low-effort "verifier-callstack formula" High report with a self-contained library PoC (construct identity key; `s` arbitrary; `R=sG`; show `verify()==true`). Coin-flip on the design-choice deflection; file as a cheap shot, not an investigation.
4. **Reseed** Codex `context.md`/`agent.md` from this file on the next chunk.
