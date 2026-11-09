**AI disclosure:** This contribution was created with AI. The submitter reports in-game testing of the combined fork only and has not reviewed the source code. These split branches have not been individually verified in-game. Automated checks below were run by AI; no human source-code review is claimed.

The quest tracker can inherit protection from its secure quest-item buttons on newer WoW clients. Refreshing protected lines and tabs during combat then triggers blocked UI actions. Keep their geometry and visibility stable during combat, defer new rows and secure buttons, and apply the layout and zoom after combat.

AI-run automated checks: the source regression fails against upstream and passes with the fix. The full guide smoke suite passes using a scratch probe that normalizes only the generated-settings CRLF comparison; the unmodified native guide run retains upstream's Windows SettingsSchema byte-comparison failure. Native profile smoke and StyLua (Windows checkout line endings) pass. The AI did not perform in-game client validation.

The profile-export JSON parsing subcheck was skipped because `python3` is unavailable in this Windows environment.

Part of the [22-topic contribution series](https://github.com/prijkes/factoruide/blob/pr/submission-plan/docs/pr-submissions/README.md).
