# Independent experimental-quality review

Reviewed 2026-11-07 after timed execution stopped. Reviewer did not author the plan, operators, runner, or report. No planner runs, compilation, source changes, new tuning, or held-out selection were performed. Review used the frozen protocol, candidate artifact, analysis `20261107T094650766713`, audit-final, complete raw JSONL, result hashes, paired CSV, anytime CSV, and runner selection/aggregation logic.

## Disposition

**Approve the bounded no-promotion conclusion from the experimental evidence.** No Critical or Important evidence/analysis defect found. The final report was still a placeholder on the first read; final prose review remains pending separately from this evidence approval.

The selected chain implementation did not satisfy the frozen practical-win criterion at this single-start, 20-second allowance. This is a failed promotion test on the specified panel, not proof of an adverse population-average effect, baseline optimality, or inferiority of every chain algorithm. Hub and exact did not satisfy all tuning gates and were not tested on held-out seeds, so neither receives an independent held-out conclusion.

## Independently verified evidence

- Complete unique run inventory: eight pilots, 64 tuning runs, 32 held-out runs; 104 total and sixteen held-out baseline/chain pairs. Every stored result-file SHA-256 matches its record.
- Frozen protocol hash matches `candidate.json`; the original first-eight-line JSONL prefix matches the frozen pilot hash; the first-72-line prefix matches the candidate's tuning hash. Thus the candidate evidence contains pilot plus tuning and no held-out rows. Candidate analyses contain tuning results only.
- Protocol file modification time is 09:16:42.778599 UTC; first tuning process inferred start is 09:16:43.254642. Candidate file modification time is 09:35:31.405988; first held-out process inferred start is 09:36:12.211977. Runner code refuses candidate freezing after any held-out record and writes new artifacts without replacement. This is internally consistent evidence of pre-held-out selection, not an external tamper-proof timestamp service.
- Selection chose the feasible arm with the lowest tuning ratio: chain 0.977020855722068, exact 0.9832303877947716, hub 0.9976118770024527. Practical win and basic feasibility are correctly distinguished: chain's 5.395765% worst tuning regression already failed the 2% gate, but did not make it execution-infeasible.
- Recomputed all aggregate statistics directly from raw records rather than importing runner aggregation. For each profile, geometrically average its two candidate/baseline ratios, then geometrically average the eight profile ratios. Positive-profile counts use mean absolute score improvement across the two seeds. Both implement the stated equal-profile design. The equal number of seeds happens to make the score aggregate equal to a geometric mean of all sixteen ratios, but the independent unit remains the profile.

| Phase / arm | Score gain | Player-time change | Positive profiles | Worst score regression | Win / tie / loss |
| --- | ---: | ---: | ---: | ---: | ---: |
| Tuning / chain | +2.297914% | -1.179006% | 7/8 | +5.395765% | 10 / 0 / 6 |
| Tuning / hub | +0.238812% | +0.112224% | 3/8 | +3.415577% | 7 / 0 / 9 |
| Tuning / exact | +1.676961% | -0.698252% | 5/8 | +3.414389% | 10 / 0 / 6 |
| Held-out / chain | -0.456956% | -1.273725% | 4/8 | +30.994428% | 9 / 0 / 7 |

- Held-out median absolute score improvement is 3,000.694619 weighted seconds; median pair percentage improvement is 1.653390%. Neither median nor 9/16 wins overrides the aggregate, coverage, and worst-case gates. The worst regression is Forever Gnome Mage 20–40 seed 20261107, not an omitted or invalid run.
- All 104 records have successful process exit, no kill, observed peak memory, valid final simulation, required validation passed, no recorded failure, computation at most 20.25 seconds, and memory below 1.5 GiB. Observed computation spans 17.104364–17.631988 seconds; peak memory spans 278.992188–348.132813 MiB. No within-phase process overlap is implied by recorded completion timestamps minus measured parent wall times; smallest within-phase gap exceeds 17 milliseconds. These artifacts do not independently prove absence of unrelated background machine activity.
- All 31,657 anytime CSV observations exactly match raw records in stage, time, score, and player time. Each of the 104 authoritative-final endpoints matches its result's score and player time and lies inside recorded computation time. Raw search callbacks and final delivered routes must remain explicitly distinct.

## Interpretation requirements and limitations

These are scope qualifications, not reasons to alter the frozen selection or discard unfavorable results:

1. Primary weighted optimizer score is worse by 0.456956% while player time is better by 1.273725%; the two endpoints must not be interchanged. The worst pair has 30.994428% weighted-score regression but only 2.931019% longer player time, with 48,384.70 additional hidden weighted seconds.
2. All 104 requests have `required_quest_count=0`. Passing hard required-content validation therefore does not empirically validate forced-content behavior. There are baseline soft omissions: 28 runs miss the same two class quests, and one baseline run has a late-timed warning. No candidate adds warning IDs or missing-content counts in the paired comparisons. Execution-valid does not mean warning-free or demonstrated coverage of every constraint mode.
3. Eight deliberately selected profiles and two held-out seeds each support descriptive, panel-specific conclusions. No confidence level, population significance claim, or broad algorithm ranking is justified. More seeds would estimate stochastic variability; new profiles would be needed for broader coverage. Neither is necessary to reject promotion under the already-failed frozen practical gates.
4. Full final-output comparisons remain the prespecified endpoint. Large callback-to-final losses are important product-quality evidence, but shared finalization can interact with the route produced by each arm. Endpoint differences alone do not establish the responsible cleanup subpass, an operator defect, or a simulator defect. Exact per-pass causal claims require additional diagnostic evidence and must not retroactively replace these final scores.
5. Common single-start kernels, one worker, BiS off, the frozen greedy bootstrap, fixed operator frequencies, and this budget do not represent the production tournament, groups, professions, forced content, every class, vendor/BiS routing, other budgets, or every implementation of the algorithm families. No default change or elimination of other research directions follows.

## Findings

- Critical: none in the measured evidence or aggregation.
- Important: none in the measured evidence or aggregation. Final report prose still pending at initial review.
- Minor: no additional analysis is required for the bounded no-promotion decision. Preserve the explicit limitations above and avoid presenting positive medians/player-time changes as a weighted-score victory.

## Final prose review

Read the completed `docs/route-algorithm-comparison-2026-11-07.md`, archived cleanup analysis, and artifact README after the report author populated the results. **Approve the completed report's evidence and bounded conclusion; no Critical or Important prose finding.** The initial pending-prose disposition above is superseded by this review.

Programmatically compared all 32 displayed held-out table rows (sixteen weighted-score rows and sixteen player-time rows) with the paired CSV, including both endpoints and rounded percentage gains: no mismatches. Verified that archived protocol, candidate, analysis, audit, paired CSV, and anytime CSV are byte-identical to the reviewed originals. Independently rechecked callback-to-final ranges/medians and the worst-pair raw best and rounded post-cleanup logs. The cleanup broad-stage localization is supported; the report correctly preserves uncertainty about the individual subpass and replay consistency.

The report correctly separates score from player time, acknowledges zero explicit hard-required quests, limits hub/exact to tuning observations, states that production-tournament validation was not performed, preserves unfavorable pairs, and makes no statistical significance or universal algorithm claim. The description of the positive-profile criterion as arithmetic mean **absolute score savings** matches the frozen runner: every complete pair has `score_delta`, so the `1 - gain_ratio` expression in `dict.get` is only an unused fallback. No correction to relative-gain wording is needed.

Two nonblocking Minor presentation observations were sent to the coordinator:

1. The plan requests anytime curves. The report preserves all curve data and gives endpoint summaries, but does not yet display a curve artifact. A small linked plot would satisfy the literal presentation requirement without changing any analysis or conclusion.
2. The artifact README describes paired CSV as containing resource metrics, whereas its columns contain scores, player times, paired ratios, usability, and new-failure flags. Resource metrics are available in the analysis/audit; the README should name the actual CSV fields.

These observations do not undermine the measured decision. No additional planner run or parameter selection is requested or warranted by this review.

## Scoped final re-review: both Minor findings addressed

Re-read the corrected artifact README, new `plot_anytime.py`, and figure caption/links in the report; visually inspected `anytime-comparison.png`. **Both Minor findings are addressed. Final evidence and conclusion approval stands, with no remaining finding from this review.**

The README now accurately lists paired score/player measurements, differences, ratios, usability and new-failure flags, and directs readers to the analysis/audit for resource measurements. The report and README now link actual PNG/SVG curves and the reproducible Matplotlib generator.

The generator validates all 31,657 input rows, phase/arm coverage, eight profiles with the two declared seeds per arm, common paired initial scores, monotone search callbacks, and separate authoritative final endpoints. It carries search scores forward on the declared 4–17 second grid, normalizes each run to its paired common bootstrap, and aggregates sixteen balanced runs geometrically. Final diamonds use separately aggregated authoritative scores at median actual computation time, as explicitly described. This preserves equal profile weight and does not substitute callbacks for the measured endpoint. No new material plot issue found.

The rendered two-panel figure is readable, uses distinct line styles and colors, visibly separates final diamonds from raw curves, and has no clipped labels. The held-out chart correctly shows chain's better raw-search curve and worse delivered endpoint. No new experiment, retuning, or broader conclusion was introduced.
