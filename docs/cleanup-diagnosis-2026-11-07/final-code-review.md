# Final source review: cleanup diagnosis and abandoned work

Decision: **Changes requested for one Important diagnostic reporting defect.** No Critical findings. No material defect found in the production observer or abandoned-work correction. No additional Minor findings.

Reviewed `e73af02..09752b6`, the supplied final-source review package, implementation plan and progress ledger, task reviews, complete changed source/tests, surrounding simulation/settlement/incremental paths, and the saved original-semantics traces. This was read-only source and artifact inspection; no builds, tests, planner executions, source edits, or artifact edits were performed. The coordinator's reported release/fast tests, formatting and Clippy results are supplied evidence, not independently rerun here.

## Important: explicit dungeon runs are omitted from actual-dungeon reporting

`crates/fg-route/examples/cleanup_diagnose.rs:218-225` constructs `actual_dungeon_entries` using only `Event::Enter`. However, `Simulator::arrive` in `plan/step.rs:784` deliberately suppresses that event for an explicit `Kind::Dungeon`; `do_apply` records the executed dungeon as `Event::Stop` instead. Consequently the diagnostic silently omits actual explicit dungeon runs, even in the captured cases this branch diagnoses.

Concrete saved evidence:

- Every chain stage, including `TrimTarget` and `DropDungeonless`, has an executed `Stop { index: 26, kind: Dungeon }` for dungeon area **16732**, Excavation Site: Wetlands. In `SelectedInput` its trace time is **42452.98515070625**, level 26. The reported `actual_dungeon_entries` contains only **1581, 721, 16544, 722**, omitting 16732. Thus the chain has five actual dungeon identities, not the four stated in `trace-analysis.md:25` and repeated in the trace review.
- The baseline trace likewise contains explicit runs for areas **16732** and **491** that its reported list omits. Its explicit stop for **721** follows an implicit entry already recorded, so simply appending every dungeon stop would double-count that dungeon.

Correct the diagnostic extraction to recognize explicit dungeon stops as well as implicit entry events, retaining the first actual occurrence per dungeon (or otherwise avoiding duplicate entries). Add a focused regression covering an explicit-only run and an implicit entry followed by its explicit stop. Correct the derived evidence lists and prose from the existing full traces while preserving the immutable original captures and their hashes; no new search is required. The new extraction should be used by both normal diagnosis and fixed-snapshot rescoring through their shared stage description.

This finding does **not** invalidate the exact callback/score/route replay gates, the isolated deletion of Accept 387, or the observed lost-training cascade. Area 717 is still absent. It does invalidate the completeness of the current actual-dungeon field and the four-dungeon enumeration used as supporting evidence.

## Production review

- The observed entry point shares the existing search/cleanup body. The default observer is a no-op; no additional RNG, simulation, route clone, or I/O is introduced for unobserved callers. Seven callbacks include unchanged boundaries. The intermediate rebind snapshot does not split the prior combined rebind/hearth route comparison or its run update. Abort and invalid-route handling remain intact.
- Legacy work records the actual local gross fighting increment calculated before XP changes, including selected alternatives, imported partial progress and stock use. It is captured only on objective execution, not through imported/shared/dungeon completion in `hold`. Actual direct shared work remains in its existing beneficiary ledger.
- Successful turn-in and abandonment settle both applicable ledgers after the existing completion-error guard. The final direct beneficiary transfers work exactly once to the orphan scalar; surviving beneficiaries retain their work; genuine shared reward still protects productive direct kills.
- The legacy ledger aggregates one row per unsettled quest, uses copy-on-write snapshots, and removes rows on settlement. The orphan scalar stores no history or IDs. Existing group behavior, quest limits and cache caps are unchanged. Initial state and incremental suffix equality include both new fields; focused ownership tests exercise both pending and orphaned differences.
- The retained global overlap cap remains the documented pre-existing approximation, not an exact redesign of arbitrary mixed legacy gathering ownership. No new regression was found within the stated conservation/import/reward-protection contract.
- The profession integration-test correction matches the unchanged producer's multi-rank first-visit behavior. The added regression verifies actual rank bits, one charged lesson and cap-150 initial-learning export rather than merely relaxing the assertion.

## Diagnostic review beyond the finding

The example restores the archived effective request, validates both request and effective parameters for the supported deterministic solo configuration, removes both timed stopping conditions, and gates request/params, attempt/improvement counts, ordered callbacks, final score/player time and typed stop identity separately. Snapshot evaluation occurs after search and finalization, in fresh ordinary planners. Rescore mode performs no search and verifies configuration, ordered boundaries and final decoded stop identities. Existing output files are protected by `create_new`. Exactness/provenance and timing limits remain explicit.

The supplied broad verification and all 16 unchanged fixed-stage score/player/stop/trace rescores support the production changes; they cannot detect the incomplete dungeon event projection identified above. Approval of the whole branch should wait for that bounded diagnostic correction and its focused verification.
