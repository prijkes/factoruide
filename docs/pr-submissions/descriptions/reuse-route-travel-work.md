**AI disclosure:** This contribution was created with AI. The submitter reports in-game testing of the combined fork only and has not reviewed the source code. These split branches have not been individually verified in-game. Automated checks below were run by AI; no human source-code review is claimed.

Route search rebuilds quest planning metadata and allocates a new candidate buffer for each attempt, while full travel caches discard their entire working set. Reuse unchanged plan data and candidate storage, retain half of full travel/farming/walk caches, and restrict flight relaxation to known nodes. The changes use the current simulator and dungeon movement interfaces.

AI-run automated checks:

- The existing 92 planner unit regressions pass before and after the change, including incremental versus full replay, timed quests, dungeon moves, movement and training.
- Five flight regressions and the verified improved-route integration test pass.
- Rustfmt and whitespace checks pass. No speed percentage is claimed.

Part of the [22-topic contribution series](https://github.com/prijkes/factoruide/blob/pr/submission-plan/docs/pr-submissions/README.md).
