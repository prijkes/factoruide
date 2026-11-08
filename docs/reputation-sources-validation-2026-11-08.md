# Reputation completion validation — 8 November 2026

Reputation targets now return a best-found guide and named remaining deficits
instead of rejecting useful leveling or reputation progress. Selected targets
retain their requested values. Mandatory quest, gear and dungeon constraints
continue to be checked.

The guide can include counted repeated turn-ins, exact supported material
acquisition, identified kill reputation, signed losses, explicit spillover and
daily circuits. Saved guides, sharing, editing and addon playback preserve the
programme and its repetition counts. Unknown writ supplies and unpriced purchases
cannot fund free deliveries. Classic-derived rules are labeled as estimates.

## Real database probes

The ten-target probe combined the saved v2 configuration with the current imported
level-21 character and the ten Exalted targets from the reported error. The exact
failed request was unavailable; this is a representative reconstruction. Original
profiles, settings and guides were read only.

The initial reconstruction correctly refused its outdated mandatory Ragefire
Chasm selection: the current character exceeds that dungeon's configured maximum
level. A separate scratch variation removed only that forced dungeon. The other
21 selected dungeons and required gear setting were retained.

That variation returned a finite format-3 guide with 959 quests and 3,354 steps in
123.08 seconds at effort 0.01. Undercity reached 42,028.25 and Argent Dawn reached
42,000. The other eight targets retained explicit deficits. Durotar Supply and
Logistics stayed at the recorded 2,000 without invented writ replenishment.

Real-source daily probes used explicitly hypothetical stock for quest 94214:

| Scenario | Supported result | Daily completion windows | Reset transitions |
| --- | --- | --- | --- |
| Two writs and two required potions; target 150 | 150 reputation | 2 | 1 |
| Same finite stock; target 300 | 150 reputation, deficit 150 | 2 | 1 |
| Today's completion already recorded | Two future deliveries | 2 | 2 |

These probes verify the maintained +75 reward, consumption of starter and
material items, circuit scheduling and partial progress. Calendar waiting remains
unknown when first availability and a reliable reset clock are unavailable.

## Supported limits

This bounded search does not prove a maximum reputation total or a globally
fastest route. Current completion travel uses supported walking paths; it reports
missing cross-continent transport setup. Ordinary leveling retains its transport
instructions. A source with multiple quest NPCs currently uses the named NPC's
verified spawns and reports that other NPC alternatives were not searched.

Farming time is deliberately conservative. It uses a verified spawn rather than
treating distant global spawns as a free local pool, and intervening travel does
not offset later modeled respawn waits. The ten-target example consequently
forecasts about 4,608 active hours, with 87.6% attributed to waiting. This is a
limitation of the time forecast, not an in-game measurement or an optimality claim.
Actual standing and bag contents determine addon completion.

Unknown loot ownership prevents automatic material farming for groups; known
inventory and supported vendor purchases remain available. Missing prerequisites,
standing caps, source eligibility, resources and explicit search limits appear
in the programme notes. Polestar exports reject counted reputation programmes
and direct the player to Factoruide Guide, which preserves their execution state.

## Verification

The focused reputation suite passed 92 tests, including the shared reward kernel,
catalog binding, programme construction, persistence and editor replay. Review
regressions cover level-crossing prefix XP parity, signed Classic bounds, stock
replay, bounded malformed circuits and delayed addon completion events.

The final Rust release workspace run passed 860 tests with zero failures. Four
existing tests were ignored (two client-dependent tests and two documentation
examples); the known Windows Lua round-trip newline test was filtered. All-target
workspace Clippy passed with warnings denied, and Rust formatting passed.

The frontend passed 15 tests, TypeScript/Vite production build, lint and 1,227
translation checks. Full and profile-only Lua smoke tests passed, including three
released guides. All tracked Lua passed formatting on newline-normalized scratch
copies; source files were preserved. The standard Windows harness skips its
optional POSIX Python probe. A separate Windows verification adapter also passed
the full smoke test with real Python JSON parsing and inventory/daily shape
assertions. The existing frontend bundle-size notice remains.

Independent review approved the implementation after the material findings were
resolved. A 333-note regression also verifies that planning notes produce a short
in-game summary, while estimate notices, current problems and actual shortfalls
remain visible.

## Delivery

The tested implementation was merged into `main`. The desktop was rebuilt with
`tauri/custom-protocol`. Both addons were installed at
`F:\Program Files (x86)\World of Warcraft\_forever_\Interface\AddOns`, version
0.16.1: 66 payload files verified, 12 changed. The four existing
Config/Routes/Services/Sent files retained their original SHA-256 values.
SavedVariables were not accessed or modified.

The desktop executable is `target/release/factoruide-app.exe`, 160,185,344 bytes,
SHA-256 `590acd0b064dbe800a77b43797ce9f16dbd7452d130a3f6ddea1613763fe6358`.
The addon archive is `target/release/Factoruide-addons-b55eb87.zip`; installation
manifests and artifact hashes are retained in the local validation evidence.

Restart the game fully to load the new TOC modules, refresh the character profile
to capture current bags and daily availability, then regenerate the guide. When
reusing the saved v2 settings with the current level-21 profile, remove their
outdated forced Ragefire Chasm requirement first.
