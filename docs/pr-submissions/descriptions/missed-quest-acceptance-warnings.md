**AI disclosure:** This contribution was created with AI. The submitter reports in-game testing of the combined fork only and has not reviewed the source code. These split branches have not been individually verified in-game. Automated checks below were run by AI; no human source-code review is claimed.

After a quest-giver interaction closes, warn about guide quests that were actually offered but remain unaccepted. Observe offers before automation consumes them, wait for delayed acceptance events, and show one yellow reminder with matching chat history. This also works with automatic acceptance disabled or Shift held.

Keep each source interaction separate while preserving its offers across panel transitions and brief reopenings. Completed, accepted, held, unrelated and repeatable turn-in quests avoid false reminders; hidden guides and route changes discard stale notices. Required keep-marked accepts are no longer skipped solely for being below the player's level.

AI-run automated checks: 18 source acceptance regressions fail against upstream, then the full guide smoke suite passes with a scratch probe normalizing only the Windows settings-schema CRLF comparison. The native guide suite retains that upstream byte-comparison failure; native profile smoke and repository-configured StyLua checks pass. The AI did not perform in-game client validation.

The profile-export JSON parsing subcheck was skipped because `python3` is unavailable in this Windows environment.

Part of the [22-topic contribution series](https://github.com/prijkes/factoruide/blob/pr/submission-plan/docs/pr-submissions/README.md).
