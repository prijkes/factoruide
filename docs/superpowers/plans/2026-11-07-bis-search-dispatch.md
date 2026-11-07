# BiS search dispatch implementation plan

> **For agentic workers:** Use `superpowers:executing-plans` to implement this single task. Steps use checkboxes for tracking.

**Goal:** Keep vendor purchases, omitted-quest insertion, and whole class-quest version changes independently reachable in local search.

**Architecture:** Preserve fallback draw 19 for quest moves. Add vendor draw 25 only when vendor BiS candidates exist, using 26 choices in that case and the existing 25 choices otherwise. Keep all move implementations and simulation rules unchanged.

The purchase regression also exposed lost starting copper. The implementation
restores that existing input when initializing the route balance; purchase rules
and score weights remain unchanged.

**Tech stack:** Rust 1.99, rand 0.10, existing fg-route miniature-world and SQLite fixtures.

**Spec:** [Route-quality assessment](../../route-quality-assessment-2026-11-07.md), vendor search reachability section. The user's subsequent “OK, continue” authorizes this first implementation phase.

## Constraints

- Preserve existing upstream refactors, cache limits, gold/BiS behavior, and gameplay validation.
- Guaranteed BiS sources remain quest rewards and vendor purchases only.
- Keep the no-vendor draw range and RNG behavior unchanged.
- No search-default, shared-kill, prerequisite, or source-data changes in this phase.
- Obtain independent correctness and quality reviews before committing as `prijkes <prijkes@noreply.com>`.

## Review focus

- Vendor candidates must not suppress ordinary omitted quests or alternative class chains.
- An already-planned purchase must not suppress either quest capability.
- Vendor purchases must still be reachable and valid when affordable and wearable.
- No-vendor searches must retain both quest capabilities and their prior deterministic output.
- Fixtures must assert nonempty vendor candidates and validate the observed route changes.

## Task 1: Restore independent dispatch and prove it

**Files:**

- Modify `crates/fg-route/src/plan/moves.rs`: random dispatch only.
- Modify `crates/fg-route/src/plan/sim.rs`: restore recorded starting copper in the balance.
- Modify `crates/fg-route/src/plan/tests.rs`: register regression module.
- Create `crates/fg-route/src/plan/tests/moves.rs`: dispatcher regression scenarios.
- Modify `CHANGELOG.md`: one user-facing guide fix entry.

**Interfaces:** `Planner::random_move(&mut Vec<Stop>, &Run, &mut SmallRng) -> Option<Change>` remains unchanged. Tests call the dispatcher on independent clones of a fixed valid base route and its matching run; no production test hooks or new runtime dependencies.

- [x] Add tests with one modeled vendor item, omitted ordinary quest, and existing whole-class-version fixture. Cover fresh and already-present buys, plus no-vendor controls. Use bounded fixed-seed trials; assert observed buys, inserted quest acceptance/turn-in, and complete valid version replacement.
- [x] Run the focused tests on the original dispatcher. Vendor-bearing quest insertion and version replacement must fail while no-vendor controls pass.
- [x] Change the vendor draw to 25, extending the range only for a nonempty vendor list; retain fallback 19 and all existing arms.
- [x] Run focused tests to green, then route library/integration/doc tests, formatting, and package Clippy. Report the known Windows fgstring newline failure separately if reproduced.
- [x] Compare fixed-seed no-vendor and vendor-bearing reduced-budget runs against the original revision. Record player time, exact weighted score, and generation wall time separately; do not generalize from this diagnostic matrix.
- [x] Resolve independent correctness and quality review findings. Update completion evidence and commit the implementation.

## Execution record

- Branch: `fix/bis-search-dispatch`, based on `e041e69`; existing handoff and assessment preserved.
- Baseline: `cargo test --release -p fg-route --lib -- --skip export::fgstring::tests::the_addon_reads_them_and_they_read_the_addon`: 249 passed, one known test filtered out.
- Execution choice: implement this one small change in the existing checkout on a dedicated local branch, reusing its build artifacts; reviewers work read-only. No separate worktree is needed for concurrent edits.
- Ruling: the vendor fixture exposed `initial.money` being discarded by `Simulator::initial_state`, which always used `Breakdown::default()`. Restore `gold_start` from existing initial money with a dedicated regression before the dispatcher fix; otherwise even a wearable upgrade with recorded starting funds cannot be bought. This adds a small initialization correction in `plan/sim.rs` without changing scoring weights or purchasing rules.
- Test setup: the miniature SQLite base predates item-price columns, so only the in-memory vendor fixture adds those columns. No persisted database is modified.
- Starting-copper regression observed RED: the simulated `gold_start` was 0 instead of the recorded 123 copper.
- Starting-copper regression GREEN after restoring the existing recorded money to the route balance. With valid, affordable vendor fixtures, the unchanged dispatcher produced the intended RED result: buys and both no-vendor controls passed; omitted quests and class-version replacement failed when vendor candidates existed.
- Dispatcher regression GREEN: all five focused tests pass, including fresh/already-present buy cases and both no-vendor controls. The starting-copper regression also passes independently.
- Independent correctness and quality reviewers reported no blocking findings. The correctness review traced starting copper through profile normalization, replay, checkpoints, incremental state cloning, and the existing primary-character group model.
- Validation: the full route-package test run reproduced only the documented Windows CRLF failure in `export::fgstring::tests::the_addon_reads_them_and_they_read_the_addon` (255 other library tests passed). With that test explicitly excluded, 255 library and 238 integration tests passed; two documentation examples remain ignored. Release package Clippy with all targets and warnings denied passed after adapting the new assertions to its diagnostic style.
- Final code validation: the filtered full route-package suite passed again after the assertion cleanup (255 library and 238 integration tests). `cargo fmt --all --check`, `git diff --check`, and `cargo build --release --locked -p fg --bin fg` passed. No app/addon files, planner defaults, or persisted game data changed.

## Completed paired probes

Sixteen sequential CLI runs compared the original `e041e69` executable with the
new build. Every run used `effort=0.05`, one thread, one candidate, no mutants,
and seed 42 or 1337. Human Warrior 1–30 controls used `bis=off`; Tauren Druid 1–5
diagnostics used `bis=prefer` and excluded quest 382. All requests started with
zero copper, so the balance correction is inert in these comparisons.

| Case | Player seconds, old → new | Weighted score, old → new | Generation seconds, old → new |
| --- | ---: | ---: | ---: |
| Classic Human 1–30, seed 42 | 114,925.98 → unchanged | 147,925.74 → unchanged | 3.948 → 3.937 |
| Classic Human 1–30, seed 1337 | 111,165.37 → unchanged | 143,906.62 → unchanged | 3.821 → 3.755 |
| Classic Tauren 1–5, seed 42 | 2,367.58 → unchanged | 3,650.58 → unchanged | 1.034 → 1.027 |
| Classic Tauren 1–5, seed 1337 | 2,349.86 → unchanged | 3,629.61 → unchanged | 1.036 → 1.004 |
| Forever Human 1–30, seed 42 | 117,354.06 → unchanged | 148,388.04 → unchanged | 5.449 → 5.493 |
| Forever Human 1–30, seed 1337 | 119,016.73 → unchanged | 150,965.53 → unchanged | 5.493 → 5.526 |
| Forever Tauren 1–5, seed 42 | 2,384.20 → 2,479.88 | 2,527.60 → 2,629.36 | 1.361 → 1.339 |
| Forever Tauren 1–5, seed 1337 | 2,346.64 → 2,381.81 | 2,583.37 → 2,525.21 | 1.358 → 1.368 |

All four no-vendor controls have exactly equal plans, exported routes, weighted
scores, unrounded player times, and warnings. Both Classic vendor diagnostics
also preserve those results. Every vendor diagnostic has exactly one candidate;
all runs execute zero buys and spend zero copper at gear vendors. Required-content
fields and warnings are unchanged, including the two known Human level-30
class-quest warnings.

Both Forever vendor probes have worse player time: **+95.69 seconds** and
**+35.17 seconds**. Seed 42 also worsens score by **101.76**; seed 1337 improves
score by **58.16**. The vendor-case draw range changes the seeded search
trajectory. These limited results justify neither a general quality-gain claim
nor tuning to these two seeds. The quality reviewer accepted the narrow
capability repair with these regressions explicitly recorded. Single-run wall
times here do not establish a generation-speed improvement.

Vendor scores use normal `Planner::simulate_full` replays of the saved core stops;
all eight reproduce every saved serialized breakdown component within floating
point tolerance. Three expanded editor replays skipped the automatic sale after
the final level-five turn-in, undercounting by 23.181685 seconds. Those mismatched
editor outputs were retained as diagnostics and were not used as scores. Human
editor replays reproduce player time; their travel/flight reclassification is
score-neutral because `flight_weight=1`. The final sixteen scores were also
independently reconstructed from the saved breakdowns and recovered hidden
penalties, with every replay total matching the saved unrounded player time.

Evidence is in
`C:\Users\Paul\AppData\Local\Temp\factoruide-phase1-quality-20261107-072903`:
`manifest.json`, `generation-results.json`, `trusted-results.json`,
`paired-results.json`, `paired-results.md`, saved guides/logs, and diagnostic
source/executables. Binary, database, and override hashes are recorded; database
and override hashes remained unchanged. Test/build logs are in
`C:\Users\Paul\AppData\Local\Temp\factoruide-phase1-validation`.

This phase restores dispatcher capabilities and recorded starting funds. Shared
direct-kill credit, vendor-source reconciliation, optional-chain insertion, and
broader search-quality experiments remain subsequent phases of the assessment.
