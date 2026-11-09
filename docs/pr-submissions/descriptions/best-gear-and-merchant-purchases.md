**AI disclosure:** This contribution was created with AI. The submitter reports in-game testing of the combined fork only and has not reviewed the source code. These split branches have not been individually verified in-game. Automated checks below were run by AI; no human source-code review is claimed.

**Dependent draft: the upstream diff is cumulative.** GitHub's Files changed view includes earlier topics. [Compare only this topic](https://github.com/prijkes/factoruide/compare/pr%2Fseries-base...pr%2Fbest-gear-and-merchant-purchases).

Prerequisite PRs: #133, #134, #135, #136, #137, #138, #139, #140, #141, #142, #143. Merge prerequisites first, then rebase this topic's single commit onto updated upstream `main` before merging this PR. The series uses a conservative ordering; it does not claim every earlier topic is a technical requirement.

Adds best quest and vendor gear choices by level band, with optional purchase stops and merchant automation. Purchases require an affordable upgrade and charge the route's available money.

AI-run automated checks passed: all-target Rust check (`--locked`), TypeScript, frontend tests, translations and formatting on this exact cumulative snapshot. The final combined tree also passed the recorded workspace release tests; that is separate from the submitter's in-game testing.

Part of the [22-topic contribution series](https://github.com/prijkes/factoruide/blob/pr/submission-plan/docs/pr-submissions/README.md).
