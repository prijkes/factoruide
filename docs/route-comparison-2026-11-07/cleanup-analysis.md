# Cleanup/finalization score-change analysis

## Post-timing numerical diagnosis

All timed phases had stopped before this section was added. I read the frozen analysis and individual result artifacts; no source changes, builds, or planners were run. The frozen source is `a3ce9878830b3414b4b200d9fbbfd1febc88cb58` from `results/protocol.json`. The observations below supersede the earlier uncertainty about *which broad stage* contains the losses; uncertainty about the individual cleanup subpass remains.

### Worst held-out pair

`results/analysis-20261107T094650766713.json` identifies **Forever Gnome Mage 20–40, seed 20261107** as the worst pair. The raw artifacts are:

- Baseline: `results/heldout-029-forever-gnome-mage-20-40-20261107-baseline/result.json`.
- Chain: `results/heldout-028-forever-gnome-mage-20-40-20261107-chain/result.json`.

| Measurement | Baseline | Chain |
| --- | ---: | ---: |
| Last raw search-best score | 170,622.35 | 159,358.07 |
| Last raw search-best player seconds | 143,647.07 | 135,999.27 |
| Post-cleanup score, rounded ±30 | 169,980 | 227,160 |
| Authoritative final score | 169,640.22 | 222,219.24 |
| Authoritative player seconds | 143,100.84 | 147,295.15 |
| Final hidden penalty | 26,539.39 | 74,924.09 |
| Search / post-cleanup / final stops | 923 / 921 / 921 | 922 / 916 / 916 |

The chain raw best is 6.6019% better in score. Cleanup then adds **67,801.93 ±30** to chain's score, compared with a **642.35 ±30 improvement** for baseline. Subsequent finalization improves chain by **4,940.76 ±30** and baseline by **339.78 ±30**. Therefore the observed 30.9944% delivered-score regression is dominated by search cleanup, not readability tidying or objective grouping after cleanup. Chain's final-minus-search-best score is +62,861.17: +11,295.88 player seconds and +51,565.29 hidden penalty. These are endpoint differences, not a per-subpass decomposition.

The exact final chain-minus-baseline score gap is **52,579.02**, consisting of +4,194.31 player seconds (2.9310% slower) and +48,384.70 hidden penalty. Using the recorded effective weights, the score difference decomposes as follows:

| Contribution | Score seconds |
| --- | ---: |
| Additional player time | +4,194.31 |
| Additional grinding × (1.5 − 1) | +7,242.47 |
| Additional excess grinding × (4 − 1.5) | +40,200.21 |
| Farming weight | +15.71 |
| Bag-overflow penalty | +1,532.04 |
| Flight-gold penalty | −485.72 |
| Three fewer zone changes | −720.00 |
| One additional abandonment | +600.00 |
| **Total** | **+52,579.02** |

Final grinding increases from 4,357.72 to 18,842.65 seconds; excess grinding from 9.63 to 16,089.72. Fighting falls by 1,469.23 seconds, dungeon time by 4,380 seconds (six runs versus five), travel by 4,729.54 seconds, and quest XP by 43,285. Grind XP rises by 60,422. These are real differences between the two delivered routes, **not proof that cleanup itself removed a dungeon run or those XP rewards**: the raw search route snapshots are absent. Both delivered routes have zero missing class/zone/dungeon/profession goals and empty late-timed warnings. No missing-goal penalty explains the difference. The baseline guide ends with a roughly 26-minute grind to level 40; chain ends through Hillsbrad turn-ins after its substantial earlier grinding.

### Classic Orc 20–40 stage localization

All artifacts below are `results/<run>/result.json`; the concise run suffixes identify the full directory names. Cleanup scores come from “improved:” and have ±30-second uncertainty. Final scores are exact replays.

| Run | Raw best | Post-cleanup rounded | Final | Cleanup increase | Later finalization change |
| --- | ---: | ---: | ---: | ---: | ---: |
| tuning-016, 42 hub | 167,479.01 | 203,400 | 201,891.27 | +35,920.99 | −1,508.73 |
| tuning-017, 42 exact | 166,851.82 | 200,160 | 199,950.03 | +33,308.18 | −209.97 |
| tuning-018, 42 baseline | 168,589.99 | 205,560 | 204,018.81 | +36,970.01 | −1,541.19 |
| tuning-019, 42 chain | 168,596.17 | 218,100 | 215,027.18 | +49,503.83 | −3,072.82 |
| tuning-020, 1337 hub | 168,067.50 | 202,200 | 201,616.72 | +34,132.50 | −583.28 |
| tuning-021, 1337 chain | 167,269.10 | 201,720 | 201,476.32 | +34,450.90 | −243.68 |
| tuning-022, 1337 baseline | 170,479.78 | 217,260 | 215,751.04 | +46,780.22 | −1,508.96 |
| tuning-023, 1337 exact | 168,092.76 | 168,120 | 167,699.34 | +27.24 | −420.66 |
| heldout-008, 20261107 baseline | 167,144.64 | 167,160 | 167,281.62 | +15.36 | +121.62 |
| heldout-009, 20261107 chain | 167,276.23 | 202,500 | 201,070.13 | +35,223.77 | −1,429.87 |
| heldout-010, 314159 chain | 167,438.33 | 202,740 | 199,263.45 | +35,301.67 | −3,476.55 |
| heldout-011, 314159 baseline | 169,113.28 | 221,220 | 217,985.58 | +52,106.72 | −3,234.42 |

Every large jump is already present immediately after cleanup; later finalization reduces it. All these final routes run six dungeons and have zero missing class, zone, and dungeon goals. The large-jump tuning finals have 9,541.62–13,491.19 seconds of grinding and 7,917.35–10,561.74 excess-grinding seconds. The small-drift exact/1337 control has only 316.80 grinding seconds and no excess grinding. Held-out baseline/20261107 likewise has 246.80 grinding seconds and no excess; the two chain finals have 9,016.79–9,837.80 grinding seconds and 7,985.64–8,509.12 excess. Held-out baseline/314159 has 14,323.88 grinding seconds and 10,908.04 excess. This effect occurs in baseline, chain, hub, and exact, so it is not specific evidence against the chain implementation or the windowed evaluator.

### What is established and what still needs reproduction

`overrides/class_quests.toml` defines no power entries for Warrior or Mage. Thus the 600-second power-block preference cannot explain these profiles. `tidy` runs after the recorded cleanup score, so its 30-second per-move preference cannot explain their large cleanup jumps either. Cleanup's other normal transformations use full-replay non-worsening guards, except the +0.5 trim allowance. **Dungeonless removal is therefore the strongest source-supported explanation**: it is the only unbounded-loss policy in the identified stage. A replay inconsistency would be the alternative requiring investigation. The artifacts do not distinguish these possibilities conclusively because they lack snapshots around each cleanup subpass.

No load-bearing correctness bug is demonstrated by these data: final scores reconcile numerically with delivered player time and recorded penalties, and requirements pass. The evidence is sufficient for the bounded conclusion that the frozen chain variant fails the held-out practical-win criterion under this common production cleanup pipeline. It does not support declaring chain universally inferior, nor treating its raw callback gain as a delivered-route gain. The headline held-out outcome remains score 0.45696% worse, player time 1.27372% faster, four of eight positive profile means, and worst score regression 30.9944%.

The next quality target is **making search quality survive cleanup**. The minimal diagnostic path is to reproduce one affected request/seed with a separate instrumented diagnostic binary that saves the best raw route, then exact score/play/breakdown and changed quest/dungeon IDs after each cleanup pass. If `drop_dungeonless` explains the jump, it is an intentional guide-consistency policy with a costly search/finalization mismatch, rather than a demonstrated replay defect; a finite regression should expose that mismatch before changing the policy or search representation. If an allegedly non-worsening full-replay pass produces the jump, preserve that before/after route as the minimal correctness reproducer and investigate before proposing a fix. Do not use a new time-budget diagnostic trajectory as if it were an exact replay of the original timed run.

The earlier source-only analysis below is retained for the pass-by-pass rules and instrumentation details.

This is a source-only diagnosis prepared while held-out timing was active. No planner, build, result parser, or numerical analysis was run; no experiment inputs or source were changed. The coordinator reported that Classic Orc 20–40 tuning runs often worsen by roughly 33,000–46,000 score units between the last search callback and authoritative final output. That observation alone does not identify which pass causes the loss.

## What the two measurements actually bracket

The search callback uses `search.run.total` and `search.run.play` after `replace_run` has replayed the accepted route (`search.rs:201` onward), not merely the windowed candidate estimate. The stored callback is therefore the best raw search route's replayed score. After search ends, `improve_while` calls `clean_up`; only then does the harness call `job::finalize_route`, which performs merge-visits, tidy, objective grouping, and required-content validation. The final artifact is then fully simulated again.

Consequently, callback-to-final deltas combine search cleanup and finalization. They cannot be attributed specifically to `tidy`, the new wrapper, approximate evaluation, or a particular experimental operator from endpoints alone. The shared source and authoritative final scores preserve the final-output comparison, while the size and variation of these deltas remain relevant to interpreting the search kernels.

## Source mechanisms

| Pass | Acceptance rule | Ability to worsen score |
| --- | --- | --- |
| Select best/current route, `search.rs:268` | Uses current if within 0.5 of best | At most 0.5 relative to last best callback |
| `power_first`, `construct.rs:619` | Full simulation; move each power quest's objective/turn-in block earlier when within `class_power_slack` of current total | Deliberate accumulated slack, default 600 per accepted power-quest block |
| `rebind` / `place_hearths`, `hearth.rs:50`, `:199` | Full simulation; keep only improvement exceeding 0.5 | No intended increase |
| `drop_unfinished`, `search.rs:344` | Full simulation; remove quest only when score is no worse within 1e-6 | No material intended increase |
| Trim stops after target, `search.rs:287` | Full run of trimmed route; accept at most +0.5 | At most 0.5 |
| `drop_dungeonless`, `search.rs:308` | Remove optional quests with any dungeon objective whose dungeon is absent; keep removal whenever simulation remains valid | **No score-loss limit** |
| `merge_visits`, `tidy.rs:72` | Exact incremental evaluation must improve by more than 0.5; rebuild full run | No intended increase; evaluation/run disagreement would be a bug |
| `tidy`, `tidy.rs:16` | Exact incremental evaluation may cost up to `tidy_slack` per move; rebuild full run | Deliberate accumulated slack, default 30 per moved stop; no whole-pass slack cap |
| `route_together`, `together.rs:73` | Full simulation, no worse within 1e-6 per move | No material intended increase |
| Required validation | Checks requirements; does not repair route | No route or score change |

`drop_dungeonless` is the only explicit, unbounded loss-accepting transformation. Its comment says that these quests must go “whatever they save”: the guide should not retain an unfinished dungeon quest just because its outdoor work, accepted state, or interaction with other quests saves time. Removing such a quest can change XP, shared objective credit, route timing, quest-log behavior, and the fallback grind to the target. The end-of-route score also weights grinding and excess grinding and penalizes missing goals (`step.rs:1088`, `sim.rs:326`), so a moderate player-time change can have a larger score effect. This makes dungeonless removal the first mechanism to investigate for very large cleanup losses, **not an established cause of these runs**.

Class-power early completion is an explicit safety/usability preference rather than required-content repair. It moves each power quest's whole work block and updates the score after each accepted block; its allowance can accumulate. Tidying similarly spends slack per move and updates its baseline, rather than imposing a 30-second allowance on the entire pass. With effective parameters and stop counts, a simple upper bound can test whether these deliberate allowances could explain the magnitude. If no dungeonless removal occurs and exact incremental evaluation agrees with full replay, positive drift should be bounded approximately by `0.5 + power_blocks * class_power_slack + 0.5 + tidy_moves * tidy_slack`, plus tiny numeric tolerances. `tidy_moves` is at most the number of route stops examined. Use effective request values rather than assuming defaults.

The `model.bundles` structure is not an additional finalization pass. It apportions a downstream multi-prerequisite quest's gear value among prerequisites in `Growth::gear_xp` (`growth.rs:321`; `derive.rs:106`), used by quest-worth/construction heuristics. Actual reward completion and replay remain separate. It can shape the raw route being cleaned but does not itself explain a score discontinuity at finalization.

## Minimal checks after all timed processes stop

1. Read only the selected Classic Orc raw artifacts: baseline and chain for both tuning seeds, plus exact seed 1337 as a useful contrast. Extract the last `stage=search` callback's score/play/stops; the `search_log` “improved:” entry; authoritative score/play; effective `class_power_slack` and `tidy_slack`; final hidden penalty and breakdown; warnings/required counts. There is no need to load all full runs.
2. The “improved:” log is emitted **after cleanup and before finalization**, and includes the post-cleanup stop count and score rounded to the nearest minute (`state.rs:609`). Convert its `HhMM` time to seconds with a ±30-second rounding interval. This immediately separates a ~40,000-point cleanup loss from a later finalization loss without rerunning a planner. It cannot separate individual cleanup passes or recover an exact intermediate score.
3. Compare final-minus-callback player time and hidden penalty. Callback hidden penalty is `score - player_seconds`. The final breakdown identifies whether the delivered route is dominated by grinding/excess-grinding, travel, missing goals, or other weights, but without the pre-cleanup breakdown it does not establish the change in each component.
4. If the loss is concentrated in cleanup, use a separate post-experiment diagnostic run to snapshot the last best route and exact route/score/breakdown after each cleanup subpass. Also record the IDs removed by `drop_dungeonless`, the missing dungeon IDs, whether those quests actually turned in before cleanup, and exact stage stop changes. Compare against the exact-control contrast. Keep this diagnostic separate from frozen measured runs.
5. If the loss is concentrated after cleanup, snapshot exact scores before/after merge-visits, tidy, and route-together, and count tidy moves. For every accepted incremental merge/tidy move, compare the predicted score with the rebuilt full-run score. A large jump despite those per-move guards is a concrete incremental-evaluation consistency bug, rather than an intended slack policy.

The current harness stores final stops/trace/guide, search callback score/play/stop counts, and rounded cleanup log, but **not callback route snapshots, pre-cleanup trace, or per-pass intermediate routes**. Thus an exact per-pass attribution of the original timed runs cannot be reconstructed from current artifacts alone. A repeated time-budget run is a diagnostic reproduction, not guaranteed replay of the same route. If the original best route was captured elsewhere it would avoid this limitation; otherwise retain uncertainty rather than presenting a reproduction as proof about the original.

## Interpretation

The source contains deliberate reasons for final output to score worse than the best raw search route. Unbounded dungeonless removal is a guide-consistency policy, not by itself proof of a scoring defect. It nevertheless creates a potential quality mismatch: the search can favor raw routes whose advantage is later removed, and the cleanup need not retain the best finalized route encountered. That limitation could affect arms differently even though all arms use the same pipeline.

Classify the observed runs only after the compact post-cleanup log check. A loss explained by dungeonless removal or accumulated bounded slack is an expected policy effect that deserves separate product-quality investigation at this magnitude. A loss violating the acceptance rules above, or an exact replay mismatch, is a material correctness/quality defect requiring diagnosis. The frozen final-output comparison remains the relevant measured outcome; a callback-best improvement is not a delivered-route improvement, and these endpoint deltas alone do not prove the responsible mechanism.
