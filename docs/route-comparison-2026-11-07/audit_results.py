#!/usr/bin/env python3
"""Independent, read-only accounting audit; never selects or tunes a candidate.

Run only after timed phases stop:
  python -B .superpowers/sdd/2026-11-07-route-algorithm-comparison/audit_results.py --timing-stopped --through-phase tuning
  python -B .superpowers/sdd/2026-11-07-route-algorithm-comparison/audit_results.py --timing-stopped --through-phase heldout

Uses only Python's standard library. JSON is printed to stdout; optional
--json-output creates a NEW supplementary file, never overwriting artifacts.
This file is deliberately outside the frozen implementation/source identity.
"""

import argparse
from collections import Counter, defaultdict
import hashlib
import json
import math
from pathlib import Path
import re
import statistics
import sys
import tempfile


ARMS = ("baseline", "chain", "hub", "exact")
CATEGORIES = ("missing_class", "missing_zone", "late_timed")
MISSING_COUNTS = ("missing_class_quests", "missing_zone_quests", "missing_dungeons", "missing_profession_quests")
STAGES = ("data", "prepare", "construction", "search_including_cleanup", "finalize", "exact_replay", "validate", "export")
TRIES = re.compile(r"\((\d+) tries, (\d+) improvements\)")
TOLERANCE = 1e-6


def sha(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def load(path):
    return json.loads(path.read_text(encoding="utf-8"))


def finite(value, positive=False):
    return type(value) in (int, float) and math.isfinite(value) and (value > 0 if positive else value >= 0)


def same_number(left, right):
    return type(left) in (int, float) and type(right) in (int, float) and math.isclose(left, right, rel_tol=1e-12, abs_tol=TOLERANCE)


def distribution(values):
    values = [value for value in values if type(value) in (int, float) and math.isfinite(value)]
    return {"n": len(values), "min": min(values), "median": statistics.median(values), "max": max(values)} if values else {"n": 0}


def geometric(values):
    values = list(values)
    return math.exp(statistics.mean(math.log(value) for value in values)) if values else None


def safe_json(value):
    """Keep invalid input numbers reportable without emitting nonstandard JSON."""
    if isinstance(value, dict):
        return {key: safe_json(item) for key, item in value.items()}
    if isinstance(value, (list, tuple)):
        return [safe_json(item) for item in value]
    if type(value) is float and not math.isfinite(value):
        return repr(value)
    return value


def nested_differences(expected, actual, prefix=""):
    if isinstance(expected, dict) and isinstance(actual, dict):
        output = []
        for key in sorted(set(expected) | set(actual)):
            name = f"{prefix}.{key}" if prefix else key
            if key not in expected or key not in actual:
                output.append({"field": name, "expected": expected.get(key), "actual": actual.get(key)})
            else:
                output.extend(nested_differences(expected[key], actual[key], name))
        return output
    return [] if expected == actual else [{"field": prefix, "expected": expected, "actual": actual}]


def audit(output, repo, through_phase):
    issues = []

    def issue(code, run=None, **details):
        issues.append({"code": code, **({"run": run} if run is not None else {}), **details})

    def read_json(path, run=None):
        try:
            return load(path)
        except (OSError, ValueError) as error:
            issue("unreadable_json", run, path=str(path), detail=str(error))
            return None

    protocol = read_json(output / "protocol.json")
    candidate_path = output / "candidate.json"
    candidate = read_json(candidate_path) if candidate_path.exists() or through_phase == "heldout" else None
    if not isinstance(protocol, dict) or (through_phase == "heldout" and not isinstance(candidate, dict)):
        return {"audit_schema": 1, "ok": False, "issues": issues}
    profiles = protocol["identity"]["requests"]
    profile_index = {profile["id"]: profile for profile in profiles}
    candidate_arm = candidate.get("arm") if isinstance(candidate, dict) else None
    if candidate is not None and candidate_arm not in ARMS[1:]:
        issue("invalid_frozen_candidate", actual=candidate_arm)
    if candidate is not None and candidate.get("protocol_sha256") != sha(output / "protocol.json"):
        issue("candidate_protocol_sha256_mismatch")
    if len(profile_index) != 8 or len(profiles) != 8:
        issue("profile_panel_count", actual=len(profiles), unique=len(profile_index))

    expected = {"pilot": [], "tuning": list(protocol["tuning_order"]), "heldout": []}
    for p, profile_id in enumerate(("classic-human-warrior-1-30", "forever-undead-mage-1-40")):
        profile = profile_index.get(profile_id)
        if profile is None:
            issue("pilot_profile_missing", profile=profile_id)
            continue
        arms = ARMS[p:] + ARMS[:p]
        expected["pilot"].extend({"profile": profile_id, "edition": profile["edition"], "request": profile["request"], "seed": 42, "arm": arm} for arm in arms)
    for p, profile in enumerate(profiles):
        for s, seed in enumerate(protocol["heldout_seeds"]):
            arms = ("baseline", candidate_arm) if (p + s) % 2 == 0 else (candidate_arm, "baseline")
            expected["heldout"].extend({"profile": profile["id"], "edition": profile["edition"], "request": profile["request"], "seed": seed, "arm": arm} for arm in arms)

    # Derive tuning membership independently of the frozen order, then compare both.
    independent_tuning = {(profile["id"], seed, arm) for profile in profiles for seed in (42, 1337) for arm in ARMS}
    frozen_tuning = [(run["profile"], run["seed"], run["arm"]) for run in expected["tuning"]]
    if len(frozen_tuning) != 64 or set(frozen_tuning) != independent_tuning:
        issue("frozen_tuning_membership_invalid")
    phase_ranks = {"pilot": 0, "tuning": 1, "heldout": 2}
    expected = {phase: schedule for phase, schedule in expected.items() if phase_ranks[phase] <= phase_ranks[through_phase]}

    rows = []
    prefix = hashlib.sha256()
    prefix_at_boundary = {}
    seen_phase = None
    last_phase_rank = -1
    record_path = output / "runs.jsonl"
    try:
        with record_path.open("rb") as stream:
            for line_number, raw in enumerate(stream, 1):
                if not raw.strip():
                    issue("blank_record_line", line=line_number)
                    prefix.update(raw)
                    continue
                try:
                    row = json.loads(raw)
                except ValueError as error:
                    issue("invalid_record_json", line=line_number, detail=str(error))
                    prefix.update(raw)
                    continue
                phase = row.get("phase")
                if phase != seen_phase:
                    if seen_phase is not None:
                        prefix_at_boundary[seen_phase] = prefix.hexdigest()
                    rank = phase_ranks.get(phase, -1)
                    if rank <= last_phase_rank:
                        issue("phase_order_invalid", row.get("id"), previous=seen_phase, actual=phase)
                    last_phase_rank = rank
                    seen_phase = phase
                prefix.update(raw)
                rows.append(row)
        if seen_phase is not None:
            prefix_at_boundary[seen_phase] = prefix.hexdigest()
    except OSError as error:
        issue("unreadable_records", detail=str(error))
    if prefix_at_boundary.get("pilot") != protocol.get("pilot_result_sha256"):
        issue("pilot_prefix_sha256_mismatch", actual=prefix_at_boundary.get("pilot"))
    if candidate is not None and prefix_at_boundary.get("tuning") != candidate.get("tuning_sha256"):
        issue("candidate_tuning_prefix_sha256_mismatch", actual=prefix_at_boundary.get("tuning"))

    counts = Counter(row.get("phase") for row in rows)
    keys = [(row.get("phase"), row.get("profile"), row.get("seed"), row.get("arm")) for row in rows]
    for key, count in Counter(keys).items():
        if count != 1:
            issue("duplicate_run_key", key=list(key), count=count)
    for run_id, count in Counter(row.get("id") for row in rows).items():
        if count != 1:
            issue("duplicate_run_id", run_id, count=count)
    for phase, schedule in expected.items():
        phase_rows = [row for row in rows if row.get("phase") == phase]
        if len(phase_rows) != len(schedule):
            issue("phase_count", phase=phase, expected=len(schedule), actual=len(phase_rows))
        for ordinal, run in enumerate(schedule):
            matching = [row for row in phase_rows if row.get("ordinal") == ordinal]
            if len(matching) != 1:
                issue("ordinal_count", phase=phase, ordinal=ordinal, actual=len(matching))
                continue
            row = matching[0]
            differences = nested_differences(run, {key: row.get(key) for key in run})
            if differences:
                issue("schedule_mismatch", row.get("id"), differences=differences)
            expected_id = f"{phase}-{ordinal:03d}-{run['profile']}-{run['seed']}-{run['arm']}"
            if row.get("id") != expected_id:
                issue("run_id_mismatch", row.get("id"), expected=expected_id)
        extras = [row.get("id") for row in phase_rows if type(row.get("ordinal")) is not int or not 0 <= row["ordinal"] < len(schedule)]
        if extras:
            issue("unexpected_ordinals", phase=phase, runs=extras)
    for row in rows:
        if row.get("phase") not in expected:
            issue("unexpected_phase", row.get("id"), actual=row.get("phase"))

    checks = Counter()
    recorded_failures = []
    derived_failures = []
    groups = defaultdict(list)
    per_arm = defaultdict(lambda: defaultdict(list))
    for row in rows:
        run_id = row.get("id", "unknown")
        result = row.get("result")
        group_name = f"{row.get('phase')}/{row.get('arm')}"
        metrics = per_arm[group_name]
        for field in ("parent_wall_seconds", "peak_memory_bytes"):
            metrics[field].append(row.get(field))
        if row.get("failures"):
            recorded_failures.append({"run": run_id, "failures": row["failures"]})
        failure = []
        if row.get("killed"):
            failure.append(row["killed"])
        if row.get("returncode") != 0:
            failure.append("process_failure")
        if not row.get("memory_observed"):
            failure.append("memory_unobserved")
        memory = row.get("peak_memory_bytes")
        if not finite(memory, positive=True):
            issue("invalid_peak_memory", run_id, actual=memory)
        elif memory > protocol["peak_memory_limit_bytes"]:
            failure.append("memory_limit")
        parent_wall = row.get("parent_wall_seconds")
        if not finite(parent_wall, positive=True):
            issue("invalid_parent_wall", run_id, actual=parent_wall)
        elif parent_wall > protocol["budget_seconds"] + protocol["parent_timeout_extra_seconds"] + 0.1:
            issue("parent_wall_above_timeout", run_id, actual=parent_wall)

        relative = Path(row.get("artifact", ""))
        artifact = (output / relative).resolve()
        if relative.is_absolute() or not artifact.is_relative_to(output):
            issue("artifact_path_outside_output", run_id, path=str(relative))
            failure.append("artifact_integrity")
        else:
            try:
                request_path = artifact.parent / "request.json"
                if sha(request_path) != row.get("request_sha256"):
                    issue("request_sha256_mismatch", run_id)
                    failure.append("artifact_integrity")
                else:
                    checks["request_sha256"] += 1
                request_value = read_json(request_path, run_id)
                if request_value != row.get("request"):
                    issue("request_value_mismatch", run_id)
                    failure.append("artifact_integrity")
                if artifact.is_file():
                    if sha(artifact) != row.get("result_sha256"):
                        issue("result_sha256_mismatch", run_id)
                        failure.append("artifact_integrity")
                    else:
                        checks["result_sha256"] += 1
                    artifact_value = read_json(artifact, run_id)
                    if artifact_value != result:
                        issue("embedded_result_differs_from_artifact", run_id)
                        failure.append("artifact_integrity")
                    else:
                        checks["embedded_result_matches_artifact"] += 1
                    sidecar = read_json(artifact.with_suffix(".timing.json"), run_id)
                    if sidecar != row.get("serialization"):
                        issue("timing_sidecar_mismatch", run_id)
                    else:
                        checks["timing_sidecar"] += 1
                elif result is not None or row.get("result_sha256") is not None:
                    issue("missing_result_artifact", run_id)
                    failure.append("artifact_integrity")
            except OSError as error:
                issue("unreadable_artifact", run_id, detail=str(error))
                failure.append("artifact_integrity")

        if not isinstance(result, dict):
            failure.append("missing_result")
            row["_audit_failures"] = sorted(set(failure))
            derived_failures.append({"run": run_id, "failures": row["_audit_failures"]})
            continue
        if result.get("schema") != 1 or result.get("experiment") != protocol.get("experiment"):
            issue("result_schema_or_experiment", run_id)
        for field in ("arm", "seed"):
            if result.get(field) != row.get(field):
                issue("result_identity_mismatch", run_id, field=field, actual=result.get(field), expected=row.get(field))
        if str(result.get("edition")).lower() != row.get("edition"):
            issue("result_edition_mismatch", run_id, actual=result.get("edition"))
        for field in ("budget_seconds", "reserve_seconds"):
            if result.get(field) != protocol[field]:
                issue("result_budget_setting_mismatch", run_id, field=field, actual=result.get(field), expected=protocol[field])
        if result.get("valid") is not True or result.get("required_validation") != "passed":
            failure.append("invalid")
        for field in ("score", "player_seconds", "compute_seconds"):
            value = result.get(field)
            metrics[field].append(value)
            if not finite(value, positive=True):
                failure.append("nonpositive_or_nonfinite_metric")
                issue("invalid_metric", run_id, field=field, actual=value)
        compute = result.get("compute_seconds")
        if finite(compute):
            violation = compute > protocol["budget_seconds"] + protocol["compute_tolerance_seconds"]
            if result.get("budget_violation") != violation:
                issue("budget_flag_mismatch", run_id, expected=violation, actual=result.get("budget_violation"))
            if violation:
                failure.append("compute_budget")
            if finite(parent_wall) and parent_wall + TOLERANCE < compute:
                issue("parent_wall_less_than_compute", run_id)
            if finite(parent_wall):
                metrics["parent_minus_compute_seconds"].append(parent_wall - compute)
        breakdown = result.get("breakdown", {})
        if not same_number(breakdown.get("total"), result.get("player_seconds")):
            issue("breakdown_player_mismatch", run_id, total=breakdown.get("total"), player=result.get("player_seconds"))
        if finite(result.get("score")) and finite(result.get("player_seconds")):
            if not same_number(result.get("hidden_penalty_seconds"), result["score"] - result["player_seconds"]):
                issue("hidden_penalty_mismatch", run_id)
        for field in ("flight_gold", "bag_overflow", "unlock_delay"):
            if field not in breakdown:
                issue("omitted_hidden_breakdown_field", run_id, field=field)
        money_fields = ("gold_start", "gold_earned", "flight_cost", "training_cost", "vendor_cost")
        if all(type(breakdown.get(field)) is int for field in money_fields):
            expected_copper = breakdown["gold_start"] + breakdown["gold_earned"] - sum(breakdown[field] for field in money_fields[2:])
            if result.get("copper_end") != expected_copper:
                issue("copper_balance_mismatch", run_id)
        for category in CATEGORIES:
            texts = result.get("warnings", {}).get(category, [])
            ids = result.get("warning_quest_ids", {}).get(category, [])
            if len(texts) != len(ids) or any(type(quest_id) is not int for quest_id in ids):
                failure.append("warning_identity_missing")
                issue("warning_identity_missing", run_id, category=category)
            metrics[f"warning_count/{category}"].append(len(ids))

        stages = result.get("stages_seconds", {})
        for stage in STAGES:
            value = stages.get(stage)
            metrics[f"stage/{stage}"].append(value)
            if not finite(value):
                issue("invalid_stage", run_id, stage=stage, actual=value)
        if all(finite(stages.get(stage)) for stage in STAGES) and finite(compute):
            residual = compute - sum(stages[stage] for stage in STAGES)
            metrics["compute_minus_stage_sum_seconds"].append(residual)
            if residual < -0.0001:
                issue("stage_sum_above_compute", run_id, residual=residual)
        serialization = row.get("serialization") or {}
        for field in ("serialization_write_seconds", "process_elapsed_seconds"):
            value = serialization.get(field)
            metrics[field].append(value)
            if not finite(value):
                issue("invalid_serialization_timing", run_id, field=field, actual=value)
        process_elapsed = serialization.get("process_elapsed_seconds")
        if finite(process_elapsed) and finite(compute):
            if process_elapsed + TOLERANCE < compute:
                issue("process_elapsed_less_than_compute", run_id)
            if finite(parent_wall) and parent_wall + 0.05 < process_elapsed:
                issue("parent_wall_less_than_process_elapsed", run_id)
            write_time = serialization.get("serialization_write_seconds")
            if finite(write_time):
                metrics["process_minus_compute_and_write_seconds"].append(process_elapsed - compute - write_time)

        tries = [TRIES.search(line) for line in result.get("search_log", [])]
        tries = [match for match in tries if match is not None]
        if len(tries) != 1:
            issue("search_tries_log_count", run_id, actual=len(tries))
        else:
            row["_tries"] = int(tries[0].group(1))
            metrics["search_tries"].append(row["_tries"])
            metrics["search_logged_improvements"].append(int(tries[0].group(2)))
        points = result.get("anytime", [])
        search_points = [point for point in points if point.get("stage") == "search"]
        final_points = [point for point in points if point.get("stage") == "authoritative_final"]
        if not search_points or len(final_points) != 1 or not points or points[-1].get("stage") != "authoritative_final":
            issue("anytime_endpoint_structure", run_id, search_points=len(search_points), final_points=len(final_points))
        previous_time = -math.inf
        previous_best = math.inf
        for point in points:
            if any(not finite(point.get(field)) for field in ("seconds", "score", "player_seconds")):
                issue("invalid_anytime_point", run_id)
                continue
            if point["seconds"] < previous_time or (finite(compute) and point["seconds"] > compute + TOLERANCE):
                issue("anytime_time_order", run_id)
            previous_time = point["seconds"]
            if point.get("stage") == "search":
                if point["score"] > previous_best + TOLERANCE:
                    issue("search_anytime_score_increase", run_id)
                previous_best = min(previous_best, point["score"])
        if final_points:
            final = final_points[0]
            for point_field, result_field in (("seconds", "compute_seconds"), ("score", "score"), ("player_seconds", "player_seconds")):
                if not same_number(final.get(point_field), result.get(result_field)):
                    issue("authoritative_final_mismatch", run_id, field=point_field)
        if search_points and all(finite(point.get("score"), positive=True) for point in search_points) and finite(result.get("score")):
            row["_initial"] = search_points[0]
            row["_final_minus_best"] = result["score"] - min(point["score"] for point in search_points)
            metrics["final_minus_search_best_score"].append(row["_final_minus_best"])
            metrics["initial_minus_final_score"].append(search_points[0]["score"] - result["score"])
        effective_request = result.get("effective_request", {})
        for field, value in row.get("request", {}).items():
            if field != "params" and effective_request.get(field) != value:
                issue("effective_request_mismatch", run_id, field=field, expected=value, actual=effective_request.get(field))
        params = result.get("effective_params", {})
        if params.get("seed") != row.get("seed") or str(params.get("bis")).lower() != "off" or str(params.get("construction")).lower() != "greedy" or params.get("construction_random") is not False:
            issue("common_parameter_override_mismatch", run_id)
        row["_audit_failures"] = sorted(set(failure))
        if failure:
            derived_failures.append({"run": run_id, "failures": row["_audit_failures"]})
        recorded_core = set(row.get("failures", []))
        derived_core = set(failure) - {"artifact_integrity"}
        if recorded_core != derived_core:
            issue("recorded_failure_list_differs", run_id, recorded=sorted(recorded_core), derived=sorted(derived_core))
        groups[(row.get("phase"), row.get("profile"), row.get("seed"))].append(row)

    initial_checks = 0
    parameter_checks = 0
    warning_diffs = []
    paired = defaultdict(list)
    for (phase, profile, seed), group in groups.items():
        baselines = [row for row in group if row.get("arm") == "baseline"]
        if len(baselines) != 1:
            issue("comparison_baseline_count", phase=phase, profile=profile, seed=seed, actual=len(baselines))
            continue
        baseline = baselines[0]
        base_result = baseline["result"]
        for row in group:
            if row is baseline:
                continue
            run_id = row.get("id")
            result = row["result"]
            first, base_first = row.get("_initial"), baseline.get("_initial")
            if first is None or base_first is None:
                issue("initial_callback_missing", run_id)
            else:
                initial_checks += 1
                for field in ("score", "player_seconds", "stops"):
                    if not same_number(first.get(field), base_first.get(field)):
                        issue("initial_callback_differs", run_id, field=field, baseline=base_first.get(field), candidate=first.get(field))
            expected_params = dict(base_result.get("effective_params", {}))
            expected_params.update(protocol["arms"].get(row.get("arm"), {}))
            differences = nested_differences(expected_params, result.get("effective_params", {}))
            parameter_checks += 1
            if differences:
                issue("undeclared_effective_parameter_difference", run_id, differences=differences)
            expected_request_params = dict(base_result.get("effective_request", {}).get("params", {}))
            expected_request_params.update(protocol["arms"].get(row.get("arm"), {}))
            request_differences = nested_differences(expected_request_params, result.get("effective_request", {}).get("params", {}))
            if request_differences:
                issue("undeclared_effective_request_parameter_difference", run_id, differences=request_differences)
            if result.get("required_quest_count") != base_result.get("required_quest_count"):
                issue("required_quest_count_difference", run_id)
            added = {}
            removed = {}
            for category in CATEGORIES:
                base_ids = set(base_result.get("warning_quest_ids", {}).get(category, []))
                ids = set(result.get("warning_quest_ids", {}).get(category, []))
                if ids - base_ids:
                    added[category] = sorted(ids - base_ids)
                if base_ids - ids:
                    removed[category] = sorted(base_ids - ids)
            missing_increases = {field: {"baseline": base_result["breakdown"].get(field, 0), "candidate": result["breakdown"].get(field, 0)} for field in MISSING_COUNTS if result["breakdown"].get(field, 0) > base_result["breakdown"].get(field, 0)}
            if added or removed or missing_increases:
                warning_diffs.append({"run": run_id, "phase": phase, "profile": profile, "seed": seed, "arm": row.get("arm"), "added_ids": added, "removed_ids": removed, "missing_count_increases": missing_increases})
            if phase in ("tuning", "heldout") and all(finite(value, positive=True) for value in (base_result.get("score"), result.get("score"), base_result.get("player_seconds"), result.get("player_seconds"))):
                paired[(phase, row.get("arm"))].append({"profile": profile, "seed": seed, "score_ratio": result["score"] / base_result["score"], "player_ratio": result["player_seconds"] / base_result["player_seconds"], "score_delta": base_result["score"] - result["score"], "new_warning_or_missing": bool(added or missing_increases), "run_failure": bool(row.get("_audit_failures") or baseline.get("_audit_failures"))})

    paired_statistics = {}
    for (phase, arm), pairs in paired.items():
        by_profile = defaultdict(list)
        for pair in pairs:
            by_profile[pair["profile"]].append(pair)
        score_ratio = geometric(geometric(pair["score_ratio"] for pair in profile_pairs) for profile_pairs in by_profile.values())
        player_ratio = geometric(geometric(pair["player_ratio"] for pair in profile_pairs) for profile_pairs in by_profile.values())
        paired_statistics[f"{phase}/{arm}"] = {"pairs": len(pairs), "profiles": len(by_profile), "score_ratio": score_ratio, "player_ratio": player_ratio, "positive_profile_absolute_two_seed_means": sum(statistics.mean(pair["score_delta"] for pair in profile_pairs) > 0 for profile_pairs in by_profile.values()), "score_delta": distribution(pair["score_delta"] for pair in pairs), "worst_score_regression_ratio_minus_one": max(pair["score_ratio"] - 1 for pair in pairs), "win_tie_loss": {"win": sum(pair["score_delta"] > 0.5 for pair in pairs), "tie": sum(abs(pair["score_delta"]) <= 0.5 for pair in pairs), "loss": sum(pair["score_delta"] < -0.5 for pair in pairs)}, "pairs_with_new_warning_or_missing": sum(pair["new_warning_or_missing"] for pair in pairs), "pairs_with_run_failure": sum(pair["run_failure"] for pair in pairs), "per_profile": {profile: {"pairs": len(profile_pairs), "score_ratio": geometric(pair["score_ratio"] for pair in profile_pairs), "score_delta_mean": statistics.mean(pair["score_delta"] for pair in profile_pairs)} for profile, profile_pairs in by_profile.items()}}

    # Verify the small frozen executable and runner identities without rehashing large databases.
    for label, path, expected_digest in (
        ("binary", repo / "target" / "release" / "examples" / ("quality_compare.exe" if sys.platform == "win32" else "quality_compare"), protocol["identity"]["binary_sha256"]),
        ("runner", repo / "scripts" / "route_quality_compare.py", protocol["identity"]["runner_sha256"]),
    ):
        try:
            if sha(path) != expected_digest:
                issue("current_frozen_file_sha256_mismatch", file=label)
            else:
                checks[f"current_{label}_sha256"] += 1
        except OSError as error:
            issue("unreadable_frozen_file", file=label, detail=str(error))

    cleanup_changes = []
    for row in rows:
        delta = row.get("_final_minus_best")
        if delta is not None and abs(delta) > TOLERANCE:
            cleanup_changes.append({"run": row.get("id"), "final_minus_search_best_score": delta})
    return {"audit_schema": 1, "purpose": "independent identity/accounting audit; no candidate selection or retuning", "through_phase": through_phase, "ok": not issues and not derived_failures, "candidate_frozen": candidate is not None, "frozen_candidate": candidate_arm, "counts": {"total": len(rows), "phases": dict(counts), "phase_arms": dict(Counter(f"{row.get('phase')}/{row.get('arm')}" for row in rows)), "expected_phases": {phase: len(schedule) for phase, schedule in expected.items()}}, "integrity_checks_passed": dict(checks), "initial_callback_comparisons": initial_checks, "effective_parameter_comparisons": parameter_checks, "issues": issues, "recorded_failures": recorded_failures, "independently_derived_failures": derived_failures, "ranges_and_medians_by_phase_arm": {group: {field: distribution(values) for field, values in metrics.items()} for group, metrics in sorted(per_arm.items())}, "warning_id_differences": warning_diffs, "cleanup_vs_search_best": {"changed_runs": len(cleanup_changes), "worsened_runs": sum(change["final_minus_search_best_score"] > TOLERANCE for change in cleanup_changes), "improved_runs": sum(change["final_minus_search_best_score"] < -TOLERANCE for change in cleanup_changes), "changes": cleanup_changes}, "supplementary_paired_statistics": paired_statistics, "limitations": ["Initial callback scores are compared, not independently replayed from their unsaved routes.", "Current large databases and all source files are not rehashed here; the frozen runner checks full identity around each measured phase.", "Parent-memory observations use the recorded operating-system peak-working-set sampler.", "Pair statistics are descriptive and do not modify frozen selection or eligibility; listed failures remain visible."]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--timing-stopped", action="store_true", help="Required acknowledgement that timed phases have stopped.")
    parser.add_argument("--through-phase", choices=("pilot", "tuning", "heldout"), required=True, help="Audit all phases through this completed stage; later-phase rows are an error.")
    parser.add_argument("--output", type=Path, default=Path(__file__).resolve().parent / "results")
    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[3])
    parser.add_argument("--json-output", type=Path)
    args = parser.parse_args()
    if not args.timing_stopped:
        parser.error("do not inspect results during timing; pass --timing-stopped only after timed processes stop")
    repo = args.repo.resolve()
    lock = Path(tempfile.gettempdir()) / ("fg-quality-" + hashlib.sha256(str(repo).encode()).hexdigest()[:16] + ".lock")
    if lock.exists():
        parser.error(f"runner lock exists: {lock}; refusing to read result artifacts")
    try:
        summary = audit(args.output.resolve(), repo, args.through_phase)
    except (OSError, ValueError, TypeError, KeyError) as error:
        summary = {"audit_schema": 1, "ok": False, "fatal_audit_error": f"{type(error).__name__}: {error}"}
    encoded = json.dumps(safe_json(summary), sort_keys=True, separators=(",", ":"), allow_nan=False)
    if args.json_output is not None:
        with args.json_output.open("x", encoding="utf-8", newline="\n") as stream:
            stream.write(encoded + "\n")
    print(encoded)
    return 0 if summary.get("ok") else 1


if __name__ == "__main__":
    sys.exit(main())
