# Task 2 diagnostic implementation review

Verdict: **Approved for the diagnostic-code scope.** No critical, important or minor findings.

Reviewed the task brief, implementation report, supplied review diff and final `crates/fg-route/examples/cleanup_diagnose.rs`, including the borrowed-slice Clippy follow-up. Independently confirmed final source SHA256 `1CC2B75C48BE8DE5AE98E8753FD217DBE21E99E839B71C98229285759AA9FC60`. Also inspected `Planner::improve_while_observed`, its zero-attempt loop condition and cleanup callbacks, and ordinary `job::finalize_route`.

## Spec compliance

- `--cleanup-snapshots` excludes both `--tries` and `--rescore-snapshots`; extraction of the existing rescore exclusion preserves that behavior.
- The new branch uses the original-schema, original-experiment loader, exact source seed/edition/request/parameter checks, eight ordered archived boundaries and typed stop parsing. It takes the archived `SelectedInput`; no construction path is entered.
- Cleanup uses `Effort::Tries(0)`. The planner loop cannot call `try_a_move` at that effort. Runtime assertions require one initial callback, unchanged typed initial callback/input boundary, seven ordered cleanup boundaries, and zero logged attempts/improvements.
- Ordinary finalization runs before diagnostic scoring. Each description creates a fresh planner after all cleanup/finalization transformations, avoiding diagnostic interference with cleanup memoization.
- Prepared configuration equality and SelectedInput identity records are checked before emitting a success report. Reports state no search, zero attempts/improvements, no callback comparison and no timing claim; provenance requirements remain explicit.
- Existing regeneration and rescore branches retain their transformations. Output uses the existing existence guard and `create_new` protection. `Train` parsing preserves the independent typed action.

## Quality and validation

The helper keeps execution and assertions together without duplicating cleanup logic. The four-stop fixture meaningfully detects accidental positive search effort; the mode/parser/source checks cover the new contract. Indexed access to callback and snapshot vectors follows validated lengths/order. The slice follow-up removes unnecessary ownership without changing execution.

No suites were rerun: the supplied report and parent verification establish 12 release example tests, all-target release Clippy, formatting and release build passing on the frozen source. Parent also reports four fresh captures reproducing pre-lint output and sixteen unchanged fixed-route controls. This review independently checked code/control flow, not those numerical results.

## Declined to judge

- Durable measurements, source/data provenance hashes, archived boundary comparisons and numerical outcome interpretation: assigned to the parent's separate evidence/final review, outside this diagnostic-code review.
- General hardening of the existing archive format against deliberately corrupt model indices: outside this task's saved-original-route contract; the new mode reuses the existing diagnostic loader and does not broaden its trust model.

Ready to proceed within this scope. Full Task 2 completion remains subject to the separate evidence and documentation review.
