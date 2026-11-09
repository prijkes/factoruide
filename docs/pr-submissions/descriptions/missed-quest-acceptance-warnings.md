After a quest-giver interaction closes, warn about guide quests that were actually offered but remain unaccepted. Observe offers before automation consumes them, wait for delayed acceptance events, and show one yellow reminder with matching chat history. This also works with automatic acceptance disabled or Shift held.

Keep each source interaction separate while preserving its offers across panel transitions and brief reopenings. Completed, accepted, held, unrelated and repeatable turn-in quests avoid false reminders; hidden guides and route changes discard stale notices. Required keep-marked accepts are no longer skipped solely for being below the player's level.

Validation: 18 source acceptance regressions fail against upstream, then the full guide smoke suite passes with a scratch probe normalizing only the Windows settings-schema CRLF comparison. The native guide suite retains that upstream byte-comparison failure; native profile smoke and repository-configured StyLua checks pass. No in-game client validation was run.

The profile-export JSON parsing subcheck was skipped because `python3` is unavailable in this Windows environment.
