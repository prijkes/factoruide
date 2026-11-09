**AI disclosure:** This contribution was created with AI. The submitter reports in-game testing of the combined fork only and has not reviewed the source code. These split branches have not been individually verified in-game. Automated checks below were run by AI; no human source-code review is claimed.

Pasted guide imports can fail when WoW's integer formatter is limited to signed 32-bit values. Serialize exact integer Lua numbers with a floating-point integer format so wide masks and numeric table keys survive import and reload.

AI-run automated checks: the regression reproduces overflow before the fix and passes afterward, covering signed 32-bit boundaries, wide masks, numeric keys and fractional values. The full guide smoke suite passes with a scratch LF-normalized settings-comparison probe; the native guide suite retains upstream's Windows CRLF comparison failure. Native profile smoke and repository-configured StyLua checks pass.

The test uses upstream's current guide format 2. This change does not bump addon versions.

The profile-export JSON parsing subcheck was skipped because `python3` is unavailable in this Windows environment.

Part of the [22-topic contribution series](https://github.com/prijkes/factoruide/blob/pr/submission-plan/docs/pr-submissions/README.md).
