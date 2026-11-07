# Required reputation and missed quest acceptance

## User decisions and scope

The user selected **Require selected reputation targets**, even when fulfilling
them needs detours or work after reaching level 60. The required outcome is the
faction total, not completion of every quest that rewards that faction.

The user selected **Warn for any offered planned quest left missing** when an NPC
interaction ends. This includes manual acceptance and quests automation did not
attempt, rather than only failed calls to `AcceptQuest()`.

The user also asked what the checkpoint skip message means and requested merging
completed branches into `main`. The merge is complete at `f9dde21`; all local
branches are ancestors of `main`, and release workspace verification passed.
No push was requested or performed.

This document proposes the implementation below for review. Product changes have
not started. Original saved requests, outcomes and guide state stay untouched.

## Confirmed diagnosis

The latest local saved request starts at level 18 and selects eight reputation
targets of 42,000 points. It has no exact required quests. Its Undercity record
starts at 1,217 and the saved route reaches 15,592. Argent Dawn is not selected.

In the bundled Forever data, The Argent Emissary (96895) rewards 150 Argent Dawn
reputation. Proof of Demise (374) rewards 100 Undercity reputation. Neither quest
appears in the saved exported steps. Proof of Demise passes the raw race
restriction and is not recorded complete or in the starting quest log, but its
quest level 7 fails the model's lower level cutoff: `7 + 10 < 18`. The later gray
quest exemption for reputation cannot recover a quest already removed here.
Other source and gameplay conditions remain authoritative after this filter is
corrected; inclusion in the model does not promise selection of that quest.

Selected reputation currently contributes finite score penalties. It is absent
from the actual hard-goal rank and final success validation. The existing ending
condition permits reputation work after the target level, but construction can
stall and a route with unmet reputation can still be returned successfully.

The checkpoint banner is a manual offer. The number counts steps, not quests.
There is no dedicated switch to hide it. Separately, `Guide:IsDone` automatically
passes steps planned at least four player levels earlier if their quest is not
in the quest log. That automatic rule ignores exported keep protection, and the
exporter does not explicitly protect reputation contributions.

Ordinary accept steps already wait for quest-log/completion evidence. Calling
`AcceptQuest()` alone does not complete such a step. Missing feedback is a
separate defect: offered planned quests are not audited after the conversation.

## What the current model can promise

The model uses bundled quest reputation rewards and at most one completion per
quest ID. It does not derive spillover or plan repeated farming. An optimistic
sum of all positive available-quest rewards, ignoring race, level, completion
history, prerequisites and exclusive alternatives, remains below 42,000 for all
eight selected factions. Undercity's bound, including the recorded starting
standing, is 25,122. These generous bounds are about the supported model, not
proof that Exalted cannot be reached in the game.

| Selected faction | Starting points | Optimistic endpoint | Target |
| --- | ---: | ---: | ---: |
| Orgrimmar | 9,306 | 34,369.75 | 42,000 |
| Undercity | 1,217 | 25,122 | 42,000 |
| Thunder Bluff | 9,426 | 27,435.75 | 42,000 |
| Darkspear Trolls | 7,937 | 21,749.25 | 42,000 |
| Booty Bay | 1,237 | 8,061 | 42,000 |
| Ratchet | 2,375 | 7,469 | 42,000 |
| Gadgetzan | 1,237 | 5,091 | 42,000 |
| Horde | 3,500, estimated baseline | 7,420.25, conditional | 42,000 |

Horde (67) is missing from the snapshot's reputation array. Its bound uses the
current race/class baseline fallback rather than a measured current standing.
Any diagnostic based on such a fallback must label the starting value as an
estimate. Do not describe this faction's result as a snapshot-confirmed bound.

The existing shortfall message only compares this route's result with the target;
it does not establish that the game has insufficient quests. Correct its wording
for previously saved guides. Supporting additional earning mechanics is separate
work requiring edition-specific evidence; this change must not invent rewards
to make the requested targets appear satisfied.

## Approach

The two policy alternatives were required outcomes and a time-versus-reputation
tradeoff. The user chose required outcomes. Implement that choice by extending
the existing hard-goal and replay validation machinery, while retaining numeric
reputation weights as search guidance. Increasing a finite penalty does not
enforce a required outcome. Marking every reputation quest mandatory would also
misrepresent a required total and conflict with alternatives.

For acceptance, the two offered approaches were verifying only automatic
attempts and auditing all planned quests actually offered. The user chose the
latter. Keep one bounded interaction audit, independently of automation being
enabled. Use existing Lua event and quest-log adapters without new dependencies.

## Reputation implementation

1. Retain otherwise eligible low-level positive-reward candidates for selected
   factions and their prerequisite candidates. Bypass only the lower level
   cutoff for this purpose; preserve upper/minimum-level, race/class, configured
   exclusion, supported-source and prerequisite rules. Carry relevance through
   prerequisites so a gray zero-reward prerequisite can unlock a useful reward.
   Do not turn the candidate set into an exact mandatory quest list.
2. Extend terminal hard-goal ranking with unmet reputation goals and a deficit
   progress signal, based on actual unrounded simulated reputation. Activate this
   path for reputation-only requests, including when gear is Off. Audit search
   acceptance, sharing, tournament selection, replay fallback, cleanup and final
   route retention for existing gear-only guards. Preserve each satisfied
   faction individually so a gain elsewhere cannot conceal its regression.
3. Make construction/repair pursue feasible contributions and their selected
   prerequisite actions for still-unmet factions. Permit a more expensive route
   when it improves required reputation, while preserving existing exact quests,
   forced runs and required gear. Validate insertions by normal simulation.
4. Validate faction targets at finalization and against an editor's actual
   explicit replay trace. Planned but unperformed turn-ins do not count; nor may
   an editor validation invent automatic actions absent from its trace.
5. An unmet required target prevents a successful new outcome. Report the
   faction, starting value and its provenance, target, best actual result and
   deficit: no satisfying route was found with the supported data and settings.
   Keep previously saved guides available. Never silently lower or clear a goal.
6. Update reputation goal help, weight help and legacy shortfall wording to
   distinguish required outcomes, search guidance and an old guide's shortfall.
   Label selected-faction start fallbacks rather than silently claiming a
   measured value. Preserve stored fractional reward values: the integer reward
   parser currently rejects an entire reward array when one component is
   fractional, also losing integral rewards in that row. For example, quest
   91285 stores `[[67,12.5],[68,50],[76,12.5],[81,12.5],[530,12.5]]`.

Early rejection is allowed only with a conservative, correctly scoped upper
bound and honest provenance. A bound above a target does not prove feasibility.
No-target routes keep their existing fast path and search behavior. Do not change
search operators, random draws, unrelated defaults, or cache/memory policies.

## Skip protection

Export keep protection for the planned contributions to selected factions and
their actual planned prerequisite work throughout the route, including routes
without a generated checkpoint. Both the manual checkpoint offer and automatic
outdated-step rule must respect it. Reuse the exported keep flag if its existing
semantics fit; ensure existing training, class and gear protection still works.

Conservatively keep all such planned contributions after the live target may
have been reached; dynamic release using live reputation is outside this change.
Explicit user skips, actual completion and unavailable exclusive alternatives
keep their current meanings. No new skip toggle is proposed: the user's question
is answered by explaining the optional banner and protecting required work.

## Missed acceptance implementation

Maintain a route-specific, bounded interaction token and the union of identified
planned quest IDs actually offered in gossip, greeting or quest detail. This
collection happens independently of Shift, auto-accept and guide-only automation
settings. Never audit unrelated quests merely because they are near the NPC.

Treat close events as possible transitions to another quest panel, not immediate
proof that the interaction ended. Reconcile only after the conversation closes
and a short grace interval permits delayed quest-log updates. Reopening or
continuing the same conversation cancels the pending close. A new interaction
must not cancel or merge the previous NPC's unresolved audit accidentally.

At reconciliation, recheck the same route, the actual quest log/completed state,
and whether the planned accept is still needed. Respect explicit skips, held
book handling, profession eligibility and exclusive alternatives. Normalize
`QUEST_ACCEPTED` arguments through the existing `arg2 or arg1` contract. Clear
stale state when the guide changes and deduplicate timer callbacks with tokens.

Proposed presentation, an implementation choice: one yellow screen message plus
chat history per closed interaction, naming missing quests and inviting manual
acceptance. Repeated events for that interaction do not repeat the message. A
new visit can warn again. Use existing localization patterns, no new sound,
automatic retry or manual-navigation block. Respect the existing guide visibility
setting, consistently with other guide advice.

Older greetings may not supply quest IDs. Identify those quests safely from
available metadata or subsequent detail; do not guess ambiguous titles or warn
for a quest that cannot be identified. Document any unavoidable coverage limit
and exercise the repository's supported edition adapters in the smoke tests.

## Verification and delivery

Use failing behavioral regressions before implementation. Independently review
the reputation and interaction-audit changes, then review their integration.

Required coverage includes old low-level reward roots and zero-reward ancestors;
prerequisite alternatives; already-completed and excluded quests; a costly but
satisfying route beating a cheap incomplete route; reputation-only requests;
post-target work; per-faction cleanup preservation; actual edited-trace results;
honest unsupported failure; no-target compatibility; fractional input if valid;
and keep flags protecting both skip paths without requiring a checkpoint.

Acceptance coverage includes a failed accept with unchanged normal advancement,
two offered quests with only one accepted, delayed confirmation within grace,
manual/Shift mode, edition event payloads, close/open transitions, a different
NPC or route, manual skip/completion, deduplication, and legacy missing IDs.

Run the relevant Rust tests and workspace release checks, addon Lua smoke suite,
frontend tests/build/lint/localization checks, and a production desktop build
with `tauri/custom-protocol`. Regenerate diagnostic routes only in ignored
storage; never overwrite saved user guides. Integrate reviewed passing work into
local `main` as already requested. Do not push or replace an active app session.
