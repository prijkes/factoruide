# Independent review: abandoned objective-work accounting

## Decision and evidence boundary

A narrowly scoped correction exists for the demonstrated loss of unrewarded work at abandonment. Use a bounded scalar for irrevocably unrewarded work and a small, copy-on-write per-active-quest provenance ledger for legacy objective work. Preserve the existing direct-kill ledger and its reward protection.

This is a source review and patch design, not an implementation or a fresh test result. I read the diagnosis plan, accounting report, the four authentic-route oracles, the shared-kill tests, and the state, simulator, objective execution, settlement and incremental-evaluation paths. I ran no build, test or planner and changed no production file.

The coordinator reports two passing controls and two failing timeout cases, both with grinding 0 instead of 96 seconds. Inspection independently supports the oracle: four actual kills add 96 seconds and 200 XP; the independent 400-XP reward ends the route without fallback grinding; neither expiration nor abandoning supplies a reward. With zero allowance and weight 4, retaining those 96 seconds as grinding adds 288 weighted seconds above player time. Changing acceptance flags cannot make the performed kills productive. Genuine reward of another actual shared beneficiary can.

Do not edit production accounting until the coordinator has captured the archived fixed-attempt route and cleanup boundaries under the old semantics. These finite oracles establish an accounting defect. They do not establish that it caused the archived cleanup loss of roughly 31 percent. That requires the separate old-semantics reproduction and boundary reconciliation.

## Why the two paths lose work

- `State::settle_kill_work(quest, false)` removes the abandoned beneficiary and deletes an entry when its beneficiary set becomes empty. No retained scalar represents that final unrewarded work.
- `Simulator::count_unfinished_as_grind` reconstructs legacy work only for `accepted.and_not(turned)`. `abandon` clears acceptance and marks the quest turned/dropped, removing it from that scan.
- Broadening that scan to dropped quests is unsafe. `initial_state` calls `hold` for imported completed objectives, and `start_state` abandons imported quests omitted from the route. Their objective bits do not prove that this route performed any work. Imported partial progress also makes full-objective nominal estimates wrong.

## Recommended patch

### State and settlement

Add `orphaned_work: f64`, initially zero. It represents performed objective work whose last potential reward beneficiary has disappeared. It never stores quest IDs or individual events.

Add `legacy_work: Rc<Vec<(u32, f64)>>`, initially empty, sorted by quest index. A small named struct is equally suitable. Record positive seconds by binary-search insertion or accumulation. Maintain at most one row per quest with performed, unsettled legacy work.

In the existing direct settlement method, keep reward behavior exactly as it is. On abandonment, remove the beneficiary; immediately before deleting an entry whose beneficiary list has become empty, add its seconds to `orphaned_work`. Entries with surviving beneficiaries remain pending. Continue canonical sorting and merging after beneficiary removal. Do not transfer an entry merely because one beneficiary disappeared.

Settle the matching legacy row through the same real abandonment and successful-turn-in paths: remove and add its seconds to `orphaned_work` on abandonment; remove without adding on a genuine turn-in. There is only one beneficiary for legacy work. The completion-error guard in `turn_in` must stay before settlement. Repeated settlement of an already removed row must do nothing.

Do not add legacy reward seconds to `rewarded_kill_work`. That scalar currently protects gross direct-kill work against the nominal/overlap cap; it is not a generic sum of all productive activities.

### Capture actual provenance at objective execution

Inside `objective_work`, the already computed `kill_work + other` is the actual gross addition to `spent.fighting` for the selected objective place. It already incorporates:

- the share imported through `started`, removed at this execution;
- this place's remaining kill count and current combat time before XP/level changes;
- uses and their existing partial-progress behavior;
- stock consumption and the resulting actual fixed extra time.

For `o.shared_kill_target().is_none()`, record that value for this quest in `legacy_work`. The existing direct branch continues recording its actual shared chunks through `share_direct_kills`, with no duplicate legacy entry. Take the value from the executing `o`, not the canonical model objective or `spot_work` estimate. `spot` selects the place; `spot_work` estimates cost and does not prove execution.

Do not record in `hold`: imported progress, shared credit and dungeon auto-completion also call it without performing this legacy objective work. Do not reconstruct costs at abandonment. Do not take the outer `apply` time delta, which includes travel, overhead and other activity.

The recommended record is the pre-`together` gross fight addition. `together` may save gathering time against preceding or following kills, so a net per-stop delta is order dependent and cannot independently replace the current aggregate overlap treatment. Preserve that treatment in this narrow fix.

### Final accounting

Replace the nominal completed-objective loop with:

`pending direct seconds + pending legacy seconds + orphaned_work`

Then retain the existing cap against `max(spent.fighting - rewarded_kill_work, 0)` and the existing transfer into grinding/grind-over. Do not also retain the legacy nominal loop: doing so double counts performed legacy work and preserves the imported-progress problem.

This intentionally changes legacy estimates to the actual work recorded during execution, including selected alternatives and partial initial progress. It does not change physical time, XP, objective completion, timers, dungeon runs, travel, reward behavior or search defaults. Start-item acquisition work in `accept` is outside the currently demonstrated completed-objective accounting defect; do not silently add it to this patch.

### Incremental evaluation and memory

Initialize both fields in `initial_state`. Add `legacy_work == old.legacy_work` and bitwise scalar equality for `orphaned_work` to the exact suffix-state comparison in `evaluate.rs`. Keep the current direct ledger and reward-protection comparisons. Otherwise two apparently identical completed/dropped states with different performed liabilities can reuse the wrong suffix total.

Use copy-on-write storage as for `kill_work`. Remove legacy rows on either settlement and aggregate by quest, so storage is proportional to active quests, not route length or the number of performed objectives. The abandoned history costs one scalar. Ordinary planning further bounds active quests by the quest log; explicit routes remain bounded by the model's quests. Do not alter cache limits, retain historical per-objective entries, or add route clones. Existing shared-ledger subset bounds remain unchanged.

## Why not reuse the direct ledger unchanged?

Adding `record_kill_work(vec![quest], kill_work + other)` in the legacy branch is attractive but unsafe. Every rewarded entry is currently added wholesale to `rewarded_kill_work`. For example, an unrewarded 96-second kill batch overlapped with a subsequently rewarded 48-second gathering objective can have only 96 physical fight seconds. Protecting the gathering's gross 48 seconds would reduce the available unrewarded classification to 48, although the actual kills still lack a reward. The existing direct mechanism deliberately protects kill time, which gathering cannot erase.

A unified ledger could work by carrying separate direct-protection seconds in each entry and merging/settling both amounts. That adds mixed semantics to every existing shared chunk and its equality/aggregation code. A separate one-row-per-quest legacy ledger has a clearer review boundary and leaves the known shared-kill algorithm untouched. Both approaches still require the orphan scalar.

## Required focused checks

1. Run all four existing abandoned-work oracles. Both timeout cases should become 96 seconds of grinding, with unchanged physical time, XP and real timeout events. Both controls must remain green.
2. Import a fully completed legacy objective and omit its quest from the route, exercising real `start_state` abandonment. Add unrelated productive route work so a nonzero fighting cap cannot hide fabricated prior work; expect no imported work to become grinding. Also cover a retained imported complete quest that expires later.
3. Import half of a four-kill legacy objective, execute its remaining two kills, then leave it active or expire it. Expect only the two actual kills' time and XP. A genuine turn-in of the same quest should remove its pending legacy liability.
4. Exercise an alternative legacy place with a different real cost and/or stock-reduced extra time. Assert the recorded classification equals the performed cost, not the canonical objective estimate.
5. Abandon two real direct beneficiaries in succession: the first abandonment retains pending work; only the last transfers it, exactly once. Keep one-abandoned/other-rewarded and reward-before-other-abandon cases productive.
6. Retain `unfinished_gathering_cannot_reclassify_rewarded_direct_kills` (48 fighting, 72 grinding). Add its expired-owner variation and gathering-before/after variants. Check rewarded gathering does not acquire direct-kill protection and thereby erase the pending kill liability.
7. Compare ordinary full simulation against incremental evaluation after moves that change timed abandonment or performed legacy work. Cover both pending legacy rows and settled orphan liability, not only the existing shared-progress test.
8. Exercise multiple actual legacy objectives on one quest and successive completed/abandoned quests. Confirm per-quest aggregation, settlement removes rows, and the scalar retains the correct total. Do not replace authentic simulator oracles with synthetic ledger entries for the main classification claims.

## Scope limit: existing overlap approximation

This correction preserves the existing aggregate overlap cap and its direct-reward protection. It does not claim exact ownership of every saved gathering second across arbitrary combinations of multiple productive and unproductive legacy quests. That broader accounting is already approximate: a global cap can distribute savings across unrelated work. A general exact solution would require tracking which gathering work the active `Together` group saved, retaining its ownership until the group is settled, and handling rewards/abandonments consistently with those reallocations. That is a separate model change, not necessary to stop authentic performed work disappearing solely because its last beneficiary is abandoned.

The narrowly correct claim is conservation of recorded unrewarded work across abandonment, exclusion of imported prior work, correct genuine shared-beneficiary settlement, and preservation of the existing overlap/protection rule. Do not advertise this patch as a complete redesign or exact solution to arbitrary overlap allocation.
