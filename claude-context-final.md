# CLAUDE AUDIT CONTEXT — Serai Bug Bounty
> Upload this file at the start of every new chat session with Claude to restore full context instantly.
> Last updated: **Session 40 — resuming Phase 2 (F-25 SUBMITTED).** **✅ IMPACT-TABLE ALIGNMENT VERIFIED against the live Immunefi page (PDF-confirmed S38).** The live Serai impacts table has NO availability/liveness/network-shutdown tier — panics map ONLY to Low "undocumented panic reachable from a public API." The 9 live impacts + tiers (VERIFIED): **Critical** = signing unintended msgs / forge proofs / key(-share) recovery / **received funds not actually received-spendable**; **High** = **incorrect/incomplete crypto formula in a verifier's callstack**; **Medium** = undocumented transcript collision; **Low** = undocumented panic from public API / non-constant-time / prover-callstack formula. **⚠ Session 37's "recalibration" was OFF BY ONE ROW** (it read a truncated rendering of the impacts list and shifted the middle tiers down one) — CORRECTED S38. **Net corrected severities:** **F-25 / F-18 / F-19 = Low** (panic bucket; no availability tier exists — this part of S37 was RIGHT, the original Gen-A/B network-shutdown tier was the error). **F-15 = High** (verifier-callstack formula; Critical ceiling IF identity-acceptance forges against a protocol-used key — DKG-PoP recon pending). **F-17 = High** (verifier-formula; Critical denied). **F-22 = Critical** (received-funds-not-spendable; still an UNVERIFIED candidate; reward = 10% of funds-at-risk capped $30k and may compute LOW or be ineligible under the pre-mainnet active-development caveat). GATE 1 + GATE 2 CLOSED (repo = serai-dex/serai confirmed live). Panic hook confirmed in ALL THREE binaries (coordinator main.rs:1315, processor main.rs:704/709, message-queue main.rs:155) — real, but a process-exit panic is still only the Low bucket. STATUS: Phase-2 ACTIVE (resuming Session 40). ✅ F-25 SUBMITTED to Immunefi (Jun 3 2026 — Low, own-key variant; independently triage-reviewed Valid/Low/high-confidence). ✅ Re-map LOCKED; gates closed; known-issues check done. **PHASE-2 SPINE (the only Critical paths):** (A) **F-22 InstructionFailure triage** [refund-vs-silent-loss + is the failure attacker-induceable; the one Critical-bucket candidate; needs a PoC]; (B) **DKG-PoP crypto-Critical recon** [settles F-15 High-vs-Critical via forge-proofs] — AWAIT user GO before firing the executor chunk; (C) **cached-preprocess reuse → key-share-recovery chain** [unproven]. THEN file F-22 (Critical, if it triages) + F-15/F-17 (one High verifier-formula report) SEPARATELY — **F-15/F-17 HELD for the recon** so we file at the right ceiling. Lower track: processor leads B/E; F-18/F-19 Lows (need trigger PoCs if filing); F-08 dropped (known via #295); F-07/F-09/F-14/F-16 OOS-lean. **F-25 (SUBMITTED, Low):** forged Unsigned SignCompleted (own-key non-validator self-signature) → unrecognized-plan handler fatal_slashes first_signer into the in-flight scan txn → same-pass still_present_shares spec.i(non-validator).expect() panic BEFORE txn.commit → global hook → process::exit(1). Durable poison = finalized block + un-advanced LastHandledBlock → restart-persistent crash-loop (does not self-heal). Filed own-key-only (F-15 NOT exposed); 4 PoC tests green @HEAD 737dbcba. (Title/root-cause corrected S39: NOT a "persisted fatal-slash" — the slash write is uncommitted at panic time.) **Read the SESSION 38 block at end for current truth; S35/S36/S37 severity conclusions are SUPERSEDED where stamped.** Chain-mapping deferred. Includes REPORT FRAMING section.

---

## AUDIT IDENTITY

- **Target:** Serai DEX | **Repo:** github.com/serai-dex/serai (develop branch) — canonical upstream; cloned under the `shedooo` GitHub account (earlier "shedooo/serai" label was account framing, NOT a divergent fork). Audited HEAD pinned in Session 37 provenance below.
- **Program:** Immunefi — https://immunefi.com/bug-bounty/serai/scope/
- **Language:** Rust | **PoC Required:** Yes | **KYC Required:** Yes
- **Started:** April 23, 2026

---

## CLAUDE'S ROLE

Senior Security Researcher specializing in blockchain/cryptographic systems. Responsibilities:
- Crafting and refining audit prompts for all LLMs in the workflow
- Strategic reasoning about attack surfaces and vulnerability classes
- Analyzing code and results shared directly in chat
- Reviewing findings, enhancing PoCs, validating severity
- Maintaining audit momentum and context across sessions

**Operating discipline (non-negotiable):**
- Severity is gated by *reachability and provenance*, decided against source — never assumed. (F-15 template: real verifier defect, but independent re-validation denied Critical → landed High.)
- Re-derive the math/logic from pasted code rather than trusting the executor's verdict.
- Clear hypotheses willingly; one finding = one root cause = one fix.
- Cannot verify what is not in the pasted output (git hashes, unseen files). When something load-bearing is unseen, say so and pull it.
- Every executor prompt MUST end with an explicit instruction to update BOTH context files (append, never overwrite), naming exactly what to record.

---

## TEAM POSTURE

We are the best in the game. crypto/schnorr is CLOSED with the strongest results: F-15 (High, closed), F-17 (High, Critical denied), F-18/F-19 panic pair. The foundation crates beneath those findings — ciphersuite, multiexp, dalek-ff-group — and the transcript layer (flexible-transcript) are all audited and CLOSED clean, which hardens F-15/F-17: no "the primitive is the real bug" escape hatch remains, and the transcript framing injectivity that several clearances rest on is confirmed. **ALL scoped crates are now audited.** Two things remain: the F-16 dedicated message-queue chunk (lone Medium candidate, severity wide-open on reachability), then the chain-mapping session to compose findings into a Critical. The F-17 → F-19 chain seed is logged. Hunt accordingly.

---

## MULTI-LLM WORKFLOW

| Phase | Tool | Status |
|---|---|---|
| Phase 1 — Understanding + file init | Claude Opus (Antigravity) | COMPLETE |
| Phase 2 — Threat research | Gemini | COMPLETE |
| Phase 3 — Audit execution | Codex (VS Code) | IN PROGRESS |
| Jules — Dynamic testing | Jules Pro (4 tasks) | ALL REVIEWED — re-run deferred |
| Codex — Q2/Q3 gap-fill | Codex (VS Code) | COMPLETE |
| Codex — bitcoin-serai Chunks 1–5 | Codex (VS Code) | COMPLETE |
| Codex — dkg-promote Chunks 1–3 | Codex (VS Code) | COMPLETE |
| Codex — crypto/schnorr Chunks 1–7 | Codex (VS Code) | COMPLETE — crate closed |
| Codex — ciphersuite Chunks 1–2 | Codex (VS Code) | COMPLETE — crate closed clean |
| Codex — multiexp completion + pedpop blame CT check | Codex (VS Code) | COMPLETE — crate closed clean |
| Codex — dalek-ff-group Chunk 1 | Codex (VS Code) | COMPLETE — crate closed clean |
| Codex — flexible-transcript Chunk 1 | Codex (VS Code) | COMPLETE — crate closed clean (ALL crates audited) |
| Prompt crafting + strategy | Claude | Active |

**Executor I/O contract:** Codex pastes full function bodies with file:line references (never summaries), maintains local `context.md` and `agent.md` (append-only), and the user relays output/prompts between Codex and this chat. A context-restore prompt is used when starting Codex in a new chat.

---

## IMMUNEFI SCOPE — AUDIT PROGRESS

| Crate | Path | Priority | Status | Verdict |
|---|---|---|---|---|
| frost-schnorrkel | crypto/schnorrkel | CRITICAL | COMPLETE | CLEAN (4 Lows, 1 Info) |
| dkg-musig | crypto/dkg/musig | CRITICAL | COMPLETE | CLEAN (2 Info) |
| modular-frost | crypto/frost | CRITICAL | COMPLETE | CLEAN |
| dkg core + sub-crates | crypto/dkg | CRITICAL | COMPLETE | CLEAN (3 Lows) |
| dkg-pedpop | crypto/dkg/pedpop | HIGH | COMPLETE | CLEAN (1 Low, 2 Info) |
| bitcoin-serai | networks/bitcoin | HIGH | COMPLETE | CLEAN — 0 findings |
| dkg-promote | crypto/dkg/promote | MEDIUM | COMPLETE | CLEAN (1 Low — F-14) |
| dleq | crypto/dleq | MEDIUM | COMPLETE | CLEAN — 0 findings |
| **schnorr-signatures** | **crypto/schnorr** | **HIGH** | **COMPLETE (Chunks 1–7)** | **F-15 High (closed), F-17 High, F-18/F-19 DoS** |
| ciphersuite | crypto/ciphersuite | MEDIUM | COMPLETE (Chunks 1–2) | CLEAN — 0 findings (read_G identity = documented root of F-15/F-17; hash_to_F transposition inert) |
| multiexp | crypto/multiexp | MEDIUM | COMPLETE | CLEAN — F-13 false positive; verify kernel sound; pedpop blame-CT candidate cleared |
| dalek-ff-group | crypto/dalek-ff-group | MEDIUM | COMPLETE (Chunk 1) | CLEAN — 0 findings; is_identity/canonical-decode/CT all sound under F-15/F-17 |
| **flexible-transcript** | **crypto/transcript** | **MEDIUM** | **COMPLETE (Chunk 1)** | **CLEAN — 0 findings; framing injectivity confirmed (W-05/hash_to_F/SignCompleted stay closed); CS-3.9.1/3.9.3 fixed; 3 informational non-findings** |

---

## IMMUNEFI IMPACTS IN SCOPE — ✅ CANONICAL (VERIFIED S38 against live page, PDF-confirmed)

> This table is CORRECT and matches the live Immunefi page exactly (4 Critical / 1 High / 1 Medium / 3 Low = 9). Session 37's "recalibrated" table near the end of the file is the OFF-BY-ONE one — defer to THIS table. No availability/network-shutdown tier exists.

| Severity | Impact |
|---|---|
| CRITICAL | Signing of unintended messages |
| CRITICAL | Ability to forge proofs |
| CRITICAL | Unintended/undocumented recovery of private spend keys or key shares |
| CRITICAL | Reportedly received funds that weren't actually received/spendable |
| HIGH | Incorrect/incomplete cryptographic formulae within a verifier's callstack |
| MEDIUM | Undocumented transcript collision |
| LOW | Undocumented panic reachable from a public API |
| LOW | Non-constant time implementation with regards to secret data |
| LOW | Incorrect/incomplete cryptographic formulae within a prover's callstack |

**OUT OF SCOPE:** BFT assumptions, best practices, experimental cross-group DLEq, test-only code, unsafe-only bugs, 51%/Sybil/centralization.

> Scope note relevant to F-17: any attack requiring a single party to hold ≥⅓ stake (to meet the consensus weight threshold) is the out-of-scope centralization case.

---

## FINDINGS REGISTER — current as of Session 37 (label corrected from the stale "Session 27"; data has been current since Session 36)

| ID | Title | Severity | Crate | Status |
|---|---|---|---|---|
| F-01 | PublicKey::from_bytes().unwrap() on identity group key in verify() | **Low** | frost-schnorrkel | Confirmed |
| F-02 | Signature::from_bytes().unwrap() in verify() | Informational | frost-schnorrkel | Cleared — trigger impossible |
| F-03 | sign_share panics on context >= 4GB | Informational | frost-schnorrkel | Cleared — unreachable |
| F-04 | verify() unwrap before sign_share() | **Low** | frost-schnorrkel | Runtime confirmed |
| F-05 | Identity nonce sum panic path | Informational | frost-schnorrkel | Cleared — infeasible |
| F-06 | Key ordering undocumented in dkg-musig | Informational | dkg-musig | Confirmed — caller responsibility |
| F-07 | key_gen() accepts participants == 0 — Ok(empty) fail-open | **Low** | dkg-dealer | Runtime confirmed |
| F-08 | ThresholdKeys::new() panic on short Interpolation::Constant | **Low → LIKELY KNOWN (issue #295)** | dkg | Confirmed reachable, but matches acknowledged issue #295 (trusted-struct deserialization panic) → likely ineligible; drop |
| F-09 | blame APIs panic on out-of-set participant IDs | **Low** | dkg-pedpop | Confirmed — adversarial by design |
| F-10 | Outsider transit tampering misattributed as sender blame | Informational | dkg-pedpop | Confirmed |
| F-11 | Fixed IV + no ephemeral scalar uniqueness check — keystream reuse on RNG failure | **Low** | dkg-pedpop | Confirmed |
| F-12 | musig_key accepts identity point without rejection | Informational | dkg-musig | Confirmed — crate contract covers it |
| F-13 | multiexp() panics on empty input | ~~Low~~ **REMOVED** | multiexp | False positive — Algorithm::Null handles empty |
| F-14 | complete() panics on sparse peer-proof map | **Low** | dkg-promote | Confirmed — same pattern as F-09 |
| F-15 | SchnorrSignature::verify() accepts identity public key — forged SignCompleted + signed tx starvation | **🔴 High CONFIRMED** | crypto/schnorr + coordinator | CLOSED |
| F-16 | message-queue client receiver missing sender-signature verification (client.rs:202 TODO) | **Medium candidate** | message-queue | Flagged — needs dedicated chunk |
| F-17 | SchnorrAggregate::verify() accepts identity validator keys on Tendermint commit path (A=identity collapses signer term; same class as F-15) | **🔴 High CONFIRMED** | crypto/schnorr + coordinator/tributary | Critical DENIED; library-level verifier defect |
| F-18 | verify_aggregate read_G(signer).unwrap() panics on non-canonical validator-ID bytes from p2p block-sync | **Low** (panic; resolved S38) | coordinator/tributary | Confirmed reachable; maps to "undocumented panic reachable from a public API" — no availability tier ⇒ Low regardless of blast radius |
| F-19 | verify_commit threshold check self.weights[&v] panics on unregistered validator ID (reached via F-17 identity forgery) | **Low** (panic; resolved S38) | coordinator/tributary | Confirmed; chained behind F-17; same Low panic bucket as F-18 |
| F-25 | Forged Unsigned SignCompleted → fatal_slash() of attacker-chosen non-validator first_signer → persisted FatalSlashes → still_present_shares spec.i().expect() panic → global hook → process exit; finalized/persisted block ⇒ restart re-detonates (permanent) | **Low ($250) — maps to "undocumented panic reachable from a public API" (the ONLY availability impact in the live table). PoC-CONFIRMED real-path, both triggers. Was mis-rated Critical on a phantom network-shutdown tier; corrected S37.** | coordinator (Primacy of Impact; + crypto/schnorr for identity variant) | Candidate; source+PoC confirmed S37; no defeater; severity = Low per live table |

**Running totals (current as of Session 38 — VERIFIED against the live Immunefi impacts table, PDF-confirmed):** 19 logged (F-13 removed → 18 active). Mapped to the 9 live impacts (4 Critical / 1 High / 1 Medium / 3 Low). **F-15 = High** (verifier-callstack formula; Critical ceiling pending DKG-PoP forge-proofs recon). **F-17 = High** (verifier-formula; Critical denied — one-slot/threshold/panic argument). **F-22 = Critical** (received-funds-not-actually-received/spendable; UNVERIFIED candidate; payout = 10% funds-at-risk capped $30k, may compute low / be ineligible under the pre-mainnet active-development caveat). **F-25 / F-18 / F-19 = Low** (undocumented panic from a public API — the only availability-flavored bucket; permanence/network-wide does NOT raise the bucket; may package together). **F-16 = Low or OUT OF SCOPE** (core missing-sig-verify maps to "signing of unintended messages"=Critical ONLY if a no-re-validation signing sink is confirmed, else no impact match → OOS; its borsh-`expect` sub-obs = Low panic). **7 Lows re-map:** F-01/04/08/14 = Low panic; F-09 = Low panic (possibly OOS — "adversarial by design" may be a documented contract); F-11 = Low prover-formula (theoretical key-share recovery only under RNG failure → won't be honored); F-07 = no clean impact match → likely OOS. **0 Critical confirmed; F-22 is the one confirmed-bucket Critical CANDIDATE; F-15→forge-proofs and cached-preprocess→key-recovery are the unproven crypto-Critical leads.** ⚠ History: Session 37 mis-shifted the impacts table one row (→ F-15 Medium, F-22 High) — corrected S38. Sessions 35/36 over-rated the panics via a non-existent network-shutdown tier — corrected S37/S38 to Low. **Unverified candidates:** F-21–F-24 (panics → Low if reachable), leads B/E, cached-preprocess Critical chain.

---

## OPEN ITEMS

| Item | Detail | Blocks? |
|---|---|---|
| F-18/F-19 severity — ✅ RESOLVED (S38) | Resolved to **Low**: the live impacts table has no availability/liveness tier, so a process-exit panic maps only to "undocumented panic reachable from a public API" (Low) regardless of network-wide blast radius or the per-tributary-handler consensus-gossip question. The handler-scope detail no longer moves severity (it only affected the defunct Medium-via-shutdown argument). Containment-refutation (global hook is real) still HOLDS as a technical fact. | No |
| F-16 dedicated chunk | message-queue client.rs:202 missing sender-signature verification. Scheduled after remaining crates. | No |
| Chain-mapping phase | After all crates. F-17 → F-19 seed already logged. | No |

---

## REPORT FRAMING — arguments to use at submission time
> Captured so they survive across sessions. These are the "why accept this at this severity" points, beyond the raw bug. **Do NOT draft report prose until flexible-transcript closes, F-16 is scoped, and chain-mapping has run — the final set isn't settled.**

**F-15 + F-17 (the two that need defending):**
- **Foundation-is-sound argument (strongest asset).** ciphersuite, multiexp, and dalek-ff-group were all audited and confirmed correct (is_identity exact, canonical decode, multiexp kernel, constant-time). State explicitly in both write-ups that the verifier defect is *isolated*, not an artifact of a buggy primitive. Preempts the "isn't this just the underlying library?" deflection.
- **Trust-boundary framing (the correct convention argument).** The defect is a missing identity guard where *untrusted network input* crosses into a verifier. F-15: `SignCompleted` first_signer from unauthenticated gossip. F-17: validator keys decoded straight from the received commit. Frame it as untrusted-input-not-guarded-at-the-boundary.
- **DO NOT cite the FROST guard (curve/mod.rs:123) as an identity-in-verify precedent.** It is a nonce-commitment guard (zero-nonce-leaks-share, W-06 class) — different object, different attack. Using it is a false parallel a sharp reviewer dismantles, costing credibility on the whole report. Dropped deliberately.

**F-17 specifically:**
- Lead with the **F-17 → F-19 chain** as the concrete impact, and be **upfront that consensus forgery is blocked**. Volunteer the airtight Critical-denial (identity is ONE point → dedup limits to one forgeable slot → can't meet ⅔ threshold → threshold check panics on the unregistered key anyway). Arguing honestly against our own escalation builds reviewer trust.
- Honest caveat: F-17's *standalone* impact is thin (its value is the verifier-formula defect by taxonomy + the F-19 chain precondition). This is the framing most likely to shift based on chain-mapping results.

**F-15 specifically:**
- Concrete impact = the unsigned-flood starvation vector (uncapped unsigned mempool, hash-only dedup, unauthenticated gossip per the p2p.rs:395 TODO). Honest boundary: `confirm_completion` independently re-validates → no custody loss → correctly High not Critical.

**F-18 / F-19:**
- Keep as TWO findings (one root cause each) even if packaged in one report. Different files/lines/triggers/fixes.
- **Do not write them up until the OPEN-ITEMS severity-refinement is resolved** (does the dying per-tributary handler carry consensus gossip → Low vs Medium).

**Cross-cutting credibility:**
- The cleared-hypotheses list is itself report material — demonstrates thoroughness, not luck.
- Mention the pedpop blame-vartime near-miss (safe by construction, not by stated intent) — worth one sentence; the Serai team may want it hardened.

---

## PHASE 2 — VERIFICATION & GAPS (the re-strategy backlog)
> **STATUS: audit is PAUSED for re-strategy after the F-16 chunk completes.** Breadth (all 13 crypto crates) is done; depth/verification is NOT. Chain-mapping is deferred until this phase is planned.

**Honest status correction — read before trusting the register.** Every "CONFIRMED" label in this file means *"reasoned valid by a single reviewer (Claude) over pasted code,"* NOT *"proven."* No PoCs have been run. No dynamic testing since early sessions. All severity calls have passed through one reasoner with no independent check. The reasoning discipline is high (Critical correctly denied twice; false positives F-13/F-20 cleared), but verification is absent. Weight accordingly.

**Triage of value (cold, unbiased):** Value is concentrated in F-15, contingent on a working PoC + a defensible impact number. F-17 is the most contestable High (thin standalone impact — verifier defect proven harmless downstream; severity rests on single-pass reasoning). F-18/F-19 severity unresolved. Lows likely accepted at small value. Completeness within scope is unverified (clean crates were *cleared, not broken*).

**Gap backlog (the actual Phase-2 work):**
1. **PoCs — highest leverage, required by the bounty anyway.** F-15/F-17 identity forgery (does verify actually accept it?); F-18/F-19 crafted-commit panic (also resolves the Low-vs-Medium question by observation). PoC failure = a finding about the finding.
2. **F-15 starvation magnitude** ("~7,700–22,900 tx/block") — inherited ~Session 23, NEVER re-derived, load-bearing for impact. Re-compute from real mempool/block constants or stop citing the number.
3. **F-15 Critical-denial** rests on `confirm_completion` re-validation in processor code we may not have fully seen — re-verify the re-validation has no gap.
4. **F-17 severity** — independent second pass on BOTH the High justification and the Critical-denial reasoning (one-point→one-slot→threshold). Load-bearing in both directions.
5. **F-16 severity** — most likely finding to move (Low↔High on transport/trust boundary); flag for independent rating, not solo.
6. **Blind re-derivation** of load-bearing claims by a model that has NOT seen Claude's reasoning (neutral phrasing: "does this property hold?" not "confirm X"). Targets: F-17 kill-argument, F-15 re-validation, multiexp vartime-on-scalars (cleared F-20).
7. **Adversarial pass on clean closes** (modular-frost, dleq, bitcoin-serai) — "assume there IS a bug, where?" — ideally a different model than cleared them.
8. **Negative-space list** — surfaces deprioritized / out-of-scope / never-asked (processor internals, InInstruction parsing, substrate pallets beyond validator-sets). Start NOW — lossy to reconstruct later.
9. **Independent severity pass** overall — every High/Medium has been one reasoner's call.

**Method note for Phase 2:** re-verify from SOURCE and PoC, not from this register. Re-reading the file only confirms we transcribed our own reasoning faithfully; it does not catch errors. Go back to the code.

**What NOT to over-invest in:** the 7 robustness Lows — diminishing returns. Concentrate the budget on F-15/F-17.

---

## F-15 COMPLETE RECORD — CLOSED

| Field | Detail |
|---|---|
| Title | SchnorrSignature::verify() accepts identity public key |
| Severity | **High — CONFIRMED AND CLOSED** |
| Crate | crypto/schnorr + coordinator |
| Root cause | verify() checks R + c*A - s*G == 0. If A = identity, collapses to R = s*G. Attacker picks any s, sets R = s*G — trivial forgery, no private key needed. Ciphersuite::read_G accepts identity as canonical; no non-identity guard exists in verify(). |
| Primary impact | Forged unsigned SignCompleted tx passes coordinator-layer verify(). Transaction kind is Unsigned — no signing key required to submit. |
| Secondary impact | Unsigned mempool has no per-sender/global cap. Dedup is hash-only. P2P gossip open to unauthenticated peers (validator-only auth TODO at coordinator/src/p2p.rs:395). Attacker floods ~7,700–22,900 forged txs per block. Block assembly orders unsigned first, trims signed from tail — signed Sign/SubstrateSign protocol txs starved. Reattempt base delay ~50 blocks (~5 min). |
| Custody impact | NOT confirmed — processor confirm_completion independently verifies against network (Bitcoin: fetch tx by eventuality txid; Monero: fetch + eventuality match; Ethereum: cryptographic claim validation). Processor is fail-closed on None/Err. |
| Attacker capability | Gossip-ingress peer. No validator stake, fee, or identity required in unsigned mempool path. |
| Immunefi bucket | High — "Incorrect/incomplete cryptographic formulae within a verifier's callstack" |
| Submission note | Add: p2p gossip lacks validator-only authentication (TODO at coordinator/src/p2p.rs:395). |

---

## F-17 COMPLETE RECORD — High, Critical DENIED

| Field | Detail |
|---|---|
| Title | SchnorrAggregate::verify() accepts identity validator keys on Tendermint commit-verification path |
| Severity | **High — CONFIRMED.** Critical (consensus forgery) DENIED. |
| Crate | crypto/schnorr (root defect) + coordinator/tributary (reachability) |
| Root cause | Same class as F-15. `SchnorrAggregate::verify` term for signer i is `z_i*R_i + z_i*c_i*A_i`. If `A_i = identity`, the `c_i*A_i` term vanishes → that slot is forgeable with no key (pick R_i = kG, s contribution = z_i*k). No identity guard in the aggregate verify path; keys reach verify decoded directly from the received commit (`commit.validators`, raw [u8;32]) via `read_G`, not from trusted validator-set lookup. |
| Why Critical is denied (airtight, scope-independent) | (1) Identity is ONE point → appears at most once per validator set (HashSet dedup in verify_commit + key uniqueness) → at most one forgeable slot. (2) One slot cannot meet the ⅔+1 weight threshold unless that single identity-keyed validator holds ⅔ stake = out-of-scope centralization. (3) Real (non-identity) slots keep unknown-dlog pubkeys in the equation; H-17b confirmed the challenge binds R, so half-aggregation is sound for them — cannot forge. (4) Even reaching the threshold check with the unregistered identity key panics at `weights[&identity]` (= F-19) before any acceptance. |
| Reachability | p2p block-sync: HeartbeatBatch → sync_block → sync_block_internal → verify_commit → verify_aggregate. `Commit::decode` performs no point validation. |
| Library-level framing | The reusable defect is in crypto/schnorr: neither `SchnorrSignature::verify` (F-15) nor `SchnorrAggregate::verify` (F-17) guards identity. Any consumer inherits it. Coordinator reachability gates severity; the defect itself is in-scope crypto. |
| Immunefi bucket | High — "Incorrect/incomplete cryptographic formulae within a verifier's callstack." Submission should be upfront that consensus forgery is blocked and lead the concrete impact with the F-17→F-19 chain. |
| Key call sites | challenge() tendermint/mod.rs:51; aggregate() :181; verify_aggregate() :201; verify_commit() ext.rs:256; weight() mod.rs:232; ingress p2p.rs:198/881/971; registration spec.rs:56 + substrate validator-sets pallet; read_G ciphersuite/lib.rs:91 + dalek-ff-group/lib.rs:429. |

---

## F-18 / F-19 RECORD — ⚠ pre-S35; containment REFUTED (S35) + severity = Low (S38)

> The task-isolation/containment argument in this block was REFUTED in Session 35 (global panic hook → whole-process exit). Severity resolved to **Low** in S38 (no availability tier). Kept for the source-traced reachability detail, which still holds.

- **F-18:** `read_G(s).unwrap()` in `verify_aggregate` panics on attacker-supplied non-canonical validator-ID bytes. Fires DURING verify_aggregate (before any signature/threshold check) — the **more accessible** DoS; any gossip peer triggers it with one malformed ID + matching Rs length, no forgery required.
- **F-19:** `self.weights[&validator]` (HashMap index) in `verify_commit` threshold check panics on an unregistered ID. Only reachable AFTER `verify_aggregate` returns true, which for an unregistered key requires the F-17 identity collapse → **chained behind F-17**.
- **Keep separate (not merged):** different files/lines, different triggering bytes, different preconditions, different fixes. May be *packaged* as one Immunefi report (same severity/bucket/attacker) but tracked as two.
- **Containment (Chunk 7):** No `catch_unwind`, no `panic = "abort"`. `sync_block_internal` runs on the spawned **per-tributary P2P handler task** (p2p.rs:881), which is the mandatory first `verify_commit` gate. Both panic inputs die there before any commit is forwarded to the core consensus task (`TendermintMachine::run`, spawned at tributary/src/lib.rs:191). The consensus task re-verifies, and `add_block`'s `assert!(verify_commit(...))` (mod.rs:362) is on the consensus task but only sees pre-validated/locally-built commits — no public-input route confirmed. Validators set is fixed per session → no TOCTOU between the two verifications.
- **OPEN:** the dying handler task is not respawned, and likely also carries consensus gossip for that tributary → potential persistent per-node consensus-liveness DoS until restart (→ Medium). Confirm the handler's full message-kind coverage before submission.

---

## ACTIVE INVESTIGATION — F-16 MESSAGE-QUEUE DEDICATED CHUNK (NEXT)

**Target:** `message-queue` — missing sender-signature verification (`client.rs:202` TODO in `next()` receiver) | **Current label:** Medium candidate
**Why it matters:** Lone Medium candidate, severity WIDE-OPEN on reachability — same gating discipline as F-15/F-17. Low/defense-in-depth if the queue is a trusted-internal-only transport; potentially High if network-injectable AND a forged inter-service message triggers something downstream not independently re-validated.

### Chunk questions
1. `next()` receiver body at `message-queue/src/client.rs:202` — quote the TODO; state exactly what verification is skipped (never checked / checked-but-ignored / deferred to caller).
2. Sender side: how messages are signed and with which keys (env-configured KEYS registry, previously cleared as not peer-injectable). Do outgoing messages carry a signature at all?
3. Trust boundary / reachability: transport (local socket / TCP / authenticated?), who can connect, network-exposed vs co-located trusted services. Show bind/listen + any connection auth. THIS IS THE HINGE.
4. Downstream impact: what a consumer (coordinator/processor) does with a received message; what a forged message would trigger (unintended signing, fund movement, state transition); does the consumer independently re-validate (like confirm_completion for F-15) or trust queue output?
5. Verdict inputs: any path where an unauthenticated/forged message is acted on without downstream re-validation (name file:line), or confirm defense-in-depth behind a trusted boundary.

Do NOT assign final severity in-chunk; report reachability + impact facts, rate together.

**After F-16:** PAUSE — do NOT proceed to chain-mapping. Move to PHASE 2 re-strategy (see PHASE 2 section). Chain-mapping happens only after verification is planned. F-16's result is the finding most likely to move on severity — flag it for independent rating during Phase 2, don't lock severity solo.

---

## PLANNED POST-AUDIT PHASE — FINDING CHAIN ANALYSIS

After all crates + the F-16 chunk, run a dedicated chain-mapping session:
- Lay all findings on the table as a directed graph; find shared preconditions and output→input feeds.
- Identify 2–3 step chains where each link is near-certain.
- Target: compose individual findings into a Critical-severity attack chain.

**Confirmed chain seed:**
- **F-17 → F-19:** identity-slot aggregate forgery satisfies `verify_aggregate`, unlocking the `weights[&v]` threshold panic. Currently capped at Low/DoS by task isolation; ceiling rises if F-18/F-19 severity-refinement shows the consensus task is affected.

**Other seeds (preliminary — not yet validated):**
- F-15 (unsigned flood starves signed txs) → does tx starvation make F-11 (keystream reuse) more likely?
- F-16 (message-queue TODO sender verification) → does unverified queue payload enable injection that bypasses processor re-validation?
- F-09/F-14 (public API panics) → can adversarial participant IDs be injected through any path F-15/F-17 opens?

---

## KEY TECHNICAL CONSTRUCTIONS (all verified)

- modular-frost binding factor: `rho_transcript = new("FROST_rho") || group_key || H(msg) || H(H(preprocesses))` — CORRECT
- dkg-musig binding factor: `hash_to_F("dkg-musig", context || keys_len || all_keys || i)` — SAFE
- frost-schnorrkel challenge: signing_context → proto_name → commit pk → commit R → challenge_scalar — CORRECT
- frost-schnorrkel transcript: LE32(context_len) || context || msg — collision-resistant
- dkg-pedpop encryption: ChaCha20 (fixed IV b"DKG IV v0.2\0") + ephemeral ECDH + external Schnorr PoP + DLEq blame
- bitcoin-serai tweak: TapTweakHash(P[1..]) reduced scalar → offset() → conditional negate — BIP-341 compliant
- dkg-promote transcript: RecommendedTranscript::new("DKG Generator Promotion v0.2") || group_key || participant → domain_separate("dleq") → per-generator (generator, nonce, point) — CORRECT
- crypto/dleq proof: r = Zeroizing(random), Rj = r*Gj, Aj = x*Gj, c = H(transcript), s = r + cx — standard joint-challenge DLEq — CORRECT
- crypto/schnorr verify: R + c*A - s*G == 0 via multiexp_vartime — CORRECT but NO identity guard on A (F-15)
- crypto/schnorr SignCompleted: first_signer from raw tx payload, Ristretto::read_G accepts identity, no non-identity gate before verify() (F-15)
- crypto/schnorr aggregate weight(): loads ceil((NUM_BITS+128)/8) bytes via transcript challenge chaining (aggregation_weight / aggregation_weight_continued), base-2^64 Horner accumulation, reduced mod order → bias < 2^-128. **CS-3.8.5 fix HOLDS** (re-derived for Ristretto: 48 bytes / 384 bits).
- crypto/schnorr aggregate verify: `Σ z_i*R_i + Σ z_i*c_i*A_i − s*G == 0`. Transcript absorbs ONLY challenges (not keys, not R) → soundness rests entirely on c_i binding R_i. **CS-3.8.4 fix HOLDS.** NO identity guard on A_i (F-17).
- Tendermint challenge(): RecommendedTranscript("Tributary Chain Tendermint Message") → genesis → key → nonce(=R_i) → message(=end_time||block_id) → from_bytes_mod_order_wide. **Binds R_i (H-17b cleared).** Does not directly hash height/round, but block_id (a hash) uniquely binds the block — not exploitable.
- Tendermint verify_commit gate order: HashSet dedup → verify_aggregate (signature) → weight-threshold (⅔+1). Threshold uses trusted Validators weight map keyed by registered pubkey bytes; ValidatorId IS the key.
- ciphersuite read_G: from_bytes (rejects off-curve/invalid) + canonical re-encode check (point.to_bytes()==encoding). NO is_identity check → identity admitted. Guarantee = "valid canonical encoding," NOT "usable non-identity key." Documented root of F-15/F-17; convention is consumer-guards-identity.
- ciphersuite read_F: rejects scalars ≥ order on all curves (dalek from_repr; k256/p256 ct_lt(ORDER); ed448 ct_lt(MODULUS) & high-byte==0). Sound.
- ciphersuite hash_to_F: Ristretto/Ed25519/Ed448 use naive dst||data (documented transposition hazard); kp256 (secp256k1/P256) uses expand_message_xmd with separated DST (safe). Transposition INERT in production — all DSTs fixed, no same-curve prefix relationship (verified by enumeration). kp256 wide reduction k=128/L=48 unbiased.
- multiexp: algorithm() dispatches Null/Single/Straus/Pippenger; multiexp & multiexp_vartime compute Σ scalar_i·point_i identically; vartime timing depends on SCALARS (digit/bucket skips), not points. is_identity on result is sound (no short-circuit). **This is the verify kernel under F-15/F-17 — confirmed correct.**
- multiexp BatchVerifier: first weight=ONE, rest random-nonzero from caller CSPRNG; reweight (scalar*u, point). Random-linear-combination defense against cross-statement cancellation holds (u_1=1 is sound). Mix-and-match resistance for schnorr batch confirmed here.
- pedpop blame vartime (CLEARED, no F-20): decrypted share converted to POINT (G*-share) with public ONE scalar before queueing + zeroized; attacker can force verify_with_vartime_blame→blame_vartime over honest statements, but their scalars are public, so no secret leaks via vartime timing. Near-miss — safe by construction, not by documented intent.
- dalek-ff-group: is_identity = ct_eq(identity) exact; to_bytes = dalek canonical compression (Edwards + Ristretto) → read_G round-trip assumption holds; from_repr rejects ≥ order; ct_eq is direct delegation (no variable-time fallback); Mul macro adds no secret-dependent branch (G*-share stays CT); from_hash = 64-byte wide reduction, output zeroized. from_bytes admits identity = documented F-15/F-17 root. **Foundation under F-15/F-17 confirmed sound across ciphersuite + multiexp + dalek-ff-group.**
- flexible-transcript (crypto/transcript): CLOSED clean. DigestTranscript framing = `kind_u8 || len_u64_le || value` per field, distinct kind tags, label explicitly framed → INJECTIVE. challenge() forks live(Continued) vs finalized-clone(Challenged) → no continuation ambiguity (CS-3.9.1 fixed). MerlinTranscript injective via STROBE meta_ad(label)+meta_ad(len)+ad(message); dom-sep reserved-label assert blocks aliasing (CS-3.9.3 fixed). **W-05, hash_to_F transposition, SignCompleted framing all STAY CLOSED.** 3 informational non-findings (not reportable): (a) SecureDigest guarantees ≥32 but not mult-of-8 while weight() assumes both — latent, NOT production-reachable (all prod digests mult-of-8; weight() Ristretto-only); (b) dom-sep assert is &'static-label dev guard, not attacker-reachable; (c) Merlin >u32::MAX message panic = F-03 class, unreachable.

---

## CLEARED HYPOTHESES — COMPLETE LIST

| Hypothesis | Verdict |
|---|---|
| Frozen Heart (frost-schnorrkel) | CLEARED |
| TSSHOCK alpha-shuffle (dkg-musig) | CLEARED |
| IetfTranscript::rng_seed unreachable | CLEARED |
| Q2 — Plaintext leak on PoP failure | CLEARED — explicit zeroize chain |
| Q3 — Bit-flip attack via unbound ciphertext | CLEARED — ciphertext bound in PoP |
| Nonce reuse in modular-frost | CLEARED — type-system prevention |
| Cross-session replay in pedpop | CLEARED — context bound in PoP |
| Cross-recipient replay in pedpop | CLEARED — ECDH mismatch + downstream rejection |
| W-01 attacker-reachable via script tree | CLEARED — tweak is non-adversarial hash fixed-point |
| W-01 Ethereum gap as negligence | CLEARED — asymmetry is mathematically justified |
| W-02 schnorrkel conversions in bitcoin path | CLEARED — no schnorrkel types used |
| W-03 Algorithm::verify() outside FROST machine | CLEARED — only in trait impl body |
| W-05 transcript collision in frost-schnorrkel | CLEARED — Merlin + LE32 prefix |
| W-06 adversarial nonce forcing R to identity | CLEARED — binding factor math |
| bitcoin-serai transaction message substitution | CLEARED |
| bitcoin-serai sighash type counterparty selection | CLEARED |
| bitcoin-serai participant set injection | CLEARED |
| bitcoin-serai replay protection | CLEARED |
| Per-output offset as second W-01 path | CLEARED — fails via None |
| dkg-promote cross-session proof portability | CLEARED — binds to exact verification share |
| dkg-promote rogue-key on complete() | CLEARED |
| dkg-promote witness leakage in promote() | CLEARED — standard ZK |
| dleq identity-point collapse in verify() | CLEARED — witness x=0, not forgery |
| dleq mix-and-match forgery via independent challenges | CLEARED — joint challenge |
| schnorr nonce reuse internal to crate | CLEARED — nonce externalized |
| schnorr batch verification mix-and-match forgery | CLEARED — fresh random scalars (BatchVerifier + verifier RNG) |
| F-15 Critical escalation via processor bypass | CLEARED — confirm_completion is independent |
| F-15 first-signer-to-real-signer upgrade in transit | CLEARED — first_signer not forwarded to processor |
| message-queue verify() peer-injectable keys | CLEARED — KEYS registry is env-configured |
| **H-17b — aggregate challenge non-binding to R** | **CLEARED — Tendermint challenge() absorbs nonce=R_i** |
| **F-17 Critical via all-identity commit** | **CLEARED — dedup limits identity to one slot; can't meet threshold; panics at F-19** |
| **F-17 Critical via signature-only caller** | **CLEARED — all production paths use full verify_commit (incl. threshold)** |
| **F-18/F-19 escalation to process abort** | **⚠ REOPENED/REFUTED (S35): the task-isolation basis was WRONG — the global panic hook DOES escalate any task panic to whole-process exit. Outcome maps to Low (S38) regardless, since no availability tier exists.** |
| **hash_to_F DST-transposition (Ristretto/Ed25519/Ed448 naive concat)** | **CLEARED — all production DSTs fixed, no same-curve prefix relationship; hazard inert** |
| **FROST identity guard (curve/mod.rs:123) as F-15/F-17 parallel** | **CLEARED as parallel — it's a nonce-commitment guard (zero-nonce-leaks-share, W-06 class), different object/context; DROP from F-15/F-17 submission framing** |
| **multiexp core arithmetic / is_identity soundness (F-15/F-17 kernel)** | **CLEARED — Straus/Pippenger correct, vartime timing scalar-dependent only, is_identity sound** |
| **multiexp BatchVerifier mix-and-match (u_1=1)** | **CLEARED — fixing first weight to ONE is sound; random weights prevent cancellation** |
| **pedpop blame vartime leaks secret share (candidate F-20)** | **CLEARED — share is pre-multiplied into a point with public ONE scalar; vartime timing leaks only public scalars** |
| **dalek-ff-group wrapper-introduced bug in is_identity / canonical decode / CT** | **CLEARED — faithful dalek delegation, to_bytes canonical, no variable-time fallback, no secret-dependent branch** |
| **flexible-transcript framing non-injective (would reopen W-05 / hash_to_F / SignCompleted)** | **CLEARED — DigestTranscript & Merlin framing both injective; prior clearances stay closed** |
| **CS-3.9.1 (digest bound) / CS-3.9.3 (Merlin domain-sep) regressed** | **CLEARED — both fixes confirmed in current code (challenge fork; dom-sep reserved-label assert)** |
| **SecureDigest/weight() mult-of-8 mismatch (candidate panic)** | **CLEARED for bounty — latent library footgun, NOT production-reachable (all prod digests mult-of-8); Informational hardening note only** |

---

## CROSS-CRATE WATCH LIST — ALL CLOSED

| ID | Pattern | Final Status |
|---|---|---|
| W-01 | Identity group key after offset/tweak | CLOSED — Informational; BIP-341 tweak non-adversarial, hash fixed-point |
| W-02 | unwrap() on schnorrkel type conversions | CLOSED — no schnorrkel conversions in bitcoin path |
| W-03 | Algorithm::verify() called outside FROST state machine | CLOSED — only in trait impl body |
| W-04 | Negligible probability identity = impossible | CLOSED — spec-compliant per RFC 9591 |
| W-05 | Context/message length assumptions | CLOSED — Merlin + explicit LE32 length prefix |
| W-06 | x(R) panic on identity nonce sum in HRAM | CLOSED — binding factor math prevents forcing |

**Zero open watches.** (New identity-handling concerns are now tracked as the ciphersuite Chunk 1 focus, not a watch item.)

---

## SESSION LOG

### Sessions 1–17 — April 23 to May 16, 2026
frost-schnorrkel, dkg-musig, modular-frost, dkg family, bitcoin-serai, W-05 resolution. All complete.

### Session 18 — May 17, 2026
dkg-promote Chunk 1: orientation, DLEq surface map, no critical findings.

### Session 19 — May 17, 2026
dkg-promote Chunk 2: cross-session portability CLEARED. F-14 confirmed (Low).

### Session 20 — May 17, 2026
dkg-promote Chunk 3 + crypto/dleq: both crates closed clean. F-14 confirmed.

### Session 21 — May 19, 2026
crypto/schnorr Chunk 1: crate mapped. F-15 flagged as High/Critical candidate.

### Session 22 — May 26, 2026
Findings register corrections (F-10/11/12 added, F-13 removed, F-04/F-07 upgrades, F-15 assigned). Chunk 2: F-15 reachability confirmed; processor re-validation independent — Critical denied. Chunk 3: confirm_completion network-backed/fail-closed; F-15 = High.

### Session 23 — May 26, 2026
Extended F-15 impact exploration. Starvation vector identified. F-16 candidate logged.

### Session 24 — May 26, 2026
F-15 starvation deep-dive (Chunk 3B). Unsigned mempool flood quantified. Signed tx starvation confirmed. No irreversible fund impact. F-15 = upper-end High, CLOSED.

### Session 25 — May 27, 2026
crypto/schnorr Chunk 4 (aggregate.rs) + Chunk 5 (Tendermint caller). CS-3.8.4/3.8.5 fixes re-verified holding. Half-aggregation equation correct. H-17b CLEARED (challenge binds R_i). **F-17 confirmed High** (identity validator keys admitted into verify, no rejection, keys decoded from commit not trusted lookup). **F-18 confirmed** (read_G unwrap panic from p2p sync).

### Session 26 — May 27, 2026
crypto/schnorr Chunk 6 (F-17 escalation). `weight()` is `self.weights[&v]` → panics on unknown ID = **F-19** (new). Substrate validator-sets pallet has no identity rejection, but identity registration with threshold weight not achievable (single point + ⅔ stake = out of scope). All production trust paths use full verify_commit; no signature-only path. F-18 task-isolated. **F-17 capped at High, Critical DENIED.** F-18/F-19 = Low/DoS task-isolated.

### Session 27 — May 27, 2026
crypto/schnorr Chunk 7 (closure). Task context resolved: sync_block_internal (isolated per-tributary handler) is the mandatory first verify_commit gate; panic inputs die there; consensus task + add_block assert shielded by double-verify; no TOCTOU. F-17→F-19 chain seed recorded. Crate finding set complete (F-15, F-17, F-18, F-19); batch path cleared. **crypto/schnorr CLOSED.** Open: F-18/F-19 severity-refinement (does dying handler carry consensus gossip → possibly Medium). Next target: crypto/ciphersuite.

### Session 28 — May 29, 2026
ciphersuite Chunk 1 (orientation + identity sweep). read_G admits canonical identity (documented contract, NOT a standalone finding — root of F-15/F-17). read_F rejects ≥ order. hash_to_F naive dst||data on Ristretto/Ed25519/Ed448 (documented transposition hazard); kp256 safe. No new finding. Flagged hash_to_F DST enumeration + frost-guard verification for Chunk 2.

### Session 29 — May 29, 2026
ciphersuite Chunk 2 (close-out). hash_to_F transposition INERT — all production DSTs fixed, no same-curve prefix relationship (enumerated Ristretto/Ed25519/secp256k1 sites; P256/Ed448 not instantiated in production). SignCompleted challenge safe (fixed DST + length-framed transcript data). FROST guard at curve/mod.rs:123 confirmed = nonce-commitment guard (zero-nonce-leaks-share), NOT a pubkey parallel for F-15/F-17 → dropped from submission framing. **ciphersuite CLOSED clean.**

### Session 30 — May 29, 2026
multiexp completion pass. Core arithmetic correct (Straus/Pippenger, both branches agree); BatchVerifier soundness confirmed (random weights, u_1=1 sound) — this is the verify kernel under F-15/F-17. F-13 stays false positive. Surfaced one downstream candidate: secret share possibly reaching vartime path in pedpop blame. **multiexp CLOSED clean** pending the pedpop check.

### Session 31 — May 29, 2026
pedpop blame constant-time check. CLEARED, no F-20: decrypted share is converted to a point (G*-share) with public ONE scalar and zeroized before queueing; multiexp vartime timing depends only on scalars, so the attacker-forceable blame_vartime fallback over honest statements leaks nothing secret. Near-miss (safe by construction, no documenting comment). multiexp stays CLOSED clean.

### Session 32 — May 29, 2026
dalek-ff-group Chunk 1 (wrapper correctness + CT). All load-bearing properties under F-15/F-17 confirmed: is_identity exact ct_eq; from_bytes rejects invalid (identity admitted = documented root); to_bytes canonical (validates read_G round-trip); from_repr rejects ≥ order; ct_eq no variable-time fallback; Mul adds no secret-dependent branch (G*-share CT confirmed); from_hash 64-byte wide reduction zeroized. cargo test passed. No new finding. **dalek-ff-group CLOSED clean.** Foundation under F-15/F-17 now audited sound across 3 crates. Next: flexible-transcript (LAST crate).

### Session 33 — May 29, 2026
flexible-transcript Chunk 1 (LAST crate). Framing injectivity CONFIRMED for both DigestTranscript (kind‖len‖value, label framed) and Merlin (STROBE meta_ad framing + dom-sep reserved-label assert) → W-05, hash_to_F transposition, and SignCompleted clearances all STAY CLOSED. CS-3.9.1 (challenge Continued/Challenged fork) and CS-3.9.3 (dom-sep assert) fixes confirmed. 3 informational non-findings, none reportable (SecureDigest/weight mult-of-8 mismatch latent & not prod-reachable; dom-sep static-label dev guard; Merlin 4GB = F-03 class). cargo test passed. **flexible-transcript CLOSED clean. ALL SCOPED CRATES NOW AUDITED.** Next: F-16 dedicated message-queue chunk, then chain-mapping.

### Session 34 — May 30, 2026
F-16 message-queue chunk DONE (last audit step). Then PAUSED and ran the two external reviews against the whole audit: ER1 (blind re-derivation, code-only) + ER2 (adversarial red-team). Three-way comparison folded in below. Outcome: F-15 High triple-confirmed; F-15 flood number corrected; F-17 Critical-denial holds but reframed (scope+panic, not code guard); F-18/F-19 pushed UP by both reviewers with our containment claim uncorroborated; NEW processor/InInstruction surfaces found (breadth was NOT as complete as claimed). F-18/F-19 fan-out/containment Codex chunk drafted + queued. Next: that chunk → F-15 PoC → processor-surface chunk.

---

## SESSION 34 — VERIFICATION RESULTS (current truth; supersedes specific earlier values)

> Calibration unchanged: every "confirmed" here is single-reviewer reasoning over source unless marked otherwise. ER1/ER2 are TWO additional single-pass reviewers (no PoC). "Triple-confirmed" = three independent reasoners agree over source — strong, still not proven.

### F-16 — message-queue missing sender-signature verification (chunk result)
- **Defect confirmed.** `next()` receiver (`message-queue/src/client.rs`, TODO "Verify the sender's signature") never verifies the sender signature — only role-sanity asserts (`msg.from` is Coordinator/Processor). The `sig` IS carried with the message (`sig.serialize()`) and IS available at dequeue, just unchecked.
- **Enqueue path IS sig-gated** (not forgeable remotely): `queue()` signs Schnorr over `message_challenge(...)`; server `queue_message()` (`main.rs:60`) verifies against env-configured `KEYS` registry keyed by `meta.from` (key from registry, NOT wire).
- **The actual defect (framing):** auth is computed, carried, and verified end-to-end *in intent*, but only checked at ONE hop (enqueue). Consumer collapses end-to-end auth into trust in the server.
- **Transport / THE HINGE:** raw TCP, binds `0.0.0.0:2287` (all interfaces, NOT local-only), NO connection auth (TODO `main.rs:234`). `Next` explicitly unauthenticated by design ("no sensitive data on this server"). README = "intranet-trust service"; deployed as separate service via `MESSAGE_QUEUE_RPC`.
- **Reachability:** enqueue NOT remotely forgeable. F-16 reachable only via `Next`-path spoof/tamper: (1) malicious queue server, (2) compromised queue DB/server — BOTH = infra compromise, OUT OF SCOPE; (3) active MITM on the unauthenticated TCP channel — the ONLY in-scope-plausible vector, and it sits AT/PAST the documented intranet-trust boundary.
- **Downstream:** consumers act on queue output as trusted. Acted-on BEFORE any sig re-validation: processor control-plane dispatch (`processor/src/main.rs:225/242`), coordinator cosign rebroadcast (`coordinator/src/main.rs:253`), batch publish (`:342`), tributary tx creation (`:430`). Protected by LATER fail-closed checks: `Completed`→`confirm_completion`; cosign sigs (`cosign_evaluator.rs:165`); slash-report extra key check (`main.rs:300`).
- **Severity — single-reviewer lean = LOW (defense-in-depth), NOT LOCKED.** Only in-scope-plausible trigger (MITM of inter-service channel) is at/past the documented trust boundary → Low. BUT contingent on a scope-policy question source can't answer: is inter-service transport treated as untrusted? If yes → clean **Medium** (forgeable authenticated-channel messages, some acted on without re-validation), contingent path higher if a no-re-validation signing sink is confirmed. **Flagged for independent rating. Neither ER1 nor ER2 covered F-16 — it remains the only finding with zero external eyes.**
- **Sub-observations to verify:** (a) `borsh::from_slice(...).expect(...)` in both `recv()` wrappers → a forged/corrupted message via the same MITM vector PANICS the consumer (distinct panic-DoS, same reachability gate) — candidate; (b) OPEN: does the consumer even HOLD the sender's pubkey to verify? client struct shows only `self.priv_key`/`self.pub_key` — if not provisioned, the "one-line fix" framing weakens and the trusted-transport reading strengthens — needs source check; (c) adjacent: unauthenticated `Next` permits READING queued inter-service messages; the "no sensitive data" assumption is unverified (keygen/sign material may transit).
- Codex set its context.md/agent.md "next step = chain-mapping" — WRONG. **Relay correction to Codex: next = PAUSE → Phase-2 verification; chain-mapping deferred.**

### Three-way convergence (raises confidence)
- **F-15 = High — triple-confirmed** (us + ER1 + ER2), same reason: real verifier defect + cheap unsigned-flood starvation, custody gated by `confirm_completion` → not Critical. Our most robust finding; only a PoC remains.
- **F-15 flood number CORRECTED — SUPERSEDES "~7,700–22,900".** ER2 re-derived: `floor((3,001,000 − 68)/(131 + tx_hash_len))`, `BLOCK_SIZE_LIMIT=3,001,000`, 68-byte overhead (64 header + 4 tx-count), tx = `131 + tx_hash_len` bytes. h=0→**22,907**; h=32→**18,410**; h=255→**7,774**. Range = **~7,774–22,907 tx/block** (our old exact endpoints 22,908/18,411 were off-by-one). Present as APPROXIMATE pending PoC (ER2 arithmetic is itself unverified). ER1 confirmed structural precondition (uncapped unsigned mempool; hash-only dedup; SignCompleted hash includes all unsigned contents → unlimited hash-distinct forgeries). Closes Phase-2 gap #2.
- **F-15 Critical-denial independently traced through the processor (ER1)** — closes Phase-2 gap #3. Three-gate (length → known-eventuality → `confirm_completion`), `Ok(Some)` only, fail-closed. PLUS the on-chain scanner (`ScannerEvent::Completed`→`resolve_plan`→`signer.completed`) is the REAL fund-resolution driver → hardens denial (forged Completed can't move funds; funds resolve from chain observation). Per-network nuance (NEW, log but not F-15 bypass): **Bitcoin** confirm = existence-only (no confirmation-depth/finality check); **Ethereum** confirm = signature-validity (`SignedRouterCommand`), NOT on-chain-execution. Neither breaks F-15 (Bitcoin txid is signer-predicted; ETH claim needs a real threshold sig). ETH "Completed = valid sig exists, not funds-moved-on-chain" = latent **claim-before-execution accounting note** (possible chain seed, not F-15).
- **F-17 Critical-denial HOLDS but reframed — SUPERSEDES "threshold check cleanly denies".** Denial rests on SCOPE + PANIC, NOT a code guard. ER2 confirms NO identity guard at registration either: `Validators::new` (mod.rs:133-158) rejects only zero weight; `TributarySpec::new` (spec.rs:56-69) uses `read_G().expect()` with no identity reject; `read_G` admits identity. Denial = (a) an identity key would need ⅔ weight = out-of-scope centralization; (b) F-19 panic at `weights[&identity]` before any acceptance. **Writeup rule: never say the threshold check "cleanly returns false" — it denies by PANICKING** (ER1 + ER2 both stress this).
- **F-17 High under pressure (no position change).** Both ER1 ("High for node liveness if reachable over P2P") and ER2 ("weakest High; production impact is a DoS chain unless identity-registration proven") independently confirm OUR OWN documented soft spot (REPORT FRAMING already says F-17's standalone impact is thin, value is taxonomy + F-19 chain). Keep High as the claimed Immunefi bucket (verifier-formula taxonomy, which it genuinely fits); lead with honest DoS-chain framing. Most contestable High.
- **F-17 clearances corroborated by ER1:** no signature-only path (all prod `verify_commit` callers use full gate; local creation only `debug_assert`); no threshold bypass; `verify_aggregate` caller chain (`verify_commit`→`signature_scheme().verify_aggregate`, `Arc<S>` forwards only).
- **F-20-cleared corroborated (ER1)** with one curve-timing caveat (is `generator*share` itself CT?) — CLOSED by our dalek-ff-group Chunk-1 finding (Mul no secret-dependent branch). F-20-cleared now stands on two independent legs.

### F-18 / F-19 — both reviewers push severity UP; containment uncorroborated
- ER1 (blind): "High for node liveness if reachable over P2P." ER2: treats the panic/DoS chain as F-17's REAL proven impact.
- **NEITHER corroborated our Chunk-7 task-isolation/containment argument** → it is now the single least-verified load-bearing claim, with two reviewers' instincts running AGAINST our Low.
- **Ceiling-setter = FAN-OUT:** does one malformed commit panic MANY nodes (gossip fan-out → network-wide liveness halt) or one node only? F-18 is the accessible trigger (any gossip peer, no forgery, fires before threshold). F-19 is chained behind the F-17 identity-collapse.
- **Codex chunk DRAFTED + QUEUED** ("PHASE-2 VERIFICATION CHUNK — F-18/F-19 REACHABILITY & CONTAINMENT"). Questions: A relay-vs-verify ordering (the fan-out hinge), B task containment/respawn + does the handler carry consensus gossip, C consensus-task shielding + add_block public-input route, D process-panic behavior (`panic="abort"`? tokio task-panic propagation), E pre-verify auth/well-formedness gate. PENDING: user's fan-out/containment analysis (incoming next).

### NEW processor / InInstruction surfaces (ER2 §3) — BREADTH-NOT-DONE correction
> Honest correction: "ALL scoped crates audited" was true for the crypto layer + the consensus/commit path, but the processor application-input layer (InInstruction / Shorthand / scanners / batch execution) is IN Immunefi scope and was NOT deeply audited. Negative-space gap #8 came due. The following are UNVERIFIED single-pass red-team leads — confirmed counts UNCHANGED. Suggested IDs (user owns numbering):

| Cand | Surface | If confirmed | Verify |
|---|---|---|---|
| **F-21** | `Shorthand::Swap`/`SwapAndAddLiquidity` → `todo!()` panic (`shorthand.rs:46-53`) reachable from external deposit data via `instruction_from_output` (`multisigs/mod.rs:55-82`) | Cheap external-input processor DoS (no privilege) | Prod-decodable or feature-gated off? crash+respawn? per-node fan-out? Clean PoC. |
| **F-22** (highest ceiling) | Partial batch execution: `execute_batch` advances `LatestNetworkBlock`/`NextBatchDb` + emits event BEFORE executing instructions; per-instruction failure → `InstructionFailure` but batch returns `Ok` (pallet `lib.rs:264-287`; processor `mod.rs:1009-1021`) | Immunefi **CRITICAL** "reportedly received funds not actually received/spendable" — IF failure is silent drop, not refund | Scope `InstructionFailure` semantics: refund (safe) vs silent loss; attacker-induceable? |
| **F-23** | Ethereum dup-event panic: one txid as both top-level transfer and router instruction (`ethereum.rs:510-553`) | Ethereum processor DoS via crafted tx | Can attacker construct such a tx? |
| **F-24** | Scanner duplicate-output panic; comment admits Monero can force a shared output ID (`scanner.rs:593-639`, esp. `:603-606`) | Monero processor DoS | Is "only valid IDs scanned" upstream-guaranteed or reachable? |
| Lead B | `Shorthand::Raw` → privileged instruction classes `GenesisLiquidity`/`SwapToStakedSRI` (mint to special accounts) | Unexamined state-mutation surface (ER2 does NOT claim unauthorized mint — balance tied to signed batch) | Can a deposit request these cause unintended state? |
| Lead E | Bitcoin scanner clones one data payload across ALL external outputs in a tx (`bitcoin.rs:730-734`) | Duplicated instructions / unwanted refunds / co-spend griefing? | Intended or exploitable? |

### Clean-crate blind spots (ER2 §4)
- **HIGH-VALUE chain lead:** modular-frost cached-preprocess reuse. Crate enforces "never reuse cached preprocess" only as a CALLER contract; reuse → key-share recovery (Critical) (`sign.rs:83-92`, `:209-224`). Unasked: does processor persistence guarantee exactly-once deletion across crashes, reattempts, **F-15 starvation**, replay? → potential **F-15 → preprocess-reuse → key-recovery CRITICAL chain** (more serious than the F-11 keystream seed). Verify processor cached-preprocess lifecycle.
- Lower: aggregate-nonce-identity from adversarial commitment sets (revisits W-06, cleared on binding-factor math) — one-line reconfirm; dleq vacuous/identity-*generator* statements (`lib.rs:160-173`, `:264-293`) — caller-reachability check; bitcoin integration boundary = Lead E.
- **P256/Ed448:** both reviewers couldn't find prod use but can't ASSERT non-use. Residual NARROWED: kp256 `expand_message_xmd` is safe regardless of instantiation (covers P256/secp256k1); only **Ed448 naive-concat** matters, and only IF instantiated. One check closes it: enumerate prod Ciphersuite instantiations + feature flags; if Ed448 instantiated, re-check its DSTs.

### Updated totals (Session 34, calibrated — additive; confirmed counts unchanged)
**Confirmed:** 2 High (F-15, F-17), 0 Critical, 1 Medium candidate (F-16 — lean Low, UNLOCKED, no external eyes), 7 reportable Lows (F-01,04,07,08,09,11,14), 2 panic-DoS (F-18/F-19 — severity pushed UP by both reviewers, pending fan-out chunk). **NEW unverified (pending verification):** 4 processor candidates (F-21–F-24) + 2 leads (B, E) + 1 Critical-chain lead (cached-preprocess) + 1 accounting note (ETH claim-before-execution).

### Revised Phase-2 priority
1. **F-15 PoC** — highest-value confirmed finding; required by bounty; settles the number.
2. **F-18/F-19-R fan-out/containment chunk** (drafted, queued) — resolves the severity both reviewers raised + the uncorroborated containment claim.
3. **Processor-surface chunk** — triage F-22 (Critical ceiling) first, then F-21 (clean PoC), then F-23/F-24, then leads B/E.
4. **Cached-preprocess lifecycle chunk** — potential F-15→key-recovery Critical chain.
5. **Closers** — Ed448 instantiation proof; W-06/dleq one-line reconfirms.

### Still single-reviewer / open
F-16 (externals didn't touch it). All §3/§4 leads unverified. Containment for F-18/F-19 (incoming). Severity locks deferred to joint rating.

### Session 35 — May 31, 2026
First Phase-2 verification chunk: F-18/F-19 fan-out & containment. Result is decisive and OVERTURNS our prior call. The Chunk-7 task-isolation containment argument is REFUTED by a global panic hook neither Codex nor the externals had surfaced before. F-18/F-19 move Low → HIGH-leaning. The hook is a global multiplier affecting every untrusted-input-reachable panic. Details below.

---

## SESSION 35 — F-18/F-19 CONTAINMENT REFUTED (technical facts current; SEVERITY conclusions SUPERSEDED by S38)

> **⚠ SUPERSEDED (S38):** the global-panic-hook / containment-refutation facts in this block are CORRECT and still hold (any task panic → whole-process exit). But every SEVERITY conclusion below (F-18/F-19 "High-leaning / Medium floor / Critical ceiling", and F-21/F-23/F-24 "→ Medium/High not Low") is VOID — it rests on a "total/temporary network shutdown" availability tier that DOES NOT EXIST on the live Immunefi page. Corrected mapping: every untrusted-input panic = **Low** ("undocumented panic reachable from a public API"). See SESSION 38.

> All source-traced by Codex with quoted bodies + re-derived by Claude from those quotes. Still no PoC. Three-way severity now converges UP (ER1 "High if reachable over P2P" → reachability CONFIRMED; ER2 panic-chain-is-the-impact; our Low REFUTED).

### The fact that breaks our prior reasoning — GLOBAL PANIC HOOK
- `coordinator/src/main.rs:1315`: `std::panic::set_hook(...)` → `std::process::exit(1)` on **ANY** task panic. Runs BEFORE unwinding → `catch_unwind` cannot save a call site. `Cargo.toml:116` release = `panic="unwind"` + `overflow-checks=true`, but the hook overrides operationally.
- ⇒ ANY task panic = whole coordinator process exit. **Task isolation is moot.** Our "Low, task-isolated" was an ERROR — we reasoned about isolation from Chunk-7 code without checking the hook in main.rs ("cannot verify what is not in the pasted output").
- ⇒ **GLOBAL SEVERITY MULTIPLIER:** every panic reachable from untrusted input is a whole-process DoS, not a Low. Fix is always per-site fallible decode (a wrapper can't help — hook defeats catch_unwind).

### F-18 reachability (source-traced)
- **Ingress unauthenticated + attacker-addressable.** P2P binds `0.0.0.0:30563`; noise authenticates only a throwaway libp2p key; explicit TODO = no validator-only filter (`p2p.rs:375`). Req/res handler treats `Request` and `Response` identically and parses kind from attacker bytes (`p2p.rs:697`) → attacker sends an **UNSOLICITED** `ReqRes(Block(genesis))` request — no prior heartbeat, no being selected as sync source. Public P2P port is reachable **BY DESIGN** (validators mesh over internet) — NOT intranet-gated like F-16. Needs only victim multiaddr + a live genesis (both observable).
- **No pre-verify gate.** Socket → kind parse → genesis match → `HeartbeatBatch` decode → block decode → `Commit` SCALE-decode (`ext.rs:131`, plain derive, NO point validation) → trailing-bytes check. None reject non-canonical validator bytes. **F-18 needs NO forgery:** 32 non-canonical bytes in `commit.validators` + a parseable aggregate with matching Rs length. Panic fires while BUILDING the arg to `aggregate.verify` (`read_G(...).unwrap()`, `mod.rs:200`), before any signature math.
- **Path:** attacker → `0.0.0.0:30563` → reqres loop (`p2p.rs:697`) → per-tributary handler routed by genesis (`p2p.rs:881`) → `HeartbeatBatch` decode → `sync_block` → `sync_block_internal` (`lib.rs:262`) decodes commit + calls `verify_commit` (`ext.rs:256`) → `verify_aggregate` (`mod.rs:200`) → `read_G(...).unwrap()` panic → hook → process exit.
- **Blast radius.** Panic task = per-tributary handler (`p2p.rs:881`), which carries BOTH sync (Heartbeat/Block) AND live consensus gossip (`Gossip(Tributary)`) — so even pre-hook the "only the sync helper dies" claim was wrong. With the hook: blast radius = ENTIRE coordinator process. No catch_unwind / respawn (moot).
- **Restart.** No local-DB crash-loop (malformed commit not persisted; commits stored only after `add_block` succeeds, `blockchain.rs:275`). BUT trivially re-triggered remotely after restart → sustained network-wide halt under active attack. "Just restart" is NOT a valid mitigation.
- **Fan-out = attacker-DIRECT**, not relay (sync path never re-gossips the commit; only `handle_message()==true` rebroadcasts and Tendermint msgs return `false`). Attacker fans out to the whole validator set themselves.
- **add_block:** asserts `verify_commit` (`mod.rs:362`) and `panic!`s on invalid block, but external commits reach it only AFTER `sync_block_internal`'s `verify_commit`, which panics FIRST for malformed bytes → sync_block_internal is the first hit.

### F-19
Still gated behind the F-17 identity-collapse: a CANONICAL identity key passes `read_G` (no F-18 panic), passes `verify_aggregate` (collapse), then panics at `weights[&identity]` (`mod.rs:238`). Codex found NO alternate path to the weight lookup. Same process-exit outcome; distinct root cause/fix; subsumed in IMPACT by F-18 (which is cheaper — garbage bytes vs crafting a collapsing aggregate). Live gossip uses the safe individual verifier (`mod.rs:168`, fail-closed).

### Severity reframe — SUPERSEDES "F-18/F-19 = Low, task-isolated" and the OPEN-ITEMS Low-vs-Medium line
- Prior **Low REFUTED** (its sole basis — containment — does not exist).
- **Single-reviewer lean = HIGH. Floor = Medium. Ceiling = Critical** (depends on the program's threat-level tiering of a total-network halt). NOT LOCKED — joint rating.
- **Mapping:** ⚠ SUPERSEDED S38 — this bullet's "availability tier (temporary/total network shutdown) = HIGH/CRITICAL" is VOID; no such tier exists on the live page, so F-18 = **Low** (undocumented panic). The one part that survives: the defect IS an incomplete verifier (`verify_aggregate` `unwrap`s where sibling individual verifier `mod.rs:168` fail-closes) — but the *unwrap-panic* maps to the Low panic bucket, not the verifier-formula High bucket (which is about academic formula incompleteness, not a missing decode guard). [Original (void) text: "the matching availability tier (temporary/total network shutdown) = HIGH/CRITICAL."]
- **Downgrade risks to preempt:** (a) `p2p.rs:375` TODO / Known Issue Assurance — counter: TODO is about SPAM, not the panic-to-exit; the verifier-unwrap defect is distinct + more severe; KIA needs actual disclosure, not a bare TODO. (b) "just restart" — invalid, re-triggerable.
- **OPEN PROGRAM-FACT (decides the ceiling):** confirm against the program's *Rewards by Threat Level* table whether a sustained total-network halt is tiered Critical or High. Read directly from the program page.

### F-17 reframe
Critical-denial (no consensus forgery) HOLDS, but the `weights[&identity]` panic is NOT a safety net — **the panic IS the harm** (network-wide liveness halt). Honest narrative: the identity defect can't forge finality (one slot, can't meet threshold), but reaching that point exits the process; F-18 reaches the same exit more cheaply. Whole-audit theme tightened: primitives sound (ciphersuite/multiexp/dalek clean), consumers fail at the trust boundary two ways — accept identity (F-15/F-17 soundness) and panic on malformed input (F-18/F-19 liveness) — and the hook makes the liveness variant maximal.

### Global-multiplier consequences (NEW Phase-2 items)
- **CHECK `processor/src/main.rs` for the same panic→exit hook.** If present, the red-team external-input panics — F-21 (`Shorthand::Swap` `todo!()`), F-23 (Ethereum dup-event), F-24 (Monero dup-output) — each become a remotely-triggerable whole-PROCESSOR-process kill via a crafted on-chain deposit → Medium/High, not Low. The F-16 borsh `.expect()` likewise (if MITM-reachable). Cheap check, gates several candidates' severity.
- **NEW WORKSTREAM — panic-reachability sweep:** enumerate every `unwrap`/`expect`/`assert`/index reachable from untrusted input (P2P, processor messages, external-chain scanner data) across coordinator + processor; the hook makes each process-fatal. Likely where more findings are.

### Updated totals (Session 35)
**Confirmed:** 2 High (F-15, F-17). **F-18/F-19 moved from "2 panic-DoS Low" → HIGH-leaning candidates** (Medium floor, Critical ceiling), pending joint rating. 0 Critical confirmed. 1 Medium candidate F-16 (lean Low, unlocked, only finding with zero external eyes). 7 reportable Lows (F-01,04,07,08,09,11,14). **Unverified:** F-21–F-24 (processor-hook check may escalate), leads B/E, cached-preprocess Critical chain, ETH claim-before-execution note.

### Revised Phase-2 priority (current)
1. **F-15 PoC** (highest-value confirmed; required; settles the number).
2. **`processor/src/main.rs` panic-hook check** (cheap; gates F-21/F-23/F-24 severity).
3. **Processor-surface chunk** — F-22 (Critical ceiling: funds-received-not-credited) first, then F-21 (clean PoC), then F-23/F-24, then leads B/E.
4. **Panic-reachability sweep** (new workstream).
5. **Cached-preprocess lifecycle chunk** (potential F-15→key-recovery Critical chain).
6. **Joint rating** of F-18/F-19 + F-16 with the external outputs; confirm program threat-level tiering for total-network halt.
7. **Closers** — Ed448 instantiation proof; W-06/dleq one-line reconfirms.

Codex updated its context.md/agent.md as the first Phase-2 chunk and corrected the plan there (Phase-2 current, chain-mapping deferred).

### Session 36 — Jun 1, 2026
Follow-up confirmations after the F-18/F-19 chunk. Closes the processor-hook check and grounds the severity ceiling against the live program rules. No new chunk run; this is verification + strategy.

---

## SESSION 36 — HOOK SCOPE + FLAT-TABLE CONFIRMATIONS (facts current; F-18 "network shutdown" severity VOID)

> **⚠ SUPERSEDED (S38) on severity only:** the panic-hook-in-all-three-binaries finding and the flat reward values ($30k/$5k/$1k/$250) below are CORRECT and current. But the "F-18 = High (temporary total network shutdown) / Critical arguable" classification is VOID — there is NO availability/network-shutdown tier on the live impacts table; F-18/F-19/F-25 = **Low** (undocumented panic). The one severity claim here that SURVIVES: **F-22 = the clean fund-impact Critical** ("received funds not actually received/spendable") — that row IS Critical on the live table (S37 wrongly shaved it to High; S38 restores Critical).

### Panic hook confirmed in all three binaries
- coordinator `main.rs:1315`; **processor `main.rs:699` `#[tokio::main]` → `:704` `set_hook` → `:709` `process::exit(1)`**; **message-queue `main.rs:155`** (same pattern). The global severity multiplier holds for every binary.
- Consequences:
  - **F-21 / F-23 / F-24 escalate** from Low → remotely-triggerable whole-PROCESSOR-process kill via crafted on-chain deposit (reachability permitting).
  - **F-16 borsh `.expect()`** sub-observation → process exit, but INHERITS F-16's transport-trust gate (sharpens the in-scope branch; does NOT resolve the scope question).
  - **NEW sweep target:** the message-queue SERVER itself (`0.0.0.0:2287`, `Next` unauthenticated) — any unwrap in its request parsing reachable from an unauthenticated connector = process-exit DoS of the central bus; same intranet-trust gate as F-16.

### Immunefi rewards table is FLAT (live page)
- Source: immunefi.com/bug-bounty/serai/information — **Critical $30,000 / High $5,000 / Medium $1,000 / Low $250.** No separate "total network shutdown" payout row.
- ⇒ Ceiling is decided in two steps: (1) classify the impact under Immunefi's standard taxonomy / impacts-in-scope, (2) map that severity onto the flat table. Serai = Standard Badge, Primacy of Impact, PoC required all severities, Known Issue Assurance.

### F-18 severity — refined — ⚠ SUPERSEDED S38: F-18 = Low (no availability tier exists)
- ~~**HIGH is the defensible classification** ("temporary total network shutdown")~~ — VOID. There is no "network shutdown" / availability tier on the live impacts table; a process-exit panic maps only to "undocumented panic reachable from a public API" = **Low**.
- ~~**CRITICAL is arguable but weaker** ("total network shutdown")~~ — VOID, same reason.
- Corrected play: file F-18 as **Low** ($250-class); the sustained-at-will / network-wide / no-privilege narrative is discretionary upside under Primacy of Impact, not a by-right higher tier. Joint rating, not locked.

### STRATEGIC — flat table + duplicate-class dynamics
- F-18's whole value on a flat table = ONE High ($5k); no fund impact is PoC-able from it.
- Immunefi buckets same-root-cause instances as one report. Once F-18 establishes "untrusted input → panic → process exit via hook," a pile of sibling unwraps likely triages as the SAME class (one combined report / possibly known). So a broad panic sweep = high effort, capped marginal payout.
- ⇒ Scope the sweep to: (a) concretely confirm F-21/F-23/F-24 reachability; (b) hunt panics with DISTINCT impact/root cause (chain split, persisted-state corruption, funds) — NOT unwrap-cataloging.
- EV concentrates in: **F-15 PoC** (surest High; settles the number), the **F-18 High/Critical argument** (in hand), and **F-22** (the ONLY candidate that classifies as a clean fund-impact Critical $30k rather than a shaveable availability finding). F-15 PoC + F-22 triage rank ≥ the sweep.

### Revised Phase-2 priority (Session 36, EV-ordered)
1. **F-15 PoC** — surest High; required; settles the contested number.
2. **F-22 triage** — only clean Critical $30k shot (partial-batch funds-received-not-credited; scope `InstructionFailure` = refund vs silent loss; is failure attacker-induceable?).
3. **Processor-surface chunk** — F-21 (clean PoC, now whole-processor kill), then F-23/F-24.
4. **Panic-reachability sweep — TIGHTLY SCOPED** (distinct impact/root cause + queue-server target), not unwrap-cataloging.
5. **Cached-preprocess lifecycle chunk** — potential F-15→key-recovery Critical chain.
6. **Joint rating** F-18/F-19 + F-16 with the external outputs.
7. **Closers** — Ed448 instantiation proof; W-06/dleq one-line reconfirms.

Codex context.md/agent.md: Session 36 appended (processor-hook + msg-queue-hook + flat-table confirmations; chain-mapping deferred / Phase-2 ongoing preserved).

---
---

### Session 37 — Jun 3, 2026
Repo shared as a zip with Claude this session for independent source checks. NEW FINDING F-25: hypothesis derived by Claude from source, then verified + PoC'd by Codex on the real handler path. First PoC-confirmed real-path finding (severity = **Low** per the live impacts table — it's a panic; the earlier "Critical candidate" framing assumed a non-existent network-shutdown tier, corrected S38). Detail below = current truth.

## SESSION 37 — F-25: FORGED SignCompleted → PERSISTED FATAL-SLASH → PERMANENT CRASH-LOOP (current truth)

> Claude re-derived every load-bearing link from Codex's pasted bodies (not just the executor's verdict). PoC-confirmed (no longer reasoning-only). Single reviewer (Claude) + executor (Codex); no independent rater yet → pending joint rating, NOT locked solo.

### The chain (all source-traced AND PoC-exercised on the real handler path)
- `SchnorrSignature::verify` (crypto/schnorr/src/lib.rs) = `multiexp[(1,R),(c,A),(−s,G)].is_identity()`, NO identity guard → A=identity ⇒ check collapses to R = s·G ⇒ trivial forgery, no key (this is the F-15 root, reused).
- `SignCompleted` = `TransactionKind::Unsigned`; tx `verify()` gates ONLY on `signature.verify(first_signer, sign_completed_challenge())` — NO validator-membership check on `first_signer`.
- Mempool `Unsigned` arm: only hash-dedup + `app_tx.verify()`; NO per-signer cap (ACCOUNT_MEMPOOL_LIMIT is Signed-only). `block()` lists unsigned first; `verify_block` re-runs `tx.verify()` ⇒ block FINALIZES network-wide; `add_block` persists the unsigned tx during consensus.
- Handler (handle.rs SignCompleted arm): unrecognized plan ⇒ `fatal_slash(first_signer.to_bytes(), …)` then `return` (BEFORE any processors.send). Attacker just picks a random/unrecognized plan.
- `fatal_slash` → `FatallySlashed::set_fatally_slashed` persists raw key bytes; `FatalSlashes::get_as_keys` reads them back via `from_bytes(key).unwrap()`.
- Post-tx-loop in `handle()`: `still_present_shares` iterates fatal slashes calling `self.spec.i(&[], removed).expect("removed party was never present")`. `spec.i` returns None for any non-validator key (`all_is.get(&key)?`) ⇒ `.expect()` PANICS (scanner.rs ~264 on live checkout / 263 in zip).
- Global panic hook (coordinator main.rs ~1320 live / 1315 zip) → `process::exit(1)` on ANY task panic, pre-empting tokio's task-catch ⇒ whole coordinator process dies.

### Why PERMANENT / unrecoverable (the impact-narrative core; distinct from F-18)
- The poison rides a VALID (verify-passing) unsigned tx that is FINALIZED + PERSISTED via consensus (`add_block`) BEFORE the post-commit scanner detonates. The panic is in the application scanner reading an already-committed block.
- `handle_new_blocks` advances `LastHandledBlock` / `txn.commit()` ONLY after `handle()` returns. The hook exits first ⇒ pointer never advances ⇒ on restart the SAME finalized poisoning block is re-scanned ⇒ re-panic. Restart does NOT recover; needs a code fix + manual DB surgery.
- Read-your-writes = YES on both backends (MemDb + parity_db check pending txn writes first) ⇒ the fatal-slash write is visible later in the SAME txn ⇒ panic fires on the POISONING block itself, not the next.

### Two triggers (both PoC-confirmed, no defeater found)
- identity variant: first_signer = identity, forged (R = s·G, s) — needs the F-15 verify defect.
- own-key variant: a fresh NON-VALIDATOR keypair with a REAL self-signature — does NOT need F-15; reaches the same crash because the key still isn't a validator. ⇒ F-25 is robust to any F-15 taxonomy haggling.

### PoC (Codex, Session 37) — real-path route (C0 = real handle() feasible)
- A: `schnorr-signatures` identity_public_key_verifies_if_r_equals_sg — PASS.
- B: forged SignCompleted passes verify() + accepted into mempool, >50 distinct accepted (no Unsigned cap); own-key variant also passes verify() — PASS.
- C: real-path `#[should_panic("removed party was never present")]` for BOTH triggers; panic at scanner.rs:264 — PASS.
- Files changed (Codex checkout): crypto/schnorr/src/tests/mod.rs; coordinator/src/tests/tributary/tx.rs.

### Severity — ✅ CORRECTED S38 (was "Critical-defensible / floor High")
- **Low** — "undocumented panic reachable from a public API." This is the live table's only availability-flavoured bucket; the permanent / network-wide / persisted / restart-proof / no-stake / single-unsigned-tx narrative does NOT create a higher bucket by-right (there is no network-shutdown tier). Model EV at $250.
- The impact narrative is still worth INCLUDING in the writeup (permanence + network-wide from an unauthenticated remote actor, distinct from F-18's transient panic): the team MAY upgrade at discretion under Primacy of Impact, but do not bank on it. The "this is more than a local library panic" argument is a discretionary upside, not a by-right tier.
- Out-of-scope list (BFT/51%/Sybil/centralization) does not touch it (single no-stake attacker), so it stays in-scope as a Low.
- DISTINCT from F-18 (different root cause: unguarded fatal_slash/spec.i validator-assumption; different recoverability: permanent vs transient) and from F-15 (own-key variant reaches it without the identity defect). Be ready to argue distinctness if the program tries to bucket it with F-18 — though both land in the same Low bucket, so the distinctness argument is about avoiding a single-report collapse, not about tier.
- **NOT locked solo — joint rating pending** (Claude seeded the hypothesis, Codex executed).

### PROVENANCE — Session 37 status (mostly resolved; 2 quick gates remain)
- **Repo: RESOLVED.** Live checkout `origin` = `https://github.com/serai-dex/serai.git` (canonical upstream), branch `develop`, tracking `origin/develop`. The register's old "shedooo/serai" was just account framing (user cloned upstream under their `shedooo` account) — NOT a divergent fork, so no fork-drift risk. Register label corrected at top of file.
- **Audited HEAD = `737dbcbaa78ab817cc1c435cb2b6c5d24d1c4391`** (develop, 2026-01-10, "fix: remove duplicate docsrs cfg_attr lines"). The 9 cited PRODUCTION files are CLEAN vs HEAD (verified: empty `status`/`diff`) → cited bodies are authentic committed code. Only working-tree diffs to tracked files are the 2 PoC test files.
- **Zip-vs-checkout drift = commit difference, not repo difference.** Claude's zip is a different/older revision than HEAD (textual diffs in `spec.i`, `set_fatally_slashed`; panic line 264 @ HEAD vs 263 @ zip). Logic identical on every load-bearing point → finding robust either way. CITE AGAINST HEAD `737dbcba`; the zip is NOT authoritative for line numbers/bodies.
- **GATE 1 (open): in-scope commit.** HEAD is 1 BEHIND `origin/develop` tip `4b89cf02` ("Remove `.github` actions, workflows"). Subject reads CI-only → almost certainly immaterial to F-25, but CONFIRM via `git diff --stat 737dbcba 4b89cf02` (expect only `.github/` paths; nothing under coordinator/ or crypto/) OR fast-forward to tip + re-run the 9-file clean check. Don't call citations "latest develop" until closed.
- **GATE 2 (open): program-page match.** Confirm the Immunefi Serai program's declared in-scope commit/branch matches `serai-dex/serai @ 737dbcba` (or the tip `4b89cf02`). This is the only thing the GitHub-account explanation does NOT cover. User action: check the program page.
- Once GATE 1 + GATE 2 close, F-25 citations lock. Working skeleton: `F-25-writeup-skeleton.md` (commit/line fields carry [PIN] placeholders).

### Revised EV ordering (Session 37) — ⚠ CORRECTED S38 (F-25 is Low, not Critical; see SESSION 38 "Next steps" for the authoritative order)
1. **F-22 triage** — the one confirmed-bucket **Critical** (received-funds-not-spendable); top EV if it triages as real (gated by funds-at-risk + pre-mainnet caveat).
2. **F-15 writeup** — own **High** (verifier-callstack formula); Critical ceiling via the DKG-PoP forge-proofs recon. Also the identity-variant trigger for F-25.
3. **F-25 writeup** — **Low** ($250-class); PoC-confirmed, permanent/network-wide impact narrative is discretionary upside only. PIN commit hash (provenance) + joint rating first. (Was wrongly ranked #1 as a "Critical candidate.")
4. Remaining processor surface (F-21/F-23/F-24, all Low panics); tightly-scoped panic sweep; cached-preprocess chain (the other unproven crypto-Critical lead).
5. Joint rating of F-18/F-19 + F-16.

Codex appended Session 37 to its own context.md / agent.md (F-25 chain + per-link status + read-your-writes + C0 verdict + PoC A/B/C + no-defeater + Critical-defensible-pending-joint-rating). *(⚠ That severity wording is now stale — F-25 = Low; see the RELAY TO CODEX item in SESSION 38.)*

---

## SESSION 37 — LIVE PROGRAM SCOPE + SEVERITY RECALIBRATION (supersedes all prior severity assumptions)

> Pulled directly from immunefi.com/bug-bounty/serai (Last Updated 24 April 2026). The register had been mapping findings to STANDARD Immunefi Blockchain/DLT tiers. Serai's real table is bespoke and crypto-centric. This section supersedes earlier severity calls.

### GATES (both CLOSED)
- GATE 1 (commit lag): CLOSED. `git diff --name-only 737dbcba 4b89cf02 | findstr /v ".github"` returned EMPTY → the 1-commit lag to the develop tip is `.github`-only (CI), immaterial to F-25. Audited HEAD `737dbcba` == tip for all code.
- GATE 2 (repo/scope): CLOSED on repo. Program repo = `github.com/serai-dex/serai`, develop. Matches checkout. (See scope caveats below.)

### Assets in Scope (11) — Blockchain/DLT
Explicit: crypto/ciphersuite, crypto/dkg/src, crypto/frost (modular-frost), crypto/dalek-ff-group, crypto/schnorrkel (frost-schnorrkel), crypto/transcript (flexible-transcript), crypto/dkg/musig, **crypto/schnorr (schnorr-signatures)**, crypto/multiexp, networks/bitcoin (bitcoin-serai), + **"Primacy Of Impact"** placeholder (added 6 Feb 2026).
- NOT explicitly listed: coordinator, processor, message-queue, substrate. Reachable ONLY via Primacy of Impact — which expands ASSETS, not IMPACTS. So a coordinator finding is in-scope only if its impact is one of the 9 below.
- F-15 lives in crypto/schnorr = EXPLICITLY in scope. F-25/F-18/F-19 live in coordinator = Primacy-of-Impact only.

### Impacts in Scope (9) — the COMPLETE rewardable set (verbatim severities) — ✅ CORRECTED S38 (was off-by-one)
- **Critical:** Signing of unintended messages
- **Critical:** Ability to forge proofs
- **Critical:** Unintended, undocumented recovery of private spend keys (or shares)
- **Critical:** Reportedly received funds which weren't actually received/spendable  *(S37 wrongly listed High)*
- **High:** Incorrect/incomplete (academic sense) cryptographic formulae within a **verifier's** callstack  *(S37 wrongly listed Medium)*
- **Medium:** Undocumented transcript collision  *(S37 wrongly listed Low)*
- **Low:** Undocumented panic reachable from a public API
- **Low:** Non-constant time implementation w.r.t. secret data
- **Low:** Incorrect/incomplete cryptographic formulae within a **prover's** callstack
- **THERE IS NO availability / liveness / network-shutdown / DoS impact** except the Low panic line. Impacts not in this list are NOT rewardable (Primacy of Rules for impacts). *(This statement was S37's one correct call and still holds.)*

### Rewards / mechanics
- Critical = **10% of funds directly affected, capped $30k** (funds-based; ratio of funds-at-risk to market cap). High/Medium/Low = FLAT $5,000 / $1,000 / $250.
- Serai appears **pre-mainnet / actively developed**. Primacy-of-Impact caveat: "Code which is historical, visibly under active development, never deployed, and/or never slated for deployment will be considered to have no real impact and remain ineligible." → real-impact / funds-at-risk for a Critical may be judged LOW. CONFIRM mainnet/TVL status before modeling Critical EV.
- Overtime rule: only impact within a 2-hour no-human-intervention window counts. (For a panic-DoS this is moot since it's Low anyway.)
- PoC required for: Blockchain/DLT Critical, AND the Low "undocumented panic reachable from a public API." (We have PoCs.)
- Out of scope: BFT-assumption attacks, 51%/governance, Sybil, centralization, tests-only code, unsafe-only bugs, cross-group DLEq (experimental), invalid hashes/curves/ciphersuites. (None of these sink F-25, but F-25 is Low regardless.)

### CORRECTED per-finding severities (re-map EVERYTHING against the CANONICAL table) — ✅ re-corrected S38
- **F-25** (coordinator panic crash-loop): **Low** — "undocumented panic reachable from a public API." No higher bucket exists. The permanent/network-wide narrative does not raise it by-right. (Submit-with-impact-narrative optional; team MAY upgrade at discretion, but model EV at $250.) [S37 correct]
- **F-18 / F-19** (panics): **Low** each; likely bucket with each other / with F-25 as same-class. [S37 correct]
- **F-15** (verify accepts identity): **High** — verifier-callstack formula (the live tier is HIGH, not Medium; S37 mis-shifted). LEAD: if identity-acceptance forges against a protocol-used key → "Ability to forge proofs" Critical (UNPROVEN — chase in the crypto crates). Floor High, Critical ceiling.
- **F-22** ("funds received but not credited"): **Critical** — "received funds not actually received/spendable" IS the Critical row (S37 wrongly shaved to High; original register's Critical was right). UNVERIFIED candidate; payout = 10% funds-at-risk capped $30k, may compute low / be ineligible pre-mainnet.
- **F-17**: **High** — verifier-callstack formula (same row as F-15). Critical (consensus forgery) DENIED.
- **F-16 + the 7 Lows** (F-01/04/07/08/09/11/14): RE-MAP each to one of the 9 impacts; any that map to none are OUT OF SCOPE. (Preliminary re-map in SESSION 38 block.)

### Revised strategy (Session 37, post-recalibration) — ⚠ EV thesis CORRECTED S38
- S37 concluded "the real EV is NOT the coordinator work (all Low), it's hypothetical crypto Criticals." HALF RIGHT: the panics genuinely are Low, BUT **F-22 sits in the Critical bucket and F-15/F-17 are confirmed Highs** — so there is more confirmed/near-confirmed value than S37 implied. Genuine crypto Criticals (forge proofs / sign unintended / key recovery) remain the top of the value stack and the DKG-PoP recon target.
- Highest-value crypto-Critical move: chase whether F-15's identity-key acceptance can forge against a key actually used in protocol (aggregate/MuSig/FROST forced to identity), or whether dkg/frost/musig have a forge-proofs / key-recovery path. These map to the $30k Critical bucket and live in in-scope assets.
- Re-map every existing finding to the 9 impacts before any writeup. **F-15 (High)** is the cleanest confirmed in-scope reward today; **F-22 (Critical)** if it triages is the top prize.
- F-25/F-18/F-19 still worth filing as Lows (PoCs exist) but EV is $250-class.

---

## SESSION 37 — CARRIED ITEM: CRYPTO-CRITICAL RECON TARGETS (not the lead; hold in mind)

> The real $30k path is a genuine crypto Critical (forge proofs / sign unintended messages / key recovery) in the EXPLICITLY in-scope crypto crates. F-15's identity-key acceptance in `crypto/schnorr verify()` is the seed. The recon below asks, at each in-protocol verify site: (Q1) can the verifying key be identity / attacker-influenced? (Q2) is the challenge bound to the public key? A "yes" to Q1 or "no" to Q2 where the verified sig grants real acceptance ⇒ Critical candidate. (Line numbers are zip-derived — confirm against HEAD 737dbcba.)

### Candidate verify call sites (in-scope crates + cosign)
- **TOP — DKG proof-of-possession (rogue-key defense):** `crypto/dkg/pedpop/src/encryption.rs:374, 385, 479` and `crypto/dkg/pedpop/src/lib.rs:323` verify PoP / commitment SchnorrSignatures. PoP exists to stop rogue-key attacks. If identity-acceptance (or an unbound challenge) lets a participant present an attacker-chosen/identity commitment key with a PoP that still verifies → defeat rogue-key defense → bias the GROUP KEY → "ability to forge proofs" / "undocumented key recovery" (Critical). This is the highest-value thread.
- **FROST group_key:** `crypto/frost/src/algorithm.rs:216` (`sig.verify(group_key, c)`) and `crypto/frost/src/sign.rs:465`. If DKG can yield a degenerate/identity `group_key`, FROST sigs become forgeable → "signing of unintended messages."
- **Cosign:** `coordinator/src/cosign_evaluator.rs:167` verifies cosigns (block attestation). Forging one = "signing of unintended messages," gated on the same upstream key-influence question.
- **DKG promote:** `crypto/dkg/promote/src/lib.rs:149` verify during key promotion (PoP-like).
- **schnorrkel:** `crypto/schnorrkel/src/lib.rs:143` (substrate signing context) — check key/challenge binding.
- **Aggregate:** `crypto/schnorr/src/aggregate.rs` (`SchnorrAggregate::verify`, batch) — check identity handling in the multiexp aggregate.

### Why this matters for F-15 sequencing
- This recon GATES F-15's CEILING. F-15 = **High** (verifier-callstack formula) as a floor; it becomes **Critical** if ANY in-scope crypto caller lets identity-acceptance forge against a protocol key. ⇒ Do NOT lock F-15 below Critical until the recon settles its ceiling — but its floor is High, not Medium (S37 error, corrected S38).
- Temper expectations: these crates are the most-audited code in the repo (Cypher Stack crypto audit, Mar 2023). Callers may all bind challenges correctly and reject identity. Highest-EV because payoff is large, not because it's likely.

### NEXT-SESSION ORDER (post-recalibration) — ✅ severities corrected S38
1. **RE-MAP every finding to the 9 live impacts** (F-15, F-16, F-17, F-18, F-19, F-22, F-25, + the 7 Lows). Some Lows may map to no listed impact ⇒ out of scope. This is the LEAD item, before any writeup. (Preliminary re-map done in SESSION 38 block — confirm/lock it.)
2. **Crypto-Critical recon** (chunk drafted; DKG PoP first) — also settles F-15 **High-vs-Critical**. AWAIT user GO before firing the recon chunk.
3. Then file: **F-22 = Critical** (if it triages); **F-15 = High** (or Critical per #2); F-25/F-18/F-19 = Low (F-25 lead with own-key variant for root-cause independence; consider whether F-18/F-19/F-25 bucket as one Low).
4. Reporting rule: F-15 and F-25 filed SEPARATELY (different impacts/layers; bundling risks one collapsed payout). Pre-submission: check Serai known-issues (github issues + audits dir) so neither is an acknowledged/ineligible bug.

This file is maintained by Claude. Always upload at session start. Append, never overwrite.

---

## SESSION 38 — Jun 3, 2026 — IMPACT-TABLE ALIGNMENT CORRECTION (current truth; supersedes S35/S36/S37 severity calls)

> Provenance: live Immunefi Serai page fetched + cross-checked against a PDF capture the user supplied this session (Last Updated 24 April 2026). The impacts table is now VERIFIED visually (badge colours), not inferred.

### Root cause of the correction
Session 37's "recalibration" was built on a **truncated/off-by-one rendering** of the live Impacts-in-Scope list, which shifted every tier from the 4th row down one level lower. The live page's authoritative, complete listing (4 Critical / 1 High / 1 Medium / 3 Low) is the CANONICAL table near the top of this file and reads:

| Impact | Live tier |
|---|---|
| Signing of unintended messages | Critical |
| Ability to forge proofs | Critical |
| Recovery of private spend keys / shares | Critical |
| Reportedly received funds not actually received/spendable | **Critical** |
| Incorrect/incomplete crypto formula in a **verifier's** callstack | **High** |
| Undocumented transcript collision | **Medium** |
| Undocumented panic reachable from a public API | Low |
| Non-constant-time impl w.r.t. secret data | Low |
| Incorrect/incomplete crypto formula in a **prover's** callstack | Low |

Two prior-error generations cancel into this truth: Gen A/B (S≤36) over-rated the panics via a network-shutdown availability tier that **does not exist**; Gen C (S37) correctly killed that tier (panics = Low) but mis-shifted the middle rows. S38 = correct tiers (Gen A) + panics-are-Low (Gen C).

### Corrected severities (locked vs the canonical table)
- **F-22 = Critical** — "received funds not actually received/spendable." UNVERIFIED candidate; reward = 10% of funds-at-risk capped $30k, and may compute LOW or be ineligible under the pre-mainnet active-development caveat. Triage is the top-EV move.
- **F-15 = High** (floor) — verifier-callstack formula. Critical ceiling IF identity-acceptance forges against a protocol-used key → DKG-PoP recon gates High-vs-Critical.
- **F-17 = High** — verifier-callstack formula. Critical (consensus forgery) DENIED (one-slot / threshold / panic argument).
- **F-25 / F-18 / F-19 = Low** — undocumented panic from a public API. No availability tier; permanence/network-wide blast radius does NOT raise the bucket by-right. May package as one class; team MAY upgrade at discretion but model EV at $250.

### Re-map of ALL findings to the 9 live impacts — ✅ LOCKED S38 (3 soft spots firmed below)
| Finding | Live impact | Tier | Note |
|---|---|---|---|
| F-22 | Received funds not spendable (C4) | **Critical** | Candidate; payout gated by funds-at-risk + pre-mainnet caveat |
| F-15 | Verifier-callstack formula (H1) | **High** | Floor; Critical ceiling (forge proofs / C2) via DKG-PoP recon |
| F-17 | Verifier-callstack formula (H1) | **High** | Critical denied |
| F-25 | Panic from public API (L1) | **Low** | Permanent crash-loop; no higher bucket |
| F-18 | Panic from public API (L1) | **Low** | Cheapest trigger (garbage bytes); clean in-scope |
| F-19 | Panic from public API (L1) | **Low** | Chained behind F-17; clean in-scope |
| F-01 / F-04 | Panic from public API (L1) | **Low** *(known-issue risk)* | schnorr/frost API-misuse panics; both arguably the "panic from crafted/trusted input" class that GitHub issue #295 already acknowledges → re-check before filing. |
| F-08 | Panic from public API (L1) | **~~Low~~ LIKELY KNOWN → drop** | ThresholdKeys::new panic on short Interpolation::Constant is squarely the acknowledged issue #295 ("Deserialization may produce an inconsistent ThresholdKeys → FROST panic"; team's stance: trusted data structure, wontfix the rebuild). Treat as known/ineligible. |
| F-09 / F-14 | Panic from public API (L1) | **Lean OUT OF SCOPE** | ✅ gate resolved S38 (Codex quotes). **F-09:** blame API documents the caller must pass a message "authenticated as actually having come from the sender" (pedpop lib.rs:615-617) → authentication is a documented caller responsibility, so reaching the panic runs through the OOS "communication protocol between library users" boundary. **F-14:** "proofs from all other participants" (promote lib.rs:119) documents the coverage expectation in prose, though impl only checks len()/range before `proofs.get(&i).unwrap()`. Both = marginal $250 Lows at best, lean OOS; do not file standalone. |
| F-11 | Prover-callstack formula (L3) | **Low** | Missing ephemeral-scalar uniqueness check = incomplete formula in the dealer/prover path. C3 (share recovery) ceiling exists ONLY under RNG failure → not a feasible/honored path. |
| F-07 | — (no match) | **OUT OF SCOPE** | participants==0 → Ok(empty) fail-open is input-validation/best-practice, not a panic/formula/funds/signing/forge/key-recovery impact. L3 is a forced stretch at best; do not file as a standalone. |
| F-16 | none (fund-forgery defeated) | **OUT OF SCOPE (lean) — NOT Critical** | ✅ gate resolved S38 (Codex round-trip). Missing local sig-check is REAL (message-queue client.rs:202 TODO; coordinator main.rs:334-373 publishes `execute_batch` with no local `batch.signature` check), BUT the on-chain pallet re-verifies the batch sig at in-instructions/pallet/lib.rs:307-317 → a forged batch (no valid threshold sig) is rejected on-chain ⇒ NO fund movement. Defense-in-depth, not an exploitable Critical (matches Codex's own S34 "no funds-loss path confirmed"). **Residual (1 body):** does coordinator main.rs:334-373 *storing* a forged batch poison local publish/dedup state so a legitimate same-ID batch is suppressed? That would be a funds-*denial* C4 path — needs the function body + still needs the MITM/trust-boundary scope call. Until shown, OOS-lean / Low-at-most (borsh-`expect` sub-obs). |
| F-21 / F-23 / F-24 | Panic from public API (L1) | **Low** | Unverified processor candidates; in scope only if prod-reachable (not feature-gated / attacker-constructable). Likely one combined same-class report. |
| Lead B | Privileged Shorthand::Raw mint/state | **? (C4 if real)** | ER2 does NOT claim unauthorized mint (balance tied to signed batch). Investigate; potential funds-Critical only if a deposit can induce unintended mint/state. |
| Lead E | Bitcoin scanner clones payload across outputs | **? (C4 if real)** | Duplicated instructions / unwanted refunds → possible funds mis-credit. Investigate. |
| Cached-preprocess chain | Key-share recovery (C3) | **Critical** | Unproven crypto-Critical lead (alongside F-15→forge-proofs). |

**Locked totals (post-Codex round-trip, S38):** in-scope today = **1 Critical candidate (F-22), 2 High (F-15, F-17), and the Low panics** (F-25/F-18/F-19 clean; F-21/F-23/F-24 if reachable; F-11 as L3). **OOS:** F-07 (firm), F-16 (lean — fund-forgery defeated by on-chain re-validation; one residual funds-denial body-check), F-09/F-14 (lean — documented caller preconditions). **0 Critical confirmed.** Both prior gate-checks are now RESOLVED; the only open F-16 item is the optional state-poisoning body read.

### Provenance — ✅ GATE 1 CLOSED (S38, Codex)
- HEAD `737dbcbaa78ab817cc1c435cb2b6c5d24d1c4391` (develop, 2026-01-10). `git rev-list --left-right --count HEAD...origin/develop` = `0 1` → HEAD is 0-ahead / **1-behind**, and that one commit is `4b89cf02 Remove .github actions, workflows` → **non-source lag, benign. GATE 1 closed.**
- Working tree: only the 2 PoC test files modified (coordinator/src/tests/tributary/tx.rs, crypto/schnorr/src/tests/mod.rs); everything else "dirty" is UNTRACKED audit artifacts (context.md, agent.md, findings.md, jules/, understanding_report.md, dkg_fuzz_property_test_spec.md). All 9 cited production files verified CLEAN vs HEAD (`git status --porcelain` + `git diff --stat HEAD` both empty). Citation pin = HEAD above.

### Next steps (corrected severities; re-map LOCKED; gates RESOLVED S38)
1. ✅ Re-map LOCKED. ✅ F-09/F-14 gate resolved (lean OOS). ✅ F-16 gate resolved (fund-forgery defeated by on-chain re-validation → not Critical; OOS-lean). ✅ GATE 1 closed (1-behind = .github-only). **Clear to proceed.** 2. DKG-PoP crypto-Critical recon — **await user GO before firing the recon chunk** (settles F-15 High-vs-Critical). 3. (Optional, low priority) one body read of coordinator main.rs:334-373 to close the F-16 funds-*denial* state-poisoning question. 4. **F-25 = SUBMISSION-READY (own-key variant)** — all chain lines confirmed @HEAD 737dbcba (transaction.rs:617, mempool.rs:153/161/163/167, blockchain.rs:219/223/249/282/306/307, handle.rs:729, scanner.rs:207/264, db.rs:93/99/108, main.rs:1325); standalone own-key PoC green (`sign_completed_non_validator_self_signature_{verify_and_add_without_unsigned_cap, crash_loops_scanner}`), no schnorr-identity test referenced → F-15 NOT exposed. Report finalized (F-25-submission.md). Claim Low; complete KYC; submit. File F-22 (Critical, if triaged) + F-15 (High) SEPARATELY later; F-15 held for the recon. 5. Pre-submission known-issues check DONE (F-25 clear; F-08 known via #295). PIN = 737dbcba.

**⚠ RELAY TO CODEX (round 2, minor):** Codex's S38 block frames the F-16 gate result as "the all-sinks-re-validate premise is false" — that overstates it. The on-chain pallet (in-instructions/pallet/lib.rs:307-317) DOES re-validate, so a forged batch is rejected and no funds move; F-16 = OOS-lean / NOT Critical (matches Codex's own S34 "no funds-loss path confirmed"). Have Codex correct that one line and, if convenient, reconcile its findings-log TABLE rows (F-07/F-09/F-14/F-16 still show pre-S38 labels) + refresh the stale Session-26 front-page totals. Also flag the remaining F-16 funds-denial body-question (main.rs:334-373 store/dedup) as the only open F-16 item.

### Known-Issues / Duplicate Check — ✅ RUN S38
Checked the program's known-issue baselines: the Cypher Stack March-2023 crypto audit (the in-repo `audits/` report, culminating commit `669d2db`; schnorr README states subsequent changes are unaudited) + the serai-dex/serai GitHub issue tracker.
- **Cypher Stack audit:** did NOT flag F-15/F-17 (verify accepting an identity public key). Nearest findings are 3.2.3 (the `random()` fn can yield identity — different function) and 3.8.1 (torsion/prime-order strictness on ed25519 verify keys — about *rejecting* valid sigs, not *accepting* identity). Audit assigns no severities and found "no findings of particular immediate concern." ⇒ **F-15/F-17 are NOT reported known issues.** Residual: the schnorr lib was in audited scope, so confirm the identity-acceptance code post-dates `669d2db` (helps eligibility); and frame F-15 as a verifier-callstack *incomplete-formula* (High) to dodge the "invalid circumstances via invalid input" OOS line.
- **GitHub issue #295** ("Deserialization may produce an internally inconsistent object → panics"): explicitly covers crafted `ThresholdKeys` → FROST panic, with the team's documented stance that ThresholdKeys is a *trusted data structure* (rebuild-on-deserialize is "not worth doing"). ⇒ **F-08 is LIKELY KNOWN/ineligible.** The broader "panic from a crafted/trusted struct" class is acknowledged → **F-01/F-04 need re-check** against it. Does NOT touch F-25 (untrusted network input, not a trusted struct), F-15/F-17 (verifier formula), or cleanly F-18/F-19 (untrusted p2p input).
- **GitHub issue #426** ("Removal of validators from Tributary" on fatal-slash; notes TendermintTx is Unsigned): the fatal-slash/validator-removal area is under ACTIVE design — reinforces the pre-mainnet/active-development reward caveat for F-25, but does NOT disclose the F-25 panic. ⇒ **F-25 is NOT a known issue.**
- **No issue discloses** F-15 identity-verify or the F-25 forged-unsigned-SignCompleted panic specifically. message-queue is confirmed internal inter-service infra ("services can talk to each other") → supports the F-16 OOS-lean (attacking it = the out-of-scope inter-service comms boundary).
- **Net:** the two highest-value items (F-15/F-17 High; F-25 Low) and F-25 specifically are clear of known issues. The panic-Low sweep is worth even LESS than thought — F-08 is known; F-01/F-04 at risk. Still pending: a final per-finding known-check at submission time + KYC readiness.


F-22 still needs its `InstructionFailure` = refund-vs-silent-loss triage (is it the Critical it buckets as, and is the failure attacker-induceable?). F-15 PoC/flood-number, GATE 1 (.github-only commit lag), GATE 2 (program-page commit match), and the PIN-COMMIT-HASH provenance step all remain as previously logged.

---

## SESSION 39 — Jun 3, 2026 — F-25 PoC HARDENED + REPORT FINALIZED (own-key variant)
Codex weaponized the PoC to close the proven-vs-inferred gaps. Four own-key tests green under `--features parity-db` (`sign_completed_non_validator_self_signature_*`):
- `verify_and_add_without_unsigned_cap` — 64 distinct own-key SignCompleted pass verify + admitted, no Unsigned cap.
- `finalizes_and_persists` — forged tx included in a finalized block + commit persisted on ALL nodes (real consensus path; build_block/add_block are pub(crate) so exercised transitively — stronger than calling them directly).
- `crash_loops_scanner` (should_panic) — real scanner path panics at scanner.rs:264.
- `restart_rescans_persisted_poison_block` — runs real `handle_new_blocks`, asserts LastHandledBlock halts at the poison block's PARENT (progress never committed past it), re-runs a fresh scanner on the same persisted parity-db → re-panics with progress unchanged, AND a second independent node (separate DB, same block/spec) also panics. Closes Gap 2 (restart-persistence) + Gap 3 (node-independence) cheaply.
New production refs: scanner.rs:679 (scan starts from LastHandledBlock), :717 (advances only after handle() returns); main.rs:1320/:1325 (panic hook → process::exit(1)).
**Honest limits (kept in report):** process::exit(1) hook is code-cited, not test-executed (would kill the runner); full N-node propagation is a deterministic inference off the 2-node demo.
**Report recalibrated** (F-25-submission.md): dropped "permanent/unrecoverable/network-wide irreversible halt" → "restart-persistent crash-loop, no self-heal; recovery needs patch or manual state/DB repair." Added a "Demonstrated vs cited-from-code" subsection. F-25 = SUBMISSION-READY.
**⚠ SUBMISSION PACKAGING:** tx.rs still contains the identity-variant test (exposes F-15). Do NOT attach the whole file — paste only the four own-key test fns + helpers, or reference test names + the filtered cargo output. Claim Low; complete KYC; submit via Immunefi "Submit a Bug" (NOT a GitHub Security Advisory).
Codex updated its own context.md/agent.md with a Session 39 note.

## SESSION 39 addendum — independent triage review + wording fix
- **Triage verdict (independent review of the F-25 report): VALID, LOW, HIGH confidence.** Reviewer confirmed the chain statically (SignCompleted unsigned; verify only checks embedded sig vs first_signer, no membership check; unrecognized-plan handler fatal_slashes first_signer; still_present_shares spec.i(non_validator).expect → panic; panic hook process::exit; restart-persistent via finalized block + un-advanced LastHandledBlock). Would NOT downgrade to Informational (externally reachable, deterministic, finalizes, restart-persistent). Could not run PoC (no Rust toolchain in their env) — reviewed statically.
- **One correction (applied):** the report's "persisted fatal-slash" framing was inaccurate. fatal_slash writes into the scanner's in-flight txn (scanner.rs:207), but the panic in still_present_shares fires BEFORE txn.commit() (scanner.rs:714-718), so the slash write is NOT durably committed — it's re-derived from the block each pass (read-your-writes within the uncommitted txn drives the same-pass panic). Durable poison = (1) the finalized/persisted block + (2) LastHandledBlock never advancing. Report title/summary/chain/steps all corrected to this framing.
- **Submission PoC excerpt (own-key only) confirmed:** identity_sign_completed, the identity crash test, the identity verify_and_add test, and the tx_test baseline are all stripped; only the 4 own-key tests + helpers remain → does NOT expose F-15. ⚠ Caveat: the 4-passed green run we have was for the FULL file (with identity tests); the STRIPPED excerpt should be recompiled/run once before attaching, since removing the identity tests may leave unused imports (e.g. Group) that fail under deny-warnings. Recommend Codex `cargo test` the stripped fragment to confirm clean build.
- **Report PoC section** now notes the PoC is a fragment for coordinator/src/tests/tributary/tx.rs (not standalone) + the imports/harness it relies on + the run command.

---

## SESSION 40 — RESUMPTION STATE (Phase 2, post F-25 submission)
**F-25 SUBMITTED** to Immunefi (Jun 3 2026, Low, own-key variant). Independent triage review = Valid / Low / high confidence; wording corrected (no "persisted fatal-slash"). Report = F-25-submission.md. Off the active list (awaiting program response).

**Active Phase-2 work, in priority order:**
1. **F-22 — InstructionFailure triage (top EV; the one Critical-bucket candidate).** Questions: is `InstructionFailure` a silent drop vs a refund? Can an attacker induce it on a validly-signed batch so funds are "reported received but not received/spendable"? Loci to re-derive from source: substrate/in-instructions pallet execute_batch per-instruction handling (~lib.rs:264-287) + processor batch/InInstruction path (~mod.rs:1009-1021). Needs a PoC (Critical requires one). Does NOT need the recon — can start immediately on GO.
2. **DKG-PoP crypto-Critical recon (settles F-15 High→Critical).** Chunk drafted in earlier session notes; DKG PoP first. Target: can F-15's identity-key acceptance forge against a key actually USED in protocol (aggregate/MuSig/FROST forced to identity) → "Ability to forge proofs" Critical. AWAIT user GO before firing the executor chunk.
3. **Cached-preprocess reuse → key-share recovery (C3 Critical) — unproven lead.**
4. **Processor leads:** B (Shorthand::Raw privileged mint/state → C4?), E (Bitcoin scanner clones payload across outputs → C4?). Reachability of F-21/F-23/F-24 panics (prod-decodable / not feature-gated?).

**Filing queue (hold until severities settle):** F-15+F-17 = ONE High "verifier-callstack formula" report — HELD for the recon to file at correct ceiling (High vs Critical). F-22 filed Critical if it triages. F-18/F-19 = Low panics (need trigger PoCs; lower priority).
**Dropped/OOS:** F-08 (known #295), F-07 (OOS), F-09/F-14 (OOS-lean documented preconditions), F-16 (OOS-lean; on-chain re-validation defeats fund-forgery; one optional state-poisoning body read remains).
**Discipline reminders:** severity gated against the live 9-impact table (no availability tier); re-derive math/logic from pasted source, not executor verdicts; every executor chunk ends with "update BOTH context.md and agent.md"; await GO before firing chunks; PIN commit 737dbcba in any report; Immunefi "Submit a Bug" channel (NOT GitHub Security Advisory); don't attach whole test files that expose held findings.
**Codex working tree:** HEAD 737dbcba; tx.rs carries the F-25 own-key + identity tests (keep — submitted PoC is the own-key subset); untracked audit artifacts (context.md/agent.md/findings.md/jules/). Recompile the stripped own-key fragment if re-attaching anywhere.
