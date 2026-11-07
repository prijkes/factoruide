#!/usr/bin/env python3
"""Plot the frozen comparison CSV; no planners, tuning, or result rewrites.

With Matplotlib already installed:
    python -B docs/route-comparison-2026-11-07/plot_anytime.py

With the experiment's local plotting environment:
    python -B docs/route-comparison-2026-11-07/plot_anytime.py --plot-env .superpowers/sdd/2026-11-07-route-algorithm-comparison/plot-env
"""

import argparse
from bisect import bisect_right
from collections import defaultdict
import csv
import hashlib
import json
import math
import os
from pathlib import Path
import statistics
import sys


HERE = Path(__file__).resolve().parent
EXPERIMENT = HERE.parents[1] / ".superpowers/sdd/2026-11-07-route-algorithm-comparison"
PANELS = (("tuning", ("baseline", "chain", "hub", "exact")), ("heldout", ("baseline", "chain")))
STYLES = {
    "baseline": ("#0072B2", "-", "Baseline"),
    "chain": ("#D55E00", "--", "Chain"),
    "hub": ("#009E73", "-.", "Hub"),
    "exact": ("#CC79A7", ":", "Exact"),
}


def load_runs(path):
    runs = defaultdict(lambda: {"search": [], "final": []})
    row_count = 0
    with path.open(encoding="utf-8", newline="") as stream:
        reader = csv.DictReader(stream)
        assert set(reader.fieldnames) == {"phase", "profile", "seed", "arm", "compute_seconds", "score", "player_seconds", "stage"}, "Unexpected CSV schema"
        for row in reader:
            row_count += 1
            if row["phase"] not in ("tuning", "heldout"):
                continue
            key = (row["phase"], row["arm"], row["profile"], int(row["seed"]))
            point = (float(row["compute_seconds"]), float(row["score"]))
            assert all(math.isfinite(value) and value > 0 for value in point), key
            stage = {"search": "search", "authoritative_final": "final"}.get(row["stage"])
            assert stage is not None, (key, row["stage"])
            runs[key][stage].append(point)
    assert row_count == 31657, f"Expected all 31,657 canonical rows; found {row_count}"
    initial_scores = {}
    for key, run in runs.items():
        search = run["search"]
        assert search and len(run["final"]) == 1, key
        assert all(a[0] <= b[0] and a[1] + 1e-6 >= b[1] for a, b in zip(search, search[1:])), key
        assert search[0][0] <= 4 and run["final"][0][0] >= search[-1][0], key
        run["initial"] = search[0][1]
        run["times"] = [point[0] for point in search]
        bootstrap_key = (key[0], key[2], key[3])
        expected = initial_scores.setdefault(bootstrap_key, run["initial"])
        assert math.isclose(expected, run["initial"], rel_tol=1e-12, abs_tol=1e-6), key
    return runs, row_count


def normalized_summary(runs, phase, arm, grid):
    selected = [(key, run) for key, run in runs.items() if key[:2] == (phase, arm)]
    assert len(selected) == 16, f"Expected 16 runs for {phase}/{arm}, found {len(selected)}"
    profile_seeds = defaultdict(set)
    expected_seeds = {42, 1337} if phase == "tuning" else {20261107, 314159}
    for key, _ in selected:
        profile_seeds[key[2]].add(key[3])
    assert len(profile_seeds) == 8 and all(seeds == expected_seeds for seeds in profile_seeds.values()), (phase, arm)
    curve = []
    for seconds in grid:
        normalized = []
        for key, run in selected:
            index = bisect_right(run["times"], seconds) - 1
            assert index >= 0, (key, seconds)
            normalized.append(run["search"][index][1] / run["initial"])
        curve.append(100 * statistics.geometric_mean(normalized))
    final_time = statistics.median(run["final"][0][0] for _, run in selected)
    final_score = 100 * statistics.geometric_mean(run["final"][0][1] / run["initial"] for _, run in selected)
    return curve, final_time, final_score


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--csv", type=Path, default=HERE / "anytime.csv")
    parser.add_argument("--output-dir", type=Path, default=HERE)
    parser.add_argument("--plot-env", type=Path, help="Optional directory containing the Matplotlib installation.")
    parser.add_argument("--mpl-config", type=Path, default=EXPERIMENT / "mpl-config")
    args = parser.parse_args()
    sys.dont_write_bytecode = True
    if args.plot_env is not None:
        sys.path.insert(0, str(args.plot_env.resolve()))
    args.mpl_config.mkdir(parents=True, exist_ok=True)
    os.environ["MPLCONFIGDIR"] = str(args.mpl_config.resolve())
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    from matplotlib.lines import Line2D

    runs, row_count = load_runs(args.csv)
    grid = [4 + index * 0.05 for index in range(261)]
    plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 10,
                         "axes.titlesize": 12, "axes.labelsize": 10,
                         "xtick.labelsize": 9, "ytick.labelsize": 9,
                         "legend.fontsize": 9, "svg.fonttype": "none",
                         "svg.hashsalt": "factoruide-route-comparison-20261107"})
    fig, axes = plt.subplots(1, 2, figsize=(11.5, 4.8), sharex=True, sharey=True, layout="constrained")
    markers = {}
    for ax, (phase, arms) in zip(axes, PANELS):
        handles = []
        for arm in arms:
            color, linestyle, label = STYLES[arm]
            curve, final_time, final_score = normalized_summary(runs, phase, arm, grid)
            line, = ax.plot(grid, curve, color=color, linestyle=linestyle, linewidth=2, label=label)
            ax.plot(final_time, final_score, marker="D", markersize=7,
                    markerfacecolor=color, markeredgecolor="#222222", markeredgewidth=0.7,
                    linestyle="none", zorder=5)
            handles.append(line)
            markers[f"{phase}/{arm}"] = {"runs": 16, "median_final_compute_seconds": final_time,
                                          "final_percent_initial_score": final_score}
        handles.append(Line2D([], [], color="#555555", marker="D", linestyle="none",
                              markersize=6, label="Final delivered score"))
        title = "Tuning: 4 arms, seeds 42 / 1337" if phase == "tuning" else "Held-out: 2 arms, seeds 20261107 / 314159"
        ax.set_title(title, pad=10)
        ax.set_xlim(4, 18)
        ax.set_xticks(range(4, 19, 2))
        ax.grid(axis="both", color="#D8DDE3", linewidth=0.7, alpha=0.8)
        ax.set_axisbelow(True)
        ax.spines[["top", "right"]].set_visible(False)
        ax.legend(handles=handles, loc="upper right", frameon=True, framealpha=0.94)
    fig.suptitle("Search progress and delivered route scores · lower is better", fontsize=14)
    fig.supxlabel("Generation compute time (seconds; includes preparation)")
    fig.supylabel("Weighted score (% of common initial score)")
    args.output_dir.mkdir(parents=True, exist_ok=True)
    png = args.output_dir / "anytime-comparison.png"
    svg = args.output_dir / "anytime-comparison.svg"
    description = "Raw search-best curves normalized to each run's first search score, geometric mean of 16 runs per arm; diamonds are authoritative finalized scores at median actual computation time."
    fig.savefig(png, dpi=180, metadata={"Description": description})
    fig.savefig(svg, metadata={"Date": None, "Title": "Search progress and delivered route scores", "Description": description})
    plt.close(fig)
    print(json.dumps({"csv_rows": row_count, "csv_sha256": hashlib.sha256(args.csv.read_bytes()).hexdigest(),
                      "matplotlib_version": matplotlib.__version__, "grid_seconds": {"start": 4, "end": 17, "step": 0.05},
                      "final_markers": markers, "png": str(png), "svg": str(svg)}, sort_keys=True))


if __name__ == "__main__":
    main()
