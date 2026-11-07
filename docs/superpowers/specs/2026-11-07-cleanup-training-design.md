# Preserve useful training during dungeon cleanup

## Intent and scope

Continue the authorized routing investigation by correcting the reproduced training loss in [the cleanup diagnosis](../../cleanup-loss-diagnosis-2026-11-07.md). Removing an optional dungeon quest must still remove that quest, while allowing its useful class-training service to survive independently. The user's continuation authorizes this focused fix and its validation; implementation details below are engineering decisions.

## Approaches

- Keep the quest when deletion is costly: violates the guide's dungeon consistency requirement.
- Introduce general service scheduling or new search operators: much broader than the demonstrated cleanup defect.
- Represent a class trainer as a route stop and compare a training substitution with bare deletion: selected because existing training simulation, export and editor persistence already support the action.

## Design

Add `Kind::Train`, with `Stop::index` identifying `Model::trainers`. Its execution reuses `train_at`, including real travel, lesson time, spell costs, abilities, teleports and refreshed power. It emits the existing `Event::Train` only, without a duplicate `Event::Stop` or generic quest-stop overhead. A redundant visit is a valid no-op; an unknown trainer is invalid. Resolve dungeon exit consistently when traveling to the trainer. Existing external `RouteStep::Train` and `Edit::Train` represent persisted/editor actions; no new saved-guide schema is needed.

For each optional dungeon quest already selected for deletion, retain the bare-deletion candidate. Attribute actual class training to each removed route slot using its original pre-stop state and the same `at`/`next` setup as ordinary simulation. Replace those slots with the ordered trainer actions to form a second candidate. Compare full valid simulations, choosing trainer substitution only when its score is strictly lower than bare deletion. A tie chooses bare deletion. Both candidates omit every stop for the unusable quest. Rebuild attribution from the current accepted route for each subsequent deletion.

The substitution is a scored alternative, not an equivalence claim: losing quest XP, timing or travel can change training value. The rule remains mandatory cleanup when only bare deletion is feasible. Preserve existing mandatory/prerequisite retention behavior, random search draws, parameter defaults, game data, cache policies and memory caps. This change does not add trainer insertion to search or change unfinished-quest cleanup.

## Validation

First demonstrate a real spell-data fixture where deleting an optional dungeon accept loses training and worsens combat. Then require quest removal, a real independent training action, and a lower score than bare deletion. Cover no-benefit/no-trainer cases, another retained training opportunity, dungeon and prerequisite consistency, no-op/unknown trainer execution, incremental replay, and one training event/export/persisted action with one lesson charge.

Extend the diagnostic example with a cleanup-only replay of archived `SelectedInput` using `Effort::Tries(0)`, no construction or search attempts, and ordinary finalization. Check input/configuration identity, one initial best callback, seven boundaries and zero attempts/improvements. Compare stages through `TrimTarget` with the archive before attributing later differences. Run both saved cases and rescore all sixteen unchanged old stage routes as controls. Report measured score, player time, training and downstream grinding/turn-ins; make no algorithm-ranking or route-generation-speed claim.

The running desktop app, saved guides and installer remain untouched. No push or merge is part of this work.
