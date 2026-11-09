**AI disclosure:** This contribution was created with AI. The submitter reports in-game testing of the combined fork only and has not reviewed the source code. These split branches have not been individually verified in-game. Automated checks below were run by AI; no human source-code review is claimed.

When two forced dungeon runs overlap in level, the next run can preempt available quest work inside the current instance. Finish that work, including hand-ins, before leaving for the next run; a completed instance cannot be entered again by the route.

AI-run automated checks:

- A standalone ordinary-quest regression failed on upstream because the second dungeon was selected before the current objective. It now checks the objective, inside hand-in, full greedy ordering and valid replay.
- Nine dungeon unit regressions and four forced-dungeon integration regressions pass.
- Rustfmt, whitespace checks and `cargo clippy --profile fast -p fg-route --lib --tests -- -D warnings` pass.

Part of the [22-topic contribution series](https://github.com/prijkes/factoruide/blob/pr/submission-plan/docs/pr-submissions/README.md).
