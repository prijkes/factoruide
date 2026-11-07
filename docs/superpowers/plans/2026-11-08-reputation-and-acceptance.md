# Required Reputation and Missed Acceptance Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task by task. Steps use checkbox syntax for tracking.

**Goal:** Enforce selected reputation outcomes, retain useful old quests, protect their guide steps, and warn about offered planned quests left unaccepted.

**Architecture:** Extend the existing model, actual-state hard-goal ranking and route validation. Export reputation keep flags and honor them in the addon. Audit identified offered quests per NPC interaction using existing quest-log adapters and event compatibility.

**Tech Stack:** Rust, SQLite, TypeScript/React, Lua/LuaJIT, existing localization tooling; no added dependencies.

**Spec:** [Approved design](../specs/2026-11-08-reputation-and-acceptance-design.md).

## Global Constraints

- Selected reputation totals are mandatory, including detours and work after level 60.
- A target does not require every quest rewarding that faction.
- Audit any identified offered planned quest left missing, including Shift/manual mode.
- No invented repeatable rewards, spillover or live starting standings.
- Preserve exact/class/gear/forced-run rules, unrelated defaults, RNG and cache policies.
- No-target requests keep their existing fast path and behavior.
- Original saved requests/outcomes and active application sessions remain untouched.
- Author and committer: `prijkes <prijkes@noreply.com>`.
- Merge reviewed passing work into local `main`; no push requested.

## Review Focus

- Reputation-only requests must activate ranking and validation with gear Off.
- A faction's satisfied target must survive changes that improve a different faction.
- Lower-level prerequisite chains and fractional data must remain useful without making all candidates mandatory.
- Interaction close/open transitions and delayed logs must not generate false warnings.
- Old guides must load and display honest shortfalls; protected new steps must survive both skip mechanisms.

### Task 1: Required reputation model and route enforcement

**Files:** `crates/fg-route/src/model/{selection,sources,chains,types,builder,mod}.rs` as needed; focused new model reputation module if useful; `plan/{availability,construct,gear,report,search,worth,mod}.rs` and focused new reputation module as needed; `job/{mod,race,reputation,edit,notes}.rs`; existing model/plan/integration tests.

**Interfaces:** Consume `Params.reputation_goals`, quest direct rewards and existing initial reputation. Produce actual-state reputation hard rank, search/cleanup preservation, contribution repair and final/edited-trace validation. Existing `Quest.reputation` remains the export-facing direct reward interface; report new starting-value fallback notes through existing notes plumbing. Do not edit exporter or frontend in this task.

- [x] Write and run functional failing tests for low-level useful rewards/zero-reward ancestors, fractional rewards, costly progress, gear-Off goals, per-faction preservation and unmet-target final/trace validation.
- [x] Retain narrow goal-relevant candidates and prerequisite alternatives, preserving all unrelated availability rules.
- [x] Extend construction/repair and hard-goal ranking through search, sharing, tournaments, cleanup/finalization and invalid replay fallback.
- [x] Validate actual outcomes and emit named shortfalls with starting provenance; label selected-faction fallback starts.
- [x] Run focused tests, formatting and appropriate lint checks; self-review; report exact evidence and remaining limits.

### Task 2: Addon protection and interaction audit

**Files:** `addon/Factoruide_Guide/Guide.lua`, `Automation.lua`, a focused audit module and TOCs if useful; `addon/tests/smoke.lua`; addon localization files.

**Interfaces:** Consume existing `kp` keep flags (export expanded in Task 3), planned accept indexes, `FG.QuestLog` and `FG:On` events. Produce a single warning per closed interaction about identified offered planned missing quests. Keep ordinary acceptance confirmation and manual navigation unchanged.

- [x] Write and run failing smoke regressions for `kp` under outdated-step skipping and all-offered missing-quest warnings.
- [x] Honor keep protection in automatic outdated-step policy as well as existing checkpoint offer.
- [x] Track bounded route/NPC interaction tokens and offered ID unions, independently of auto-accept/Shift; handle close transitions and deferred log confirmation.
- [x] Warn through yellow screen text plus chat, respect `uiShown`, deduplicate callbacks; cancel stale route/mark/completion state. Normalize accepted event payloads.
- [x] Cover two offers/one accepted, delayed confirmation, manual/Shift, close/reopen/different NPC, route change, skips, compatibility and unidentified legacy metadata; run the full Lua smoke suite and self-review.

### Task 3: Export protection and truthful frontend messaging

**Files:** `crates/fg-route/src/export/checkpoints.rs` and local tests; `app/src/components/{ReputationPanel,RouteSummary}.tsx`, `app/src/paramSchema.ts`, app locale files and existing relevant test helpers.

**Interfaces:** Consume goal faction IDs, modeled direct quest rewards and the actual exported route steps. Mark selected-faction contribution quests and their planned prerequisite/held/handed support `Step.keep`, including guides with no checkpoints; preserve existing keep flags. Describe required outcomes and legacy route shortfalls using existing UI patterns.

- [x] Write and run failing export regressions for relevant quest support with/without checkpoints and unrelated quest behavior.
- [x] Implement global reputation keep marking without making unused alternative quests mandatory.
- [x] Update reputation goal/weight help and old shortfall wording; keep old guide schemas compatible.
- [x] Run focused Rust tests, frontend tests/build/lint and localization checks; self-review.

### Task 4: Integration, review and delivery

**Files:** Updated plan, handoff and changelog; sanitized validation record if useful. Private diagnostics remain in ignored plan workspace.

- [ ] Review each task for spec compliance and code quality; fix consequential findings and obtain scoped verification.
- [ ] Regenerate the saved Exalted request into ignored diagnostics and verify honest named failure with original inputs unchanged; also generate a bounded feasible reputation-target case and verify actual completion, keep flags and no false warning contract.
- [ ] Run workspace release tests (document existing exclusions), all-target release Clippy and formatting; frontend checks; Lua smoke suite.
- [ ] Build the production desktop executable with `cargo build --release --locked --offline -p factoruide-app --features tauri/custom-protocol` after frontend build.
- [ ] Obtain independent whole-branch review; resolve material findings and rerun affected checks.
- [ ] Update handoff/evidence, merge into local `main`, verify merged tree/status and report executable identity and remaining model limits.
