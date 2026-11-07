# Cleanup training evidence

The [assessment](../cleanup-training-assessment-2026-11-07.md) explains the correction and measured limits. The [design](../superpowers/specs/2026-11-07-cleanup-training-design.md) and [plan](../superpowers/plans/2026-11-07-cleanup-training.md) record the implementation requirements.

- [Chain cleanup](chain-cleanup.json) and [baseline cleanup](baseline-cleanup.json) retain all eight new stage routes, exact scores, breakdowns, actual turn-ins/dungeons and derived training/grinding trace summaries. Full trace arrays are omitted, with raw capture paths and hashes recorded in each file.
- [Measurement summary](measurement-summary.json) records the verified unchanged controls, score/player differences and final trace facts.
- [Provenance](provenance.json) records source `10bb20a`, the binary, frozen database/lockfile, overrides tree, original-source and fresh raw hashes. Its `archive_sha256` map covers the three core evidence JSON files; review prose is indexed separately here.
- [Implementation report](implementation-report.md) and [implementation review](implementation-review.md) cover the training-stop semantics and cleanup substitution, including the functional regression before implementation.
- [Diagnostic report](diagnostic-report.md) and [diagnostic review](diagnostic-review.md) cover zero-attempt cleanup execution and identity checks.
- [Review record](reviews.md) records final verification and review status.
- [Final source and evidence review](final-review.md) independently verifies the whole change and numerical claims, with no outstanding findings.

The [earlier diagnosis evidence](../cleanup-diagnosis-2026-11-07/README.md) supplies the original requests and fixed stage snapshots. Those captures remain unchanged. Full new raw traces and sixteen fixed-stage control replays are retained in `.superpowers/sdd/2026-11-07-cleanup-training/`. Final raw files end in `-v2.json`; earlier pre-lint files remain immutable too, and all four final JSON outputs equal those earlier outputs in full.

## Reproduce the fixed-route checks

Build only the diagnostic example; this does not rebuild the running desktop app:

```powershell
cargo build --release -p fg-route --example cleanup_diagnose
target/release/examples/cleanup_diagnose.exe --repo . --artifact docs/cleanup-diagnosis-2026-11-07/chain-input.json --cleanup-snapshots docs/cleanup-diagnosis-2026-11-07/chain-original-semantics.json --output new-chain-cleanup.json
target/release/examples/cleanup_diagnose.exe --repo . --artifact docs/cleanup-diagnosis-2026-11-07/baseline-input.json --cleanup-snapshots docs/cleanup-diagnosis-2026-11-07/baseline-original-semantics.json --output new-baseline-cleanup.json
```

Use `--rescore-snapshots` instead of `--cleanup-snapshots` to replay the eight unchanged old stage routes without running cleanup. Output paths must be new. Neither mode performs construction or search attempts. Regenerating the complete historical search is separate from these fixed-route checks.
