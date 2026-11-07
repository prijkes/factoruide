# Cleanup loss diagnosis: 2026-11-07

**Both saved cases were regenerated autonomously with all recorded observables
matching exactly.** The chain run used 59,526 attempts and the baseline 64,872;
both reproduce the recorded callbacks, effective request and parameters,
improvement counts, final typed routes, final scores, and player times. The
chain loss is now isolated to `DropDungeonless`: deleting quest 387's accept
increases score by 67,881.105287 and player time by 12,345.748046 seconds.

The removed visit had supplied Mage training. Its loss changes later combat
gates and grinding, eventually displacing fifteen tail turn-ins. A separate
abandoned-work accounting defect was confirmed by controlled oracles and has a
narrow correction; it does not explain this captured cleanup loss.

## Reproduction and evidence

The cases are Forever Gnome Mage 20–40, seed `20261107`, from the
[measured algorithm comparison](route-algorithm-comparison-2026-11-07.md).
Work follows the [diagnostic plan](superpowers/plans/2026-11-07-cleanup-loss-diagnosis.md)
on `diagnostic/cleanup-loss`, based on `e73af02`.

The diagnostic restores the archived effective request and uses fixed attempt
counts in place of both wall-time stopping conditions. Annealing and LAHC are
zero; no sharing member is attached. It checks the entire ordered callback
sequence, excluding timestamps, and exact final stop identities rather than
assuming a repeated timed run follows the same path. Chain reproduces 367
logged improvements and baseline 364. Fresh simulation of `SelectedInput`
matches each raw search-best callback.

Observation only copies stops at cleanup boundaries. Scoring and tracing happen
afterward through fresh ordinary planners, without replay mode or warmed search
memo tables. The seven cleanup boundaries, including unchanged stages, preserve
the production ordering and combined rebind/hearth update. Normal callers use
the same search implementation with a no-op observer. Independent instrumentation
and example reviews approved this method; see the [review record](cleanup-diagnosis-2026-11-07/reviews.md).

The original comparison artifacts remain immutable. New captures are separate
under `.superpowers/sdd/2026-11-07-cleanup-loss-diagnosis`; the example refuses an
existing output path. Compact durable captures retain stop vectors but omit
the full trace arrays. They are derived evidence, not byte-identical copies of
the original JSONs; [provenance and raw hashes](cleanup-diagnosis-2026-11-07/provenance.json)
identify both forms. Full original trace JSONs remain immutable in the workspace:

- [Chain, original semantics](cleanup-diagnosis-2026-11-07/chain-original-semantics.json).
- [Baseline, original semantics](cleanup-diagnosis-2026-11-07/baseline-original-semantics.json).
- [Chain, corrected accounting](cleanup-diagnosis-2026-11-07/chain-corrected-accounting.json).
- [Baseline, corrected accounting](cleanup-diagnosis-2026-11-07/baseline-corrected-accounting.json).
- [Trace and XP analysis](cleanup-diagnosis-2026-11-07/trace-analysis.md).

The Forever database SHA-256 still matches the frozen experiment:
`1c93c95bfa5d236728ce75d0d9059133a070cff9aa7fdaf62be902f7c3582db7`.
`Cargo.lock` also matches:
`aa2b1fdca5d9dc159b2ec7cc18986d999b5702a2659966c5684bb67854aba57c`.
Exact reproduction here means equality of the recorded observables under the
original accounting semantics; it does not claim unavailable internal-state
snapshots or a new timing result.

## Exact stage localization under original semantics

Scores include weighted penalties; player times are simulated seconds. Values
below are exact replays displayed to six decimals. Full precision and stops are
retained in the captures. `Finalized` includes the later common finalizer.

| Stage | Chain score | Chain player seconds | Baseline score | Baseline player seconds |
| --- | --- | --- | --- | --- |
| SelectedInput | 159358.072035 | 135999.266260 | 170622.349791 | 143647.067682 |
| PowerFirst | 159358.072035 | 135999.266260 | 170622.349791 | 143647.067682 |
| Rebind | 159358.072035 | 135999.266260 | 170622.349791 | 143647.067682 |
| PlaceHearths | 159270.498676 | 135909.578682 | 170622.349791 | 143647.067682 |
| DropUnfinished | 159260.450063 | 135899.530070 | 169992.921219 | 143617.639110 |
| TrimTarget | 159260.450063 | 135899.530070 | 169992.921219 | 143617.639110 |
| DropDungeonless | 227141.555349 | 148245.278116 | 169992.921219 | 143617.639110 |
| Finalized | 222219.241811 | 147295.150283 | 169640.224341 | 143100.837689 |

Chain's `TrimTarget` route has 917 stops. `DropDungeonless` produces exactly the
same route with one stop removed: zero-based route index 99, the accept of
**387, Quell the Uprising**. Its required dungeon, **717, The Stockade**, is
absent, and the quest never turns in. Actual dungeon-entry identities do not
change. The baseline pass changes nothing.

This deletion alone raises chain score from 159260.4500628052 to
227141.55534933577, a difference of **67881.10528653057**, including
**12345.748046475375** additional player seconds. Later finalization improves
score by 4922.313538418384. This supersedes the earlier rounded-log uncertainty:
the precise cleanup pass and finite stop mutation are established.

## Training, gate, and XP cascade

The traces agree through event 117, a Stormwind flight at level 22. Before
deletion, event 118 accepts quest 387 and event 119 trains at Mage trainer 7;
power rises from 21.250962 to 22.005221 without a gear or quest-XP change.
After deletion, both the accept and that training event disappear. Training is
currently an implicit event chosen after route stops; the quest visit supplied
a nearby service opportunity.

The changed route later needs a 4081.162440-second grind to level 24 before
quest 126, Howling in the Hills. At timed quest 212, A Meal Served Cold, the
pre-deletion route grinds 389.099604 seconds to level 38; the changed route
instead grinds 15869.068821 seconds to level 39, then trains. The trace proves
those gate outcomes and ordering. The source explains how training deficit and
current level affect the gate, but exact internal gate inputs were not separately
serialized.

The earlier grinding reaches level 40 before the last fifteen turn-ins, leaving
34 tail stops unexecuted. Lost quest XP reconciles exactly: 41,170 from those
fifteen rewards, 1,665 from six retained rewards scaled down at higher levels,
and 500 from the gray-abandoned delivery quest 97914, totaling **43,335 XP**.
Across all sources, quest −43,335, mob −29,692, farming −5,152, exploration
−1,608, and grinding +82,146 sum to +2,359 XP, exactly the final-state difference.
The [trace analysis](cleanup-diagnosis-2026-11-07/trace-analysis.md) retains the
individual events, quest IDs, and limits on finer XP attribution.

Quest 387 performs no objective work before abandonment; quest 97914 has no
objectives. Neither has performed work for the accounting defect below to erase.
The captured deletion loss is therefore separate from that defect.

Keep the dungeon/quest consistency rule: retaining an unusable quest solely for
its training side effect would violate the guide requirement. An explicit
training/service visit is a plausible future representation. **No restored-
training counterfactual has been implemented or run**, and no recovered score
is claimed for that future design.

## Confirmed abandoned-work accounting correction

Authentic full-route oracles exposed an independent error: abandoning the last
reward beneficiary erased performed unrewarded work from grinding classification.
Four actual unrewarded kills produced **0 instead of 96 grinding seconds** for
both direct and legacy objective work. Active unfinished-work and genuinely
shared-reward controls passed. Expanded failing controls also covered 48 seconds
of newly performed partial work and the last of two beneficiaries.

The narrow correction records actual legacy objective work in a sorted
copy-on-write ledger, with at most one active row per quest. Losing its final
beneficiary transfers actual work once into an orphaned-work scalar; a genuine
reward settles the row without creating that liability. Finish classification
includes pending and orphaned work while retaining the existing fighting cap
and direct rewarded-kill protection. Initialization and exact incremental suffix
equality include the new state. This changes work classification, not physical
combat time, XP, rewards, travel, quest rules, or search defaults.

The fourteen focused regressions cover direct/legacy expiry, imported complete
and partial progress, actual alternative/stock cost, genuine shared rewards,
both gathering-overlap orders, repeated settlement, copy-on-write snapshots and
bounded ledger rows, plus full/incremental replay with different work ownership.
Imported progress is excluded: the fix records work actually executed during
this route and does not infer historical work from a character's imported log.
The existing global gathering cap remains an approximation for arbitrary mixed
legacy work; this correction does not claim a complete overlap-attribution redesign.

The coordinator reports fourteen focused tests green. Independent review
approved the patch and its oracles with no Critical, Important, or Minor findings.
That review is source inspection, not an independent test rerun. Original-
semantics captures finished before accounting edits, preserving the causal
diagnostic evidence.

## Broad final verification

Final fast-profile and release package checks passed: **321 library tests and
238 integration tests**, plus **eight diagnostic-example tests and three existing
comparison-example tests** in release. Format and all-target release Clippy
with warnings denied passed. The known Windows fgstring newline test was
explicitly skipped; two documentation examples remain ignored and are not
counted as passes.

The broader check exposed a brittle profession-test assumption: the first
Leatherworking visit legitimately learned apprentice and journeyman together,
exporting cap 150 with an initial-learning instruction. That behavior predates
this patch. The integration assertion now requires the genuine initial-learning
instruction, retaining its other cap, rank-order, location and on/off checks.
A focused real level-gate regression verifies both learned bits, one charged
lesson and the combined-cap export. Both tests passed and received a scoped
independent review; profession-training production behavior was not changed.

The archived compact inputs and fixed snapshots also passed a standalone
rescore smoke check. Separate reviews approved the observer, diagnostic tool,
trace attribution and accounting correction. None of these checks establishes
a new algorithm ranking or fixes the separate training-visit preservation issue.

Final review also corrected a diagnostic-metadata omission: explicit executed
dungeon stops now supplement implicit entry events, deduplicated by first
observation. Two real-route regressions demonstrated the omission before the
fix and passed afterwards. Fresh fixed-route rescores retain all original
scores, player times, stops and traces; only the derived dungeon-identity lists
are corrected. Original raw captures remain immutable, with correction and hash
provenance recorded in the compact artifacts.

## Corrected fixed-route rescores

The corrected release diagnostic rescored all eight saved chain stages and all
eight baseline stages without searching. **All sixteen scores, player times,
typed stop vectors, and full traces are exactly unchanged** from their original-
semantics counterparts. Configuration and fixed-route identity checks passed.
The stage table above therefore also describes the corrected fixed routes.

The outputs identify `cleanup_loss_snapshot_rescore`, with
`search_performed=false` and `callback_comparison_performed=false`. They verify
the existing routes' accounting, not a regenerated search trajectory, timing
result, or new algorithm-quality benchmark. Together with the traces showing
no performed work for the abandoned quests, the unchanged results establish
that the confirmed accounting bug is separate from this captured cleanup loss.

The older timed comparison remains historical evidence under its original
model. These fixed-route checks do not rerank algorithms under corrected
accounting or show how that correction affects other search trajectories.

Autonomous CLI regeneration required no manual app operation. The installed
application session and saved guides were untouched. This diagnosis makes no
new algorithm-win, timed-speed, production-default, installer, or deployment claim.

## Final review and source

Source `09752b6` adds the observer/reproduction tools and corrects abandoned-work
accounting; `df96a30` fixes explicit-dungeon diagnostic reporting. The final
whole-branch review found no production observer or accounting defect. Its one
Important metadata finding was fixed, demonstrated with two real trace
regressions, and re-reviewed as addressed with no new material issue. Independent
trace review also approved the bounded training-loss explanation, without
claiming an unperformed restored-training counterfactual.

The [evidence index](cleanup-diagnosis-2026-11-07/README.md) preserves inputs,
snapshots, source/binary identities, original raw hashes, metadata correction
provenance and reviews. Full raw traces remain available locally in the ignored
diagnostic workspace. Useful training-visit preservation is still the next
implementation step; this completed phase supplies its concrete reproducer.
