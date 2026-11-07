# Route-quality assessment and proposed next work

Date: 2026-11-07. Source revision: `e041e69`.

Implementation follow-up: after the user's approval, the first dispatcher fix
and an associated starting-copper correction were implemented. See the
[phase-one implementation and validation record](superpowers/plans/2026-11-07-bis-search-dispatch.md).
The assessment below records the source and experiments before those changes.

This completes the assessment requested in [the Astra handoff](../ASTRA_HANDOFF.md).
The scope remains read-only inspection and bounded experiments. No algorithm,
parameter default, quest prerequisite, or game-data change was made. The work
below is proposed, not an approved implementation design.

## Findings that determine the next step

1. **Confirmed dispatch defect:** vendor BiS candidates take the only small-move
   dispatch slot that normally tries an omitted quest or a class-quest version
   swap. Restore independent reachability of all three capabilities first.
2. **Confirmed narrow simulation gap:** two accepted direct-kill objectives for
   the same mob are charged independently, including separate mob XP. Correct
   shared credit and its accounting before judging search changes by this model.
3. **Confirmed vendor-data discrepancy:** limited-stock metadata contains gear
   whose vendor source links are absent from the table BiS enumerates. Reconcile
   the source evidence before treating a vendor benchmark as representative.
4. **Confirmed search limitation:** local search cannot generally introduce a
   complete omitted optional normal prerequisite chain in one move. A bounded
   chain move is a useful next experiment after the correctness work.
5. **Screening and exploration remain hypotheses:** the existing probes do not
   justify disabling screening or enabling late acceptance by default.

Preserve the model, simulation, and heuristic-search architecture. Existing zone
visit gathering, whole-visit cleanup, town tidying, objective relocation, local
chain valuation, memory caps, gold tracking, and BiS features already contribute
to this architecture and should be extended where necessary.

## Completed weighted-score comparison

The four original probes used Forever, Human Warrior, levels 1–30, seed 42,
`threads=1`, `candidates=1`, `race_mutants=0`, and `effort=0.05`. They are reduced
budget probes, not app-default tournament benchmarks.

The exported `total_time` is player time. It is not the optimizer score. Further,
`Breakdown` skips `flight_gold`, `bag_overflow`, and `unlock_delay` in serialization
([state.rs](../crates/fg-route/src/plan/state.rs), lines 468–478). Adding the visible
export fields alone therefore cannot reproduce the score.

A temporary diagnostic loaded each saved request and expanded plan, restored its
saved walking memory, and replayed it with `Planner::explicit().edit()`. It linked
the existing current release library and opened `data/forever.sqlite` read-only;
it did not run optimization. All four replays had zero invalid steps and reproduced
the saved unrounded player time within `1.5e-11` seconds. Scores include the
weights in [sim.rs](../crates/fg-route/src/plan/sim.rs), line 321, and end penalties
in [step.rs](../crates/fg-route/src/plan/step.rs), line 972.

| Probe | Player time (seconds) | Weighted score (lower is better) | Original generation wall time (seconds) | Quests |
| --- | ---: | ---: | ---: | ---: |
| Default search | 117,354.06 | 148,388.04 | 5.925 | 295 |
| Exact evaluation, `eval_window=0` | 116,714.21 | 148,895.93 | 6.186 | 291 |
| Late acceptance, `lahc=512` | 122,345.44 | 156,817.88 | 6.543 | 289 |
| `bis=prefer` | 117,354.06 | 148,388.04 | 5.919 | 295 |

Exact evaluation saved **639.85 seconds of player time but worsened weighted score
by 507.89**. Its additional penalties totalled 1,147.74 score units: more grinding,
grinding beyond the cap, farming, flight gold, and two additional zone changes,
partly offset by less bag overflow. This result does not prove screening improves
quality generally, or isolate any particular falsely rejected candidate.

Late acceptance worsened player time by 4,991.38 seconds and score by 8,429.84.
`bis=prefer` matched both default totals. Every probe still reports the same two
missing class quests at the level-30 boundary: The Islander (1718) and The Affray
(1719). Their shared missing-class penalty is 14,400 score units. These warnings
remain limitations of these outputs, not regressions introduced by this assessment.

## Vendor search reachability

In [moves.rs](../crates/fg-route/src/plan/moves.rs), lines 104–118, `random_move`
draws from `0..25`. Without vendor BiS items, value 19 reaches the fallback that
tries `swap_version(...).or_else(add_unused_quest)`. With any vendor BiS item,
value 19 invokes `buy_visit` and every draw value has an explicit arm. The fallback
is unreachable even if the purchase attempt returns `None`, including because the
buy stop already exists (lines 338–339).

This depends on candidate availability, not an actual purchase. It removes these
two capabilities from this dispatcher; it does not remove construction, deletion,
cross-search sharing, or every other way quest sets can change.

The proposed minimal repair is a dedicated vendor draw separate from fallback 19,
with an additional draw slot only when vendor candidates exist. That preserves
the no-vendor draw range. It changes the vendor-case move distribution, so compare
that behavior with fixed seeds. Falling back only on failed purchases would still
couple quest insertion and version swaps to purchase failure.

Focused regression scenarios should call the dispatcher, rather than only its
helpers, with a fixed valid base route and a verified nonempty vendor candidate
list. Prove that buys, an omitted ordinary quest, and a whole alternate class
chain each remain reachable. Repeat with the buy already present and with no
vendor candidates. Existing direct version-swap tests, including the test at
[plan/tests.rs](../crates/fg-route/src/plan/tests.rs), line 1158, do not cover this
dispatch interaction.

## Shared direct-kill credit

The existing `Together` mechanism overlaps gathering with combat; it stores
aggregate times and position, not target identity
([state.rs](../crates/fg-route/src/plan/state.rs), line 194;
[step.rs](../crates/fg-route/src/plan/step.rs), line 875).
`objective_work`, at line 901, consumes progress for its own quest/objective,
charges its remaining kills, grants their XP and loot, then completes only that
objective. Another accepted quest for the same mob receives no credit there.

A temporary miniature world used mob ID 777 for both quests, the same position,
level-10 character and mobs, zero quest reward XP, no object uses, no on-the-way
farming, no overhead, and no extra time. No level-up occurred during these stops.
Each kill cost 24 seconds and gave 95 XP. The table reports objective combat and
mob XP, excluding final leveling grind.

| Scenario | Current combat / mob XP | Expected with shared direct-kill credit |
| --- | ---: | ---: |
| Both accepted, each needs 2 kills | 96 seconds / 380 XP | 48 seconds / 190 XP |
| Both accepted, counts 2 and 5 | 168 seconds / 665 XP | 120 seconds / 475 XP |
| Both accepted, counts 5 and 2 | 168 seconds / 665 XP | 120 seconds / 475 XP |
| Different mob IDs, counts 2 and 5 | 168 seconds / 665 XP | 168 seconds / 665 XP |
| Second accepted after the first 2 kills, then needs 5 | 168 seconds / 665 XP | 168 seconds / 665 XP |

Turning in the accepted 2-kill quest immediately after completing the 5-kill
objective is also rejected as incomplete. This confirms missing shared completion
in this direct-kill scenario. The current XP is internally consistent with the
extra kills the simulator models; simply subtracting time would be incorrect.

The proposed initial correction should cover deterministic direct-kill credit and
keep combat time, XP, rested XP, loot/bags, progress, and later levels consistent.
Honor acceptance time, objective ordering, initial progress, abandonment, and
actual target identity. Co-located different mobs must not share credit. Preserve
existing gathering overlap and dungeon fan-out, which already credits accepted
dungeon objectives with a single run budget (step.rs, lines 940–966).

Do not generalize this to `max(expected_kills)` for arbitrary loot objectives:
overlapping target pools and random drops need a separate joint-work model. Also
review isolated objective cost estimates in [spot.rs](../crates/fg-route/src/plan/spot.rs),
line 73, and incremental evaluation/state equivalence when introducing shared
progress. Real-data prevalence and the overall route-quality effect are unmeasured.

## Vendor-source data and a verified diagnostic case

BiS enumerates vendor sources exclusively through `m_item_source` rows whose
`from_type` is `vendor` ([bis.rs](../crates/fg-route/src/model/bis.rs), line 122).
The separate `m_vendor_stock` table supplies limited-stock/restock metadata
([entities.rs](../crates/fg-route/src/model/entities.rs), line 177).

The current databases contain only two distinct vendor-source item IDs that join
`m_item_stats`: Executor Staff (3277) and Simple Wildflowers (3421). The flowers
have no positive modeled gear value. The staff is also a reward of Horde quest
382, The Red Messenger, so an eligible loaded quest can take precedence over its
vendor source. All three original non-BiS probes used `bis=off`; the prefer probe
has 68 BiS entries, all quest-sourced. Thus zero vendor candidates in those probes
does not demonstrate absent power data. The tables and usable power are present.

Both databases also contain 768 `m_vendor_stock` rows covering 284 items. Of those,
62 distinct items join `m_item_stats`, but none of those 62 has a vendor source
link in `m_item_source`. Examples are Feral Blade (4766), Ironwood Maul (4777),
Solstice Robe (4782), and Stable Boots (4789). This establishes a discrepancy for
source-ingestion review. It does not establish that every stock row is an
unrestricted, reachable, currently available guaranteed purchase; do not blindly
import it as one. Preserve faction, location, price, restrictions, and stock rules.

One bounded diagnostic confirmed an actual vendor-bearing model: **Forever,
Tauren Druid, levels 1–5, `bis=prefer`, `excluded_quests=[382]`, `effort=0`, seed
42**, one thread/candidate and no mutants. Its nine BiS entries include band-5
two-hand Executor Staff (3277), sold by Ott (3539). The route has no buy steps and
zero vendor expenditure. This constrained case verifies the dispatch guard's
precondition; it is not a representative leveling-quality benchmark or evidence
that visiting Ott is worthwhile. No source data or default was altered.

The original `bis_vendor_items` metric counts distinct vendor-sourced entries in
the exported candidate list. Export serializes the BiS list without requiring a
route purchase ([export/mod.rs](../crates/fg-route/src/export/mod.rs), line 312).
Future measurements must record candidate count, actual buy stops, and vendor
spend separately.

## Optional prerequisite-chain exploration

`add_unused_quest` samples one quest
([moves.rs](../crates/fg-route/src/plan/moves.rs), line 479). Linked insertion
follows held-parent and handed-item relationships, but not general missing
`pre_all` / `pre_any` ancestry
([construct.rs](../crates/fg-route/src/plan/construct.rs), lines 814–851).
ALNS reinserts its selected removed collection; prerequisite ordering does not
invent missing optional ancestors. Default improvement-only acceptance can stop
a route entering a useful chain when its prerequisites individually worsen score.

This is not a lack of all prerequisite foresight. Greedy construction values
downstream local chains ([worth.rs](../crates/fg-route/src/plan/worth.rs), line 71;
[chains.rs](../crates/fg-route/src/model/chains.rs), line 28), explicitly required
quests recursively select prerequisites, and mandatory quests have repair paths.
Simulation still rejects invalid prerequisite order.

A proposed bounded optional-chain move should evaluate a complete valid bundle
at once. Its decisive regression is an omitted normal `A -> B -> C` chain where
`A` and `A+B` individually lose but the complete chain improves score. Additional
cases need existing/completed ancestors, AND prerequisites, sufficient alternative
prerequisites, incompatible choices, quest-log limits, and timed/held children.
No overall quality gain from this proposed operator has yet been demonstrated.

## Proposed implementation and experiment order

Implementation remains outside the handoff's current assessment scope. These
phases are proposed for approval together or separately; their detailed designs
must follow the chosen scope. After an approved phase, use separate correctness
and quality reviewers, resolve findings, and commit as
`prijkes <prijkes@noreply.com>`.

1. **Restore dispatcher reachability.** Add regressions proving vendor buys,
   omitted quests, and whole class-version changes coexist. Run relevant route
   tests and a small paired quality comparison, preserving the no-vendor behavior.
2. **Correct direct-kill sharing.** Start with identity-safe deterministic kills
   and synchronized accounting. Include the miniature-world controls above,
   incremental versus full replay comparisons, and regression coverage for
   dungeon credit and gathering overlap. Rebaseline quality after simulation
   semantics change; scores across the old/new models alone cannot prove faster
   actual gameplay.
3. **Reconcile vendor-source evidence.** Trace source ingestion and authoritative
   stock information for Classic and Forever. Validate constrained/missing-source
   examples and eligibility before correcting data or candidate loading. Keep
   quest rewards and vendor purchases as the only guaranteed BiS sources.
4. **Evaluate bounded chain insertion.** Keep the existing ALNS and cleanup
   machinery; compare a dedicated bundle move with the restored dispatcher as
   baseline. Measure completed-chain gains and generation cost separately.
5. **Measure screening error, then tune exploration.** Sample rejected estimates
   for exact evaluation without letting the diagnostic change the search path.
   Record false rejection rate and missed score improvement, including gear,
   flights, training, and prerequisites. Compare selective exact evaluation,
   adaptive windows, and the current window before considering a default change.
   Only then compare modest escape/diversity strategies. The existing single
   `lahc=512` loss does not rule out every exploration setting.

## Proposed fixed-seed quality matrix

Freeze source revision, database hashes, effective parameters, request JSON,
seed, and thread/search count. Use paired baseline/change runs with seeds
`42`, `1337`, and `20261107`. Begin with reduced-budget screening; then run app
defaults for representative surviving cases before promoting any default. A
single seed and the synthetic 2,000-try timing benchmark cannot establish quality.
This table defines coverage; executable requests for every row have not yet been
created or validated.

| Case family | Profile | Coverage |
| --- | --- | --- |
| Existing baseline, both editions | Human Warrior 1–30 | Alliance melee; class-boundary warnings tracked |
| Caster, both editions | Undead Mage 1–40 | Horde caster; longer horizon and travel/training |
| Mid-level, both editions | Orc Warrior 20–40 | Horde melee; different starting range |
| Mid-level, both editions | Gnome Mage 20–40 | Alliance caster; different starting range |
| Professions and forced content, both editions | Tauren Druid 10–30 | Explicit valid profession goals and fixed required quest/dungeon selections |
| Group, both editions | Same-faction Warrior plus healer 10–20 | Fixed synthetic/imported start profiles; member XP and synchronization |
| Vendor diagnostic, Forever | Tauren Druid 1–5; exclude 382 | Proven vendor candidates; off/prefer/required controls |

For the profession, forced-content, and group cells, freeze actual validated
requests before execution rather than inventing different constraints for each
run. `fg share` hardcodes empty profession and group lists; use a structured
`PlanRequest` harness or `fg route` with all generated outputs redirected to
temporary directories for those cells. Group mates require a starting profile,
one faction, and level 10 or above. Add a representative vendor-bearing quality
case in each edition after the source reconciliation; the constrained level-5
diagnostic is only a reachability
control. Use off/prefer/required BiS variants on selected cases rather than an
unbounded cross-product.

Record player time, exact final weighted score and component penalties,
generation wall time, quest completions, grind/over-cap grind, zone changes,
travel, warnings/invalid steps, required content, vendor candidates/buys/spend,
and gold. Reject constraint regressions before considering score improvements.
Report paired per-case results, aggregate improvement, and worst regressions;
do not hide poor cases behind an average. Preserve prerequisites, XP and rested
XP, quest-log limits, deadlines, dungeon grouping, deterministic sourcing, client
icons, memory caps, and existing gold/BiS features.

## Evidence and reproduction

Temporary artifacts remain outside the repository:

- `C:\Users\Paul\AppData\Local\Temp\opencode\route-quality-analysis-20261107`:
  original four `.fgguide` files, logs, `probe.py`, `results.json`, new
  `rescore_saved.rs`, `rescore_saved.exe`, `rescored-results.json`, and
  `tauren5-vendor-diagnostic.fgguide`.
- `C:\Users\Paul\AppData\Local\Temp\factoruide-shared-kills-20261107`:
  `probe.rs`, `probe.exe`, and `results.txt`.

The replay diagnostic was compiled with Rust 1.99.0 against
`target/release/deps/libfg_route-880c0d98f2e5bd8d.rlib`, the release library used
by the existing CLI build. The shared-kill diagnostic used the current
testing-feature library `libfg_route-3cb8fd1616afb8cf.rlib`. These are temporary
assessment tools, not permanent regression tests. Repository algorithm source
was neither edited nor rebuilt. The existing full-suite results in the handoff
were not rerun and are not claimed as fresh validation here.

Replaying the saved plans again requires only the temporary diagnostic executable:

```powershell
& "$env:TEMP\opencode\route-quality-analysis-20261107\rescore_saved.exe"
& "$env:TEMP\factoruide-shared-kills-20261107\probe.exe"
```

The bounded vendor diagnostic used the following command from the repository
root. Its effort is zero, so its output verifies candidate presence, not search
reachability or route-quality improvement:

```powershell
.\target\release\fg.exe --edition forever share --race tauren --class druid --to-level 5 --effort 0 --name tauren5-vendor-diagnostic --out "$env:TEMP\opencode\route-quality-analysis-20261107\tauren5-vendor-diagnostic.fgguide" --param threads=1 --param candidates=1 --param race_mutants=0 --param seed=42 --param bis=prefer --param 'excluded_quests=[382]'
```

For either database, these read-only queries reproduce the vendor counts and
missing source-link count (2, 768/284, and 62 respectively):

```sql
SELECT count(DISTINCT s.item_id)
FROM m_item_source s JOIN m_item_stats i ON i.id = s.item_id
WHERE s.from_type = 'vendor';

SELECT count(*), count(DISTINCT item_id) FROM m_vendor_stock;

SELECT count(DISTINCT v.item_id)
FROM m_vendor_stock v JOIN m_item_stats i ON i.id = v.item_id
WHERE NOT EXISTS (
  SELECT 1 FROM m_item_source s
  WHERE s.item_id = v.item_id AND s.from_type = 'vendor'
);
```
