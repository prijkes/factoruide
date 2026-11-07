# Final whole-branch source and evidence review

Reviewed source `4ae4e9a..10bb20af5c1afda0d43b49da2ede653651a92b18`, the supplied `final-review.diff`, design and implementation plan, both implementation reports and independent task reviews, assessment, handoff, changelog, evidence index and review record. Source remained frozen; review did not mutate product source, run a search, rebuild an app, or change branches.

## Verdict

**Approved; no outstanding Critical, Important or Minor findings.** One Minor documentation precision issue was corrected and independently rechecked during review. The change is ready to merge after the already planned completion of the documentation/review record. This verdict does not authorize an actual merge or push.

## Findings

### Critical

None.

### Important

None.

### Minor — addressed

- `docs/cleanup-training-assessment-2026-11-07.md:25`: the old 4081-second and 15869-second grinds are from `DropDungeonless`, while the new 389.294349-second grind and the totals in the same paragraph are from `Finalized`. The underlying numbers are real, but the unlabelled mixed-boundary comparison is misleading beside the final-route discussion. Use the old final grinds (3991.0565683901646 seconds before quest 126 and 14767.596518009725 seconds before quest 212), or explicitly label both boundaries. This does not affect the reported final scores, player times, total grinding, XP or turn-in benefit.

Root corrected the paragraph to begin “Comparing finalized routes” and use 3991.056568 and 14767.596518 seconds. Re-read the final wording and verified those rounded values against the raw finalized traces. This resolves the finding. Product source and numerical artifacts were unchanged; final source and executable identities were rechecked afterward.

## Source assessment

- `crates/fg-route/src/plan/search.rs:347`: dungeon-quest eligibility and mandatory/prerequisite retention remain the existing rules. Each accepted deletion removes every route stop for that quest. A valid bare-deletion simulation remains required; a valid training substitution must score strictly lower, with ties selecting bare deletion.
- `crates/fg-route/src/plan/search.rs:362`: attribution is reconstructed from the current accepted route on every iteration. Saved states are pre-slot states; `states.len() - 1` bounds executed slots. Replaying a removed slot with restored `at` and ordinary `next_pos` captures ordered actual class-training events, including implicit activity inside the slot. It does not take training from a retained slot or reuse stale attribution after an earlier deletion.
- `crates/fg-route/src/plan/step.rs:410`: standalone training uses existing `train_at`, which owns travel/detour, lesson, rank copper, learned movement abilities, city teleports and power refresh. It bypasses generic stop overhead and `Event::Stop`. A redundant trainer is a valid no-op. Clearing dungeon membership before an actual visit follows the existing departure convention; preserving the landing marker lets the existing travel path account for onward walking once.
- `crates/fg-route/src/plan/availability.rs:211` and `places.rs:137`: executed unknown trainers are rejected; look-ahead safely previews an invalid later trainer. Trainer indices do not enter quest bitsets. The new kind is mapped in place/zone/start-position resolution and remains outside `Stop::is_quest`.
- `crates/fg-route/src/plan/edit.rs:481`, `edit.rs:627`, `crates/fg-route/src/job/edit.rs:192`, and `crates/fg-route/src/export/writer.rs:766`: editor execution, event ownership, expansion, persistence and export use the existing training action. The normal trace produces one lesson/event/action, not a duplicate generic stop or flight-master row. No saved-guide schema changes.
- `crates/fg-route/examples/cleanup_diagnose.rs:390` and `:539`: cleanup replay takes the archived selected route, calls `Effort::Tries(0)`, and uses ordinary finalization. The production search loop tests effort before any move. Assertions require one unchanged initial route callback, seven ordered cleanup boundaries, and zero attempts/improvements. Snapshot descriptions use fresh planners only after cleanup/finalization. Mode exclusion, configuration equality, ordered source schema, typed input equality and stop-identity comparison support the claims emitted by the mode.

The extra work is a full attribution run and at most one additional full candidate simulation per proposed quest deletion. The original saved-state run leaves scope before candidate scoring; there is no new persistent cache or simultaneous retained-run accumulation. This is an expected cleanup cost, not an established timing improvement. Search insertion operators, random draws, defaults, cache policies, memory caps, game data and unfinished-quest cleanup are unchanged. Existing move/finalization paths treat training as an independent visit and use the new location mappings.

## Independent evidence audit

Executed read-only Python assertions against full raw captures and compact JSON; did not merely accept the parent's checker output. Read `check_measurements.py` and confirmed its equality and outcome assertions, then extended the audit to every summary field, every compact field/trace summary, original manifest linkage, current binary and frozen inputs.

- All **19** hashes in the manifest's four raw, twelve source and three compact-artifact entries match actual bytes. Source-artifact hashes also match the earlier diagnosis manifest. Both final raw cleanup captures and both control captures equal their pre-lint counterparts in full JSON.
- HEAD is exactly `10bb20af5c1afda0d43b49da2ede653651a92b18`; source, data, overrides and lockfile have no staged or unstaged drift. Current diagnostic executable SHA256 is `c2f44c17d1481f405229744af2f5ff966151841f57a2b63baa057170135e36c3`, matching the manifest.
- The actual Forever database hash and Cargo.lock hash match the manifest and the original algorithm-comparison protocol. Overrides tree `d9e313e0aa78233771ed1dbd98ef23f7508ecf9a` matches both the frozen source and original protocol revision; recorded override file hashes also match. No data/overrides/lockfile content changed between review base and source head.
- All **16 old fixed-stage controls** match original corrected-accounting captures in stage, exact score, player time, typed stops, breakdown, complete trace, actual turn-ins, abandonments and dungeon entries. Both cases' first six cleanup boundaries match (**12** identities), and all **8 baseline boundaries** match.
- Every non-trace compact stage field equals its raw counterpart. All sixteen compact training/grinding trace summaries were independently reconstructed from complete raw arrays, including event indices and every event field. Top-level compact metadata equals raw output. Every field in both measurement-summary case objects was independently recomputed exactly.
- Configuration, seed and edition equal their old captures. Cleanup flags record zero attempts/improvements, no search or timing claim, no callback comparison, verified configuration/input identities, one initial callback and seven boundaries. The source establishes these flags' execution meaning.
- Chain final score is exactly **222219.2418109174 -> 158149.95837158317** and player seconds **147295.15028327564 -> 135374.44339284918**. Reported reductions and percentages recompute exactly; final score is 1110.491691222036 below old `TrimTarget`.
- Quest 387's sole accept slot becomes trainer index 7 at the same slot. Quest 387 is absent from final stops; Stockade 717 is absent from listed and actual dungeon entries. Final event 117 is trainer 7 at level 22, since 20, outbound 0.0. Final stop count is 917.
- Final turn-ins are **303 -> 319**; recovered IDs are 193, 196, 202, 205, 207, 595, 597, 599, 600, 606, 607, 611, 622, 627, 628 and 97914. The new set equals the old `TrimTarget` set. Delivery 97914 turns in at level 23. Quest XP is **540925 -> 584260**, a **43335** gain; actual abandonments are one -> zero.
- Final grinding is **18842.65308639989 -> 925.9861419571806** seconds, excess grinding **16089.716782810914 -> 0**, lesson time **210 -> 240**, and rank copper **238000 -> 238000**. Raw trace inspection confirmed the disappearance of the quest-126 gate and the level-38 quest-212 grind of **389.2943486346377** seconds; it also identified the minor boundary wording issue above.

## Validation and limits

Per assignment, did not rerun the existing suites. The reviewed reports/root verification record 333 library plus 238 integration tests passing in both fast and release profiles, twelve release diagnostic tests, formatting and release all-target Clippy with warnings denied. The known Windows newline-sensitive export test was explicitly skipped, and two existing doc examples ignored. Reviewed regression bodies substantiate real spell-data benefit, strict ties, retained opportunities, repeated slots and multiple-deletion provenance, mandatory/prerequisite behavior, no-op/invalid trainers, incremental state replay, travel/landing accounting and existing persistence/export. No concrete uncovered source defect warranted another focused test.

Declined to judge as established outcomes:

- General protection against deliberately malicious archive model indices. The standalone local diagnostic already trusts original saved-route indices in its existing rescore path; the new mode reuses that contract and does not expose a network/file-import trust boundary in the product. This is not a material new risk for these validated originals. The review does not claim arbitrary corrupt archives are safely rejected.
- Search-algorithm ranking, route optimality, route-generation timing and the benefit across other classes/editions/routes. These two fixed-route replay cases cannot establish those outcomes, and the documentation explicitly limits its claims.
- Passing the skipped Windows newline test or ignored doc examples; no such claim is warranted.
- Independent reproduction of the historical RED/GREEN test executions or binary build process. Those are supported by task reports and parent verification. This review independently verified current source/binary identities and evidence contents, not a fresh rebuild or rerun of reported suites.
- Running application behavior, installer deployment and existing user guide files. They are outside scope and were not touched by this review.

The final review section and Task 2 completion record were intentionally pending during review. Their planned completion after this verdict is a workflow follow-up, not a source defect.
