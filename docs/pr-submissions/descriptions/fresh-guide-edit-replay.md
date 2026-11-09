**AI disclosure:** This contribution was created with AI. The submitter reported the original editing failure and has previously tested the combined fork in-game, but has not reviewed the source code. These changes and this dependent PR branch have not yet been confirmed in-game. Automated checks below were run by AI; no human source-code review is claimed.

**Dependent draft: the upstream diff is cumulative.** This topic is one commit directly above the prepared head of #154 (`4ec7b7a`), which depends on #153 and the earlier reputation series. GitHub's Files changed view includes those prerequisite topics. [Compare only this fix](https://github.com/prijkes/factoruide/compare/pr%2Freputation-dungeon-quest-order...pr%2Ffresh-guide-edit-replay).

Merge the prerequisites first, then rebase this single topic commit onto updated upstream `main` before merging this PR.

A newly generated reputation guide could immediately show ten impossible steps when opened in Edit without any changes. Reputation repair appended dungeon pickups after the earlier cleanup, including quests taken after their only usable dungeon visit. The editor correctly rejected that work. Separately, JSON parsing could introduce a tiny floating-point rounding error in persisted reputation source coordinates, causing strict source binding to reject farming phases and reducing the displayed estimate from roughly 4,434 hours to 879 hours.

- Clean up optional unfinished dungeon work after reputation repair, based on actual terminal objective progress rather than the mere presence of a dungeon visit. Retain dependency/training safeguards and required work and reputation progress.
- Conservatively repair affected explicit saved guides in memory when opening them: remove only unused optional dungeon accepts, require an error-free replay, and preserve required outcomes and selected reputation progress. Subsequent edits and saves keep strict validation.
- Enable exact JSON float parsing so persisted acquisition and turn-in coordinates still bind to their supported sources. Save/reopen retains the farming phases and time estimate.

AI-run automated validation on this exact dependent snapshot:

- Tests-first reproduction: five expected failures before the production patch, covering late dungeon accepts, legacy-guide opening, acquisition/turn-in coordinate persistence and generated-prefix programme replay.
- Complete `fg-route` suite: **804 passed** (557 unit and 247 integration), zero failures, with two preexisting doctests ignored and only `the_addon_reads_them_and_they_read_the_addon` filtered because its Lua fixture is newline-sensitive on Windows.
- The strict draft regression still rejects a user edit that leaves a dungeon quest without its run; it also passed separately.
- All-target `fg-route` Clippy with warnings denied, repository Rust formatting and Git whitespace checks pass.

During diagnosis on the combined fork, a copy of the affected saved guide replayed and saved/reopened in memory with zero errors, all 1,072 completed quests and all 10 farming phases retained. Native `edit_open` also returned zero errors, and the combined workspace release run passed 892 tests with the same known Windows fixture excluded. Those combined-fork checks are separate from the exact split-branch validation above; original saved guides were not overwritten.

The focused topic changes nine files: planner/editor behavior, the JSON feature, regression coverage and two Unreleased guide notes. Internal workflow documents, the unrelated loot-vendor fix, packed databases, addon code and release-version changes are excluded from this topic.
