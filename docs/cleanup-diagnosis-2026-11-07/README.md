# Cleanup diagnosis evidence

See the [diagnosis report](../cleanup-loss-diagnosis-2026-11-07.md) for the findings and limits.

The [review and verification record](reviews.md) summarizes the completed gates and remaining scope limits.

- [Chain input](chain-input.json) and [baseline input](baseline-input.json) retain the archived requests, parameters, callbacks, final stops and scores; full guide/trace payloads are omitted.
- [Original chain snapshots](chain-original-semantics.json) and [original baseline snapshots](baseline-original-semantics.json) retain all eight stage routes, exact scores, traces' summary facts and equality gates.
- [Corrected-accounting chain](chain-corrected-accounting.json) and [corrected-accounting baseline](baseline-corrected-accounting.json) rescore those same fixed routes without searching. All sixteen stage scores, player times, stops and full raw traces are unchanged.
- [Provenance](provenance.json) records raw and compact artifact hashes and diagnostic binary identities. Compact JSON files identify the exact fields omitted and their original raw file hashes. Full raw traces remain in the ignored `.superpowers/sdd/2026-11-07-cleanup-loss-diagnosis` workspace.
- [Trace analysis](trace-analysis.md) and [independent trace review](trace-review.md) document the one-stop deletion, training loss, grinding gates and exact XP reconciliation.
- [Accounting design](fix-design-review.md) and [accounting fix review](accounting-fix-review.md) document the independent conservation defect and bounded provenance correction.
- [Observer review](observer-review.md) and [diagnostic-tool review](example-review.md) cover instrumentation and replay identity safeguards.
- [Profession test diagnosis](profession-test-diagnosis.md) and [scoped review](profession-test-review.md) explain why an initial multi-rank lesson legitimately exports cap 150, and why the test now checks the actual initial-learning instruction.
- [Final source review](final-code-review.md), [metadata correction](dungeon-metadata-fix-report.md) and [scoped re-review](metadata-rereview.md) record the explicit-dungeon reporting finding and its verified resolution at `df96a30`.

The new `cleanup_diagnose` example accepts a source input, fresh output file and optional fixed snapshots:

```powershell
cargo build --release -p fg-route --example cleanup_diagnose
target/release/examples/cleanup_diagnose.exe --repo . --artifact docs/cleanup-diagnosis-2026-11-07/chain-input.json --rescore-snapshots docs/cleanup-diagnosis-2026-11-07/chain-original-semantics.json --output new-chain-rescore.json
```

Omitting `--rescore-snapshots` performs construction and the recorded number of search attempts. Regeneration on a corrected model is a new diagnostic search; it must not be presented as an original-model replay unless its recorded-observable gates all pass. Rescoring explicitly performs no search or callback comparison. Output files must be new and are never overwritten.

For historical original-model reproduction, use source `e73af02`, the [observer-only instrumentation patch](instrumentation.patch), the diagnostic example from this diagnostic branch, and the frozen data/overrides/Cargo identities in the report. The patch adds observation only; it excludes the subsequent accounting correction. The original captures matched every checked observable before that correction was applied. Platform, toolchain and dependency differences can affect deterministic equality.

No new timed benchmark, algorithm promotion, trainer-visit preservation feature or installer deployment is established by these diagnostics. The running app was not changed.

## Diagnostic metadata correction

Final review found that the first diagnostic extractor counted implicit `Enter`
events but missed explicit executed `Dungeon` stops. The corrected extractor
includes both, retaining the first observation of each dungeon identity.
Its event timestamp is an observation time, not necessarily a dungeon-run start.
The chain traces contain five identities, including Excavation Site: Wetlands;
the baseline traces contain six. This does not change the absent Stockades run,
the one-stop mutation, any score or the training-loss explanation.

The original raw captures are immutable and keep their original extractor's
metadata. These derived compact snapshots transparently correct only the
`actual_dungeon_entries` field from fresh rescored traces proven identical to
the original traces. Each compact artifact records that correction and the raw
and rescore hashes; provenance retains the original and corrected tool identities.
The earlier trace review's four-identity statement describes the initial
extractor and is superseded by the corrected trace analysis and scoped re-review.
