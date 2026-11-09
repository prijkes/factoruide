Route search rebuilds quest planning metadata and allocates a new candidate buffer for each attempt, while full travel caches discard their entire working set. Reuse unchanged plan data and candidate storage, retain half of full travel/farming/walk caches, and restrict flight relaxation to known nodes. The changes use the current simulator and dungeon movement interfaces.

Validation:

- The existing 92 planner unit regressions pass before and after the change, including incremental versus full replay, timed quests, dungeon moves, movement and training.
- Five flight regressions and the verified improved-route integration test pass.
- Rustfmt and whitespace checks pass. No speed percentage is claimed.
