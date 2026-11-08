# Reputation dungeon prerequisites and cave sources

The saved v3 guide runs Wailing Caverns at level 22, then turns in Raptor Horns
at level 24. Smart Drinks (1491), which requires Raptor Horns (865), is omitted
despite its recorded +100 Ratchet reward and the selected Ratchet target.

Dungeon preparation follows unfinished prerequisite chains only for mandatory
quests. Reputation relevance does not make every quest mandatory. Separately,
the objective builder treats Smart Drinks as instance work because of its quest
category, ignoring placed outdoor Devouring Ectoplasm sources of Wailing Essence.

A replay also exposed a second scheduling failure: after accepting Smart Drinks
and entering Wailing Caverns, the planner immediately prepared another forced
instance before executing the current instance's objectives. Its one-visit model
then made the unfinished collection unavailable. The actual request's Wailing
Caverns planning window is 14–23; a separate last-level regression is also fixed.

## Intended behavior

- Finish executable work inside the current dungeon before scheduling another
  forced run. Keep existing level, acceptance and objective validity checks.
- Before a dungeon run, pursue feasible prerequisites of quests that help a
  selected reputation goal still below its target, as well as mandatory quests.
  Preserve normal restrictions, blocked branches, quest log limits and level
  bounds. Already satisfied or unrelated reputation targets do not add waits.
  At the last permitted level, useful reputation preparation remains eligible
  only when an ordinary simulated next step stays within the run's level limit.
  Mandatory-only release behavior stays unchanged, including mixed-root chains.
- For mixed inside/outside item sources, retain the instance path while its run
  is enabled or forced. When the instance is excluded or disabled without a
  forcing reason, a usable placed outdoor path may use the ordinary costed item
  model. Keep instance-only objectives inside. Never turn cave collection into
  free completion or invent a source. Preserve group/elite eligibility and costs.
  Preserve the existing outside-only quarry/group handling. Dynamic alternatives
  after a run and outleveled-instance fallback need wider modeling and are deferred.
  An instance forced by another exact required quest counts as enabled: resolve
  this from actual settled requirements and reconstruct from fresh metadata.
  Outdoor container drop rates must come from providers with actual outdoor
  positions; an instance-only source cannot improve an outdoor drop estimate.
- Finish Raptor Horns before accepting Smart Drinks; collect and turn in the
  required essence. Protect useful reputation work from automatic level skips.
- Do not change reputation rewards, farming estimates, daily rules, or the
  one-visit instance representation. Original saved guides remain untouched.

## Approaches

The selected approach fixes generic prerequisite preparation and a conservative
source fallback, with regressions for the reported chain. Unconditionally
replacing instance work with an outdoor farm discards the available group path;
the instance remains preferred when enabled. Forcing all dungeon
quests adds unrelated work. Allowing arbitrary instance revisits expands the
route model unnecessarily and does not fix the discarded outdoor source.
No quest-ID-specific scheduling overrides are needed.

The user requested this fix after the two defects were explained and previously
delegated design approval/review. Implementation proceeds under that authority.

## Delivery

Run focused regressions, the Rust workspace checks and independent review.
Replay/generate a scratch route using the actual saved request to check the
reported quest, without replacing the user's guide. Rebuild the desktop planner
and CLI; update installed addon files only if this fix changes addon code.
