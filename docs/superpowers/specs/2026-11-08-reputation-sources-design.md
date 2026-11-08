# Repeated reputation sources and best-effort goals

## Intent and accepted decisions

Extend reputation planning beyond one completion per quest: repeated turn-ins,
explicit creature farming, and faction spillover. Preserve leveling and other
mandatory content when a reputation goal cannot be reached. Return the best
supported reputation result found and explain every remaining shortfall.

The user explicitly chose daily activities with required-day reporting and
Classic fallback estimates with clear warnings. The latest instruction
supersedes the earlier requirement to reject guides with unmet reputation.
Design and implementation review are delegated to the agent in this session.

## Approach

Use an ordinary leveling route followed by a costed reputation-completion
program. This preserves the existing single-lifecycle quest search and supports
all three requested earning mechanisms. A joint search that interleaves arbitrary
repeatable activities with every quest could improve efficiency, but would
require replacing quest identity throughout construction, cleanup and editing.
The completion program is the chosen initial approach. No globally fastest route
or proven maximum reputation is claimed.

Work in managed worktree `feat/reputation-sources`, based on main `0e531b4`.
Do not change the installed app/addon until implementation passes review and
verification. Preserve the user's saved requests, generated guides and settings.

## Best-effort behavior

- An unreachable, unsupported or search-limited reputation target never aborts
  an otherwise valid guide. Continue other factions and leveling.
- Prioritize the selected targets; retain useful reputation work and its
  prerequisites. Complete attainable goals and improve remaining deficits.
- Reaching the selected rank is sufficient; do not farm beyond its target just
  to fill a reputation bar. Stop sources at their eligibility/cap/resource limits.
- Return forecast starting/ending totals, target, deficit, provenance and a
  concrete limitation per unmet faction. Distinguish an exhausted search budget
  from a demonstrated source cap. Call results best-found, not proven maxima.
- Preserve errors for invalid requests and missing mandatory quest, gear or
  dungeon content. This change concerns reputation goals only.

## Source data and confidence

Ship a provenance-bearing reputation catalog alongside the existing data.
Read current NPC locations, item drops, quest prerequisites, objectives and
prices from the selected edition's database. Keep earning-rule confidence
separate from whether an acquisition path is supported.

- Existing bundled quest reward vectors are resolved effects: apply exactly once
  and never infer another layer of spillover from them.
- Explicit primary awards may use explicit, non-recursive spillover templates.
  Preserve signed secondary losses, recipient caps and suppression flags.
- Classic-derived rules on Forever are estimates. Mark them in calculation
  notes, route summaries and affected activity instructions. The user permits
  these estimates; provide a way to disable them without changing targets.
- Default the fork's missing-rule fallback to enabled per the user's choice.
  Prefer confirmed edition-specific records over estimates.
- No earning rule, NPC, material supply, price or unlimited repeat schedule may
  be invented to make a target succeed.

Initial data includes the four city Runecloth repeatables and other imported
repeatable quests with actual material paths; the sourced Classic outdoor kill
and spillover catalog; and the 150 primary-cache Durotar daily writ definitions.
Correct quest 94214's missing +75 Durotar reward through maintained data rules.
Preserve source URLs/builds, release hashes, patch selection and licenses.

All Durotar starting writs currently lack acquisition data, and most enders are
unknown. Classification as a daily is not permission to grant free turn-ins.
There is no known repeatable direct Horde-67 earning source in the audited data.
These gaps must produce warnings and partial results, not fabricated success.

## Common reputation accounting

Introduce one transition engine for source effects and spillover. Track the
closure of factions needed for selected goals, source conditions and spillover.
Unselected faction losses and gates remain relevant.

Each event identifies its edition, source, confidence, signed awards, reward
encoding (resolved or primary), eligible standing ranges, cap semantics,
modifiers and spillover policy. Apply modifiers once. Human bonuses affect gains,
not losses. Preserve fractional resolved quest rewards.

Classic kill estimates retain the pinned baseline semantics: pre-kill inclusive
rank gates; full final award can cross a rank boundary; ordinary spillover shares
the primary gate; separate team awards follow their own rule; per-kill spillover
and team halves truncate before batching. Gray-level and rounding assumptions
are explicit source metadata, never silently applied to resolved quest vectors.

Batch simulation must be equivalent to ordered individual events across caps,
rank boundaries, losses and spillover changes. Optimization may skip constant
segments, but cannot multiply an initial reward across a changing cap.

## Reputation program and costs

Keep ordinary `Stop` identity unchanged. Persist a compact program with stable
source IDs and activity/run IDs. Its actions identify exact creatures, repeated
quest deliveries, material acquisition, and required setup. Replay everything on
the same character state, then finalize accounting once.

- Split ordinary target-level completion from terminal accounting. The existing
  unfinished-work conversion is not idempotent and must not be run twice.
- Explicit known-creature kills may earn reputation. Generic zone grinding or
  unidentified on-way kills do not acquire invented creature reputation.
- Repeated turn-ins consume materials for every completion, including starter
  items. Track shared stock explicitly; the same item cannot fund two activities.
- Obtain missing materials from a supported exact farm or priced vendor. Charge
  combat, respawn, travel, interaction and money costs. Unpriced AH purchases are
  unsupported; the existing generic common-item time is not a free supply.
- Imported bag inventory may supply real materials when newly recorded. Missing
  inventory fields in older profiles mean zero known stock, not assumed items.
- Respect quest exclusions, side/race/class, level, prerequisites, standing gates,
  mob difficulty and supported group-credit rules. Explain excluded candidates.
- Item drops and Classic rules yield forecasts. Guide execution checks real
  inventory and standings, and must never declare an estimated reward completed.

Use bounded search over source programs, with batches at target/cap/resource
boundaries and explicit limits. Retain a usable best partial program. Do not
stop all work merely because one faction has no usable source. Account for
interacting gains/losses; check selected standings at the terminal state.

## Daily activities

Group daily actions into reset circuits: do each eligible source once, then wait
for the next reset before repeating the circuit. Never schedule one daily quest
for all days before doing the other daily quests.

Persist compact circuit phases with a repeat count, followed by any partial tail.
Show active play time separately from daily completion opportunities and reset
transitions. These are minimum requirements for the computed program, not an
exact completion date or a global optimum. Use recorded server reset timing and
daily completion state when available. Label unknown/stale first availability;
do not assume midnight, an unverified global daily limit, or zero waiting.

One-time introductions such as Shipping Label do not become repeatable. A daily
source needs new materials and a valid source on every cycle; unknown future
replenishment cannot be treated as unlimited stock.

## Saved guides, editing and continuation

One combined replay produces the trace, forecast totals, costs, calendar facts,
exported guide and continuation character. Persist the program in saved plans
and share files; reopen/edit/save must replay it without losing or duplicating
effects. The continuation character must include final reputation, money and
remaining tracked materials instead of inheriting stale initial values.

Keep existing guides readable. Bump all guide-format constants together because
older addons silently pass unknown step kinds. Preserve released-guide fixtures.

## Addon execution

Add protected activity steps for repeated deliveries, exact farming, daily
circuits and final reputation checks. All activity/support steps have `kp=true`.

- Count repeated deliveries from matching `QUEST_TURNED_IN` events for the active
  run. Persist progress across reload/navigation and route switching; never use
  permanent completion flags as a repeat counter.
- Permit repeatable acceptance/turn-in automation only for the active expected
  source while work remains. Unrelated NPC offers/turn-ins cannot advance it.
- Check actual inventory for material gathering and actual faction standing for
  reputation progress. XP or estimated kill counts cannot prove standing.
- Daily circuits track each source's completion in the current server-reset
  window, then wait until the next opportunity. Route-version migration cannot
  carry incompatible counters into a new program.
- A best-effort guide may finish with disclosed reputation shortfalls. Final
  checkpoints report the actual result; an unsupported target must not trap the
  player on an impossible mandatory final step.

## Verification

Use meaningful test-first regressions for nonblocking shortfalls, progress on
other factions, capped/lossy/spillover events, resolved-vector no-double-credit,
per-kill rounding, material consumption and balance, daily circuit ordering and
counting, replay/edit/share parity, continuation state and addon repeated events.
Run current supported source probes and the user's saved request without
overwriting it. Report remaining model/data gaps by faction. Then run appropriate
Rust, frontend, localization and Lua checks, independent review, merge, push and
build/install the deliverables authorized earlier in this session.
