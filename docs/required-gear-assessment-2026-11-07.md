# Supported required gear and quest-warning correction

## User-visible problem and chosen policy

A saved Tauren Warrior leveling guide reported 25 missing mandatory class quests. Diagnosis established fourteen selected gear-reward quests and eleven prerequisites, all introduced by required BiS, while the applicable Warrior stance requirements were satisfied. The old counter combined several mandatory-goal categories under a class-only label.

The user chose the best supported alternative with an explanation when the preferred gear source is unsupported. The implementation keeps that policy separate from final completion: unsupported winners are replaced before requirements are committed, while a successful Required route must actually complete the supported quest sources selected for it.

## Resulting behavior

- Gear ranking checks every modeled giver and selects a coherent prerequisite/action closure, including held parents, N-of-M choices and recorded progress. Windows and combat estimates use the selected outside/inside location category.
- Unsupported winners receive an explained fallback. For example, Kirtonos the Herald (5384) requires Scholomance re-entry under the model; the selected alternative is Lethtendris's Web (7489). This describes a planner limitation, not impossibility in the game.
- Required source work is merged into the existing route, preserving prerequisite and breadcrumb order, timed work, earlier dungeon windows and one entry marker per visit. Actual completion takes priority over a cheaper incomplete candidate throughout search, sharing and finalization.
- Fresh warnings separate class, gear and other origins, including shared prerequisites. The historical aggregate still contributes its original single penalty per representative. Old guides retain unknown categories and neutral wording without replay or reclassification.

The leveling endpoint, exact quest semantics, search operators/defaults/random calls, cache caps and game data remain unchanged. Required guarantees selected quest sources; vendor purchases remain advisory and mutually exclusive reward ownership is not promised.

## Source-matched route verification

Final source `66d21231ecbf833155f247e4a370904c57c925e8` passed a fresh generation from the saved request:

| Check | Result |
| --- | --- |
| Selected source roots actually completed | 71/71 |
| Selected completion-support quests completed | 193/193 |
| Missing class / gear / other mandatory goals | 0 / 0 / 0 |
| Historical missing mandatory aggregate | 0 |
| Level range | 18–60 |
| Applicable forced visits / actual runs / entry markers | 20 / 20 / 20 |
| Missing applicable dungeons | 0 |
| Exported quests | 884 |
| Simulated player time | 553,525 seconds (153h45) |
| Generation call wall time, compilation excluded | 19.474 seconds |
| Distinct fallback explanations | 22 |

This was a bounded correctness run. Only six budget fields changed: candidates 32→1, effort 3→1, final tries 195→1, race mutants 224→0, screen 8→1 and threads 8→1. All gameplay settings and starting state were preserved. It is not an algorithm-quality comparison or proof of route optimality.

The request lists 22 modeled dungeon IDs. Stockade (717) and Deadmines (1581) have no modeled quest objectives for this profile and receive an unplanned minimum level under the existing rule. Twenty areas satisfy `min_level < target`; all twenty actual area identities were verified. No eligibility rule was changed to obtain this result.

## Compatibility and delivery checks

The original saved outcome's exact steps, time metadata, notes, aggregate 25 and absent categories survive deserialization. The original request, outcome and metadata in app storage still match their captured hashes. Private requests, character data and full outcomes remain in ignored storage.

Frontend verification passes nine tests, production build, lint and checks for all 1,202 translated texts. The existing large-chunk build advisory remains. Final release workspace tests pass 754 cases, including 423 route unit and 238 integration tests. Two contribution tests requiring game clients and two doc examples remain ignored; the known Windows CRLF test is explicitly filtered. The allocator stress target is gated to its supported Wasi/Unix platforms and is not exercised on Windows. Release all-target workspace Clippy, formatting and whitespace checks pass.

The production desktop executable was built without an installer or app launch:

```text
cargo build --release --locked --offline -p factoruide-app --features tauri/custom-protocol
```

The production protocol embeds the already-verified frontend. The local artifact is `target/release/factoruide-app.exe`, 159,474,688 bytes, SHA-256 `0525f72ca072fe9de74f3a38c05bb979b7919dac7e29c32df49ccfbb3638b32f`. The [sanitized validation and artifact record](required-gear-validation-2026-11-07.json) independently derives completion counts and area identities from exported steps, checks the executable bytes, and records the frozen input hashes. It contains no character/realm names, positions, request bodies, logs or private notes.

The selector and repair are deterministic bounded heuristics. They can return an honest named NoRoute when attempted schedules fail; neither is an exhaustive time-window solver. The [implementation plan](superpowers/plans/2026-11-07-reachable-required-bis.md) records the regression evidence, task reviews and final review gates.

## Implementation decisions

- Continue in the existing checkout on a local fix branch. A different isolation choice would cost reversible branch/checkout rework; original guides are unaffected.
- Retain the quest-source guarantee, advisory vendors and normal endpoint. An endgame item-ownership contract would require a separate feature.
- Add optional authoritative categories while preserving the historical aggregate. A different metadata design would cost a compatible migration, without changing saved route steps or times.
- Implement the frontend independently against the frozen counter contract. An interface mistake would cost frontend rework, checked by tests and the integrated build.
- Apply the existing single-visit dungeon rule during source selection. A mistaken classification would cost a conservative fallback; inside-only sequences, history cuts and alternatives have regressions.
- Retain the selected action-to-visit witness for runtime placement. An interface mistake would cost focused model/planner rework, covered by alternate-giver/objective and window tests.
- Isolate the runtime merge fix in a managed worktree while reporting proceeds. The cost is an additional local checkout and reversible integration work. Both local branches and private diagnostic evidence are retained.
