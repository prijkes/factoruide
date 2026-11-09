**AI disclosure:** This contribution was created with AI. The submitter reports in-game testing of the combined fork only and has not reviewed the source code. These split branches have not been individually verified in-game. Automated checks below were run by AI; no human source-code review is claimed.

**Dependent draft: the upstream diff is cumulative.** GitHub's Files changed view includes earlier topics. [Compare only this topic](https://github.com/prijkes/factoruide/compare/pr%2Fshared-quest-kill-credit...pr%2Fbounded-search-experiments).

Prerequisite PRs: #147. Merge prerequisites first, then rebase this topic's single commit onto updated upstream `main` before merging this PR. The series uses a conservative ordering; it does not claim every earlier topic is a technical requirement.

Adds opt-in chain insertion and bounded neighborhood repair, with prerequisite and atomicity checks. Includes a reproducible comparison harness for evaluating these search alternatives.

AI-run automated checks passed: all-target Rust check (`--locked`), TypeScript, frontend tests, translations and formatting on this exact cumulative snapshot. The final combined tree also passed the recorded workspace release tests; that is separate from the submitter's in-game testing.

Part of the [22-topic contribution series](https://github.com/prijkes/factoruide/blob/pr/submission-plan/docs/pr-submissions/README.md).
