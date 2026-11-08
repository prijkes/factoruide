# Upstream issue audit, 8 November 2026

All eight open issues in `brouznouf/factoruide` were checked against local
`main`, source `843e617c3dc1bbfb2801049dd361527f6937fdc5` (product integration
`20c96ab`). The fetched upstream `c27f59d` is already included; local main was
63 commits ahead and zero behind when assessed. The issue snapshot was retrieved
through the GitHub connector at 02:50 UTC on 8 November 2026, including all comments.

Several reported problems are already addressed by upstream fixes now merged
into our branch. Other issues contain both fixed and outstanding parts. Our QoL
changes do not resolve every open report.

## Findings

| Issue | Current branch | Evidence and remaining scope |
| --- | --- | --- |
| [108: Mandatory quest minimum levels](https://github.com/brouznouf/factoruide/issues/108) | Reported mechanism fixed | The explicit minimum of a required dungeon quest now gates the dungeon run. The Hall of Thanes regression proves a minimum of 18 prevents running it at 16. The reporter's original guide was unavailable. |
| [114: Class quests for new race/class pairs](https://github.com/brouznouf/factoruide/issues/114) | Partly fixed | All 22 documented race gates are open for the four reported race/class pairs. Actual database route probes complete the Orc Mage examples. Other variants remain deliberately unverified and some remain closed. |
| [124: Very long unfinished quest with grinding disabled](https://github.com/brouznouf/factoruide/issues/124) | Scoring fixed; spawn coverage remains | Unrewarded objective work is counted as grinding, including fork safeguards for shared work and abandonment. The planner still counts spawns within a 200-yard pool rather than modeling movement among distant groups. The historical exact 12-hour route was not replayed. |
| [125: Forced dungeons omit quests](https://github.com/brouznouf/factoruide/issues/125) | Main examples pass; related quest still fails | Replaying the attached request completes all 18 listed quests and each of the eight forced dungeons once. The four C dead-drop quests also plan successfully. Related quest 92401, A Frightened Request, fails preparation because the real database has unknown exploration coordinates. |
| [126: Quests supplied by guildmates](https://github.com/brouznouf/factoruide/issues/126) | Not implemented | The planner still prices normal quest-giver pickup travel. There is no option to value dungeons assuming shareable quests are supplied by party/guild members. Group support and addon acceptance automation do not supply that planning feature. |
| [128: Ahead of planned level and time](https://github.com/brouznouf/factoruide/issues/128) | Unverified | Relevant XP and group accounting improvements exist, but neither the duo report nor the solo follow-up includes a matching saved request/profile/guide. There is insufficient evidence to declare the level or elapsed-time mismatch fixed. |
| [129: Import completed quests](https://github.com/brouznouf/factoruide/issues/129) | Capability already present; instructions need correction | Character profiles carry completed quest IDs through import into planning, where they are excluded from repeat completion. Released-addon import and completed-quest tests pass. The addon export window still directs users to the removed Characters page. |
| [130: Map zoom and selected-step routes](https://github.com/brouznouf/factoruide/issues/130) | Requested controls still absent | Zoom, current-zone filtering and selected-step highlighting already exist, but the zoom cap is unchanged and there is no selected-only or multi-step route filter. These controls predate the report and our QoL changes. |

“Fixed” above describes the tested mechanism or named examples, not every possible
configuration or a verified live-game result. No issues were commented on, closed
or otherwise changed during this assessment.

## Evidence for the remaining problems

### Forced-dungeon quest 92401

The original attachment excludes A Frightened Request. With that exclusion
removed and the quest required by level 18, planning exits before optimization:

```text
cannot plan required quest 92401: quest 92401: exploration area unknown
```

Its current `m_quest_objective` contains the trigger coordinates
`{"16611":[[-1,-1]]}`. The location resolver at
[`objectives.rs`](../crates/fg-route/src/model/objectives.rs) rejects this because
there is no usable cluster. The synthetic dungeon fixture instead has valid
Undercity coordinates, so its passing test does not cover this database defect.
The report's older late-acceptance/double-run symptom was not reproduced: the
current failure happens earlier.

### Spawn coverage and grinding

[`model/crowd.rs`](../crates/fg-route/src/model/crowd.rs) still uses a 200-yard
spawn pool. Shared respawn waits improve work at one camp but do not simulate
roving among camps during one objective. The current database still has the
Imperial Plate Belt's 20-bar objective, approximately 9.68% drop chance and 14
Fel Interloper spawn points in Burning Steppes. The historical selected
two-spawn cluster and exact wait were not re-created.

The separate scoring exploit is covered: upstream `9dff261` is merged, and
[`plan/step.rs`](../crates/fg-route/src/plan/step.rs) classifies actually performed
unrewarded work as grinding. Fork regressions also cover abandonment and avoid
charging rewarded shared work again. This agrees with the
[maintainer's distinction between the fixed scoring issue and remaining spawn coverage](https://github.com/brouznouf/factoruide/issues/124#issuecomment-6625860168).

### Quest sharing, maps and calibration

[`job/group.rs`](../crates/fg-route/src/job/group.rs) retains a quest as already
accepted only when all eligible group members who need it already carry it.
Otherwise [`plan/places.rs`](../crates/fg-route/src/plan/places.rs) resolves pickup
to the quest's normal giver locations. The addon can confirm certain shared
quest dialogs, but the planner has no party-supplied quest source option.

[`GuideMap.tsx`](../app/src/components/GuideMap.tsx) caps viewer zoom at 24 times
the whole-continent fit. Its Layers menu offers the whole guide or current zone,
plus move-type and marker toggles. Selecting a step highlights its path while
the zone path remains visible. The supplied screenshot is of this generated-guide
viewer, not the separate generation preview. These files are unchanged by our
fork relative to the merged upstream snapshot.

For issue 128, internal XP tests cannot establish agreement with a player's
actual session. A matching saved guide and character data, edition, XP buffs,
group settings and a step where actual/predicted XP diverge are needed to
diagnose it. The solo follow-up means group configuration alone cannot explain
all reports. Predicted combat/travel time also remains an estimate, not a
measurement of the reporter's play pace.

## Completed-quest workflow

1. Enable the matching edition's Factoruide_Profile addon. In game use `/fg`
   and **Save the profile now**, or log out, to flush the current profile.
2. In the desktop app's **New guide**, choose the recorded character under
   **Start from a character**. The game folder and edition must be configured
   for profile discovery.
3. Alternatively use `/fg` → **Export my character**, copy the text, and use
   **New guide → Paste a character's export…**. This works for web import too.
4. Check the completed-quest count and profile timestamp before generating.
   Save/export again after further play to use updated history.

[`Export.lua:143`](../addon/Factoruide_Profile/Export.lua#L143) still calls the
destination the Characters page. Current import lives in New guide; that wording
is a concrete usability defect. A blank/new character has no imported history.
In a group, a quest may correctly remain for companions who have not completed
it, even when another member has.

## Verification and limits

- Read all eight issue bodies and comments, inspected current source and
  database data, and compared relevant ancestry with upstream.
- Focused existing Rust tests passed: unfinished/abandoned/shared work,
  respawn waits, XP buffs/accounting, dungeon level gates, forced quest chains,
  race access, released-addon profile import and completed-quest behavior.
  These were run using the current release test executables; no rebuild was needed.
- The issue 125 v3 attachment was parsed as JSON data and replayed with the same
  gameplay settings except effort reduced from 3 to 0. It completed all 18
  reported quest accept/turn-in pairs, with no missing goals or forced dungeons.
  Search used eight constructions and no optimization tries. This demonstrates
  feasibility for that request, not all seeds or original-effort route quality.
- Additional full-database probes completed the C chain and Orc Mage quests
  1943 and 1960. Requiring 92401 reproduced the preparation failure above.
- Database SHA256 was
  `64b05f5c05046ca404a4b81a3585a29fffbea674fdc7168a6d93c8c768ed2d14`.
- No product or test code was changed, no app/game was launched, and nothing
  was deployed or published. Raw issue snapshots, probe requests, logs and
  specialist reports are retained in the ignored local workspace
  `.superpowers/sdd/2026-11-08-upstream-issue-audit/`.

The remaining concrete bug candidates are the invalid 92401 location data and
stale import instructions. Map controls and party-supplied quests remain feature
requests; spawn coverage remains a planner limitation. Issue 128 needs a
reproducible session comparison before choosing a fix. The remaining class/race
variants need availability evidence rather than blanket unlocking, as explained
in the [maintainer's issue 114 comment](https://github.com/brouznouf/factoruide/issues/114#issuecomment-6625857037).
