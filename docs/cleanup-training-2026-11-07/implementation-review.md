# Task 1 independent review

Verdict: **Approved**. No Critical, Important, or Minor findings. No source fixes requested.

## Scope and evidence

Reviewed the Task 1 brief, approved design, implementer report, and supplied staged review snapshot based on `962fec6`. The snapshot contains eleven changed Rust files, including the new focused regression module. The initial combined tool response truncated part of the snapshot; only the omitted implementation ranges were subsequently retrieved to complete the same diff review.

This is a review of the supplied source snapshot, not a claim that a committed source tree has already been matched to it. The root agent owns that identity check. New diagnostic replay modes, archive benchmarks, and other Task 2 validation are outside this review.

## Specification compliance

- `plan/search.rs:347`: the existing optional-dungeon-quest eligibility and mandatory-quest exclusion remain intact. Bare deletion still has to simulate successfully, preserving the existing prerequisite retention behavior.
- `plan/search.rs:362`: each deletion reconstructs attribution from the current accepted route. `run.states[k]` is used only for the `states.len() - 1` executed slots, with `at` and `next` reset before local slot replay. Ordered `Event::Train` events are retained only for removed quest slots. The original `Run` and its states leave scope before candidate simulations.
- `plan/search.rs:388`: both alternatives filter every stop of the removed quest. The training alternative is considered only after a valid bare score, and selected only for a valid strictly lower score. Equal scores select bare deletion. Subsequent deletions operate on the newly accepted route.
- `plan/step.rs:410`: the standalone training branch calls existing `train_at`, avoiding generic arrival overhead and `Event::Stop`. It clears dungeon membership before real travel, preserves a flight/transport landing marker for the existing onward-walk accounting, and permits a redundant lesson as a no-op.
- `plan/availability.rs:211`, `plan/evaluate.rs:506`, and `plan/places.rs:137`: trainer bounds are validated, invalid trainer look-ahead is safe, and an actually executed unknown trainer is rejected. The location, zone, usual-position and dungeon-membership mappings cover the added kind.
- `plan/edit.rs:481`, `plan/edit.rs:626`, and `job/edit.rs:192`: existing editor training actions reuse the stop branch and retain the existing persisted `RouteStep::Train` representation. Action ownership recognizes the standalone training event. No new persistence schema is introduced.
- `export/writer.rs:766`: a standalone training event follows the existing trainer export path. A generic training stop event is intentionally unreachable, preventing an erroneous flight-master row or duplicate action.
- The diff does not change random draws, search insertion operators, parameter defaults, game data, cache policies, memory caps, or unfinished-quest cleanup. It adds the designed bounded candidate simulations during dungeon cleanup.

## Code quality and targeted unchanged-code inspection

The implementation is localized and uses existing state, event, scoring and persistence interfaces. The candidate logic is direct, and the comments explain the two non-obvious invariants: slot provenance and releasing the retained run before full candidate scoring. No new abstraction or unrelated refactoring is needed for this change.

Inspected unchanged code only to resolve these concrete integration risks:

- `plan/evaluate.rs:136` and `plan/evaluate.rs:161`: checked saved-state timing, early termination, slot count and validation. This establishes that `states[k]` is the original pre-slot state and that `states.len() - 1` excludes all unexecuted suffix slots.
- `plan/sim.rs:343` and `plan/step.rs:386`: checked that local provenance replay sets the same look-ahead and runs the same apply pipeline as ordinary simulation.
- `plan/step.rs:151`, `plan/step.rs:770`, and `plan/on_the_way.rs:45`: checked actual lesson/copper/ability/teleport/power updates, dungeon-exit convention, detour semantics and consumption of landing markers. The new branch delegates those costs once and bypasses generic stop overhead.
- `plan/edit.rs:115`, `plan/edit.rs:307`, `plan/edit.rs:523`, and `export/writer.rs:107`: checked invalid-stop rejection, explicit action ownership, expansion and trainer-event export. Expansion emits the existing single `Edit::Train` for the lesson; it does not also retain a generic training stop.
- `plan/state.rs:47` and the relevant move/cleanup matches: checked that `Kind::Train` is not classified as a quest and that this addition does not introduce a new random search draw or trainer insertion operator.

## Validation assessment and limits

The focused tests substantiate the intended behavior with real Classic spell data, score improvement versus bare deletion, tie preference, retained-slot attribution, refreshed provenance across multiple removals, repeated quest stops, mandatory/prerequisite/dungeon controls, redundant and unknown trainers, incremental replay, real copper and ability/teleport state, one export/persisted action, and lesson/landing accounting.

Per the review assignment, no test suite was rerun. The implementer report records 12 passing focused tests and 333 unit plus 238 integration tests passing under the fast profile, with the known Windows CRLF-sensitive export test explicitly skipped. It also records passing formatting and whitespace checks. Static inspection left no specific unresolved code concern warranting an additional focused run.

Cannot independently verify from this review alone: reproduction of those reported test executions, the skipped CRLF-sensitive test, release-profile behavior, archived-case replay results, and equality between the supplied snapshot and the eventual source commit. The root agent owns commit identity and later release verification; archived-case replay belongs to Task 2. These limits do not reveal a Task 1 source defect or block this snapshot's approval.
