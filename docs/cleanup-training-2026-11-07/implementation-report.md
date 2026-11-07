# Task 1 report: preserve useful class training during dungeon cleanup

## Scope and changes

Implemented the approved training-stop and scored-cleanup substitution on top of plan/spec commit `962fec6`.

Source commit: `be5bd00` (`fix: preserve useful training during dungeon cleanup`), authored and committed as `prijkes <prijkes@noreply.com>`. The task report is a local workflow artifact outside the source commit.

- Added `Kind::Train`, indexed by `Model::trainers`, with location/zone resolution, validity, invalidity explanation, and existing `RouteStep::Train` mapping.
- Its simulator branch resets objective grouping, preserves the landing marker for correct onward flight walking, leaves a dungeon before traveling, and reuses `train_at` without generic arrival overhead or `Event::Stop`. Redundant learned training is a valid no-op; unknown trainers are rejected. Look-ahead safely previews an invalid trainer until its slot can be rejected.
- Existing `Edit::Train` now uses the same stop branch, including unknown-index validation and dungeon exit. Editor action ownership recognizes the standalone training event.
- For each dungeon quest selected by existing cleanup rules, build attribution from a fresh run of the current accepted route. Clone only actual executed removed-slot pre-states (`run.states[k]`), reset `at`/`next`, apply that slot with local tracing, and retain class-training events in order. The original run goes out of scope before either candidate simulation.
- Both full-score alternatives omit every stop of the removed quest. Choose training substitution only when valid and strictly cheaper than valid bare deletion; a tie chooses bare deletion. Bare-deletion validity retains prerequisite controls.
- No random draw, parameter default, game-data, cache/memory-policy, general trainer-search, or unfinished-cleanup behavior changed. Running app, saved guides, installer, push and merge were untouched. The diagnostic example is reserved for Task 2 and remains untouched here.

## RED evidence

Before any production edit, ran:

`cargo test --profile fast -p fg-route --lib cleanup_training`

Result: compiled successfully, exit 101; 0 passed, 1 failed. The functional regression `cleanup_training_preserves_useful_accept_training_after_removing_dungeon_quest` first established that actual Classic level-20 mage spell-data training occurred on the optional dungeon accept, bare deletion lost it, and trained downstream grinding was faster. It then failed on required cleanup training: actual event count 0, expected 1, message `cleanup must retain the useful training as its own action` (original test line 58).

An additional unknown-trainer look-ahead regression was run before its fix:

`cargo test --profile fast -p fg-route --lib cleanup_training_stop_is_a_valid_noop`

Result: compiled, exit 101; 0 passed, 1 failed. `[valid Train, unknown Train]` panicked at `places.rs:136` indexing trainer 1 of a one-trainer model. A safe preview in `Places::pos` fixed this while leaving the actual unknown stop invalid.

## GREEN evidence

`cargo test --profile fast -p fg-route --lib cleanup_training`

Final result: exit 0; 12 passed, 0 failed, 322 filtered out. This includes the real-data functional regression; bare/no-trainer cases; retained-slot attribution; exact tie preference; repeated stops/multiple deletions with refreshed provenance; mandatory/prerequisite/dungeon controls; redundant/unknown trainers including look-ahead; adding/removing/reordering training with full-versus-incremental score equality; saved-state suffix replay equality for trained level, hunter Cheetah abilities, mage city-teleport taught bits, real spell-rank copper cost, power and lesson time; one event/export/persisted action and one lesson; and dungeon-exit/flight landing accounting.

`cargo test --profile fast -p fg-route -- --skip export::fgstring::tests::the_addon_reads_them_and_they_read_the_addon`

Result: exit 0; 333 unit tests and 238 integration tests passed, 0 failures. Two existing doctests were ignored. The one known Windows CRLF-sensitive export test was explicitly skipped as authorized; this run does not assert that test passes. No release suite run here; the root agent owns later complete-source release verification.

`cargo fmt --all -- --check`

Result: exit 0 after formatting.

`git diff --check`

Result: exit 0. Git printed normal LF-to-CRLF working-copy notices for touched Rust files.

During fixture iteration, one new prerequisite-control test failed because its `pre_held` fixture supplied index 0 instead of the quest's game ID. Correcting the fixture to the game ID produced the intended valid-original/invalid-deletion control; no product change was made for that test failure.

## Self-review and remaining concerns

Reviewed the complete source diff against the task brief and design. Confirmed `Train` is not a quest, produces no generic stop or flight-master row, persists through the existing schema, and does not acquire duplicate lesson time. Confirmed fresh accepted-route attribution each deletion and explicit executed-slot bounds; the multiple-deletion regression specifically needs refreshed attribution because the second quest originally supplied no lesson, but supplies one after bare deletion of the first.

Confirmed full candidate simulations happen after the original run's scope ends. The new cleanup does extra bounded full simulations per removed quest as designed; no new cache or retained-run memory policy was introduced. No performance/ranking claim is made.

Independent task review is still required and is delegated to the root agent after this source commit. Task 2 replay and final release verification remain outside this report's implementation scope.
