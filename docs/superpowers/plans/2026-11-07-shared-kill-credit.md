# Shared direct-kill credit implementation plan

> **For agentic workers:** Use `superpowers:executing-plans`, test-driven development, and independent correctness and quality reviews.

**Goal:** Count a concrete mob's kills once while progressing eligible accepted direct-kill objectives together, with consistent XP, loot, and unfinished-work scoring.

**Spec:** [Route-quality assessment](../../route-quality-assessment-2026-11-07.md), shared direct-kill credit. The user's subsequent “OK - continue” authorizes this continuation. Base revision: `ddfddb5`.

## Design and constraints

- Retain explicit concrete target identity as `Objective.direct_kill: Option<i64>` only for ordinary hostile creature-kill objectives. Proxy kill-credit, random loot, interaction, and dungeon objectives keep their current behavior.
- Build an immutable target-to-objective index once per planner.
- When a direct objective performs its remaining kills, grant progress to other accepted, active, unfinished objectives for that target whose ordering and handed-item prerequisites already hold. Freeze eligibility before the batch's completions, preventing retroactive credit.
- Reuse `State.started` for partial progress; charge the source's actual time, mob/rested XP, loot, bags, and waits once. Match identity across locations; proximity alone never grants credit.
- Track pending direct work by the active quests benefiting from it. Split at their remaining-kill thresholds, merge equal quest sets, and share the pending ledger across cloned states with copy-on-write. Any beneficiary turn-in settles that work as productive. Abandonment removes membership and drops empty groups, preserving current abandoned-quest handling.
- At finish, use pending actual direct work instead of summing nominal work for free shared completions. Retain existing scoring for unsupported objectives and the existing gathering overlap. Pending work participates in incremental state equivalence.
- Direct-objective alternative-place estimates account for existing partial progress.
- Preserve all gameplay constraints, memory caps, gold/BiS features, and parameter defaults. Rebaseline after this model correction before judging alternative search algorithms.

## Task 1: Reproduce and correct direct sharing

Files: `model/types.rs`, `model/objectives.rs`, `model/testing.rs`, `plan/indexes.rs`, `plan/state.rs`, `plan/sim.rs`, `plan/step.rs`, `plan/spot.rs`, `plan/evaluate.rs`, focused `plan/tests/shared_kills.rs`, existing full objective constructors in unit/integration fixtures, and `CHANGELOG.md`.

- [x] Add failing deterministic equal/unequal-count regressions through the simulator.
- [x] Verify failures expose independent charging and missing completion.
- [x] Implement concrete target classification, indexed sharing, and pending-work accounting.
- [x] Cover late acceptance, different IDs, ordering/handed prerequisites, partial progress, alternatives, abandonment, rested XP/loot/bags, unfinished-work attribution, and incremental/full replay agreement.
- [x] Run focused and route-package checks, formatting, Clippy, and a release CLI build. Report the existing Windows CRLF failure separately.
- [x] Obtain independent correctness and quality reviews and address material findings.
- [x] Update the handoff/validation record and commit as `prijkes <prijkes@noreply.com>`.

## Execution record

- Separate algorithm review runs in parallel. Its report must distinguish player time, weighted score, and generation time, and must not claim an unbenchmarked replacement is faster or better.
- Design call: eligibility is frozen for each aggregate source batch. Objectives unlocked partway through another objective's batch do not receive retrospective credit; finer event-by-event within-batch modeling remains outside this correction.
- TDD: the initial seven cases produced five expected failures (duplicate work/XP, absent shared completion, and unusable shared turn-ins) and two passing negative controls. All seven passed after implementation.
- Extended validation exposed 284 instead of 285 XP after imported partial progress. Direct remaining-kill counts now remove floating-point noise within `1e-9` of an integer; loot expectations retain their existing fractional work.
- Correctness review found a perishable-item timer edge: two kills could finish at 48 seconds before item expiry, although the source batch ended at 120 seconds. Its regression failed before the fix. Shared credit now counts only kills earned by each recipient deadline, allowing an item used in time to clear its timer while keeping quest-wide turn-in deadlines intact. Time within the aggregate batch is distributed over its kills, consistent with the current aggregate work model.
- Combined-case regressions exposed gathering-overlap timing and nominal gathering penalties stealing productive kill work. Both failed before correction. Deadline inversion now uses the same piecewise overlap clock as `Together`, including saturation of prior gathering; settled direct work is reserved from later nominal unfinished-work estimates. The correctness reviewer verified both fixes by inspection.
- The complete route suite passed with 275 library tests and 238 integration tests, excluding only the documented Windows CRLF test; two documentation examples remain ignored. The focused module has 20 cases. Final checks follow the exact-bit scalar state comparison required for safe incremental reuse.
- Final verification repeated those 513 passing tests after the scalar comparison cleanup. `cargo clippy --release -p fg-route --all-targets --locked -- -D warnings`, `cargo fmt --all --check`, `git diff --check`, and `cargo build --release --locked -p fg --bin fg` passed. The known excluded test remains `export::fgstring::tests::the_addon_reads_them_and_they_read_the_addon`; this phase does not change it.
- Correctness and quality reviews are complete. Every material correctness finding was reproduced with a failing regression and corrected; the focused timing/accounting follow-up found no remaining material issue. The algorithm report received an independent source-accuracy review; its request-validation and quest-sampling wording was corrected.

## Bounded model smoke comparison

Eight sequential CLI runs compared the preserved `ddfddb5` binary with this
model correction: Classic/Forever Human Warrior 1–30, seeds 42/1337,
`effort=0.05`, one thread/candidate, no mutants, and `bis=off`. This is a model
smoke comparison, not an alternative-algorithm benchmark. Old guides were not
rescored under the new model.

| Case | Player seconds, both builds | Grinding seconds, old → new | Generation seconds, old → new |
| --- | ---: | ---: | ---: |
| Classic, seed 42 | 114,925.98 | 572.02 → unchanged | 6.222 → 13.839 |
| Classic, seed 1337 | 111,165.37 | 1,242.65 → 1,120.29 | 5.331 → 5.193 |
| Forever, seed 42 | 117,354.06 | 2,409.66 → unchanged | 11.757 → 9.126 |
| Forever, seed 1337 | 119,016.73 | 3,263.51 → 3,074.88 | 7.808 → 7.863 |

All eight jobs produced finite outputs. All four pairs have identical saved
expanded plans, total player times, and XP components. The seed-1337 differences
are fighting/grinding classification only: 122.36 seconds in Classic and 188.64
seconds in Forever move back to quest fighting. Requests, effective parameters,
notes, required-content fields, and database/override hashes are unchanged. The
known Islander/Affray warnings remain; all cases have zero vendor buys/spend.

Single-run generation timings vary considerably, including a slower Classic
seed-42 run. They establish neither a general speed gain nor stable overhead.
Corrected model scores must not be interpreted as better search performance;
the [algorithm comparison protocol](../../route-search-alternatives-2026-11-07.md)
requires a fixed corrected simulator and equal computation budgets.

Artifacts and exact values:
`C:\Users\Paul\AppData\Local\Temp\factoruide-phase2-modelsmoke-20261107-080954`
(`paired-model-smoke.json`, `paired-model-smoke.md`, `manifest.json`, requests,
guides and logs). Test, lint and build logs:
`C:\Users\Paul\AppData\Local\Temp\factoruide-phase2-validation`.

## Remaining scope

Random-drop objectives, proxy kill credits, and dungeon work retain their existing
behavior. Within-batch ordering remains conservative as described above. Vendor
data reconciliation remains open. The next proposed algorithm experiment is
prerequisite-closed chain/hub repair, evaluated against the current heuristic at
equal wall-time and memory budgets; no unbenchmarked default has been enabled.
- Independent quality review found no blocking issue with indexed recipient lookup or copy-on-write active work. New allocation/copying cost must still be measured; corrected simulation results do not demonstrate a faster or better search algorithm.
