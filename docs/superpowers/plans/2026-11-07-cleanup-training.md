# Cleanup Training Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox syntax for tracking.

**Goal:** Remove unusable dungeon quests without discarding a beneficial class-training opportunity.

**Architecture:** Add a class-training route stop using the existing simulation and external training actions. Dungeon cleanup compares bare deletion with a fully scored substitution of training actually performed during removed slots. Validate against the saved selected routes without searching.

**Tech Stack:** Rust, existing Scenario fixtures and serde JSON diagnostics, PowerShell.

**Spec:** [Cleanup training design](../specs/2026-11-07-cleanup-training-design.md).

## Global Constraints

- Preserve mandatory/prerequisite retention, random search draws, parameter defaults, game data, cache policies and memory caps.
- Both cleanup candidates omit every stop for the unusable quest; choose training only for a strictly lower valid score, with bare deletion winning ties.
- Reuse existing training travel, time, copper, power, abilities and teleport semantics; emit exactly one training action and no bogus flight-master action.
- Keep original diagnostic captures immutable and write fresh outputs only.
- Leave running app, saved guides and installer untouched. No push or merge. Commit as `prijkes <prijkes@noreply.com>`.

## Review Focus

- No learning debt or a retained nearby opportunity must not add needless training.
- Training associated with implicit quest actions must be attributed to the actual route slot.
- Removing multiple quests must not reuse stale states or training provenance.
- Train stops must replay/export/save correctly and handle redundant or invalid trainers.
- Zero-attempt cleanup must not be presented as regenerated search or algorithm ranking.

### Task 1: Training stop and scored dungeon-cleanup substitution

**Files:** Modify `crates/fg-route/src/plan/{state,places,availability,step,search,evaluate,edit}.rs`, `crates/fg-route/src/job/edit.rs`, exhaustive matches in `crates/fg-route/src/export/writer.rs`, and diagnostic parser if required for compilation. Create `crates/fg-route/src/plan/tests/cleanup_training.rs`; register in `plan/tests.rs`. Add narrowly scoped job/export tests where existing fixtures make these clearer.

**Interfaces:** Produce `Kind::Train` with trainer model index. Keep `drop_dungeonless(&self, route: &[Stop]) -> Option<Vec<Stop>>` private to the crate. Existing `RouteStep::Train`/`Edit::Train` are the persistence interfaces.

- [ ] Write a compiling functional regression using real class spell data: establish accept-triggered training, plain deletion loss, then require cleanup to remove the quest and retain useful training with a lower full score than plain deletion.
- [ ] Run `cargo test --profile fast -p fg-route --lib cleanup_training` and record the expected functional failure before implementation.
- [ ] Implement the training stop and substitution described in the spec. For slot attribution, use `run.states[k]` only for executed slots, reset `at` and `next`, apply the original slot with local tracing, and collect `Event::Train` in order. Release any original run before candidate evaluations where possible.
- [ ] Cover no-benefit/no-trainer, retained opportunity, multiple deletion/provenance, dungeon/prerequisite controls, no-op/unknown trainer, incremental equality, and single training export/persistence with no duplicate time charge. Run focused tests, then package tests; record the known Windows newline test separately.
- [ ] Self-review, record evidence in the task report, and commit source as prijkes. Obtain independent task review before completion.

### Task 2: Fixed-route cleanup replay and measured evidence

**Files:** Modify `crates/fg-route/examples/cleanup_diagnose.rs`. Create a report and compact evidence under `docs/cleanup-training-2026-11-07/` and `docs/cleanup-training-assessment-2026-11-07.md`; update `ASTRA_HANDOFF.md`, `CHANGELOG.md` and this plan.

**Interfaces:** Consume Task 1's `Kind::Train`. Add `--cleanup-snapshots PATH`, mutually exclusive with `--rescore-snapshots` and `--tries`, loading original SelectedInput and running `improve_while_observed(..., Effort::Tries(0), ...)` plus `job::finalize_route`. Report `search_performed=false`, `search_attempts=0`, and `callback_comparison_performed=false`.

- [ ] Add and run failing helper tests for mode exclusivity, SelectedInput identity and zero-attempt execution; implement cleanup-only mode with fresh output protection and exact effective configuration checks.
- [ ] Build the release example, run both saved cleanup cases and all sixteen original fixed-stage rescores into fresh ignored artifacts. Verify unchanged fixed-route controls and archived stage identity through TrimTarget.
- [ ] Record chain quest387 removal, absent Stockade717, actual training and downstream score/player/grinding/turn-in outcomes. Use baseline as no-mutation control. Preserve hashes and source provenance in compact durable evidence.
- [ ] Run `cargo fmt --all -- --check`, release package tests and release all-target Clippy with `-D warnings`. Obtain an independent final review, address findings, update handoff and commit documentation as prijkes.

## Completion record

Pending implementation and measurement.
