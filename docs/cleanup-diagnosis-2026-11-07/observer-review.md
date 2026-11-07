# Cleanup instrumentation independent review

Assessment: **Approve with one Minor test coverage finding.** No Critical or Important findings in the reviewed production changes.

Reviewed the uncommitted changes against `e73af02` in `crates/fg-route/src/plan/search.rs`, `plan/mod.rs`, and `plan/tests/cleanup_observation.rs`, together with the implementation plan and `observer-report.md`. Diagnostic example and accounting tests are outside this review.

## Findings

- **Minor — stop-callback test does not execute its stop callback.** In `cleanup_observation.rs:74`, the initial route contains only two stops. The search condition at `search.rs:136` short-circuits on `search.route.len() > 2` before calling `keep_going`. The test therefore covers cleanup after a short route exits search, rather than cleanup after a caller stops attempts. Use two completed quests (four stops) and a `Cell` counter in the false-returning callback, then assert it was called once. This does not indicate a production defect: the relevant loop and abort check are unchanged.

## Source checks

- `improve_while` delegates to the same search body with a zero-capture no-op observer. The search loop, acceptance, RNG, sharing, initial simulation, logging, and best callbacks match the base implementation.
- The seven callbacks are unconditional at their respective cleanup boundaries, so unchanged passes remain visible.
- `SelectedInput` occurs after the existing current-versus-best threshold and any required best-route run. It observes the actual selected cleanup input, rather than assuming the last best callback always supplies that input.
- Rebind output is borrowed before being consumed by hearth placement. The comparison against the prior route and conditional run update occur only after the combined rebind/hearth result, exactly as before. No intermediate simulation is added.
- Remaining cleanup decisions, scores, run updates, ordering, and tolerances match the base implementation. The final snapshot is the returned route.
- Invalid initial routes and pre-cleanup aborts retain their earlier return paths and emit no cleanup callbacks. The patch introduces no new abort decisions.
- Instrumentation adds no RNG calls, route clones, simulations, or file I/O to the no-op path. Callback side effects are caller-controlled; the API documentation correctly directs snapshot scoring until after search completes.
- Equivalence tests use fresh ordinary planners and compare complete best-callback sequences and final route/score. The unchanged-boundary and abort tests check their stated behavior. A distinct current-versus-best selection case and invalid-input case are not directly exercised by these tests, but their source paths are preserved and the selected callback placement is correct.

## Verification limits

`git diff --check` passed for the reviewed source/test files. Per coordinator instructions, this reviewer ran no builds, tests, or planners and made no production/test edits. The coordinator supplied RED evidence (three expected failures and the abort control passing); GREEN verification was still ongoing at dispatch. Approval is a source/instrumentation review, not confirmation of test completion or exact archived reproduction. Those require the coordinator's fresh results and subsequent example/artifact review.
