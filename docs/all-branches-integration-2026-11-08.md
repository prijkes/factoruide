# All-branch integration

All local and fetched remote branches are integrated into local `main`. The
remote refresh found 15 new upstream commits, which are included through
`c27f59d`. Nothing was pushed. Branch names and externally managed worktrees were
retained.

| Merge | Result |
| --- | --- |
| `d0f7ccc` | Integrates upstream normally and preserves both feature sets. |
| `b6a3561` | Records the obsolete group-guide branch, whose commits have rebased counterparts; retains the later implementation and intentional removals. |
| `20c96ab` | Records the dependency branch already represented by an identical Node types update. |

Both ancestry-only merges have exactly the same tree as `d0f7ccc`. All 14 local
and remote ref tips at the fetched snapshot were verified as ancestors of the
integrated source. Final changes after `20c96ab` are documentation only.

## Conflict resolution and preservation

The merge retains upstream quest XP/count observations, shared respawn waits,
dungeon-only planning, forced-dungeon fixes, addon 0.16.0 and updated data. It also
retains the fork's required reputation/gear validation, quest eligibility and
skip protection, acceptance audit, gold, vendor, icon and cache features.

The Forever database combines all incoming rows with the three fork money
columns. All 62 incoming table rowsets and row counts were independently compared
in both directions after accounting for those columns and 820 source tags.
The result contains 5,812 quests and preserves 71 negative quest costs, positive
rewards, vendor prices and NULLs. Integrity and compression roundtrip checks pass.
The normalized working cache matches the packed database.

Two integration defects were demonstrated by failing tests and corrected:
single-dungeon planning inherited fork BiS goals, and dungeon quest/prerequisite
provenance omitted the forced-dungeon origin. No speculative respawn or fractional
progress redesign was introduced. Fork-only changelog entries remain Unreleased.

## Verification and delivery

- Release workspace: **788 passed, zero failures**. Two client-dependent tests
  and two documentation examples are ignored; the existing Windows CRLF-sensitive
  roundtrip is filtered. Allocation stress remains platform-gated on Windows.
- Release workspace all-target Clippy and formatting pass. Frontend nine tests,
  build, lint and 1,214-text localization checks pass. Full addon/profile smoke
  passes, including three released-guide fixtures and the acceptance/keep tests.
- Independent specification, source/data quality and delivery review passed with
  no unresolved consequential findings. Dataset and artifact hashes are in the
  [validation record](all-branches-validation-2026-11-08.json).
- The production executable is `target/release/factoruide-app.exe`, built with
  `tauri/custom-protocol`. The addon archive is
  `target/release/Factoruide-addons-20c96ab.zip`, with 68 addon files plus one
  installation note. Both artifacts were independently checked.
- The previously requested Forever installation was refreshed to addon 0.16.0.
  All 68 installed files and all 70 backup files passed independent hash checks.
  Existing legacy files, generated guides and SavedVariables were preserved.
  The original saved request, outcome and metadata match their frozen hashes.

No game client or native app session was launched or closed. Event behavior is
covered by smoke tests rather than live gameplay. Previous route measurements
and artifact records remain historical evidence for their own frozen source/data.
