# Upstream contribution series

All 22 topic branches are published as draft pull requests against `brouznouf/factoruide`.

**AI disclosure:** This contribution was created with AI. The submitter reports in-game testing of the combined fork only and has not reviewed the source code. These split branches have not been individually verified in-game. Automated checks below were run by AI; no human source-code review is claimed.

## Independent fixes and features

These eleven PRs each have one focused commit above upstream `1c8e835`. They can be considered separately; some touch common files and may need conflict resolution when several land.

| Topic | Upstream draft PR | Branch comparison | Description |
| --- | --- | --- | --- |
| fix: include web game data when building on Windows | [#133](https://github.com/brouznouf/factoruide/pull/133) | [pr/windows-web-data-paths](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Fwindows-web-data-paths) | [Description](descriptions/windows-web-data-paths.md) |
| fix: defer protected quest tracker layout during combat | [#134](https://github.com/brouznouf/factoruide/pull/134) | [pr/combat-safe-quest-tracker](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Fcombat-safe-quest-tracker) | [Description](descriptions/combat-safe-quest-tracker.md) |
| fix: preserve wide numeric guide metadata when importing | [#135](https://github.com/brouznouf/factoruide/pull/135) | [pr/wide-guide-import-numbers](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Fwide-guide-import-numbers) | [Description](descriptions/wide-guide-import-numbers.md) |
| fix: fit settings dropdowns to their options | [#136](https://github.com/brouznouf/factoruide/pull/136) | [pr/settings-dropdown-layout](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Fsettings-dropdown-layout) | [Description](descriptions/settings-dropdown-layout.md) |
| fix: warn when offered guide quests are left unaccepted | [#137](https://github.com/brouznouf/factoruide/pull/137) | [pr/missed-quest-acceptance-warnings](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Fmissed-quest-acceptance-warnings) | [Description](descriptions/missed-quest-acceptance-warnings.md) |
| fix(data): enforce required quest prerequisite chains | [#138](https://github.com/brouznouf/factoruide/pull/138) | [pr/quest-prerequisite-corrections](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Fquest-prerequisite-corrections) | [Description](descriptions/quest-prerequisite-corrections.md) |
| perf(route): reuse search plans and cached travel work | [#139](https://github.com/brouznouf/factoruide/pull/139) | [pr/reuse-route-travel-work](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Freuse-route-travel-work) | [Description](descriptions/reuse-route-travel-work.md) |
| test(web): match allocator stress test platform support | [#140](https://github.com/brouznouf/factoruide/pull/140) | [pr/windows-allocator-test-guard](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Fwindows-allocator-test-guard) | [Description](descriptions/windows-allocator-test-guard.md) |
| feat(economy): import item prices and project quest gold | [#141](https://github.com/brouznouf/factoruide/pull/141) | [pr/quest-gold-and-item-prices](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Fquest-gold-and-item-prices) | [Description](descriptions/quest-gold-and-item-prices.md) |
| fix(route): finish the current dungeon before another forced run | [#142](https://github.com/brouznouf/factoruide/pull/142) | [pr/finish-current-dungeon-before-next-run](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Ffinish-current-dungeon-before-next-run) | [Description](descriptions/finish-current-dungeon-before-next-run.md) |
| fix(route): use placed containers for outdoor item drop costs | [#143](https://github.com/brouznouf/factoruide/pull/143) | [pr/outdoor-object-drop-rates](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Foutdoor-object-drop-rates) | [Description](descriptions/outdoor-object-drop-rates.md) |

## Dependent feature drafts

These eleven PRs target upstream `main`, so their GitHub Files changed views currently include prerequisite topics. Each PR prominently discloses this and links its prerequisite PRs. The comparisons below isolate each topic against its prepared parent.

Merge prerequisites first, then rebase the single topic commit onto updated upstream `main` before merging its PR. `pr/series-base` is the aggregate preview of the eleven independent topics, rather than an extra upstream PR. The stack is a conservative tested ordering; it does not claim every earlier topic is a technical requirement.

| Order | Topic and focused comparison | Upstream draft PR | Prepared parent | Description |
| ---: | --- | --- | --- | --- |
| 1 | [Plan best quest/vendor gear and merchant purchases](https://github.com/prijkes/factoruide/compare/pr%2Fseries-base...pr%2Fbest-gear-and-merchant-purchases) | [#144](https://github.com/brouznouf/factoruide/pull/144) | `pr/series-base` | [Description](descriptions/best-gear-and-merchant-purchases.md) |
| 2 | [Extract and bundle client item icons](https://github.com/prijkes/factoruide/compare/pr%2Fbest-gear-and-merchant-purchases...pr%2Fclient-item-icons) | [#145](https://github.com/brouznouf/factoruide/pull/145) | `pr/best-gear-and-merchant-purchases` | [Description](descriptions/client-item-icons.md) |
| 3 | [Show equipment slots and item tooltips with rounded DPS](https://github.com/prijkes/factoruide/compare/pr%2Fclient-item-icons...pr%2Fequipment-tab-and-tooltips) | [#146](https://github.com/brouznouf/factoruide/pull/146) | `pr/client-item-icons` | [Description](descriptions/equipment-tab-and-tooltips.md) |
| 4 | [Share accepted quest kill credit without duplicate work](https://github.com/prijkes/factoruide/compare/pr%2Fequipment-tab-and-tooltips...pr%2Fshared-quest-kill-credit) | [#147](https://github.com/brouznouf/factoruide/pull/147) | `pr/equipment-tab-and-tooltips` | [Description](descriptions/shared-quest-kill-credit.md) |
| 5 | [Add opt-in bounded chain and hub search experiments](https://github.com/prijkes/factoruide/compare/pr%2Fshared-quest-kill-credit...pr%2Fbounded-search-experiments) | [#148](https://github.com/brouznouf/factoruide/pull/148) | `pr/shared-quest-kill-credit` | [Description](descriptions/bounded-search-experiments.md) |
| 6 | [Preserve performed quest work and useful training during cleanup](https://github.com/prijkes/factoruide/compare/pr%2Fbounded-search-experiments...pr%2Fcleanup-accounting-and-training) | [#149](https://github.com/brouznouf/factoruide/pull/149) | `pr/bounded-search-experiments` | [Description](descriptions/cleanup-accounting-and-training.md) |
| 7 | [Replay and inspect archived cleanup outcomes](https://github.com/prijkes/factoruide/compare/pr%2Fcleanup-accounting-and-training...pr%2Fcleanup-replay-diagnostics) | [#150](https://github.com/brouznouf/factoruide/pull/150) | `pr/cleanup-accounting-and-training` | [Description](descriptions/cleanup-replay-diagnostics.md) |
| 8 | [Select supported gear sources and enforce actual completion](https://github.com/prijkes/factoruide/compare/pr%2Fcleanup-replay-diagnostics...pr%2Frequired-gear-source-completion) | [#151](https://github.com/brouznouf/factoruide/pull/151) | `pr/cleanup-replay-diagnostics` | [Description](descriptions/required-gear-source-completion.md) |
| 9 | [Prioritize required reputation work and return best-found deficits](https://github.com/prijkes/factoruide/compare/pr%2Frequired-gear-source-completion...pr%2Frequired-reputation-targets) | [#152](https://github.com/brouznouf/factoruide/pull/152) | `pr/required-gear-source-completion` | [Description](descriptions/required-reputation-targets.md) |
| 10 | [Support counted repeatables, farming, spillover and daily reputation](https://github.com/prijkes/factoruide/compare/pr%2Frequired-reputation-targets...pr%2Freputation-programmes) | [#153](https://github.com/brouznouf/factoruide/pull/153) | `pr/required-reputation-targets` | [Description](descriptions/reputation-programmes.md) |
| 11 | [Finish useful reputation objectives before leaving forced dungeons](https://github.com/prijkes/factoruide/compare/pr%2Freputation-programmes...pr%2Freputation-dungeon-quest-order) | [#154](https://github.com/brouznouf/factoruide/pull/154) | `pr/reputation-programmes` | [Description](descriptions/reputation-dungeon-quest-order.md) |

## Additional contributions

These later contributions are separate from the original 22-topic split. Standalone changes are based directly on upstream; dependent changes name their prerequisite topic and provide a focused comparison.

| Fix | Upstream draft PR | Validation | Description |
| --- | --- | --- | --- |
| Respect class access at loot-selling shops (Kayla Smithe) | [#155](https://github.com/brouznouf/factoruide/pull/155) | Standalone; 499 planner tests passed; two ignored doctests and one known Windows fixture excluded | [Description](descriptions/class-gated-loot-vendors.md) |
| Automatically select a guide matching the logged-in character | [#157](https://github.com/brouznouf/factoruide/pull/157) | Standalone on `f812eb4`; both addon smoke suites, changed-file StyLua and whitespace pass | [Description](descriptions/automatic-profile-guide.md) |
| Preserve generated reputation guides when editing | [#158](https://github.com/brouznouf/factoruide/pull/158) | Depends on [#154](https://github.com/brouznouf/factoruide/pull/154); [focused diff](https://github.com/prijkes/factoruide/compare/pr%2Freputation-dungeon-quest-order...pr%2Ffresh-guide-edit-replay); 804 planner tests plus strict draft regression, Clippy and formatting pass | [Description](descriptions/fresh-guide-edit-replay.md) |

[Additional PR metadata](additional-pull-requests.json) records exact bases, heads, scope and dependencies. The same AI disclosure applies; these new changes have not yet been confirmed in-game.

## Automated validation and source coverage

[AI-run composition validation](validation.md) records the automated tests, platform limitations and excluded checks. The submitter's reported in-game testing applies to the combined fork, not each split PR. No human source-code review is claimed.

[Source coverage](source-coverage.md) maps surviving fork changes and explains excluded duplicates and historical artifacts. [Database verification](data-verification.md) describes the monetary overlay while retaining all current upstream nonmonetary data and 27 new Forever quests.

[The manifest](manifest.json) contains exact heads, parents, changed paths and validation. [The upstream PR list](upstream-pull-requests.json) records all 22 draft URLs and head revisions.

The original 22-topic split was frozen from fork `main` at `37b3f5f`; later contributions are listed separately. PR publication does not update the installed application, addons or saved guides. Addon release versions remain maintainer-owned and are unchanged in the submission patches.
