# Reputation Sources Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [x]`) syntax for tracking.

**Goal:** Return a useful leveling/reputation guide using repeated turn-ins,
explicit farming, spillover and daily circuits, with warnings for remaining goals.

**Architecture:** Preserve ordinary quest search. Add a provenance-bearing reward
kernel and catalog, then a costed completion program with one combined replay.
Persist occurrence-aware activity instructions through export/edit/addon playback.

**Tech Stack:** Rust, SQLite, serde, existing React/TypeScript and Lua addon.

**Spec:** `docs/superpowers/specs/2026-11-08-reputation-sources-design.md`

## Global Constraints

- Reputation shortfalls never abort an otherwise valid guide; other mandatory
  content remains validated.
- Include daily sources and show minimum required completion opportunities.
- Allow Classic fallback estimates with clear warnings, preferring confirmed data.
- Never invent rewards, caps, materials, prices, NPCs or repeat schedules.
- Existing resolved quest reward vectors apply once without new spillover.
- Keep original saved inputs and installed unique recovery files intact.
- Worktree branch `feat/reputation-sources`; commits use prijkes noreply identity.

## Review Focus

- One unsupported faction must not discard leveling or progress for other factions.
- Resolved multi-faction quest vectors must not receive a second spillover pass.
- Shared stock, starter items and money must be consumed on every repetition.
- Daily circuits must repeat all available daily sources together per reset.
- Reload/edit/share must retain counters/effects and finish from actual standings.

## Task 1: Make reputation goals best effort

**Files:** `plan/reputation.rs`, `plan/report.rs`, `job/mod.rs`, `job/edit.rs`,
`plan/tests/required_reputation.rs`, targeted integration tests.

**Interfaces:** Add reusable warning generation from route/trace standings;
retain structural `validate_required` checks without reputation rejection.

- [x] Write failing tests: an unsupported faction still returns a leveling
  guide and named deficit; another reachable faction still gains its reward;
  edited guides report fractional shortfalls; mandatory gear errors still fail.
- [x] Run the focused tests and confirm the expected old rejection.
- [x] Return warnings via outcome notes and existing reputation summary data.
  Retain useful old quests, prerequisites and best-found progress preservation.
- [x] Run focused tests, inspect diff, obtain independent task review and commit.

## Task 2: Reward kernel and source contracts

**Files:** new `src/reputation/{mod,types,transition,tests}.rs`, `src/lib.rs`.

**Interfaces:** Public serializable `Standings`, `SourceConfidence`, signed/capped
award and spillover records; a pure ordered transition API with applied deltas;
compact program/action/circuit types shared with the planner and exporter.
Pin exact field names in `reputation/types.rs` before dependent workers start.

- [x] Test gains/losses, cap pre-check and crossing, fractional resolved vectors,
  no recursive/double spillover, positive-only bonuses, per-kill truncation and
  batched-versus-individual parity.
- [x] Implement the pure transition engine without planner or addon side effects.
- [x] Include confidence and signed applied changes in serializable results.
- [x] Run focused tests, review interfaces and implementation, commit.

## Task 3: Catalog and supported acquisition

**Files:** new `data/reputation/*`, new `model/reputation_sources.rs`, relevant
model/source/type loaders, maintained database overrides, `NOTICE.md`.

**Interfaces:** Load edition-aware source definitions with provenance, schedule,
requirements, effects and acquisition support; source IDs remain stable.

- [x] Test latest-patch selection, signed fallback slots, resolved vectors,
  primary Durotar daily flags/reward correction, one-time introductions, and
  rejection of unsourced writs/unknown enders/unpriced purchases.
- [x] Import the reviewed Classic and Forever quantitative catalogs; do not
  include third-party prose or private profile data.
- [x] Build repeatable candidates with real prerequisites/material sources;
  outdoor kill candidates bind exact current NPCs, spawns and difficulty.
- [x] Add material inventory/profile/reset fields compatibly, or explicitly
  retain zero known imported stock until capture is implemented.
- [x] Run focused data/source tests, review provenance/attribution and commit.

## Task 4: Completion program and combined replay

**Files:** new `plan/reputation_program.rs`, `plan/{sim,step,evaluate,report,state}.rs`,
`job/{mod,start,request,edit}.rs` and focused tests.

**Interfaces:** Combined replay returns final state, trace, breakdown and program;
ordinary target-level completion and terminal accounting are separate operations.

- [x] Test single terminal accounting, prefix-only parity, signed effects, exact
  material consumption, insufficient balance, acquisition travel/respawn/XP, and
  progress on reachable factions when another has no source.
- [x] Replay explicit source actions after the valid prefix. Suppress unspecified
  incidental kill reputation. Track stock/money and source eligibility.
- [x] Add bounded source-program construction, keeping the best partial outcome
  and distinguishing cap/acquisition/search-limit warnings.
- [x] Add grouped daily circuit phases and final partial circuits; test two
  different daily quests execute per reset, never as serial multi-day batches.
- [x] Update continuation reputation/money/materials from combined end state.
- [x] Run focused unit/integration regressions, independent review and commit.

## Task 5: Persistence, exports and UI

**Files:** route/edit/share serialization, export types/format/templates/writer,
frontend API/reputation summaries/settings/localization.

**Interfaces:** Saved `RouteStep` program records and exported protected activity
steps carry stable source/run IDs, counts, costs, confidence, goals and circuits.

- [x] Test save/reopen/share/edit replay parity and old guide/profile compatibility.
- [x] Export from the combined replay without recomputing prefix-only totals.
- [x] Show fallback warnings, best-effort shortfalls, active time and daily facts;
  expose the estimate policy as a meaningful setting.
- [x] Bump guide-format constants consistently with the addon update.
- [x] Run targeted export tests, frontend tests/build/lint/i18n, review and commit.

## Task 6: Reliable addon activity execution

**Files:** guide import/playback/automation/tracking, optional profile inventory
and daily snapshot capture, addon fixtures/smoke tests and changelogs.

**Interfaces:** Decode the exported action/circuit contract; persistent counters
are keyed by route version and activity/run identity.

- [x] Test reload mid-repeat, unrelated turn-in, same quest repeated in distinct
  runs, stop-at-count behavior, disabled automatic acceptance, daily reset/circuit
  order, old released guides and best-effort final checkpoints.
- [x] Implement active-run event counters, actual standing/inventory checks and
  reset-aware circuits. Protect all supporting steps from level-based skipping.
- [x] Add safe profile capture for bag stock and known daily/reset state if used.
- [x] Run full Lua/profile smoke and formatting, review and commit.

## Task 7: Real-data verification and delivery

- [x] Run real city turn-in/farming/spillover and daily-circuit probes with explicit
  source provenance and resource checks. Preserve the user's original inputs.
- [x] Reconstruct the reported targets with the available saved settings and current profile in scratch outputs (the exact failed request was not saved). Verify a guide is
  returned with best-found reputation and named remaining gaps; do not assert
  all ten Exalted goals are attainable.
- [x] Run required Rust release tests/Clippy/fmt, frontend checks and Lua smoke;
  record existing platform exceptions separately from new failures.
- [x] Obtain whole-branch independent review and resolve material findings.
- [x] Merge into main, push the user's fork, rebuild the production desktop with
  `tauri/custom-protocol`, and update the authorized WoW addon installation while
  preserving SavedVariables and generated/user files.
- [x] Verify binary/archive/installation identities and save a concise handoff
  with actual coverage, remaining source gaps and test evidence.
