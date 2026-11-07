# Scoped re-review: explicit dungeon metadata

Verdict: **ADDRESSED. Approve the correction at `df96a30`.** The Important finding from `final-code-review.md` is closed. No new material issue found in this bounded fix; no further source fix wave is requested.

Reviewed the `09752b6..df96a30` package, current example/tests, revised report/trace analysis, compact evidence and manifest. Ran only read-only JSON/hash checks; no builds, tests, planners, source edits or artifact edits. The reported eight helper passes in fast/release and release all-target Clippy are coordinator-supplied execution evidence.

## Source and regression coverage

The shared stage-description path now recognizes both `Event::Enter` and executed `Event::Stop` with `Kind::Dungeon`. A set keyed by dungeon index retains the first occurrence in trace order. This adds explicit-only runs while avoiding double counting when implicit entry precedes the explicit stop. Timestamp and observation-kind labels correctly distinguish observed event time from run start.

The two new tests use actual ordinary simulator traces. One proves an explicit-only run has no `Enter` event and still appears in the output. The other demonstrates an implicit event before the same dungeon's explicit stop, verifies first-event time/level preservation, and includes a second explicit-only dungeon. No search, simulator or accounting production behavior changes in this commit.

## Independently verified evidence correction

- Every raw-artifact hash and every listed compact-archive hash in the current provenance manifest matches its file. The immutable original chain/baseline captures retain the previously reviewed hashes.
- Across all sixteen old/v2 stage pairs, stage labels, scores, player times, typed stop vectors and full trace arrays compare exactly equal.
- Independently parsing the immutable original trace strings reproduces every v2 dungeon index sequence, first-observation timestamp, level and observation kind. Explicit dungeon area IDs agree with their actual stop events.
- All eight chain stages now report five identities: 1581, 16732, 721, 16544, 722. All eight baseline stages report six: 1581, 16732, 721, 16544, 491, 722. Duplicate 721 is eliminated.
- Each compact snapshot is equal to its declared raw source after removing trace arrays and applying only the declared `actual_dungeon_entries` correction. Its original-source hash and fresh-rescore correction-basis hash match. No other numerical or route evidence was silently rewritten.
- The main report now states eight diagnostic-example tests. The revised trace analysis explicitly corrects its former four-dungeon claim to five and explains the omission, unchanged raw captures and event-time semantics. This re-review and that correction supersede the four-identity enumeration in the historical trace review; its training-cascade and score conclusions remain unchanged.

The updated metadata closes the original reporting defect without changing the observed diagnosis, accounting result or provenance claims. Approval remains bounded to the completed diagnosis; it does not claim a training-preservation implementation, a new algorithm ranking or timed performance improvement.
