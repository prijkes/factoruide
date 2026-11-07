# Profession trainer integration assertion diagnosis

## Observed failure and diagnostic evidence

The coordinator's broad fast-profile verification reported 320 passing library tests and 237 passing integration tests, with `routes::profession_trainer_stops_are_off_unless_chosen` failing at the requirement for an exported Leatherworking (`line=165`) step with `skill=75`.

Temporary diagnostic output was added only to that test, without changing its assertions or production behavior. The coordinator reran the test and reported an actual first Leatherworking step at character level 17 with cap 150 and text `Learn Leatherworking from Mak`; a later step at level 24 had cap 225. Training was present. The original assertion equated learning the profession with a separately exported apprentice-only visit.

The temporary prints were removed after diagnosis. No training, export, scoring, or accounting policy was changed to address this test failure.

## Existing contract

`plan/profession_trainers.rs` intentionally computes all currently open ranks that a trainer can teach and records their bits in one `Event::ProfTrain`. `train_profession_at` charges one visit/lesson and applies all those bits. `export/writer.rs::profession_train` exports the highest learned rank as the step's `skill`, and uses the initial `profession.learn` instruction whenever the apprentice bit is present. This same multiple-rank behavior and export rule are present in source at base `e73af02`.

Consequently, a valid first visit teaching apprentice and journeyman together has rank bits `0b11`, an explicit initial-learning instruction, and exported cap 150. Requiring a separate cap-75 event was an unsupported route-shape assumption. Source comparison establishes the contract predates this accounting patch; no baseline execution was performed or claimed by this worker.

## Test correction

The integration test now finds each profession's first exported trainer step and requires `Learn <profession name> from ...`. All original checks for the disabled option, additional travel time, valid cap values, target/map/world coordinates, profession membership, and strictly increasing caps remain. The correction verifies initial learning explicitly while accepting the intentional combined-rank event.

A focused regression was added to `plan/tests/profession_trainers.rs`. A real quest acceptance gate brings a new character to level 15 before its first tailoring trainer visit. The test asserts no imported ranks, actual learned bits `0b11`, one lesson's training time, one exported profession step with line 197 and cap 150, the initial-learning label, and trainer target/location. It would fail if combined learning stopped, were charged twice, exported only cap 75, or lost the initial-learning instruction.

## Verification boundary

The coordinator owns Cargo and planner execution. This worker ran only test-file formatting and scoped `git diff --check`, both successful. The corrected integration assertion and new regression are ready for the coordinator's focused rerun and subsequent broader checks. No production source was modified for this diagnosis.
