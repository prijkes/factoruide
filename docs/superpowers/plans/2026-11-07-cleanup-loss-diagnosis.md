# Cleanup loss diagnosis implementation plan

> **For agentic workers:** Use systematic debugging, subagent-driven development, focused TDD, independent review and fresh verification. The user accepted cleanup investigation and asked whether autonomous regeneration is possible; proceed without requiring manual app operation.

**Goal:** Reproduce the saved cleanup loss autonomously and identify the responsible transformation before changing route behavior.

**Architecture:** Add an opt-in observed search entry point sharing the existing search and cleanup implementation. Capture owned stop snapshots only during cleanup, then score and trace them afterwards through fresh ordinary planners. Regenerate the archived case with its exact request, seed and fixed attempt count, without using wall time as a quality endpoint.

**Tech stack:** Existing Rust planner and example infrastructure, JSON artifacts, PowerShell orchestration.

**Spec/evidence:** [Measured comparison](../../route-algorithm-comparison-2026-11-07.md), [cleanup investigation](../../route-comparison-2026-11-07/cleanup-analysis.md). Base `e73af02`; branch `diagnostic/cleanup-loss`.

## Global constraints

- Preserve simulator semantics, defaults, quest/dungeon consistency, group/profession/gear/gold features, data and cache/memory caps. No speculative fix.
- Normal search remains the same implementation, with a no-op observer. No extra RNG, simulations, file I/O or route clones for unobserved callers.
- Observer callbacks only copy stops and stage labels; score/trace snapshots after transformations finish to avoid warming memo tables midway through cleanup.
- Keep the existing combined rebind/hearth comparison and run update even while observing the intermediate route.
- Use ordinary planning mode, not `Planner::replaying()`, for equality checks.
- No timed performance claims while the user may be generating a route. Fixed-attempt diagnostic results are separate from frozen experiment measurements.
- Do not alter the app's current generation or saved guides. Commit as prijkes; do not merge, push or rebuild the installer implicitly.

## Ownership and review focus

- Observer worker: search.rs, plan/mod.rs exports and dedicated cleanup observation tests. Coordinator owns test registration to avoid shared-file edits.
- Diagnostic example worker: new examples/cleanup_diagnose.rs and its helper tests only.
- Accounting-oracle worker: new focused tests under plan/tests; no production fix until an actual failing oracle is reviewed.
- Coordinator: serialized Cargo, artifact/data hashes, replay execution, evidence interpretation, documentation and commits.
- Review: unchanged stage ordering and run updates; disabled observer compatibility; every boundary including unchanged passes; input/best selection; fresh ordinary replay; complete callback and final-route equality; last-beneficiary abandonment accounting.

## Task 1: Opt-in cleanup observation

- [x] Add tests and stubs, observe RED for complete ordered snapshots and observed/unobserved output equivalence at fixed attempts.
- [x] Add public `CleanupStage` and `improve_while_observed(..., on_cleanup: impl Fn(CleanupStage, &[Stop]))`; retain `improve_while` as the no-op-observer wrapper.
- [x] Snapshot selected input, power-first, rebind, hearth placement, unfinished removal, target trim and dungeonless removal. Include unchanged boundaries and preserve abort behavior.
- [x] Verify focused tests and independent review before trusting diagnosis.

## Task 2: Archived-request regeneration

- [x] Build a separate example reading an archived result and writing new diagnostic artifacts without modifying the original.
- [x] Restore effective_request exactly, use direct one-planner construction and fixed tries: chain 59,526; baseline 64,872. Replace both timed stopping conditions. Reject annealing/sharing cases rather than claiming deterministic replay without proof.
- [x] Compare initial and complete search callback sequences (excluding timestamps), improvement count, final score/player and final typed stop sequence against the archived run.
- [x] Capture cleanup snapshots without extra scoring during search. Afterwards use fresh ordinary planners for exact score/player/breakdown, full trace, quest IDs, listed/actual dungeon entries, turn-ins and abandonments. Retain pre/post route identity and removed quest IDs.
- [x] Locate first major score divergence and reconcile its weighted components. If archived equality fails, label it a diagnostic reproduction and investigate before claiming exact replay.

## Task 3: Independent accounting oracle

- [x] Exercise real outdoor kills followed by an unfinished quest remaining active, its last beneficiary abandoned, and a genuine shared reward. Zero unrelated allowances/farming to make accounting interpretable.
- [x] Observe whether abandoning alone makes performed unrewarded work disappear from grinding classification. Retain one-abandoned/other-rewarded behavior as control.
- [x] If the oracle fails, report the minimal reproducer and identify its causal relevance separately from cleanup attribution. Do not change production accounting speculatively.
- [x] After actual RED and independent design review, correct lost abandoned work using actual route-performed provenance, a bounded orphaned-work scalar and active legacy ledger. Preserve imported-progress exclusions, direct reward protection, overlap cap, initialization and incremental suffix equality; validate import, partial, gathering and settlement controls.
- [x] Rescore the saved old-semantic fixed routes under the corrected accounting without searching, to separate model-rating changes from new search trajectories.

## Task 4: Diagnose and review

- [x] Verify source/data provenance, snapshot equality and exact score reconciliation.
- [x] Obtain independent correctness review of instrumentation and conclusions.
- [x] Record whether the loss is a confirmed defect, intentional cleanup exposing a search-objective mismatch, or unresolved; propose only the evidence-supported correction.
- [x] Run appropriate checks, preserve artifacts, update handoff and commit the diagnostic result.

## Initial evidence

- Forever Gnome Mage 20–40, seed 20261107: identical initial route callback (981 stops, score251896.69785715017, player161906.22146329362). Chain raw best159358.0720351529, final222219.2418109174; baseline raw best170622.3497908, final169640.2243407399.
- Anneal=0, lahc=0, no sharing. Elapsed progress does not affect acceptance or RNG in this configuration; exact fixed-attempt replay still requires callback/route checks.
- The user app process was found as factoruide-app; it is never stopped or modified. A short CPU sample was idle, but no timing claim depends on its activity.
- Autonomous replay succeeded for both archived cases before accounting changes: all initial/complete callbacks, attempts, improvements, effective request/params, final score/player and final typed stop sequence match exactly. Fresh selected-input replay matches the raw search score.
- The chain jump is exactly `DropDungeonless`, removing the sole Accept of quest387, Quell the Uprising (absent dungeon717): score159260.4500628052 to227141.55534933577, delta67881.10528653057; player time increases12345.748046475375. Later finalization improves4922.313538418384. Exact pass attribution is now established, superseding the previous endpoint-only uncertainty.
- Independent trace analysis found that removing that accept also removes a nearby Mage training event; accounting causality remains separate. Do not keep an unusable quest as a speculative workaround.
- Four authentic accounting oracles produced2PASS controls and2FAIL expiry cases: unrewarded four-kill grinding0vs96seconds for both direct and legacy work. Independent design review confirmed the defect and route-provenance correction; accounting source edits were held until both original-semantic captures finished.

## Completion record

- Implemented and committed source correction and opt-in diagnostics as `09752b6`; metadata correction is `df96a30`. Both are on `diagnostic/cleanup-loss`, authored prijkes.
- Final release verification passed 321 library, 238 integration, eight diagnostic-example and three comparison-example tests, format and all-target Clippy with warnings denied. The known Windows fgstring newline check was explicitly skipped; two doc examples remain ignored. Fast-profile package verification also passed.
- The full original chain and baseline fixed-attempt replays matched all recorded-observable gates. All sixteen fixed snapshot routes retain identical score/player/stops/full traces under corrected accounting; no new search or benchmark was inferred from rescoring.
- Final source review approved observer/accounting/profession-test changes and requested one diagnostic completeness correction. Explicit Dungeon stops are now included alongside implicit entries, deduplicated by first identity occurrence; eight example tests pass. Corrected v2 snapshots and original captures have identical full traces and scores; derived metadata is transparently corrected while original raw JSON remains immutable. Scoped re-review marked the finding addressed with no new material issue.
- The completed [diagnosis report](../../cleanup-loss-diagnosis-2026-11-07.md) and [evidence archive](../../cleanup-diagnosis-2026-11-07/README.md) record exact pass attribution, training cascade, accounting correction, coverage and remaining limits. The next routing feature is independent training/service preservation, not retaining an impossible dungeon quest. No such counterfactual feature, installer update, merge, push or new algorithm promotion was performed.
