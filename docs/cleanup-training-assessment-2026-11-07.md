# Cleanup training assessment: 2026-11-07

## Change and scope

This continuation addresses the training loss identified by the [cleanup diagnosis](cleanup-loss-diagnosis-2026-11-07.md). Dungeon cleanup still removes unusable optional quests. It can now replace training performed during a removed route slot with an independent class-trainer action, selecting that alternative only when a full valid simulation scores it strictly better than plain deletion.

The internal `Kind::Train` uses the existing training simulator and exports the existing training instruction. It accounts for travel, lesson time, spell copper, learned abilities, teleports and combat power. It emits no duplicate quest-stop event or flight-master instruction. Saved guides continue to use the existing `RouteStep::Train` representation.

Training is attributed to the removed route slot from its actual pre-stop state, including actions performed implicitly while there. Each subsequent deletion uses the current accepted route. A tie chooses plain deletion. Mandatory and prerequisite rules, search random draws, defaults, game data, cache policies and memory caps remain unchanged.

## Measured fixed-route validation

Both cases use the archived selected route as input, with zero search attempts. Exact configuration and selected-input identities, one initial best callback and seven ordered cleanup boundaries passed. All sixteen unchanged old stage routes retained identical full traces, typed stops, scores, player times, breakdowns, actual turn-ins and dungeon observations. In both cases the six boundaries through `TrimTarget` also match their old captures exactly; the change starts at `DropDungeonless`.

| Case and boundary | Previous score | New score | Previous player seconds | New player seconds |
| --- | --- | --- | --- | --- |
| Chain: `DropDungeonless` | 227141.555349 | 158601.878630 | 148245.278116 | 135840.958637 |
| Chain: `Finalized` | 222219.241811 | 158149.958372 | 147295.150283 | 135374.443393 |
| Baseline: `Finalized` | 169640.224341 | 169640.224341 | 143100.837689 | 143100.837689 |

The chain's final simulated player time decreases by **11920.706890 seconds**, about **3 hours 19 minutes**. Weighted score decreases by **64069.283439**, or **28.831564%**. These are distinct measurements: the larger score benefit includes reduced grinding penalties. All eight baseline boundaries retain identical routes, scores and complete traces.

The chain replaces quest 387's accept with trainer 7 at the same route slot. The quest is absent and Stockade 717 is still absent. At final trace event 117 the Mage trains at level 22, since level 20, with zero outbound detour. This is a real trainer visit with different travel from visiting the former quest giver, rather than a replay-equivalent quest accept. The route has 917 stops after deletion/substitution and finalization.

The later gate cascade is removed. Comparing finalized routes, the former 3991.056568-second grind before quest 126 disappears. The grind before quest 212 is approximately 389.294349 seconds to level 38, rather than the old 14767.596518-second grind to level 39. Total grinding decreases from 18842.653086 to 925.986142 seconds; excess grinding decreases from 16089.716783 to zero. Training time increases from 210 to 240 seconds and spell copper remains 238000: the measured improvement includes the actual service costs.

The final route executes 319 turn-ins instead of 303. It recovers the fifteen previously displaced tail rewards and the delivery quest 97914, which now turns in at level 23. Quest XP rises from 540925 to 584260, recovering the previously reconciled **43335 XP** loss; abandoned quests fall from one to zero. The final score is also 1110.491691 lower than the old `TrimTarget` score. That comparison includes both the corrected dungeon cleanup and later finalization; it is not attributed solely to training.

## Correctness and review

The functional regression uses real Classic Mage spell data and compiled before implementation, failing because cleanup removed training. Twelve focused regressions now cover real combat benefit, no-service and retained-opportunity controls, strict ties, repeated slots and refreshed provenance across deletions, mandatory/prerequisite/dungeon behavior, invalid and redundant trainers, incremental replay and state, real spell copper, movement abilities, learned teleports, landing accounting, and one training event/export/persisted action with one lesson charge.

The independent implementation review approved the training stop and cleanup policy with no findings. Fast and release package checks each passed 333 library and 238 integration tests. The known Windows newline-sensitive export test remains explicitly skipped, and two existing doc examples are ignored. Diagnostic example tests pass twelve cases, including functional failure before mode implementation, zero search counts, selected-input identity, mode exclusivity, source-contract rejection and training decoding.

Formatting and release all-target Clippy with warnings denied pass. The independent diagnostic-code review approved the zero-attempt execution and identity safeguards with no findings. A borrowed-slice lint correction was rebuilt and all four measurements rerun into fresh files; the complete JSON outputs equal their earlier captures exactly. The [review record](cleanup-training-2026-11-07/reviews.md) preserves verification and review scope.

Production source is `be5bd00`; diagnostic source is `4fe8de4` plus `10bb20a`. The [provenance manifest](cleanup-training-2026-11-07/provenance.json) identifies the final source, binary, database, lockfile, overrides tree, raw captures and compact evidence hashes.

The [final source and evidence review](cleanup-training-2026-11-07/final-review.md) approved the complete change with no outstanding findings. It independently checked every compact/summary field, all stage identities, source/binary identities and nineteen raw/source/archive hashes. One documentation ambiguity was corrected to compare grinding at the same finalized boundary; no source or measurement correction was needed.

## Reproduction and limits

The new diagnostic mode starts from the archived `SelectedInput` stop vector and runs ordinary cleanup with zero search attempts, followed by production finalization. It does not construct a route or compare a regenerated search callback sequence. The inputs remain the two Forever Gnome Mage 20–40 cases, seed `20261107`, with their archived effective requests and parameters.

This validates a concrete cleanup correction. It does not rerank search algorithms under the corrected model, establish optimal routes, measure route-generation speed, or promote the experimental chain operator. Original captures remain immutable. The running desktop app, saved guides and installer are untouched; no push or merge is included.

See the [evidence index](cleanup-training-2026-11-07/README.md) and [implementation plan](superpowers/plans/2026-11-07-cleanup-training.md).
