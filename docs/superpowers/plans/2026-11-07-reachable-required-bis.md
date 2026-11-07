# Supported Required Gear Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans. Execute task-by-task with functional regressions and independent review.

**Goal:** Replace unsupported gear sources with explained supported alternatives, enforce selected Required quest sources, and report true class/gear goals without corrupting old guides.

**Architecture:** A nonmutating source/action selector feeds BiS ranking and construction. Required source goals remain separate from exact quest requirements. Provenance feeds optional authoritative warning counts while the old aggregate retains scoring/compatibility semantics.

**Tech Stack:** Rust, existing Scenario/database fixtures, React/TypeScript, Node tests, PowerShell.

**Spec:** [Supported gear design](../specs/2026-11-07-reachable-required-bis-design.md).

## Global Constraints

- User-confirmed policy: choose the best supported alternative and explain the fallback.
- Preserve exact `q.required` semantics, class/zone/dungeon rules, search random draws/operators/defaults, cache policies and memory caps. Keep the leveling endpoint.
- Required guarantees selected quest sources; vendor purchases remain advisory. Never promise mutually exclusive reward ownership.
- Preserve original saved guides/requests and keep private character data out of commits. New diagnostic outputs must use fresh paths.
- Keep old aggregate scoring once per representative. Old unclassified guides must never be called true class misses.
- Commit as `prijkes <prijkes@noreply.com>` on the local fix branch. Build the desktop app after verification; do not create an installer, launch/replace app sessions, push or merge.

## Review Focus

- Multiple givers, completed/unmodeled prerequisites and per-member knowledge must not create false support.
- Held/handed parent-child actions, N-of-M choices and pure cycles must be distinguished; failed choices leave no selection residue.
- Target-only endgame sources must fall back; Required must not become unbounded post-target planning.
- Actual hard goal completion must survive search, tournament selection, cleanup and editor save without extra work for BiS Off.
- Legacy counters, overlapping origins and comparisons must not fabricate class failures or score improvements.

### Task 1: Supported source selection and ranking

**Files:** Create `crates/fg-route/src/model/feasibility.rs` and focused tests (split into a sibling test module if needed). Modify `model/bis.rs`, `model/mod.rs`, `model/derive.rs`/`chains.rs` only for recorded-progress filtering if necessary, `job/start.rs` for reuse of the existing progress matcher, `job/notes.rs` or its current note seam, and narrow fixture helpers as needed.

**Interfaces:** Produce `BisQuestAction::{Accept(u32), Objective { quest:u32, objective:u8 }, TurnIn(u32)}`, `BisQuestPlan { root:u32, quests:Vec<u32>, actions:Vec<BisQuestAction> }`, and all `Bis` getters specified in the design, including the final global source DAG/participants. Re-export model-owned public types. Preserve existing Bis entry/gives/vendor interfaces.

- [x] Write and run a compiling functional RED: a higher-value reward with a missing uncompleted predecessor must fall back to a lower supported reward with a reason. A recorded predecessor completion must make the higher reward eligible.
- [x] Implement nonmutating, bounded, deterministic action-closure selection with original N-of-M cardinality, branch rollback/backtracking, initial/member evidence, held vs completed semantics, handed items, cycle detection, exclusivity/breadcrumbs, levels/windows and dungeon support.
- [x] Validate one-continuous-visit topology before ranking, including the outside-hand-in/inside-return pattern in Scholomance's 5382→5515→5384 chain. Preserve fully inside progress, cut recorded satisfied visits, and enforce acceptance before blackbox entry credit; never promise repeated modeled runs.
- [x] Cut satisfied historical edges for initial completions, accepts and objective progress using the existing matcher. Retain legitimately satisfied logged objectives through pre-selector unhanded filtering. Cover a held-only parent whose plan needs Accept but no TurnIn; it must not become a fabricated missed completion goal.
- [x] Rank all item givers after eligibility, commit compatible closures across bands/slots, deduplicate roots/share library choices, mark only supported Required closures mandatory and emit deduplicated fallback notes. Prefer creates no hard roots; Off bypasses the selector.
- [x] Cover alternate givers/storage ordering, 2-of-3 and 25-of-38 shared selection, conflicting cheapest alternatives, valid held cycles/pure completion cycles, recorded group differences, exact/class conflicts, target floor/final band and no-alternative/selection-limit diagnostics.
- [x] Run focused tests plus the package fast suite (known Windows newline test explicitly skipped/reported), fmt and self-review; commit source and obtain independent task review.

### Task 2: Required source construction and actual completion

**Files:** Create `crates/fg-route/src/plan/gear.rs` with focused tests; modify `plan/mod.rs`, `construct.rs`, `report.rs`, `evaluate.rs`, `search.rs`, `share.rs`, relevant cleanup/finalization seams, `job/race.rs`, `job/start.rs` and `job/mod.rs` only where needed. Retain the selected action-to-visit witness in `model/feasibility.rs`/`bis.rs` and consume it consistently in actual placement seams when alternate inside/outside locations differ; test helpers remain test-only.

**Interfaces:** Consume Task 1 source plans. Expose a focused `Planner` method to obtain missing accepted source roots from actual state/trace and a `repair_required_gear` route transformation. Exact naming may follow local conventions; record the final public names in the report. Do not mutate `Quest::required` or extend `Simulator::ended` for BiS.

- [x] Write and run functional RED for a valid source costing more than `mandatory_penalty`, including its prerequisite closure; Required must actually complete it, while Prefer may omit it.
- [x] Insert whole selected action closures transactionally using actual initial/progress state and legal pre-target placement. Accept increased cost when source fulfillment improves without losing already satisfied exact/gear goals; reject invalid timers/windows/dungeons/log states.
- [x] Align selected-plan recorded acceptance with start initialization for unbegun log entries and held parents with no new action. Do not require a historical prerequisite again or change the BiS-Off importer policy.
- [x] Adapt dungeon-objective intents to one legal existing/synthesized dungeon run, accepting the relevant quests beforehand; do not emit an invalid direct dungeon Objective stop, rerun a dungeon or drop a forced run. Cover held-only and handed-parent ordering.
- [x] Keep known gear-complete routes over cheaper incomplete candidates in search/tournament/finalization, with cheap/no added simulations when no Required roots exist. Preserve RNG draw schedules and cache/state memory policies.
- [x] Carry compact hard-goal rank through sharing and qualifying/final ranking as well as Search.best. Bypass scalar estimate rejection when a current incomplete Required route needs exact hard-progress evaluation; preserve the original fast path without Required roots. Test two sharing members where the expensive route is complete.
- [x] Extend ordinary and editor-trace validation to accepted specific source roots using actual completions. Fail with named unmet gear sources if a complete route is not found. An unrelated exclusive alternative or a planned post-target turn-in must not count.
- [x] Cover held-parent closure scheduling, N-of-M actual completion, slow source, false intended completion, completed-start source, source-vs-exclusive, hard candidate retention through cleanup/race, edit validation, and unchanged Off/explicit/class/forced-dungeon regressions.
- [x] Run focused and package fast checks once, self-review, commit and obtain independent task review.

### Task 3: Provenance, truthful warnings and compatibility

**Files:** Modify `model/types.rs`, `model/quest.rs`, `model/chains.rs`, `model/derive.rs`, `model/required.rs` and `model/mod.rs` where provenance enters/releases; `model/testing.rs`; `plan/state.rs`, `step.rs`, `report.rs`; `job/mod.rs`; `export/diff.rs`; `app/src/api.ts`, `components/RouteSummary.tsx`, `catalog.ts`, `paramSchema.ts`. Add focused Rust and pure frontend display-selector tests; extend existing Node test script only if required.

**Interfaces:** Model-owned `MandatoryOrigins` named mask; serialized `MandatoryGoalCounts { class_quests:u32, bis_quests:u32, other_quests:u32 }`; `Breakdown.mandatory_goal_counts: Option<MandatoryGoalCounts>` with serde default/skip-None. Preserve deprecated aggregate `missing_class_quests` meaning and all penalty weights. Re-export necessary types for public callers.

- [x] Write functional RED for a missing gear root + prerequisite reported as gear2/class0, and a genuine old aggregate25 displayed as generic unclassified goals rather than class25.
- [x] Seed/propagate/retire actual requirement origins, union new origins through already-mandatory shared ancestors, and count from the actual simulated end with existing exclusive deduplication. Fresh results write Some including zeroes; old results remain None without replay/migration.
- [x] Update per-quest notes and API warnings for class/gear/other/legacy, Required/fallback/vendor help, translations and comparison summaries. Never infer old categories from request mode or free text. Keep plain visible warning text and native calculation-note controls.
- [x] Cover overlapping/deep origins and score invariance, class-category gear-only quests, forced-dungeon/exact generic goals, released class versions, exclusive/initial/post-target state, old/fresh round trips and old/new comparisons.
- [x] Run focused Rust tests, package fast suite, frontend tests/build/lint/i18n. Perform a scoped accessibility review of warning wording/reading order; do not add a UI testing dependency or unnecessary alerts. Commit and obtain independent task review.

### Task 4: Real-case verification, delivery and handoff

**Files:** Ignored fresh diagnostic outputs; a privacy-safe assessment and compact evidence under `docs/`; update `ASTRA_HANDOFF.md`, `CHANGELOG.md` and this plan. Real-profile failures may require a scoped repair in `plan/gear.rs`, its tests and a narrow prefix-evaluation helper; diagnose, capture a functional RED, verify and independently review those changes before final integration.

- [x] Re-run read-only preparation of the saved request under the corrected selector, recording selected roots, support choices, rejected winners and fallback reasons. Verify actual Warrior requirements remain valid and unsupported original chains do not become hard goals.
- [x] Generate a bounded fresh version of the real profile in ignored output using explicit recorded budget overrides. Verify actual turn-ins for all accepted roots and no false class warning; if scheduling fails, diagnose and repair within scope rather than claiming the model check alone fixes the route.
- [x] Verify old guide deserialization/view data preserves steps/time and produces neutral legacy warning behavior. Do not overwrite the user's original guide or publish private data.
- [x] Run release Rust package tests, relevant workspace checks, fmt, release all-target Clippy, frontend tests/build/lint/i18n, and desktop application build. Known newline skip and ignored doc examples must be reported accurately.
- [x] Obtain final whole-branch source/evidence review; address findings, commit assessment/handoff as prijkes, verify clean status and source/artifact identity. Retain raw private diagnostics locally; no app launch/installer/push/merge.

## Completion record

Completed on 2026-11-08. Final source `66d21231ecbf833155f247e4a370904c57c925e8` passes fresh real-profile generation with 71 source roots, 193 support quests, zero class/gear/other misses, and 20 applicable visits matching 20 entry markers at the unchanged 18–60 endpoint. Requested Stockade and Deadmines remain unplanned under the existing model rule.

Final release workspace verification passes 754 tests, including 423 route unit and 238 integration tests. The named Windows CRLF fixture is filtered; two game-client contribution tests and two doc examples remain ignored, and allocator stress is gated to supported Wasi/Unix platforms. Release workspace all-target Clippy, formatting, frontend nine tests/build/lint/i18n and the strict actual saved-guide compatibility check pass.

The production desktop build uses `--features tauri/custom-protocol` to embed the verified frontend. Its source and executable SHA-256 are recorded in the [sanitized validation artifact](../../required-gear-validation-2026-11-07.json). Original app-storage files retain their captured bytes. Final whole-branch review approved delivery after the single Important location/window finding was fixed and scoped re-reviewed; no unresolved blockers remain. All commits use prijkes, local branches/managed worktree and raw private diagnostics are retained, and no app launch, installer, push or merge was performed.
