# Route-search alternatives and comparison protocol

Subsequent evidence: the [equal-budget measured comparison](route-algorithm-comparison-2026-11-07.md)
tested chain insertion, bounded hub repair and exact evaluation. The selected
chain arm failed its held-out promotion rule and remains off by default. The
original research-stage assessment below is preserved; its unmeasured status
describes that earlier stage.

Date: 2026-11-07. Algorithm inspection at HEAD `ddfddb5`.

**A different search design could find better routes within the same computation
time. We have reviewed the existing heuristic, but have not compared alternative
solvers and have no proof that it is best, or that no better algorithm exists.**
The candidates below are ranked experiments, not measured improvements.

This report adds primary-source research to the
[existing route-quality assessment](route-quality-assessment-2026-11-07.md).
It does not change algorithm source, defaults, or game data, and involved no builds
or planner runs. Shared direct-kill credit is being corrected separately; that
modeling change is not evidence for or against a search algorithm.

## What should improve

Keep three measurements separate:

- **Player time:** simulated elapsed gameplay time.
- **Weighted score:** the optimizer's objective, including player time, weighted
  costs, and end penalties. See [sim.rs](../crates/fg-route/src/plan/sim.rs)
  (`weights`, line 324) and [step.rs](../crates/fg-route/src/plan/step.rs)
  (`finish`, near the end of the file).
- **Generation wall time:** time taken to construct, search, clean up, and produce
  the final route. Faster generation alone does not establish a better route.

A slower individual move can still improve quality at a fixed wall-time budget.
A faster evaluator can help quality if its saved time is reinvested in useful
search. Neither effect has been demonstrated for the alternatives below.

## Existing capabilities and the closest research models

The planner already combines stateful simulation, greedy construction, a
multi-start tournament, small moves, and adaptive large-neighborhood search
(ALNS). See [race.rs](../crates/fg-route/src/job/race.rs),
[construct.rs](../crates/fg-route/src/plan/construct.rs), and
[search.rs](../crates/fg-route/src/plan/search.rs).
ALNS already has six operators, including zone/window repair, visit gathering,
and late-chain deletion. Its operator weights reward new bests and improvements;
most repair operators reinsert their removed collection using leveled insertion
([alns.rs](../crates/fg-route/src/plan/alns.rs), lines 13, 40, and 181).

Selective routing with precedence and resource constraints is a closer analogy
than ordinary traveling-salesman optimization. The
[multi-visit orienteering paper](https://www.sciencedirect.com/science/article/pii/S0377221719308021)
selects task bundles subject to prescribed ordering, and studies branch-and-cut
and kernel-search methods. Its results concern that formulation and benchmarks.
Factoruide also has state-dependent XP, shared work, gear, money, unlocks, and
level boundaries; importing a standard formulation would require additional
modeling and validation.

## Ranked candidate experiments

### 1. Add richer chain and hub repair to the existing ALNS

Allow a bounded repair to introduce an omitted optional quest together with its
missing normal prerequisites, or replace a small collection of related hub
quests. Evaluate the complete valid bundle rather than requiring each ancestor
to improve the route independently. Compare cheap greedy repair with bounded
regret insertion, which prioritizes decisions whose alternative placements cost
more, before introducing a larger search inside repair.

This extends existing machinery. Greedy construction already values downstream
local chains ([worth.rs](../crates/fg-route/src/plan/worth.rs), line 74;
[chains.rs](../crates/fg-route/src/model/chains.rs), line 28).
The narrower gap is that `add_unused_quest` samples one quest from the whole model
and rejects already-used draws
([moves.rs](../crates/fg-route/src/plan/moves.rs), line 482), while linked insertion
handles held-parent and handed-item relationships rather than generally adding
missing `pre_all`/`pre_any` ancestry
([construct.rs](../crates/fg-route/src/plan/construct.rs), line 803).

The [original ALNS paper](https://pubsonline.informs.org/doi/abs/10.1287/trsc.1050.0135)
supports combining competing removal and insertion heuristics, with selection
adapted to historical performance. It does not prove which repair is best here.
After profiling, compare the current rewards with cost-aware operator selection
while preserving exploration; gain per millisecond is a hypothesis, not a finding.

### 2. Search a small neighborhood with the simulator

Choose a small hub/window and dependency closure, keep the outside route order,
and search alternative selections and valid orderings inside that neighborhood.
Use the existing simulator for transitions and complete affected suffix replay.
State changes to XP, gear, money, or unlocks can affect costs far beyond the hub.
Keep the incumbent, impose node/time/memory limits, and return the best feasible
complete route found when the limit is reached.

This follows the principle of
[Shaw's constraint-based LNS](https://doi.org/10.1007/3-540-49481-2_30): local search
selects a neighborhood and tree search explores its repair. A simulator-native
prototype can avoid duplicating gameplay semantics in an external solver.
Call a result exact only for a precisely specified neighborhood that was fully
searched; an interrupted search generally provides no local-optimality proof.

[Resource-constrained shortest-path labeling](https://onlinelibrary.wiley.com/doi/abs/10.1002/net.20033)
also suggests state reuse and dominance. Here, a proposed dominance rule must
preserve feasible continuations and their final objective. Matching position,
XP, and current score is insufficient when quest progress, inventory, gear,
gold, rested XP, unlocks, timers, or group state differ. Begin with narrowly
proved equivalence; full global dynamic programming could require too many
distinct states. Existing suffix reuse already compares extensive state
([evaluate.rs](../crates/fg-route/src/plan/evaluate.rs), line 338).

### 3. Try selective beam construction or rollout

At consequential hub, chain, flight, training, or purchase decisions, keep a
small set of partial routes and rank alternatives through greedy completions.
Reuse the current constructor as the completion policy and retain the best
fully simulated complete route. Spend a fixed portion of the existing
construction/tournament budget on this experiment, including all completion
costs. A diversity-aware portfolio can retain different quest sets or hub orders
instead of filling the retained set with similar routes.

The [MIT rollout paper](https://web.mit.edu/dimitrib/www/rollout.pdf) formalizes
improving a base heuristic through lookahead and heuristic completions.
Its improvement guarantees require stated consistency and termination
conditions; truncated or pruned beam search does not automatically inherit them.
This is a larger and less certain change than improving ALNS repair.

## Measure screening and computation costs first

The current screen extrapolates from matched XP
([evaluate.rs](../crates/fg-route/src/plan/evaluate.rs), line 392), then exact
evaluation is normally attempted only if the estimate passes acceptance
([search.rs](../crates/fg-route/src/plan/search.rs), line 164).
The estimate is not established as a bound on the final score.
Audit a reproducible sample of rejected proposals with exact replay, without
changing the search path or random stream. Record missed improvements, magnitude,
move type, and audit cost. Then compare selective exact evaluation or adjusted
windows against the baseline; disabling screening globally is not yet justified.

Profile the full pipeline before choosing CPU optimizations. Candidate copying,
planned-state rebuilding, state snapshots, simulation, travel/path lookup,
construction, and cleanup are inspection-based candidates, not measured hot
spots. Record cache hit rates and peak memory alongside elapsed costs. Preserve
existing cache caps. Compare profiling/audit overhead separately from production
search so instrumentation does not create an apparent algorithm advantage.

## Fair comparison and decision rule

1. Finish model correctness work first, including shared-kill accounting and
   incremental/full replay agreement. Freeze source revision, database hashes,
   request JSON, effective parameters, and relevant routing memory. Comparing
   scores under different simulators does not isolate search quality.
2. Build tiny exact oracles over a stated finite set of permitted actions and
   routes. Include an omitted `A -> B -> C` chain whose prefixes lose but complete
   bundle wins, AND/OR prerequisites, quest-log limits, timers, shared kills,
   gold/gear tradeoffs, and required-content controls. Exactness applies only to
   that bounded oracle problem, not the full game.
3. Use equal total wall-time and peak-memory limits, the same worker/CPU budget,
   and the same accounting boundaries. Include construction, screening, repair,
   cleanup, and final validation. Equal `effort` or move counts need not mean
   equal computation time when move costs differ.
4. Use paired seeds `42`, `1337`, and `20261107`, after creating and validating
   requests from the [proposed quality matrix](route-quality-assessment-2026-11-07.md#proposed-fixed-seed-quality-matrix):
   both editions, melee/casters, both factions, multiple level ranges, forced
   content, professions, groups, and verified vendor-bearing cases. First use
   reduced budgets to screen candidates, then representative app-default runs.
5. Separate tuning cases/seeds from held-out evaluation. Compare each change
   alone before combinations, and repeat timings sufficiently to expose noise.
   Record exact best-score-over-time curves, player time, score components,
   valid completions, warnings, required content, buys/spend, and peak memory.
6. Reject correctness and constraint regressions first. Report paired per-case
   results, aggregate gains, and worst regressions. Prefer an alternative only
   when it improves the observed quality/time tradeoff across the evaluation
   set; retain uncertainty when gains are small or case-dependent.

The immediate research priority is richer optional-chain repair, followed by a
small simulator-driven neighborhood experiment. Comparative measurements remain
necessary before changing defaults or replacing the current search architecture.
