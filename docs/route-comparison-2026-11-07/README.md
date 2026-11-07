# Route comparison artifacts

These files accompany the [measured comparison report](../route-algorithm-comparison-2026-11-07.md).

- [Frozen protocol](protocol.json): source, executable, data and request identities; budgets, order and decision criteria.
- [Frozen candidate selection](candidate.json): selection from tuning only, recorded before held-out execution.
- [Complete analysis](analysis.json): tuning and held-out paired measurements, including unfavorable pairs.
- [Independent integrity audit](audit-final.json): 104-run completeness, identities, bootstrap equivalence, accounting and resource checks.
- [Audit implementation](audit_results.py): independent standard-library audit; run after timing with explicit `--repo . --output .superpowers/sdd/2026-11-07-route-algorithm-comparison/results --timing-stopped --through-phase heldout`. It never chooses a candidate.
- [Cleanup investigation](cleanup-analysis.md): stage-localized score changes, penalty decomposition, and limits on causal attribution.
- [Review and verification record](reviews.md): independent code/evidence reviews, regression fixes, check counts and remaining limits.
- [Paired measurements](paired.csv): exact finalized scores, player times, paired differences and ratios, usability, and new-failure flags. Resource measurements are in the analysis and audit JSON files.
- [Anytime curve data](anytime.csv): search callbacks and authoritative final endpoints, with phase, profile, seed and arm labels. Finalization can worsen a search score; final endpoints must remain distinct from search-best curves.
- [Anytime figure, PNG](anytime-comparison.png) and [SVG](anytime-comparison.svg): tuning and held-out search progress, with authoritative finalized-score diamonds.
- [Figure generator](plot_anytime.py): Matplotlib source for both figure formats; no planner execution or retuning. Run `python -B docs/route-comparison-2026-11-07/plot_anytime.py` with Matplotlib installed, or add `--plot-env .superpowers/sdd/2026-11-07-route-algorithm-comparison/plot-env` for the local plotting environment. The default font/config cache stays in the ignored experiment directory.

The figure samples raw search-best scores on a common 4–17 second grid with last-known values. Each run is divided by its first search score; the curves show geometric means across sixteen runs per arm, equally covering eight profiles. Diamonds show authoritative delivered scores at each arm's median actual computation time. The vertical axis is percent of the common initial score, with lower values better; time includes preparation. Curves and final markers measure different stages and make no causal claim.

The JSON selection and analysis use compact references to the original per-run artifacts. Full traces, guides, process logs and append-only run records remain in `.superpowers/sdd/2026-11-07-route-algorithm-comparison/results` in the experiment workspace. They are excluded from Git because they duplicate approximately 116 MB of full run records plus individual artifacts. The executable and source/data hashes above identify the frozen experiment.

Timing is measured from input loading through export. Artifact serialization and process overhead are separate. The comparison covers a single-start kernel with BiS off; it does not establish superiority over the application's default tournament.
