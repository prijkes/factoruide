# Supported required gear and accurate quest warnings

## Intent and confirmed failure

The user authorized fixes after a saved Tauren Warrior 18–60 guide reported “Mandatory class quests missed: 25.” Diagnosis found fourteen gear-reward quests and eleven prerequisites, all introduced by required BiS; the applicable Warrior class requirements were satisfied. Unsupported prerequisite chains were ranked as guaranteed gear sources, and a successful route could omit the selected mandatory gear goals.

The user explicitly chose: **“Choose the best supported alternative and explain the fallback.”** This is the policy for an unsupported preferred source. No private saved request or character data will be committed.

## Approach

Merely renaming the warning would leave unsupported goals and incomplete required routes. Making every existing `mandatory` quest an exact `required` quest would change unrelated class, dungeon and completion-window behavior. Instead, select coherent supported gear-source plans before ranking, enforce their actual completion separately, and preserve explicit reporting provenance.

Required continues to mean the selected **quest sources**, as documented by `BisMode`; it does not promise every mutually exclusive reward item or every vendor purchase. Vendor recommendations remain optional purchases when affordable and useful. The leveling endpoint stays unchanged; unsupported target-only endgame chains fall back to supported leveling rewards. Explicit exact quest requests retain their existing behavior.

## Source eligibility and selection

Add a nonmutating prerequisite selector under `model/feasibility.rs`. It receives modeled quests, dungeons, skipped reasons, profile, known class prerequisites, and the recorded starts in member order. Do not use the group union of completed quests as evidence every participating member completed a prerequisite. No recorded start means no invented completion of an unsupported gear prerequisite.

Resolve `pre_all`, the original `pre_need` distinct alternatives, `pre_held`, handed objective items, initial completions/accepts, symmetric exclusivity, breadcrumbs, effective level floors, explicit windows, dungeon windows and selected exact/class constraints. Held parents require an accept before the child, not a completed parent. Accept/objective/turn-in action ordering must reject pure completion cycles while permitting supported parent/child held-item patterns. N-of-M selection is transactional and backtracks through conflicting choices. Use deterministic order and bounded work; exceeding a limit is reported as a selection limit, never as proof of impossibility.

Build a residual action closure: initially completed quests, already accepted quests and recorded completed objectives cut their satisfied historical dependency edges. Reuse the current objective-progress matching rules rather than inventing a second matcher. Audit earlier `drop_unhanded` filtering so it does not discard a logged objective already satisfied by recorded progress before source selection can consider it. A held parent must remain accepted until the child's accept; it need not remain unfinished through the child's turn-in unless an actual objective dependency says so.

Retain all quest givers of an item until selection. Rank item/source candidates deterministically, preserving the existing item tie rule; an unsupported lower-index giver must not hide another supported giver of the same item. Commit one compatible source closure across bands/slots, deduplicating roots and sharing alternatives. Library tiers 10/20/25 of 38 reuse one growing book set. Mark only selected supported closure quests mandatory in Required; Prefer filters sources but creates no hard goals, and Off preserves its existing path.

Support must respect the simulator's one-continuous-visit rule for each dungeon. Validate residual action topology before ranking: an unavoidable inside-D action followed by an outside hand-in/accept and a later inside-D action requires unsupported re-entry. Keep ordinary sequential NPC/objective work entirely inside the same visit eligible. Blackbox `Objective.dungeon` credit is stricter still: acceptance must precede the sole entry. Distinguish it from a legal direct objective whose location is inside. Recorded progress may cut the earlier visit dependency; alternative selection must backtrack before declaring no supported source.

A failed conservative class witness does not authorize a direct conflict with that selected class goal or its unavoidable support. Keep exclusion protection while allowing genuinely unrelated gear when an independent class goal has unsupported data; do not mistake a plain-gear estimate for proof that the class goal can be abandoned.

Keep the final equipment band, but reject uncompleted actions requiring the target level or an already elapsed dungeon window. A pre-target source remains eligible in the target band. This is structural support, not proof that travel, XP, timers, money and log capacity can be scheduled; simulation remains authoritative.

Emit deduplicated human-readable fallback notes naming rejected sources, their support reason, and the selected alternative or absence of an alternative. Use existing calculation notes. Do not silently clear an unsupported goal after selecting it.

## Shared interfaces

`model::BisQuestAction` is a model-owned ordered action enum: `Accept(u32)`, `Objective { quest: u32, objective: u8 }`, `TurnIn(u32)`. `model::BisQuestPlan` contains `root: u32`, `quests: Vec<u32>` and `actions: Vec<BisQuestAction>`. It represents a selected root and its supported action closure, including held-parent ordering.

`Bis` exposes `required_roots() -> &[u32]`, `required_plans() -> &[BisQuestPlan]`, `requires(quest: usize) -> bool` for selected support membership, and `fallback_notes() -> &[String]`. Existing `gives`, entries and vendor methods keep their meanings. Required roots are specific sources; completing an unrelated exclusive alternative is insufficient.

Here `requires` means **turn-in completion support**: a selected root or support quest with an unsatisfied TurnIn action. `BisQuestPlan.quests` includes all action-support participants. A held-only parent can have an Accept action without acquiring a fabricated turn-in goal, mandatory flag or missed-gear count. Independent genuine completion requirements on that parent keep their own origins.

Participants include recorded held parents even when their accepted state needs no new action. Align source-plan residual acceptance with runtime initialization: the existing start importer may treat unbegun logged quests as abandonable/retakable. Preserve a selected source plan's actual logged state instead of reintroducing a historical prerequisite the recorded acceptance already satisfied. Keep the BiS-Off start policy unchanged.

`Bis::source_actions() -> &[BisQuestAction]`, `source_edges() -> &[(BisQuestAction, BisQuestAction)]`, and `source_quests() -> &[u32]` expose the finalized coherent source DAG/participants in Prefer and Required. The global edges are authoritative when later roots add cross-root ordering, including a child's accept before an earlier selected held parent's turn-in. The planner must not infer ordering by concatenating stale root-local action lists. These getters also allow runtime to preserve a selected source's recorded acceptance in Prefer without creating hard goals.

The final action order is also a visit-contiguous support witness. Dependency edges alone do not permit arbitrary inside/outside interleaving; the adapter must retain one continuous visit per dungeon and verify the actual locations chosen by simulation. Multiple modeled location alternatives are structural choices, not a promise that the nearest-place heuristic automatically reproduces the witness.

`Bis::source_visit(action: BisQuestAction) -> Option<Option<usize>>` retains the exact successful global witness's location category: absent action, explicitly outside, or a modeled dungeon index. Required placement filters actual location and objective choices to that category, then retains normal nearest/cost selection within it. Repair must use the retained visit rather than infer it from the first location. This does not introduce a stop kind or a new eligibility policy.

## Construction and enforcement

Repair missing required gear by inserting a whole selected action closure before the leveling stop, not isolated reward hand-ins. Reuse initially completed objectives/quests and actual recorded log progress. Keep existing gameplay constraints and validate each candidate transactionally. Permit extra cost when hard-goal fulfillment improves, while protecting already satisfied exact/gear goals.

Action adapters must synthesize/reuse legal dungeon runs: a model dungeon objective is not a directly valid `Kind::Objective` stop. Accept required quests before the one relevant run, respect existing run windows, and do not run the dungeon twice or remove a forced run. Preserve the chosen held/handed objective ordering.

Merge source actions and continuous visit blocks into the existing route at appropriate levels. Do not move completed support behind its unchanged dependents. Imported ordinary dungeon companions retain cross-quest constraints, including breadcrumb order, as well as their own action order. Outside work and different dungeon visits need not form one contiguous block; early deadline work must remain separable from later combat work.

Preserve a known gear-complete candidate over a cheaper incomplete one in search, cleanup/finalization and tournament ranking. Do not add random draws, change search operators/defaults, or use an arbitrary huge penalty as a hard constraint. Do not retain multiple full-state runs or alter cache/memory-cap policies. BiS Off must not pay extra route simulations for this logic.

Carry compact hard-goal rank information through search best tracking, sharing (`Exchange`/`Member`), qualifying rounds and final ranking. A scalar-score sharing slot must not discard an expensive complete route. While a current Required route is incomplete, scalar estimate rejection must not hide hard-goal progress; use exact evaluation where necessary. Preserve the original fast evaluation path when there are no accepted Required roots.

Both `validate_required` and `validate_required_trace` check actual source completion, including valid initial completion. Intended turn-ins after simulation ends do not count. A successful Required outcome must satisfy all accepted source roots. If supported goals cannot be scheduled, fail with named gear sources and “no route was found,” rather than claiming game impossibility or returning a silently incomplete success. Do not extend the leveling endpoint to finish endgame gear.

## Reporting and compatibility

Track mandatory origins on quests as a compact named mask for class, gear, forced dungeon, exact and forced zone. Seed class provenance at the existing applicable selected-class expression, not from class-category labels or power. Propagate new origins through already-mandatory shared prerequisites until stable, and retire released/version origins consistently. Gear provenance follows the selected source closure, including chosen N-of-M branches.

Add `Breakdown.mandatory_goal_counts: Option<MandatoryGoalCounts>` containing `class_quests`, `bis_quests`, `other_quests`. Fresh simulation emits `Some`, including zeroes. Keep `missing_class_quests` as the explicitly deprecated historical non-zone aggregate for compatibility/scoring; never use it for a new class claim. Category counts may overlap for shared origins; each legacy representative still contributes only once to the existing penalty. Preserve exclusive-version deduplication and existing zone/run/profession/reputation scoring.

`other_quests` means non-zone mandatory goals without class or gear origin, including a defensive fallback for missing provenance. Held-only action support is not a missing completion goal. Class/gear provenance on a genuine zone goal can be reported while the existing zone counter and penalty remain separate and unchanged.

Old guides deserialize with categorized counts absent. Show their aggregate under neutral legacy wording, without guessing categories or replaying/replacing the guide. New warnings use authoritative class/gear/other counts and explain that gear counts include prerequisites. New per-quest notes use actual simulated completion and correct origins. Comparisons must not present old aggregate25 to new class0 as an improvement in class quests. When either side is unclassified, compare the legacy aggregate under a generic mandatory label.

Preserve existing plain warning containers and calculation-note controls, reading order and keyboard behavior. Warnings identify category/count in text; do not add redundant alerts/live regions. Update the small warning/help translation set and Required help to describe supported quest sources, explained fallbacks and advisory vendors accurately.

## Verification and delivery

Use compiling functional RED tests before product changes. Cover unsupported winners, completed/missing prerequisites, multiple givers, cycles/held ordering, N-of-M backtracking/shared tiers, exclusivity, member completions, target floors, actual source validation, expensive required repairs, feasible candidate retention, provenance overlap/penalty invariance, truthful legacy warnings and comparison behavior.

Use the private saved request only in ignored diagnostics. Prepare it to inspect supported fallbacks, then generate a bounded real route and verify every accepted source against actual turn-ins. Preserve original request/guide files. Run appropriate Rust package/workspace checks, frontend tests/build/lint/i18n, independent task and final reviews, and a desktop application build. Keep app launch/replacement of active sessions, installer creation, push and merge outside this task.
