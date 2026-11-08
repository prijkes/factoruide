# Smart Drinks route correction — 8 November 2026

Smart Drinks (1491) rewards 100 Ratchet reputation in the supported Forever data.
The reported guide omitted it despite the selected Ratchet target. Raptor Horns
(865), its prerequisite, was scheduled after Wailing Caverns.

The corrected planner prepares useful reputation quest chains before their
dungeon run and finishes executable work in the current instance before another
forced run. A real-request construction trace exposed the latter defect: the old
ordering accepted Smart Drinks, entered Wailing Caverns, then immediately prepared
another forced instance without collecting the essence.

Last-level preparation checks the actual next action, including travel and
incidental XP, against the dungeon level ceiling. Unrelated mandatory-only chains
retain their existing last-level release behavior.

## Cave sources and required runs

Mixed inside/outside item sources retain their instance path when the run is
enabled, explicitly forced, part of a dungeon guide, or required by another
quest. Implicit required runs are resolved from settled requirements, followed
by a fresh reconstruction before reward, loot and index loading.

When an instance is disabled or excluded, a verified outdoor path can use the
ordinary costed acquisition model. Existing elite/group restrictions and drop
counts apply. Outdoor containers use only physically available providers' drop
rates: six items at 10% require 60 uses even if an instance-only source has a
100% rate. Existing outside-only quarry/group handling is preserved.

General dynamic inside/outside alternatives after a run and outleveled-instance
fallback remain outside this fix. Reputation rewards, farming wait formulas,
daily rules and the one-visit instance model are unchanged.

## Actual saved-request verification

A scratch replay used the reported level-22 Tauren Warrior request and its
selected reputation targets, required gear and forced dungeons. Only optimization
effort changed from 3 to 0.01 to bound diagnostic work; this is not a performance
comparison or a proof of maximum attainable reputation.

The final production CLI exported 3,258 steps and 927 quests. Verified order:

1. Finish Raptor Horns at level 22 (step 9).
2. Accept Smart Drinks at level 22 (step 10).
3. Enter Wailing Caverns (step 22).
4. Collect Wailing Essence (step 28), before the next dungeon (step 34).
5. Turn in Smart Drinks at level 25 (step 136).

Every exported step of both quests has its keep flag set. The replay reports zero
missing class quests and zero missing dungeon runs. Original request, outcome and
metadata files retain their SHA-256 values; the diagnostic guide was not installed
over the user's guide.

## Validation and delivery

The new regressions first reproduced the failures, then passed: 11 reputation
dungeon tests and 10 mixed-source tests. Independent review approved the final
source with no open material findings. Workspace Clippy with warnings denied,
Rust formatting, frontend production build, 15 frontend tests, lint and 1,227
translation checks passed. The existing frontend bundle-size notice remains.

The final release workspace run passed 881 tests with zero failures; four
existing tests were ignored and one known platform fixture was filtered.
An unfiltered run reproduced the previously documented Windows CRLF/LF fixture
failure in `export::fgstring::tests::the_addon_reads_them_and_they_read_the_addon`;
the final run excludes only that fixture. Its source is unchanged.

The production desktop was rebuilt with `tauri/custom-protocol` at
`target/release/factoruide-app.exe`, 160,197,120 bytes, SHA-256
`4e7a4506026d5d54e773586ac9c50a91e1c6d098b8c0a65fe9ca59606ac5f835`.
The CLI SHA-256 is
`b8e67f12f7402c1efe3e091d571c5920d871feb50fe9d69a5ed7054271cb11e9`.

This is a planner correction. The installed Guide 0.16.2 addon and guide format 3
remain compatible and need no file changes. Reopen Factoruide, regenerate the
guide, and import the new text into WoW. Existing guide text keeps its old steps.

The verified source fix is commit `b77706f`, integrated into `main` by a
fast-forward merge. The compiled code matches the independently reviewed source.
