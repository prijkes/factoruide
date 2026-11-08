# Factoruide route-quality analysis: handoff to Astra

Latest all-branch integration complete, 2026-11-08: local main now includes every
local and fetched remote branch. The refreshed upstream had 15 new commits through
`c27f59d`; normal merge `d0f7ccc` preserves the fork features and incoming data.
Obsolete pre-rebase group guides and the already-equivalent dependency update are
recorded by tree-preserving ancestry merges `b6a3561` and `20c96ab`. No obsolete
profile-sharing behavior was restored. Branches/worktrees were retained; no push.

See the [integration report](docs/all-branches-integration-2026-11-08.md),
[validation record](docs/all-branches-validation-2026-11-08.json) and
[completed plan](docs/superpowers/plans/2026-11-08-all-branches-merge.md).
Product source is `20c96ab`, followed only by final documentation. Independent
specification, code/data quality and delivery review passed without unresolved
consequential findings. Do not repeat completed merge work.

Verification: 788 release workspace tests pass, zero failures, with the same four
ignored tests/examples and one known Windows CRLF test filtered. Release workspace
all-target Clippy, formatting, frontend nine tests/build/lint/i18n (1,214 texts),
and full addon/profile Lua smoke pass. Three released-guide fixtures are covered.
The updated 5,812-quest Forever database preserves all 62 incoming table rowsets,
plus the three fork money columns and 820 source attributions. Signed quest costs
and NULLs remain intact. Raw DB SHA256 is
`64b05f5c05046ca404a4b81a3585a29fffbea674fdc7168a6d93c8c768ed2d14`.
Single-dungeon mode now clears inherited fork BiS goals and retains correct
mandatory provenance; both integration defects have RED/GREEN regressions.

Current production executable `target/release/factoruide-app.exe`: 159,678,464 bytes,
SHA256 `4f655ad9c6618103f50cfdec4a428c3fdbdcf3b921ed655ff8de3822d936f05e`,
built with `tauri/custom-protocol`. Current addon archive
`target/release/Factoruide-addons-20c96ab.zip`: 713,975 bytes, SHA256
`a975a525fac9be3340444886e05d6aae587bd55165b208dc54e6e958af9baf74`.
The two addons in the explicitly requested Forever AddOns folder were refreshed to
0.16.0 after review. All 68 installed payload files and all 70 backup files were
independently verified. Legacy guide/settings files, generated data and
SavedVariables were preserved. Original saved request/outcome/meta hashes remain
unchanged. Native app sessions and the game were not launched or closed.

Private merge/data/review/deployment evidence remains in ignored
`.superpowers/sdd/2026-11-08-all-branches/`. Earlier records below are historical;
their data, artifact hashes and main state apply to their frozen source versions.

Latest continuation complete, 2026-11-08: required reputation, low-level quest
eligibility, guide skip protection and missed-acceptance warnings are implemented,
independently reviewed and merged into local `main`. All completed local branches
are ancestors of main. Product source is `95356fd`; reviewed integration is
`b0a588c`, followed only by final documentation. No push occurred.

The [assessment and delivery record](docs/reputation-and-acceptance-assessment-2026-11-08.md)
and [machine-readable validation](docs/reputation-and-acceptance-validation-2026-11-08.json)
contain the exact results, artifact identities and limits. The
[implementation plan](docs/superpowers/plans/2026-11-08-reputation-and-acceptance.md)
is complete. Do not repeat completed implementation or ask the resolved policies
again: selected reputation totals are mandatory, including detours and work after level 60;
warn for any identified offered planned quest left missing. The user approved
the design and delegated subsequent reviews/approval while asleep.

Final verification: 776 release workspace tests pass, no failures, with four
known ignored tests/examples and one known Windows CRLF test filtered. Release
all-target workspace Clippy, formatting, frontend tests (9), build, lint and i18n (1,206 texts),
full addon and profile-only Lua smoke pass. Production desktop built with
`tauri/custom-protocol`: `target/release/factoruide-app.exe`, 159,502,848 bytes,
SHA256 `44cc211853b1cb987fb94054c0ad040b806053d771edde396ee08af59e3862bb`.
Addon archive `target/release/Factoruide-addons-3fdc85d.zip`, 713,881 bytes,
SHA256 `fa0512c6bb780aa85815c023134672f2b88b7c870e53a093450ee759c7fea445`.
Both were independently verified. Install/update them before regenerating;
active app sessions, game addon installs and original saved guides were untouched.

Proof of Demise (374) now exists in the actual saved level-18 model with 100 Undercity
reward and its 427 prerequisite, without being made mandatory. The Argent
Emissary (96895) rewards 150 Argent Dawn, which that saved request does not select.
A real-data positive reputation case (Gear Prefer) reaches 5,900 Thunder Bluff versus
required 5,000: 21 actual turn-ins independently sum 1,900, all 68 quest steps kept,
zero checkpoints. Combined Required gear/exact/reputation tests also pass.

The original eight Exalted request now fails honestly with named actual deficits;
its final bounded candidate reaches 19,797 Undercity versus 42,000. Even the deliberately
loose direct-once quest bound reaches only 25,122. Seven faction bounds use measured
starts; Horde (67) is missing from the snapshot and its bound uses a labeled estimated
baseline. No repeatable farming or derived spillover is modeled. These are model
limits, not proof of in-game impossibility. Generic bounded gear search can also
fail; same missing sources were reproduced with no reputation and in the prior
binary, so no unrelated gear scheduler was changed. No optimality claim is made.

Ignored private evidence and the completion ledger remain in
`.superpowers/sdd/2026-11-08-reputation-and-acceptance/` for continuation/audit.
Final whole-branch spec/quality/delivery review passed with no unresolved findings.
The earlier continuation records below are historical; their branch, authorization
and merge wording describes those earlier stages.

Current continuation: correcting the saved guide's misleading 25-class-quest
warning and Required gear completion is in progress on `fix/reachable-required-bis`.
The [supported gear assessment](docs/required-gear-assessment-2026-11-07.md) and
[implementation plan](docs/superpowers/plans/2026-11-07-reachable-required-bis.md)
track this work. Source selection, actual-completion enforcement, reporting and
route repair are integrated. Final review's location/window finding is corrected
and scoped-review approved in `66d2123`. Fresh generation on that source completes
71 sources and 193 support quests, with class/gear/other counts all zero and 20
applicable forced visits matching 20 entry markers, in 19.5 seconds at the recorded
small budget. Original-guide compatibility, 754 release workspace tests and release
all-target Clippy pass, with documented test exclusions. The production desktop
executable is built with the bundled frontend; its source/hash and sanitized route
evidence are in the [validation record](docs/required-gear-validation-2026-11-07.json).
Final whole-branch source and evidence review approved local delivery with no
unresolved blockers. This continuation is complete. Original guides remain
untouched; no app launch, installer, push or merge was performed.

Latest implementation continuation: useful class training survives dungeon-quest
cleanup as an independent trainer action when full simulation scores it better
than bare deletion. See the [training assessment](docs/cleanup-training-assessment-2026-11-07.md).
The saved failing chain case improves from 222219.241811 to 158149.958372 weighted
score and saves 11920.706890 simulated player seconds, about 3 hours 19 minutes.
All sixteen old fixed-stage routes replay identically, both inputs match their
old boundaries through TrimTarget, and all eight baseline boundaries are unchanged.
Quest 387 is removed, Stockade 717 remains absent, and level-22 Mage training is
restored. These are zero-search cleanup measurements, not a new algorithm ranking.
The experimental chain operator remains off by default; reranking requires a
new controlled comparison under the corrected model and cleanup.

Source fix `be5bd00` has twelve focused regressions and independent approval.
Fast and release package checks passed 333 library and 238 integration tests;
the known Windows newline test remains skipped and two doc examples ignored.
The diagnostic example adds cleanup-only replay with twelve passing tests.
Diagnostic sources are `4fe8de4` and `10bb20a`. Final whole-branch source and
evidence review approved the change with no outstanding findings, independently
checking every summary field, compact/raw equivalence and all provenance hashes.
The desktop app, saved guides and installer remain untouched.

Earlier diagnostic continuation: autonomous fixed-attempt regeneration reproduced
both saved cases, including all checked search callbacks and final route/score
identities. The loss is now traced exactly to `DropDungeonless` removing quest
387's accept and thereby losing a Mage-training opportunity. See the
[cleanup diagnosis](docs/cleanup-loss-diagnosis-2026-11-07.md). At that stage,
preserving useful training independently was the next routing change; the
implementation and counterfactual validation are now recorded above.

A separate confirmed defect was corrected in `09752b6`: performed unrewarded
objective work remains counted after abandonment, with actual route provenance
excluding imported earlier work. All sixteen saved fixed-route stages retain
identical score, player time, stops and full traces under that correction; the
bug does not explain this captured training-loss case. Parameter defaults and
game data are unchanged. The running desktop app and saved guides were not
modified or redeployed. Release checks passed 321 library, 238 integration,
eight diagnostic-example and three comparison-example tests. The known Windows
newline test remains skipped and two doc examples ignored. Final review's
explicit-dungeon metadata omission was fixed in `df96a30`, with raw captures
preserved and corrected derived metadata re-reviewed as addressed.

Latest measured continuation: the equal-budget algorithm comparison is complete
on `experiment/route-quality-comparison` (experiment source `a3ce987`). See the
[measured report](docs/route-algorithm-comparison-2026-11-07.md) and
[archived protocol and evidence](docs/route-comparison-2026-11-07/README.md).
Eight pilots, 64 tuning runs and 32 held-out runs passed execution/resource
checks. The tuning-selected chain operator reduced aggregate held-out player
time by 1.27%, but worsened weighted score by 0.46%, with a 30.99% worst-case
score regression. It fails the declared promotion rule and remains off by
default. Its saved endpoints alone did not establish the exact cleanup subpass;
the subsequent diagnostic continuation above now supplies that evidence.

This supplies a bounded measured answer, not proof that the current heuristic
is optimal or that no faster/better algorithm exists. The experiment covers a
single-start kernel, BiS off, eight solo profiles and two held-out seeds; the
production tournament was not tested or changed. Requests contained no explicit
hard-required quests. Vendor-source reconciliation remains separate work.

Latest continuation: shared ordinary creature-kill credit and its XP, loot,
timer, and unfinished-work accounting are implemented on `fix/shared-kill-credit`.
See the [phase-two implementation and validation record](docs/superpowers/plans/2026-11-07-shared-kill-credit.md).
The [route-search alternatives report](docs/route-search-alternatives-2026-11-07.md)
records the algorithm inspection and research that preceded the measured
comparison linked above.
No proof that the current search is best, or that no faster/better search exists,
has been established.

Implementation continuation: the user subsequently approved the first fix with
“OK, continue.” The dispatcher and starting-copper corrections are implemented on
`fix/bis-search-dispatch`; see the [implementation plan and validation record](docs/superpowers/plans/2026-11-07-bis-search-dispatch.md).
The original handoff's read-only authorization below describes the earlier
assessment stage. Vendor-source reconciliation remains subsequent work; the
comparative search experiment is now complete as recorded above.

Continuation completed on 2026-11-07: see the
[route-quality assessment and proposed next work](docs/route-quality-assessment-2026-11-07.md).
It resolves the weighted-score comparison, confirms a narrow shared-kill gap,
checks vendor-source coverage, and proposes a staged plan. No algorithm or
parameter-default changes were made. The original handoff below is preserved.

Date: 2026-11-07

## Purpose and authorization

Assess how to generate **smarter routes**, not merely calculate them faster, while preserving gameplay constraints and the fork's gold and BiS features.

Distinguish three different outcomes:

1. Simulated player time.
2. The optimizer's weighted score.
3. Route-generation time.

**The analysis is paused for handoff. Current authorization is read-only assessment and bounded experiments, not implementation or parameter-default changes.** Finish the assessment and present a prioritized plan before changing the algorithm.

The four existing experiments have completed. No further planner runs were performed while writing this handoff. No algorithm source files or parameter defaults were changed by the analysis; this Markdown handoff is the only repository change made for this handoff request.

## Repository and completed-work state

- Repository: `E:\git\extern\factoruide`.
- Last known branch: `main`.
- HEAD, rechecked when writing this handoff: **`e041e69`**, merging all 36 upstream commits through `e78cc88`.
- `git status --short` was clean immediately before this file was created.
- At the previous branch comparison, `main` was **19 commits ahead of `origin/main`**, unpushed. That comparison was not refreshed for this handoff.
- Analysis artifacts are outside the repository, in the temporary directory documented below.
- This handoff has not been committed or pushed.

Earlier phases implemented planner performance improvements, quest-gold tracking, guaranteed-source BiS, vendor-buy stops, bundled client icons, addon gear purchasing, WoW-styled tooltips, and the Gear paperdoll tab.

The upstream merge preserved those features, ported `buy_visit` to `self.sim()`, resolved planner/cache conflicts, imported upstream data, and reran `fg gold` and `fg icons` for Classic and Forever. Preserve upstream refactors and memory caps alongside the fork's working-set-preserving eviction for its two large caches.

Prerequisite work was committed in `91bb8ef`, `ae0f152`, and `e860efa`. The earlier 24-finding sweep corrected 8 flagged quests plus 6 camping siblings, accepted 12 findings, and left 4 unresolved.

Both merge reviewers signed off. **The route-quality analysis findings below have not yet been independently reviewed or implemented.**

## Current algorithm

Retain the existing **model -> simulation -> heuristic-search** architecture unless evidence supports changing it.

Relevant defaults and behavior observed in the merged code:

- 32 initial candidates plus 224 mutants.
- Screening down to 8 candidates, ultimately using 8 searches on 1 finalist.
- `round_tries=0` and `share_every=500`.
- Greedy construction by default.
- Approximately 25% adaptive large-neighborhood moves; the remainder are smaller moves.
- `anneal=0` and `lahc=0`: the main search normally accepts improvements only. Configurable escape mechanisms already exist but are disabled by default.

Upstream already implements **zone-visit gathering, whole-visit cleanup, town-before-departure tidying, and objective relocation**. Further proposals must extend these rather than describe them as absent.

## Findings, ordered by confidence and priority

### 1. Confirmed: vendor BiS disables two existing search capabilities

Location: `crates/fg-route/src/plan/moves.rs:104-118` at `e041e69`.

The dispatcher draws from `0..25`. Arm 19 invokes `buy_visit` when `bis.vendor_items()` is nonempty. The remaining values are covered by explicit arms, leaving the fallback containing `swap_version` and `add_unused_quest` unreachable whenever vendor BiS items are available.

Consequences:

- The search cannot use that dispatcher to add quests omitted from its starting route.
- It cannot use that dispatcher to switch class-quest alternatives.
- A `buy_visit` attempt returning `None` does not restore the fallback.

This is confirmed by source inspection, but there is not yet a focused regression test. Other search mechanisms can still modify routes; do not overstate this as every possible quest-set change being disabled.

**First implementation candidate, once approved:** preserve reachability of vendor buys, class-version swaps, and omitted-quest insertion, with regression coverage proving the capabilities coexist.

### 2. Confirmed structural limitation: introducing missing prerequisite chains

Locations: `crates/fg-route/src/plan/moves.rs` and `crates/fg-route/src/plan/construct.rs`.

- `add_unused_quest` normally inserts one omitted quest.
- Insertion glues some special parent/child relationships, but does not generally insert the complete missing normal prerequisite chain.
- Deletion, by contrast, follows dependent chains.

A useful chain may require several individually unhelpful additions before its valuable endpoint becomes available. Improvement-only acceptance may make that difficult.

**Opportunity:** prerequisite-aware chain or hub insertion/replacement moves. This is a limitation in exploration, not evidence that the current simulator permits prerequisite violations.

### 3. Potential bias: approximate evaluation can miss delayed benefits

Location: `crates/fg-route/src/plan/evaluate.rs`.

The default 150-stop screening estimate extrapolates by matching XP. It is **not a proven bound**. Candidates rejected there do not receive exact evaluation.

This could reject changes whose benefits appear later, such as gear, flights, training, or prerequisite investments.

One reduced-budget probe found lower simulated player time with exact evaluation. **Its weighted-score comparison remains unfinished.** Do not disable screening or declare a general improvement based on that result alone.

### 4. Unverified: shared-kill accounting across objectives

Locations: `crates/fg-route/src/plan/step.rs`, `state.rs`, and `together.rs`.

Consecutive objectives already share some gathering/kill time. However, `objective_work` charges kills and grants mob XP per objective.

It remains unverified whether multiple active quests benefiting from the same kills receive correct combined time and XP accounting. Do not call this a confirmed defect yet.

**Next diagnostic:** use a small scenario to check shared kill credit, mob XP, quest acceptance timing, and different objective counts before proposing a change.

### 5. Further hypotheses, not established improvements

- State-aware vendor insertion.
- Prerequisite-preserving chain/hub moves.
- Diversity-aware candidate selection rather than selection solely by current score.
- More deliberate exploration of changes with delayed benefits.

Each requires controlled comparison against the existing implementation.

## Completed real-data experiments

Artifacts directory:

`C:\Users\Paul\AppData\Local\Temp\opencode\route-quality-analysis-20261107`

Contents:

- `probe.py`: bounded experiment runner.
- `results.json`: collected metrics and notes.
- `human30-default-search.fgguide` and `.log`.
- `human30-exact-evaluation.fgguide` and `.log`.
- `human30-late-acceptance.fgguide` and `.log`.
- `human30-bis-prefer.fgguide` and `.log`.

### Experiment setup

All four cases used:

- Forever, Human Warrior, levels 1-30.
- Seed 42.
- `threads=1`.
- `candidates=1`.
- `race_mutants=0`.
- `effort=0.05`.

These are **controlled reduced-budget probes, not app-default tournament benchmarks**. The source was not modified between cases.

### Results

| Case | Simulated player time, seconds | Generation wall time, seconds | Completed quests |
| --- | ---: | ---: | ---: |
| Default search | 117,354 | 5.925 | 295 |
| Exact evaluation: `eval_window=0` | 116,714 | 6.186 | 291 |
| Late acceptance: `lahc=512` | 122,345 | 6.543 | 289 |
| `bis=prefer` | 117,354 | 5.919 | 295 |

### Interpretation and limitations

- Exact evaluation saved approximately **640 seconds of simulated player time**, but grinding increased from approximately 2,410 to 3,289 seconds and zone changes increased from 40 to 42. Its overall weighted-score advantage has not been established.
- `lahc=512` performed worse in this probe. This argues against simply enabling late acceptance without tuning and broader testing.
- `bis=prefer` matched the default route result exactly.
- All four cases reported **zero vendor-sourced BiS items**, so these probes did **not** exercise the confirmed vendor-dependent dispatch limitation. Inspect actual BiS entries and power availability before diagnosing missing power data.
- Every case reported the same two unplanned class quests at the level-30 boundary:
  - The Islander: **1718**, reported as only after the target level.
  - The Affray: **1719**, blocked by the uncompleted prerequisite 1718.
- Account for these warnings when comparing route completeness. Do not label lower exported player time alone a better overall result.

## Recommended continuation

1. Finish the read-only assessment and present a prioritized optimization plan before coding.
2. Inspect the saved experiment outputs, including warnings and weighted scores.
3. Design focused regression scenarios for:
   - Omitted-quest insertion and class-version swapping remaining reachable alongside vendor buys.
   - Missing normal prerequisite-chain insertion.
   - Shared kills and mob XP across objectives.
4. If implementation is approved, establish a fixed-seed **Classic/Forever quality matrix** covering:
   - Melee and casters.
   - Both factions.
   - Different leveling ranges.
   - BiS modes, including a verified vendor-bearing case.
   - Professions, groups, and forced content.
5. Compare route quality and generation cost separately. Do not generalize from one seed or the synthetic timing benchmark.

There is no technical blocker; execution was paused at the user's request to hand the work to Astra.

## Relevant code and data

Paths in this section are relative to `E:\git\extern\factoruide`.

- `crates/fg-route/src/plan/moves.rs`: move dispatch, quest insertion attempts, vendor-buy moves.
- `crates/fg-route/src/plan/construct.rs`: construction and quest insertion.
- `crates/fg-route/src/plan/search.rs` and `alns.rs`: search and large-neighborhood operators.
- `crates/fg-route/src/plan/evaluate.rs`: incremental evaluation and screening.
- `crates/fg-route/src/plan/sim.rs`, `step.rs`, `spot.rs`, and `travel.rs`: simulation.
- `crates/fg-route/src/plan/tidy.rs` and `together.rs`: existing cleanup and objective grouping.
- `crates/fg-route/src/job/race.rs`: candidate generation, mutations, screening, tournament, and cross-search sharing.
- `crates/fg-route/src/job/mod.rs`: complete generation pipeline, cleanup, and validation.
- `crates/fg-route/src/params.rs`: defaults and objective weights.
- `crates/fg-route/src/model/`: quest constraints and sources; `bis.rs` ranks guaranteed-source gear and `mod.rs` connects BiS to mandatory quests.
- `crates/fg-route/src/power.rs`: gear valuation, class power, prices, and conditions under which `PowerModel::load` returns `None`.
- `crates/fg-route/tests/`: integration tests and miniature-world fixtures.
- `crates/fg-route/benches/search.rs`: synthetic 2,000-try timing benchmark; **not a route-quality benchmark**.
- `crates/fg/src/route.rs`: `fg share` reproduces app planning and writes inspectable JSON `.fgguide` files.
- `ARCHITECTURE.md`: high-level overview; verify its claims against current code.
- `overrides/`: prerequisite corrections, audit accepts, player questions, and edition settings.
- `data/`: merged databases, terrain/maps, and client-derived icon packs.
- `app/src/components/`: existing Gear/BiS UI and `ItemTip.tsx`; preserve these features during future planner changes.

## Requirements to preserve

- **Guaranteed BiS sources are quest rewards and vendor purchases only. Drops never count as guaranteed BiS sources.**
- Gold/BiS feature work targets **Classic and Forever**, not TBC.
- Icons and assets must come from the installed game client, not a CDN.
- Preserve XP accounting, prerequisites, quest-log limits, deadlines, dungeon grouping, and deterministic BiS sourcing.
- Every quest-prerequisite correction requires live-game evidence. Unverified corrections remain unresolved.
- Quest corrections belong in `overrides/forever/quests.toml` or root `overrides/quests.toml`; audit accepts and player questions remain separate.
- After approved implementation phases, use two independent reviewers: **correctness** and **quality**. Address findings before committing.
- Commit as **`prijkes <prijkes@noreply.com>`**.
- Do not discard unrelated user changes, undo upstream refactors, or remove existing memory caps and fork features.

## Previous validation and deployment context

Merge validation reported:

- 238 integration tests passing.
- 13 app tests passing.
- 8 web tests passing.
- 249 route-library tests passing, plus one known failure.
- Both Lua smoke suites passing.
- TypeScript, oxlint, i18n checks covering 1,199 texts, and StyLua clean.

Existing machine-sensitive failures, not introduced by the merge:

- `export::fgstring::tests::the_addon_reads_them_and_they_read_the_addon`: Windows stdout CRLF difference.
- Native `fg-web` `alloc_stress`: references wasm-gated `fg_web::alloc`.

The installer was rebuilt at `target/release/bundle/nsis/Factoruide_0.1.0_x64-setup.exe` (126.90 MiB). The build's final signing error was expected because `TAURI_SIGNING_PRIVATE_KEY` was absent.

Updated Guide/Profile addons were deployed to `F:\Program Files (x86)\World of Warcraft\_forever_\Interface\AddOns`. Release-generated `CHANGES.md` files were preserved.

The source used to restore gold/prices after upstream data rebuilds was:

`C:\Users\Paul\AppData\Local\Temp\opencode\fg-gold\sqlite-dump\mangos.sqlite`

## Separate prerequisite-audit caveats

These are background context, not authorization to resume unrelated prerequisite editing:

- The reported unresolved chain-order questions are **96259, 96261, 97284, and 98461**.
- The last explicitly written `need_help.toml` instead contains **96259, 96261, 92909, and 98461**. This file was not rechecked for the handoff. Do not claim the two lists match without checking.
- The merge added accepts for 97558 and the `Teldrassil: ` islands finding.
- The 77 class-quest audit findings also reproduced using pre-merge data; do not automatically attribute them to the merge.
