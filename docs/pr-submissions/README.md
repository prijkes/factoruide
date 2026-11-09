# Prepared upstream submissions

The fork's changes are separated into 22 topic branches based on upstream `1c8e835`. Each topic contributes one focused commit. This index and its descriptions are preparation material; no upstream pull requests have been opened.

## Independent fixes and features

These 11 branches each have the exact upstream snapshot as their sole parent. They can be proposed separately. Some touch the same files, so apply the usual conflict resolution when merging several.

| Topic | Branch comparison | PR description |
| --- | --- | --- |
| fix: include web game data when building on Windows | [pr/windows-web-data-paths](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Fwindows-web-data-paths) | [Description](descriptions/windows-web-data-paths.md) |
| fix: defer protected quest tracker layout during combat | [pr/combat-safe-quest-tracker](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Fcombat-safe-quest-tracker) | [Description](descriptions/combat-safe-quest-tracker.md) |
| fix: preserve wide numeric guide metadata when importing | [pr/wide-guide-import-numbers](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Fwide-guide-import-numbers) | [Description](descriptions/wide-guide-import-numbers.md) |
| fix: fit settings dropdowns to their options | [pr/settings-dropdown-layout](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Fsettings-dropdown-layout) | [Description](descriptions/settings-dropdown-layout.md) |
| fix: warn when offered guide quests are left unaccepted | [pr/missed-quest-acceptance-warnings](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Fmissed-quest-acceptance-warnings) | [Description](descriptions/missed-quest-acceptance-warnings.md) |
| fix(data): enforce required quest prerequisite chains | [pr/quest-prerequisite-corrections](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Fquest-prerequisite-corrections) | [Description](descriptions/quest-prerequisite-corrections.md) |
| perf(route): reuse search plans and cached travel work | [pr/reuse-route-travel-work](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Freuse-route-travel-work) | [Description](descriptions/reuse-route-travel-work.md) |
| test(web): match allocator stress test platform support | [pr/windows-allocator-test-guard](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Fwindows-allocator-test-guard) | [Description](descriptions/windows-allocator-test-guard.md) |
| feat(economy): import item prices and project quest gold | [pr/quest-gold-and-item-prices](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Fquest-gold-and-item-prices) | [Description](descriptions/quest-gold-and-item-prices.md) |
| fix(route): finish the current dungeon before another forced run | [pr/finish-current-dungeon-before-next-run](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Ffinish-current-dungeon-before-next-run) | [Description](descriptions/finish-current-dungeon-before-next-run.md) |
| fix(route): use placed containers for outdoor item drop costs | [pr/outdoor-object-drop-rates](https://github.com/prijkes/factoruide/compare/1c8e8352cdf7c1cac35e59efbc04e6d81b0d00e5...pr%2Foutdoor-object-drop-rates) | [Description](descriptions/outdoor-object-drop-rates.md) |

## Dependent feature series

These 11 branches form a conservative review stack. `pr/series-base` combines the independent topics; each following branch adds only its own feature to the preceding branch. The comparison links below use the declared parent to isolate each topic.

Before opening a dependent topic against upstream `main`, merge its prerequisites upstream and rebase its single topic commit onto the updated upstream branch. Opening these directly against current upstream would include earlier topics. The preview stack preserves a tested composition; it does not assert that every earlier topic is a technical requirement.

| Order | Topic and comparison | Parent | PR description |
| ---: | --- | --- | --- |
| 1 | [Plan best quest/vendor gear and merchant purchases](https://github.com/prijkes/factoruide/compare/pr%2Fseries-base...pr%2Fbest-gear-and-merchant-purchases) | `pr/series-base` | [Description](descriptions/best-gear-and-merchant-purchases.md) |
| 2 | [Extract and bundle client item icons](https://github.com/prijkes/factoruide/compare/pr%2Fbest-gear-and-merchant-purchases...pr%2Fclient-item-icons) | `pr/best-gear-and-merchant-purchases` | [Description](descriptions/client-item-icons.md) |
| 3 | [Show equipment slots and item tooltips with rounded DPS](https://github.com/prijkes/factoruide/compare/pr%2Fclient-item-icons...pr%2Fequipment-tab-and-tooltips) | `pr/client-item-icons` | [Description](descriptions/equipment-tab-and-tooltips.md) |
| 4 | [Share accepted quest kill credit without duplicate work](https://github.com/prijkes/factoruide/compare/pr%2Fequipment-tab-and-tooltips...pr%2Fshared-quest-kill-credit) | `pr/equipment-tab-and-tooltips` | [Description](descriptions/shared-quest-kill-credit.md) |
| 5 | [Add opt-in bounded chain and hub search experiments](https://github.com/prijkes/factoruide/compare/pr%2Fshared-quest-kill-credit...pr%2Fbounded-search-experiments) | `pr/shared-quest-kill-credit` | [Description](descriptions/bounded-search-experiments.md) |
| 6 | [Preserve performed quest work and useful training during cleanup](https://github.com/prijkes/factoruide/compare/pr%2Fbounded-search-experiments...pr%2Fcleanup-accounting-and-training) | `pr/bounded-search-experiments` | [Description](descriptions/cleanup-accounting-and-training.md) |
| 7 | [Replay and inspect archived cleanup outcomes](https://github.com/prijkes/factoruide/compare/pr%2Fcleanup-accounting-and-training...pr%2Fcleanup-replay-diagnostics) | `pr/cleanup-accounting-and-training` | [Description](descriptions/cleanup-replay-diagnostics.md) |
| 8 | [Select supported gear sources and enforce actual completion](https://github.com/prijkes/factoruide/compare/pr%2Fcleanup-replay-diagnostics...pr%2Frequired-gear-source-completion) | `pr/cleanup-replay-diagnostics` | [Description](descriptions/required-gear-source-completion.md) |
| 9 | [Prioritize required reputation work and return best-found deficits](https://github.com/prijkes/factoruide/compare/pr%2Frequired-gear-source-completion...pr%2Frequired-reputation-targets) | `pr/required-gear-source-completion` | [Description](descriptions/required-reputation-targets.md) |
| 10 | [Support counted repeatables, farming, spillover and daily reputation](https://github.com/prijkes/factoruide/compare/pr%2Frequired-reputation-targets...pr%2Freputation-programmes) | `pr/required-reputation-targets` | [Description](descriptions/reputation-programmes.md) |
| 11 | [Finish useful reputation objectives before leaving forced dungeons](https://github.com/prijkes/factoruide/compare/pr%2Freputation-programmes...pr%2Freputation-dungeon-quest-order) | `pr/reputation-programmes` | [Description](descriptions/reputation-dungeon-quest-order.md) |

## Validation and coverage

Each description records focused validation and relevant limitations. [Composition validation](validation.md) records the final combined checks. [Source coverage](source-coverage.md) maps the surviving fork changes and explains excluded duplicates and historical artifacts. [The manifest](manifest.json) contains exact heads, parents, changed paths and source revisions.

The economic data merge retains all current upstream nonmonetary data and the 27 newly added Forever quests. [Database verification](data-verification.md) records the monetary overlay checks.

Fork `main` remains at `37b3f5f`; these preparation branches do not update the installed desktop application, addons or saved guides. Addon release versions remain maintainer-owned and are unchanged in the submission patches.
