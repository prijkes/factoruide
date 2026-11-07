# Task 2 diagnostic implementation report

## Scope and implementation

Owned only `crates/fg-route/examples/cleanup_diagnose.rs` and this ignored task report. The parent owns measured replay/rescore runs, durable evidence, assessment, handoff, changelog, plan updates, final package checks and independent review. Library source remained frozen at the independently approved Task 1 implementation.

Added `--cleanup-snapshots PATH`, mutually exclusive with both `--tries` and `--rescore-snapshots`. The mode reuses `snapshot_routes` to require the original diagnosis schema/experiment, exact source seed/edition/effective request/effective parameters, all eight ordered archived boundaries and valid typed stop decoding. `Train` now decodes as `Kind::Train`.

The cleanup branch verifies exact prepared request/parameter identity, selects the decoded archived `SelectedInput`, calls public `improve_while_observed` with `Effort::Tries(0)` and then calls ordinary `job::finalize_route`. It does not call construction or add search operators. Snapshot descriptions occur after cleanup/finalization. The helper checks exactly one initial best callback, the unchanged typed initial callback route and SelectedInput boundary, all seven ordered cleanup callbacks and parsed log counts `(0 tries, 0 improvements)`. Main additionally checks SelectedInput quest/stop identity records against the archived diagnosis in the prepared model.

The new report uses experiment `cleanup_training_cleanup_only_replay`, mode `cleanup_snapshots`, `search_performed=false`, `search_attempts=0`, `search_improvements=0`, `callback_comparison_performed=false`, configuration/input identity flags, initial callback count and cleanup boundary count, the cleanup log and callbacks, exact effective configuration and eight stage descriptions. Existing fresh-output protection and create-new report writing are reused. Original rescore and full-regeneration paths retain their behavior.

## RED evidence

Command: `cargo test -p fg-route --release --example cleanup_diagnose`

Before implementation, existing mode validation was extracted unchanged into `validate_modes`; the new cleanup helper was an explicit unsupported-operation stub. The tests compiled and ran. Result: 8 passed, 3 failed, exit 1, in 0.01 seconds (13.58 seconds build).

Expected failing tests and observed reasons:

- `cleanup_mode_excludes_search_override_and_snapshot_rescoring`: accepted `["--cleanup-snapshots", "--tries"]`.
- `archive_stop_decoder_preserves_independent_training`: decoder returned `unknown stop kind`.
- `cleanup_only_uses_fixed_input_with_one_callback_and_zero_attempts`: helper returned `cleanup-only replay is not implemented`.

## GREEN evidence

After implementation, the same release example command passed 11/11 tests. The zero-attempt fixture was then strengthened to four stops so an accidental positive effort enters the real search loop, and an additional source-schema/experiment rejection test was added.

Final command: `cargo test -p fg-route --release --example cleanup_diagnose`.

Final result: 12 passed, 0 failed, 0 ignored, 0 filtered out, exit 0, in 0.01 seconds (12.25 seconds build). This includes existing configuration drift, original replay contract, stage ordering and typed stop tests as well as the new mode, Train, source and zero-attempt tests.

Formatting: initial `cargo fmt --all -- --check` identified formatting differences only in the edited example; `rustfmt --edition 2024 crates/fg-route/examples/cleanup_diagnose.rs` corrected them. Final `rustfmt --edition 2024 --check crates/fg-route/examples/cleanup_diagnose.rs` passed. `git diff --check -- crates/fg-route/examples/cleanup_diagnose.rs` passed; Git printed only its existing LF-to-CRLF advisory.

Release example build: `cargo build --release -p fg-route --example cleanup_diagnose` passed, exit 0, in 16.18 seconds. Binary: `target/release/examples/cleanup_diagnose.exe`. Source frozen after build and parent notified before commit. Precommit source diff Git hash: `a4722ef7ba98c8b86afcb8d26223d29274f97bf8`; source file SHA256: `D1E9CF9BF6A40C75EBF7B6EC144721052F87100BE0C9225CC684C96F8CA8AD33`.

## Self-review and remaining validation

The four-stop test catches accidental search effort; the helper independently rejects changed input, missing/extra callbacks, missing/reordered boundaries and nonzero parsed counts. Source/configuration checks reuse the same validated source loader as fixed-route rescoring. Existing regeneration and rescoring transformations were not moved or changed.

No known implementation concerns. The helper tests do not establish archived data/source provenance or assert real saved-route outcomes; those require the parent's two measured cleanup runs and sixteen fixed-stage control rescores. The parent explicitly requested avoiding repeated package-suite runs while their release package tests run on the frozen library source. Broad package tests, all-target Clippy and independent final review remain parent-owned.

No desktop app, installer, saved guides, game data, model parameters, random search behavior, push or merge changes.

## Post-commit Clippy follow-up

Initial implementation committed as `4fe8de4` (`feat: replay archived cleanup without search`), one example file only, author/committer `prijkes <prijkes@noreply.com>`.

Parent's all-target release Clippy found `clippy::needless_pass_by_value` at the helper's owned `initial: Vec<Stop>` parameter. Verified the helper only clones it for the planner and compares it afterward; changed the parameter to `&[Stop]`, used `to_owned()` for the planner call and borrowed both call sites. No behavioral or output change was introduced.

Verification after that change:

- `cargo test -p fg-route --release --example cleanup_diagnose`: 12 passed, zero failures, exit 0, in 0.00 seconds (10.19 seconds build).
- `cargo clippy --release -p fg-route --all-targets -- -D warnings`: passed, exit 0 (0.50 seconds).
- `rustfmt --edition 2024 --check crates/fg-route/examples/cleanup_diagnose.rs`: passed.
- `git diff --check -- crates/fg-route/examples/cleanup_diagnose.rs`: passed, with the Git LF/CRLF advisory only.
- `cargo build --release -p fg-route --example cleanup_diagnose`: passed, exit 0 (16.76 seconds).

The source was frozen again and parent notified before the follow-up commit. Final source SHA256: `1CC2B75C48BE8DE5AE98E8753FD217DBE21E99E839B71C98229285759AA9FC60`. Rebuilt executable SHA256: `C2F44C17D1481F405229744AF2F5FF966151841F57A2B63BAA057170135E36C3`. Follow-up diff against initial commit Git hash: `68c295f271793fa9a6ca9b2acafc97126c60ab65`. Parent will create fresh `-v2` measurements and preserve original raw outputs.
