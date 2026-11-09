# Source coverage for the 22-topic submission series

Source: frozen fork `37b3f5f`, common base `c27f59d`, and current upstream `1c8e835`. The series assigns every surviving fork behavior to a topic while preserving upstream objective guides and current quest data. [Machine-readable source coverage](source-coverage.json) records all source commits, split ownership and exclusions.

## Coverage totals

All 82 fork-authored nonmerge commits are classified: 67 contribute covered code, tests, notices or user release notes; 12 contain only omitted historical archives; one duplicates another patch; one is already represented upstream; and one contains superseded documentation cleanup. Six authored merges are classified separately.

The original `c27f59d..37b3f5f` source diff contains 281 paths: all 203 nonarchive paths have an owner or an explicit exclusion, and 78 historical archive paths are excluded. These are source-ownership counts; the final upstream-to-series patch has its own path count after adaptations, redundant changes are removed, and regressions are added. There are no unassigned source behaviors identified by the commit, merge-hunk and residual-tree audits. Per-PR tests and cumulative composition checks are recorded separately in the submission manifest.

## Topic ownership

The first eleven topics have independent branches. The remaining eleven are delivered as a dependent stack; the submission manifest records their exact bases.

| Topic | Covered behavior |
| --- | --- |
| `windows-web-data-paths` | Resolve web game-data paths correctly on Windows. |
| `combat-safe-quest-tracker` | Defer protected tracker layout and secure item-button creation during combat. |
| `wide-guide-import-numbers` | Serialize wide exact numeric guide values without a 32-bit integer formatter overflow. |
| `settings-dropdown-layout` | Keep final dropdown sizing and remove redundant reset controls. |
| `missed-quest-acceptance-warnings` | Warn about offered planned quests left unaccepted, retaining audit state across source changes. |
| `quest-prerequisite-corrections` | Correct remaining prerequisite chains and retain active chain-order/island audit acceptances. |
| `reuse-route-travel-work` | Reuse reordered quest plans and candidate buffers, retain travel-cache working sets and relax known flight nodes. |
| `windows-allocator-test-guard` | Match allocator stress-test platform gating to the WASI/Unix allocator module. |
| `quest-gold-and-item-prices` | Import money/prices, retain starting copper and project quest gold through routes and exports. |
| `finish-current-dungeon-before-next-run` | Finish available inside objectives and hand-ins before another forced run leaves the instance. |
| `outdoor-object-drop-rates` | Cost outdoor containers from placed providers; exclude instance-only provider rates. |
| `best-gear-and-merchant-purchases` | Plan guaranteed quest/vendor equipment, buy route upgrades and preserve quest exploration; include supporting dynamic translations. |
| `client-item-icons` | Extract client icons through CASC and bundle CLI, desktop and web access with attribution. |
| `equipment-tab-and-tooltips` | Show equipment slots, item tooltips and client-style assets, with rounded weapon DPS. |
| `shared-quest-kill-credit` | Share accepted quest creature progress while counting combat, XP, loot and respawn work once. |
| `bounded-search-experiments` | Provide opt-in bounded chain/hub moves and reproducible comparison tools; no performance-benefit claim. |
| `cleanup-accounting-and-training` | Retain performed unrewarded work and useful training during cleanup and route edits. |
| `cleanup-replay-diagnostics` | Inspect explicit dungeon cleanup and replay archived outcomes without rerunning search. |
| `required-gear-source-completion` | Select supported sources and enforce actual completion, visit windows and mandatory-goal provenance. |
| `required-reputation-targets` | Retain useful reputation work, prioritize targets and return best-found shortfalls through replay/editing. |
| `reputation-programmes` | Carry costed repeatable/farming/daily programmes through source catalogs, profiles, playback, editing, cancellation and exports. |
| `reputation-dungeon-quest-order` | Prepare and complete useful reputation chains around a single dungeon run; preserve mixed-source acquisition and objective-guide semantics. |

## Shared commits and integration behavior

- `b77706f` is split three ways: current-dungeon priority, outdoor container drop costs, and the remaining reputation dungeon work. The generic fixes include ordinary-quest regressions and are already prerequisites of the cumulative reputation topic.
- `09752b6` assigns its example to cleanup diagnostics and its core/tests to cleanup accounting. `ddfddb5` assigns starting copper to economy and vendor dispatch to gear planning. `f48cb59` assigns the allocator guard separately from gear-test adjustments.
- `c4d05a8`, `e041e69` and `d0f7ccc` retain model/export/simulator compatibility, CASC/data integration, mandatory provenance, objective-guide clearing of inherited goals, and shared-kill respawn handling. The shared respawn regression belongs to shared kill credit. The active `e041e69` audit acceptances for quest 97558 and the Teldrassil finding belong to prerequisite corrections.
- Generic dynamic translation checks and existing-setting translations from `4f339bb` remain assigned to the gear topic. Source notices and functional data catalogs remain included.

## Deliberate exclusions

- `91bb8ef` is already represented by upstream data: quests 92422 and 95216 have the sole required `pre_any` prerequisite 99156 and 451 respectively, in both the common base and current upstream.
- `c88f19e` duplicates `0178881` exactly; its implementation is included once.
- The old Joel group-guide commits and Dependabot Node-types ancestry are not resubmitted. Their current upstream implementations remain intact.
- The 78 historical report, plan, handoff and raw-result paths remain outside product diffs. Relevant attribution and concise user release notes stay with their topics.
- Fork-only addon release bumps are omitted; upstream release version 0.16.0 remains. Functional TOC declarations and guide format metadata remain feature-owned.
- Historical worker-wide cache/training-memo/XP-ladder changes removed by earlier integration merges are not resurrected. Experimental chain/hub algorithms remain opt-in; no improvement in route quality or computation speed is claimed.
