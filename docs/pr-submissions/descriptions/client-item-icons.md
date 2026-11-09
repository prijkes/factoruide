**AI disclosure:** This contribution was created with AI. The submitter reports in-game testing of the combined fork only and has not reviewed the source code. These split branches have not been individually verified in-game. Automated checks below were run by AI; no human source-code review is claimed.

**Dependent draft: the upstream diff is cumulative.** GitHub's Files changed view includes earlier topics. [Compare only this topic](https://github.com/prijkes/factoruide/compare/pr%2Fbest-gear-and-merchant-purchases...pr%2Fclient-item-icons).

Prerequisite PRs: #144. Merge prerequisites first, then rebase this topic's single commit onto updated upstream `main` before merging this PR. The series uses a conservative ordering; it does not claim every earlier topic is a technical requirement.

Reads item icons from an installed game client and packages them for desktop and browser guides. Adds the extraction command and icon loading for packaged guide assets.

AI-run automated checks passed: all-target Rust check (`--locked`), TypeScript, frontend tests, translations and formatting on this exact cumulative snapshot. The final combined tree also passed the recorded workspace release tests; that is separate from the submitter's in-game testing.

Part of the [22-topic contribution series](https://github.com/prijkes/factoruide/blob/pr/submission-plan/docs/pr-submissions/README.md).
