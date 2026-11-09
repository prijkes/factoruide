**AI disclosure:** This contribution was created with AI. The submitter reported the original issue in-game and has previously tested the combined fork in-game, but has not reviewed the source code. This change and its standalone PR branch have not yet been confirmed in-game. Automated checks below were run by AI; no human source-code review is claimed.

A character can keep following a saved guide for another race/class even when its own guide is imported. A Tauren Warrior was following an Undead Rogue guide, so it was shown quests that were correctly absent from the generated Warrior guide.

Reuse the existing `Import.Fits` check when restoring a saved choice, accepting a legacy character assignment, and explicitly choosing a guide. An incompatible saved choice falls back to the existing automatic selector: the unique matching guide, or a compatible legacy assignment when several guides match. A valid saved choice stays selected, group members remain supported, and ambiguous choices are left to the guide list. Disable Follow for incompatible guides. Route switches use the existing per-character, per-route progress storage.

AI-run automated validation on this standalone branch:

- The added regressions fail against unchanged upstream before the four selection guards are applied; both complete LuaJIT addon smoke suites pass with the change.
- Coverage includes independent race/class mismatches, login and import refresh, same-name guide replacement, compatible/incompatible assignments, ambiguous matches, direct selection, the Follow button, group members and saved progress restoration.
- StyLua on the changed Lua files and Git whitespace checks pass.
- The Windows generated-settings byte comparison was run with that unchanged file checked out using its tracked LF bytes. The optional profile-export JSON parsing subcheck remains skipped because `python3` is unavailable.

This is one standalone topic commit directly above upstream `main` at `f812eb4`. It changes the two addon selection/UI files, smoke coverage and matching changelog entries. It does not require the reputation PR series or change addon release versions.
