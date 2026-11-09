**AI disclosure:** This contribution was created with AI. The submitter reports in-game testing of the combined fork only and has not reviewed the source code. These split branches have not been individually verified in-game. Automated checks below were run by AI; no human source-code review is claimed.

A web app built on Windows silently omitted its game databases and map images because URL.pathname retained the drive-leading slash. Resolve the data URL with fileURLToPath so filesystem checks and copies use a native path.

AI-run automated checks: a controlled Windows fixture in a path containing spaces reproduced the missing database before the fix and copied all three databases, map images and host headers after it; Node syntax and Git whitespace checks passed.

Part of the [22-topic contribution series](https://github.com/prijkes/factoruide/blob/pr/submission-plan/docs/pr-submissions/README.md).
