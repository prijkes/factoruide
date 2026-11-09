When two forced dungeon runs overlap in level, the next run can preempt available quest work inside the current instance. Finish that work, including hand-ins, before leaving for the next run; a completed instance cannot be entered again by the route.

Validation:

- A standalone ordinary-quest regression failed on upstream because the second dungeon was selected before the current objective. It now checks the objective, inside hand-in, full greedy ordering and valid replay.
- Nine dungeon unit regressions and four forced-dungeon integration regressions pass.
- Rustfmt, whitespace checks and `cargo clippy --profile fast -p fg-route --lib --tests -- -D warnings` pass.
