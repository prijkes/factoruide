# Required reputation and missed acceptance: delivery record

The approved fixes are implemented, independently reviewed and merged into local
`main`. Product source is `95356fd`; the reviewed branch integration is `b0a588c`.
All completed local branches are integrated. Nothing was pushed or published.

The [validation record](reputation-and-acceptance-validation-2026-11-08.json)
contains artifact identities, test results and sanitized diagnostic evidence.
The [implementation plan](superpowers/plans/2026-11-08-reputation-and-acceptance.md)
is complete.

## The four reported issues

1. The level-checkpoint banner is an offer requiring a click; its number counts
   steps. There is no dedicated disable switch. A separate automatic rule passes
   old steps. Both paths now respect exported keep protection, including planned
   reputation contributions and their prerequisite work. This protection also
   applies to newly generated guides without checkpoints.
2. Selected reputation totals are required outcomes. Search and cleanup preserve
   fulfillment, construction can take costly old quests and continue after the
   target level, and final/edited routes validate actual rewards. Useful old
   prerequisite candidates remain eligible without making every reputation quest
   compulsory. Fractional reward arrays are parsed without losing their other
   faction rewards. Missing starting standings are labeled per faction.
3. The addon records identified planned quests actually offered during an NPC
   interaction, including manual/Shift acceptance. After closure and a short log
   grace, one yellow message and chat entry name any still-needed missing quests
   and invite manual acceptance. Reopening, source changes, delayed confirmation,
   explicit skips and guide changes are handled. Ordinary advancement still
   requires confirmation. Unidentified legacy offers are not guessed.
4. Previously completed branches were merged at `f9dde21`. The new reviewed work
   was then fast-forwarded to `main`, with the integrated tree verified identical
   to the reviewed tree. Branches and the externally managed worktree remain.

In the bundled Forever data, **The Argent Emissary (96895)** grants 150 Argent
Dawn reputation. **Proof of Demise (374)** grants 100 Undercity reputation. The
investigated level-18 request selects Undercity but not Argent Dawn. The old lower
level filter excluded Proof of Demise; final-source preparation now retains it
and its prerequisite, without marking it mandatory. This proves eligibility,
not that every route must choose it or why every other quest was omitted.

## Verification and limits

- Release workspace: **776 passed, zero failures**. Two client-dependent tests
  and two documentation examples are ignored; the existing Windows CRLF-sensitive
  roundtrip test is filtered. Allocation stress is platform-gated on Windows.
- Release workspace all-target Clippy and formatting pass. Frontend nine tests,
  build, lint and 1,206-text localization check pass. Full addon and profile-only
  Lua smoke pass; changed Lua files pass StyLua.
- A real-data Tauren Warrior 1–10 case with required Thunder Bluff 5,000 and
  **Gear Prefer** reaches **5,900 from 4,000**. Its 21 actual unique turn-ins
  independently sum 1,900. All 68 quest steps are protected; 75 total steps are
  kept, including seven nonquest steps. It has zero checkpoints. Combined exact
  quests, Required gear and reputation also have positive behavioral regressions.
- The original eight-Exalted request returns a named required-target failure.
  The final bounded candidate reaches 19,797 Undercity, leaving 22,203 to 42,000.
  A deliberately generous sum of every available positive direct quest reward,
  even ignoring eligibility and completion, reaches only 25,122 from its recorded
  starting standing. The model plans one completion per quest ID and does not
  derive spillover or repeatable farming. This establishes a model/data limit,
  not game-wide impossibility. The Horde starting value is an explicit estimate.
- Separate short Required-gear probes failed safely. The large bounded control
  misses the same seven gear roots with reputation removed; a prior binary also
  misses those roots. This pre-existing heuristic/repair limit remains recorded.
  A failure means no satisfying route was found, not proof none exists. No faster
  or optimal-search claim follows from these fixes.
- Lua/event behavior is covered by smoke tests; no live game client was connected.
  Original request, outcome and metadata hashes match the frozen baseline.

## Local artifacts and use

The production desktop executable is `target/release/factoruide-app.exe`, built
with `tauri/custom-protocol` and the bundled frontend. The addon archive is
`target/release/Factoruide-addons-3fdc85d.zip`. Exact sizes and SHA-256 values are
in the validation record. The archive contains the two code addons and an install
note, with neither `Factoruide_Data` nor SavedVariables.

Update the desktop executable and the two addons with the game closed, then
regenerate to obtain the new reputation keep flags. Existing guides are preserved
and their old shortfalls use neutral wording. The investigated Exalted settings
remain unchanged and produce the explicit supported-model failure described
above. Active app sessions and installed game files were not replaced.

## Implementation decisions

Work used the dedicated feature branch and existing caches; independent workers
owned disjoint files and root serialized commits. A Windows diagnostic-executable
lock was corrected by running a copied binary. Code integration at `b0a588c`
matched the reviewed tree; final documentation was added afterward. Private
diagnostic evidence is retained in the ignored plan
workspace so the measurements and controls remain auditable.

The unrelated gear scheduler was left unchanged after baseline controls established
the existing bounded-search limitation. Its cost is that those constrained probes
still report no route. Additional earning mechanics and broader gear scheduling
remain separate work; the approved design explicitly excludes inventing them.
