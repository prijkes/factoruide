**AI disclosure:** This contribution was created with AI. The submitter reports in-game testing of the combined fork only and has not reviewed the source code. These split branches have not been individually verified in-game. Automated checks below were run by AI; no human source-code review is claimed.

**Dependent draft: the upstream diff is cumulative.** GitHub's Files changed view includes earlier topics. [Compare only this topic](https://github.com/prijkes/factoruide/compare/pr%2Frequired-reputation-targets...pr%2Freputation-programmes).

Prerequisite PRs: #152. Merge prerequisites first, then rebase this topic's single commit onto updated upstream `main` before merging this PR. The series uses a conservative ordering; it does not claim every earlier topic is a technical requirement.

Adds counted repeatable turn-ins, supported material farming, signed rewards and explicit spillover. Daily circuits report the minimum required quest days and server resets, with supplies, acquisition costs, money and reset availability accounted for.

Unmet targets retain a best-found guide with named shortfalls and source limits. Classic-derived rates are labeled as estimates, and unknown calendar waiting remains explicit.

Editing, saving and exports preserve the counted programme. Format-3 guides have matching addon playback and progress across reloads; unsupported Polestar exports explain the compatibility limit. Source provenance, supported rules and estimate limits are documented in `data/reputation/README.md`.

AI-run automated checks passed: all-target Rust check (`--locked`), TypeScript, frontend tests, translations and formatting on this exact cumulative snapshot. The final combined tree also passed the recorded workspace release tests; that is separate from the submitter's in-game testing.

Part of the [22-topic contribution series](https://github.com/prijkes/factoruide/blob/pr/submission-plan/docs/pr-submissions/README.md).
