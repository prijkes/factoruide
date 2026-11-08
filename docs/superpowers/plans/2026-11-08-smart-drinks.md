# Smart Drinks Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Include useful reputation dungeon chains and verified cave collection.

**Architecture:** Extend existing dungeon preparation to unmet reputation work;
let existing costed objective modeling use verified outdoor item sources.

**Tech Stack:** Rust, existing Scenario/SQLite tests, Tauri desktop planner.

**Spec:** docs/superpowers/specs/2026-11-08-smart-drinks-design.md

## Global Constraints

- Preserve exclusions, prerequisites, standing needs, level/log bounds and elite eligibility.
- No invented/free acquisition, hardcoded quest IDs or new instance revisits.
- Do not alter rewards, farming estimates, daily rules, addon format or saved guides.
- Test first; inspect RED before implementation; independent review before delivery.

## Review Focus

- Satisfied/unselected reputation must not hold a dungeon for optional chains.
- An impossible or blocked prerequisite must not starve other useful route work.
- Mixed outside/inside item sources must retain real acquisition cost and count.
- Instance-only sources must remain instance objectives.
- Initial-log prerequisites, export keep flags and actual request completion must agree.

### Task 1: Reputation prerequisite preparation

**Files:** crates/fg-route/src/plan/construct.rs and focused plan tests.

**Interface:** Consume Availability::missing_reputation and existing dependency
traversal; preserve Planner's existing construction API.

- [x] Add a regression: a selected, unmet reputation dungeon quest with an
  unfinished prerequisite is accepted before its single dungeon run and turned in.
  Include an already-held prerequisite and controls for met/unselected goals.
- [x] Run focused tests and confirm the ordering regression fails on the baseline.
- [x] Extend preparation/waiting consistently for useful reputation quests and
  feasible prerequisite steps. Preserve mandatory behavior and other gates.
- [x] Add boundary XP and mixed-root controls, and a two-forced-run regression
  matching the actual request. Finish current-instance objectives first.
- [x] Run focused plan tests; report exact RED/GREEN evidence and changed files.

### Task 2: Costed outside item sources

**Files:** model/objectives.rs, item.rs, mod.rs, derive.rs and inline tests.

**Interface:** Produce ordinary costed Objective values for usable outside item
paths; preserve the existing Objective and dungeon representations.

- [x] Add regressions with dungeon-category mixed drops: an enabled/forced run
  retains its group path; disabled/excluded runs can use a concrete outside path
  with real count, kill/drop work and no instance prerequisite. Add instance-only
  controls and eligibility coverage. Preserve outside-only quarry handling.
- [x] Run focused tests and confirm the reported failure before implementation.
- [x] Let verified usable outdoor paths reach the existing acquisition machinery
  when the instance is unavailable by settings, retaining enabled group work,
  instance-only and unrelated behavior. Do not infer dungeon bounds from quest level.
- [x] Restore implicitly required runs using a fresh second build/settlement;
  test the distinct required-instance and lone-required-mixed cases.
- [x] Filter container drop rates to actual outdoor providers; test six items
  at a 10% outdoor rate against a 100% instance-only source (60 uses).
- [x] Run focused model tests; report RED/GREEN evidence and changed files.

### Task 3: Integration, review and delivery

**Files:** Verification notes in docs; production files only for reviewed fixes.

- [x] Independently review each task, fix material findings, then review the branch.
- [x] Run full Rust release workspace tests, Clippy and formatting. Report the
  pre-existing Windows fixture failure separately if it occurs.
- [x] Build release CLI and scratch-generate the saved request; verify Smart
  Drinks prerequisites, actual collection/turn-in, reward and keep flags.
- [ ] Build the desktop app with tauri/custom-protocol, identify the delivered
  binary, then integrate under the user's existing main/push authorization.
- [x] State that existing guides need regeneration; addon-only refresh is not
  sufficient for a planner fix. Preserve the installed Guide 0.16.2 import fix.
