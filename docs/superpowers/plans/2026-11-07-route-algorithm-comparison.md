# Route algorithm comparison implementation and experiment plan

> **For agentic workers:** Use `superpowers:subagent-driven-development`, test-driven development, independent review, and fresh verification. Execute through the evidence and conclusion without another permission checkpoint.

**Goal:** Determine whether practical search alternatives improve route quality within the same computation budget on the corrected simulator.

**Spec:** [Search alternatives and comparison protocol](../../route-search-alternatives-2026-11-07.md). The user explicitly requested: “continue until we know.” Base revision: `d1c733c`.

**Architecture:** Keep the corrected simulator fixed. Add two bounded search operators behind parameters defaulting to zero. Compare them with the existing search and exact-evaluation control using the same greedy starting policy, common cleanup/finalization, one worker, and full measured computation budgets. This first experiment compares search kernels, not the app-default multi-start tournament. A promising winner needs a separate production-tournament check before default promotion.

## Global constraints

- Preserve XP/rested XP, prerequisites, quest-log limits, completion windows, timers, dungeon grouping, deterministic sourcing, gold/BiS features, client assets, upstream refactors, and existing memory caps.
- Guaranteed BiS sources remain quest rewards and vendor purchases only; no game-data edits are part of this experiment.
- Baseline parameters and RNG consumption remain unchanged when both experimental probabilities are zero.
- All candidate routes receive authoritative full simulation and final required-content validation; no comparison across different simulation semantics.
- New code remains experimental and off by default unless later held-out evidence and production validation support promotion.
- No planner or compiler jobs run concurrently with timed experiment processes.
- Commit reviewed work as `prijkes <prijkes@noreply.com>`; do not push or merge implicitly.

## Interfaces and ownership

- Chain worker: create `crates/fg-route/src/plan/chains.rs`, including its unit tests. API `Planner::add_optional_chain(&mut Vec<Stop>, &Run, &mut SmallRng) -> Option<Change>` and deterministic `insert_optional_chain(..., endpoint: u32)` for the finite oracle.
- Neighborhood worker: create `crates/fg-route/src/plan/neighborhood.rs`, including its unit tests. API `Planner::repair_nearby_quests(&mut Vec<Stop>, &Run, &mut SmallRng) -> Option<Change>`.
- Harness worker: create `crates/fg-route/examples/quality_compare.rs` plus the smallest production finalization wrapper in `job/mod.rs`; create a standard-library Python sequential runner under `scripts/`. Do not modify the operators or parameters.
- Coordinator: register modules in `plan/mod.rs`; add `Params::chain_insert` and `Params::hub_repair` as probabilities defaulting to `0.0`; gate their dispatcher calls before drawing RNG. No test-only production hooks.
- Experimental frequencies are fixed before tuning: chain `0.05`, neighborhood `0.02` of small-move dispatcher calls, each as its own arm. With default ALNS at 25%, their overall expected rates are 3.75% and 1.5%. Control arms use both zero; exact control also sets `eval_window=0`.

## Review focus

- Omitted optional chains must reuse completed/existing ancestors, satisfy AND/OR cardinality, and reject cycles, conflicts, and over-budget closure atomically.
- Neighborhood repair must preserve its selected stop multiset and unrelated mandatory content; invalid leaves cannot mask a valid improvement.
- Off-default behavior must preserve baseline draws, simulation, and output.
- Computation budget includes preparation, construction, search cleanup, finalization, exact replay, validation, and export. Search-only timing is insufficient.
- All failures, missing-content changes, budget/memory overruns, and unfavorable cases remain in the report.

## Task 1: Bounded optional-chain insertion

- [x] Write and observe a failing finite `A -> B -> C` oracle: A and A+B lose against a remote XP fallback, but the complete local chain wins. Include initial-completed/existing ancestry, AND/OR thresholds, conflicting alternatives, cycles/missing prerequisites, closure bounds, and unchanged route on rejection.
- [x] Collect missing normal prerequisites with at most eight newly inserted quests, depth sixteen, and sixty-four branch visits. Reuse existing insertion and linked-quest machinery; include any automatically added partners in the cap.
- [x] Insert into a clone, preserve prerequisite order, and commit atomically. Simulation remains authoritative for candidate acceptance.
- [x] Integrate probability gating, verify zero-mode compatibility, run focused tests, and review.

## Task 2: Bounded neighborhood repair

- [x] Write and observe a failing spatially inefficient two/three-quest case where a valid reconstruction improves exact score.
- [x] Select two or three complete optional outdoor quests near a seeded anchor. Exclude timed, linked, dungeon, initial-log, and partial quests for this initial experiment.
- [x] Try at most six insertion-order permutations through existing prerequisite ordering and leveled insertion. Exact-simulate leaves, preserve stop multiset, check start zone, and retain the best valid improvement exceeding 0.5.
- [x] Cover no-improvement/rejection atomicity, prerequisites/followers, unrelated mandatory content, and reproducibility. Integrate zero-default gating and review.

## Task 3: Reproducible comparison harness

- [x] Expose one common finalization wrapper for production and the example: merge visits, tidy, group objectives, and validate requirements in the same order.
- [x] Implement a Rust example accepting edition/request, seed, variant, total budget, reserve, and output path. Prepare and construct once, then use public `Effort::Time` with preparation/construction deducted and an outer cutoff. Do not use `abortable` in a way that skips cleanup.
- [x] Record exact final `simulate_full` score, player time, hidden penalties, breakdown, warnings/required counts, route/guide, stage timing, search log/tries, effective request, and best-score-over-time callbacks. Final finalized score supersedes the callback endpoint when cleanup changes it.
- [x] Implement fresh-process, strictly sequential execution, counterbalanced order, hashes/manifests, peak-memory observation, bounded timeouts, and resumable append-only artifacts. Serialization/process overhead is recorded separately from measured computation.
- [x] Pilot before freezing reserve and budgets. Run route-package tests, format, Clippy, and release example build. Review correctness and protocol before timing.

## Frozen experiment protocol

1. Use eight profiles: Classic and Forever, each Human Warrior 1–30, Undead Mage 1–40, Orc Warrior 20–40, and Gnome Mage 20–40. Validate every request and its baseline before interpreting a candidate. Use `bis=off` to isolate search from the outstanding vendor-data reconciliation.
2. Common single-worker greedy bootstrap and full finalization. Full computation allowance is initially 20 seconds, with a uniform 3-second finalization reserve. Pilot Classic Human and Forever Undead before tuning; if needed, increase the uniform reserve or budget once, record the reason, and freeze before comparison.
3. A run exceeding the frozen budget by more than 0.25 seconds is a budget violation. Observe a common peak-memory limit of 1.5 GiB. Keep overruns/failures in the result table; they cannot establish a win. Parent timeout is budget plus 10 seconds.
4. Tuning uses seeds 42 and 1337, all eight profiles, four arms: baseline, chain insertion, neighborhood repair, exact evaluation. Latin-square/counterbalanced arm order; 64 runs at 20 seconds is about 21 minutes.
5. Freeze one candidate from tuning using equal-profile geometric-mean exact final score; no per-case oracle selection. If no candidate meets basic feasibility/resource criteria, retain the best non-broken arm for a clearly labeled negative/inconclusive held-out comparison.
6. Held-out seeds are 20261107 and 314159. Compare frozen candidate versus baseline on all eight profiles, alternating AB/BA; 32 runs at 20 seconds is about 11 minutes. Do not tune on held-out outcomes.
7. A practical broader win requires at least 1% equal-profile geometric-mean score improvement, positive two-seed mean improvement in at least six of eight profiles, no individual score regression above 2%, no player-time geometric-mean regression above 0.25%, and no new feasibility, required-content, memory, or budget failures. Show every paired value, win/tie/loss at 0.5 score units, median, worst regression, and anytime curves. Seeds are replicates within profiles, not independent profiles.
8. Near-threshold or case-dependent results remain inconclusive. Failed candidates establish a bounded negative result for these implementations/budgets, not proof that no better algorithm exists.
9. Any winner remains off by default until representative production-tournament checks at the same resource limits confirm the result. If none wins, record what failed and the next evidence-driven direction; do not manufacture a positive conclusion.

## Task 4: Execute, review, conclude

- [x] Complete and review the finite oracles and harness.
- [x] Freeze the pilot-adjusted protocol, source, databases, requests, and runner before tuning.
- [x] Run tuning, freeze the candidate, and execute held-out comparisons without source changes or concurrent builds.
- [x] Analyze all metrics and limits; investigate validity or accounting discrepancies before trusting scores.
- [x] Obtain separate correctness and quality reviews of implementation and conclusions, address material findings, update the handoff, and commit the measured conclusion.

## Execution record

- Source starts from corrected shared-kill/accounting revision `d1c733c`; existing known Windows fgstring CRLF failure remains separately reported.
- Scope call: this first comparison uses a common single-start kernel to give operator changes a measurable search budget. It cannot alone establish superiority over the app's default tournament.
- Pre-flight: workers own separate new operator files and the harness/finalizer; the coordinator owns parameter/dispatcher/module integration. Shared signatures are fixed above. Builds are serialized; timed runs wait for all builds to finish.
- Pre-pilot implementation reviewed independently. Corrected linked-partner cycle/bounds bypass, per-member OR prerequisites, warning-ID comparison, and exceptional child cleanup. All four Important review findings were re-reviewed as addressed.
- Final pre-pilot checks passed: format, release Clippy across all route targets with warnings denied, 302 library tests, 238 integration tests, three example tests, 14 Python protocol checks, and release example build. The known Windows fgstring newline test is explicitly skipped; two doc examples remain ignored. No game data or production defaults changed.
- All eight pilots passed without adjusting the 20-second allowance or 3-second reserve. Frozen source `a3ce987` then completed all 64 tuning and 32 held-out runs sequentially, without compiler/planner overlap. No failed attempt was replaced. All 104 execution/resource checks passed.
- Tuning selected chain insertion by the declared rule: 2.297914% score improvement, 1.179006% player-time improvement, seven positive profiles, but a 5.395765% worst regression. Hub and exact remained tuning-only observations.
- Held-out chain insertion failed promotion: weighted score worsened 0.456956%, player time improved 1.273725%, four of eight profiles had positive two-seed mean savings, nine pairs won and seven lost, and worst regression was 30.994428%. No new content/resource failure was found. Both experimental defaults remain zero; no production-tournament trial, merge, or push was performed.
- The independent 104-run audit verified hashes, 70 bootstrap/configuration comparisons, stage/player accounting and all recorded constraints; a separate evidence review verified all 31,657 anytime points, freeze chronology and lack of process overlap. Explicit hard-required quest counts were zero in this timed panel, a stated coverage limit.
- The largest score loss is localized to search cleanup before later finalization. Excess-grinding penalties dominate the delivered gap. Dungeonless-removal policy is a source-supported hypothesis, not a proven subpass cause or confirmed correctness bug; the endpoints do not preserve per-pass route snapshots.
- The [measured report](../../route-algorithm-comparison-2026-11-07.md) and [archived evidence](../../route-comparison-2026-11-07/README.md) preserve all pairs, frozen identities, curve data, plot and cleanup investigation. Final whole-branch code and independent evidence/prose reviews approved the bounded no-promotion conclusion; retained Minor implementation finding is quadratic neighborhood eligibility scanning.
