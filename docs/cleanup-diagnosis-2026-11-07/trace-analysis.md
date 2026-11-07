# Exact chain cleanup trace analysis

## Finding

Deleting the sole accept stop for quest **387, Quell the Uprising**, removes a class-training opportunity. That changes subsequent combat gates, adds early grinding and later a large grind before a timed quest, and reaches level 40 before the route's final fifteen turn-ins. This is a concrete search-versus-cleanup mismatch involving an incidental service. The captured loss is not evidence that quest 387's abandoned kill work was undercharged: it performed no objective work at all.

This is read-only analysis of `chain-original-semantics.json`, SHA-256 `8d6715590cb809fa4305eadd78a104e1aa64568587f4f6eecd066b9a018d9c90`. The artifact reports `recorded_observables_equal: true`; the coordinator separately verified archived replay gates. No planner, build, or counterfactual simulation was run by this analysis worker. The input artifact was not modified.

All event and route indices below are zero-based. Quest IDs are game IDs, not internal model indices. Quest names and the fact that 97914 has no objectives were checked against `data/forever.sqlite` using a read-only SQLite connection.

## Exact finite reproducer

The artifact's `TrimTarget.stops` has 917 stops. `DropDungeonless.stops` has 916 and is exactly equal to the former with only route index **99**, `{ index: 253, kind: Accept, quest_id: 387 }`, removed. No other stop identity or order changes. The named quest's required dungeon is 717, The Stockade; that dungeon is absent. The quest never turns in.

| Measurement | Before deletion, TrimTarget | After deletion, DropDungeonless |
| --- | ---: | ---: |
| Score seconds | 159260.4500628052 | 227141.55534933577 |
| Player seconds | 135899.53006959998 | 148245.27811607535 |
| Hidden penalty seconds | 23360.91999320523 | 78896.27723326042 |
| Trace events | 1186 | 1140 |
| Actual turn-ins | 319 | 303 |

The exact score increase is **67881.10528653057**, including **12345.748046475375** additional player seconds. The previously reported approximately 67802 cleanup increase brackets the whole cleanup stage from an earlier search score; this pair isolates the dungeonless deletion itself.

The five actual dungeon identities are unchanged: 1581, 721, 16732 (Excavation Site: Wetlands), 16544 and 722. No dungeon run is removed by this mutation. The finalization stage subsequently improves the score to the archived final result.

Correction from final source review: the initial diagnostic metadata counted only `Event::Enter` and omitted explicit `Kind::Dungeon` stops, which are recorded as `Event::Stop`. The preserved full chain traces include the explicit run for area 16732 at every stage; SelectedInput records it at trace time 42452.98515070625, level 26. That is the observed stop-event time, not necessarily the run's entry/start time. The five-identity statement above corrects the earlier four-identity enumeration using those existing full traces. Original captures and their SHA-256 hashes remain unchanged. Updated derived dungeon metadata must include both event forms and retain only the first actual occurrence per dungeon index, since an implicit entry may precede an explicit stop for the same dungeon.

## First divergence: the removed quest was a training visit

The traces are identical through event **117**, a flight into Stormwind at time **11231.256316676723**, level **22**, XP **9618**.

Before deletion:

- Event **118** accepts quest 387 from Warden Thelwater at **11301.161080034703**, still level 22 / XP 9618.
- Event **119** trains at trainer index **7**, at **11414.4944104616**, with `since: 20` and outbound travel **41.66666521344866** seconds. Power increases from **21.25096165450597** to **22.00522105780094** while gear remains **0.005221057800943071**. This is a spell-training change, not gear or quest XP.
- Event **120** learns the Sentinel Hill flight path at **11768.56583903303**, XP **9712**, with the new power.

After deletion, event **118** is already the Sentinel Hill flight-path stop at **11579.137263244804**, XP **9712**, but power is still **21.25096165450597**. Both the accept and the training trip have vanished. No XP divergence has occurred yet.

Source mechanism: `Simulator::do_apply` calls `train` after a stop (`step.rs`), and `train_class` decides from the character's current position and the next stop whether training is near enough or warrants a dedicated trip. The effective `train_radius` is 300 yards, with gain thresholds 0.03 and 0.12. The visit to Warden Thelwater creates an eligible training opportunity; deleting it removes the call from that location. Training is an implicit replay event: `Kind` has no explicit class-trainer stop.

## Early consequence: Howling in the Hills needs an extra level

At the accept of quest **126, Howling in the Hills**, both routes are level **23**, XP **14995**, and have the same gear contribution **0.17823906695293087**. Their class power differs:

| Event | Before | After |
| --- | --- | --- |
| Accept quest 126 | event 217; time 21946.597421444476; power 23.17823906695293 | event 215; time 21808.164401114253; power 22.42397966365796 |
| Objective 0 | event 219; time 22102.53134589698; level 23 / XP 15465 | event 221; time 27167.11258031368; level 24 / XP 173 |

Between accept and objective, only the deleted-quest route now performs:

1. Event **216** farms 22 mobs for **2810 XP** on its changed trip, reaching level 23 / XP 17805.
2. Event **217**, at **26452.948249921206**, grinds **4081.162440293144 seconds** to level **24 / XP 0**.
3. Event **219** trains at trainer 7 at **26822.49405624498**, `since: 20`, raising power to **24.162222007548895**.
4. Event **221** finally performs quest 126's objective.
5. Event **222** abandons quest **97914, Expanding Horizons**, at **27169.11258031368**, level 24 / XP 173.

The full trace stores exact times; rounded/approximately labelled values here identify events rather than introduce another numerical oracle. Quest 97914 is a level-17 delivery quest with no objectives and no timer. Its abandonment follows the level increase and the `abandon_hopeless` gray-quest policy. Before deletion it instead turns in at event **240**, time **24068.277454213276**, level **23**, for **500 XP**. The early missing training thus changes leveling and other quest outcomes, not merely combat duration.

## Main amplification: A Meal Served Cold starts one level later

Later, the same ordered route reaches quest **212, A Meal Served Cold**, a timed quest with a database limit of 1800 seconds. Its accept is gated by the strength required for its future objective (`stop_gate` calls `timed_gate` for timed accepts).

Before deletion:

- Event **1045**, Accept 81947, is level **37 / XP 74102**, power **38.59718** approximately.
- Event **1046** grinds only **389.09960408804704 seconds**, reaching level **38 / XP 0** at **120450.27587232397**.
- Event **1048** accepts quest 212 at **120490.8949192164**, level 38 / XP 0, power **38.55868863225944**, gear **1.564474690313136**.
- Event **1049** trains at trainer 8 at **120543.03777635926**, `since: 36`, restoring power to **39.56447469031313**.

After deletion:

- Event **1049**, TurnIn 1421, already reaches level **38 / XP 2716**.
- Event **1051**, Accept 81947, is level **38 / XP 4460**, with power **38.55868863225944** approximately.
- Event **1052** farms another **1381 XP** (with additional exploration on that movement); the state is level **38 / XP 6171**.
- Event **1053** grinds **15869.068820898037 seconds** to level **39 / XP 0**, at **140143.12962155842**.
- Event **1055** accepts quest 212 at **140183.74866845086**, level 39 / XP 0, power **39.387672374886705**, gear **1.3934584329403994**.
- Event **1057** trains at trainer 8 at **140264.67539314253**, still `since: 36`, restoring power to **40.3934584329404**.

The source-supported interpretation is a training-sensitive gate discontinuity. `refresh_power` recomputes spell deficit at the current level. At level 37 the pre-gate non-gear bonus is approximately zero; at level 38 it is approximately **-1.0057860580537** because training is still at 36. `level_for` projects a required level using the supplied current `Might`; `grind_to_gate` grinds to the resulting gate and only then calls training. Thus reaching this position slightly earlier in XP progression can produce a smaller gate, while arriving already level 38 with newly missing spell ranks produces a large additional grind. The trace proves the gate outcomes and training order; exact internal gate inputs are not separately serialized, so this formula-level interpretation is distinguished from those observed events.

## Why quest XP drops by exactly 43335

After the large grind, the deleted-quest route reaches level 40 at event **1139**, quest **500, Crushridge Bounty**, time **148245.27811607535**, ending at **level 40 / XP 2359**. Before deletion that same turn-in is event **1129**, time **128538.12070605195**, only **level 39 / XP 9996**.

The route still contains the remaining stops. The simulator's normal `ended` condition stops execution once the target is reached. In the deleted-quest route, the quest-500 turn-in is stored route index **881**; the remaining **34** stops are not executed.

Fifteen missing tail turn-ins account for **41170 XP**:

| Quest ID | Quest | XP before deletion |
| --- | --- | ---: |
| 207 | Kurzen's Mystery | 3550 |
| 595 | The Bloodsail Buccaneers | 2450 |
| 606 | Scaring Shaky | 3300 |
| 597 | The Bloodsail Buccaneers | 2450 |
| 611 | The Curse of the Tides | 3150 |
| 628 | Excelsior | 2850 |
| 607 | Return to MacKinley | 2450 |
| 196 | Raptor Mastery | 2450 |
| 193 | Panther Mastery | 3150 |
| 600 | Venture Company Mining | 3300 |
| 627 | Favor for Krazek | 2100 |
| 599 | The Bloodsail Buccaneers | 820 |
| 202 | Colonel Kurzen | 3150 |
| 205 | Troll Witchery | 3150 |
| 622 | Return to Corporal Kaleb | 2850 |

Six retained turn-ins give less XP at their higher completion levels: quests **65: -270**, **168: -270**, **469: -165**, **659: -260**, **690: -390**, **942: -310**, totaling **-1665 XP**. Together with the lost **500 XP** from gray-abandoned quest 97914, the arithmetic is:

`41170 + 1665 + 500 = 43335` lost quest XP.

## Mob, farming and total XP reconciliation

| Source | Before | After | Delta |
| --- | ---: | ---: | ---: |
| Quest XP | 584260 | 540925 | -43335 |
| Mob XP | 275688 | 245996 | -29692 |
| Farming XP | 93599 | 88447 | -5152 |
| Exploration XP | 29575 | 27967 | -1608 |
| Grinding XP | 4078 | 86224 | +82146 |

The omitted tail after the pre-deletion quest-500 turn-in contains **8294 explicitly recorded farming XP**. Earlier farming gains in the changed route offset **3142** of that loss, giving exactly **-5152**. The same omitted tail contains **25843 combined non-farm, non-quest XP** before the final grind, comprising objective kills and exploration. Earlier level differences also change mob XP per kill and camp/travel choices. The final combined mob/exploration loss is **31300**; therefore **5457** of the combined reduction occurs before that common turn-in boundary.

The trace does not emit exploration as its own event or per-event XP-source breakdown, so it does not support an exact split of the tail's 25843 into mob versus exploration XP. The separate overall -29692 and -1608 values are authoritative stage breakdowns. It would be incorrect to attribute all -29692 to deleted objectives or to quest 387 itself.

Across all sources, `-43335 -29692 -5152 -1608 +82146 = +2359`, exactly matching the end-state difference: level 40 / XP 0 before, level 40 / XP 2359 after. This explains the lost quest and kill opportunities as displaced by earlier grinding, not missing XP in replay.

Explicit Grind events sum to **949.6917866019846** seconds before and **19950.23126119118** after. The final after-breakdown adds **84 seconds** of unfinished-work reclassification, reaching **20034.23126119118**. The complete grinding delta is **19084.539474589197** seconds; its excess-grind delta is **17281.294957602207** seconds. Their additional score contributions are **9542.269737294599** and **43203.23739400552**, respectively. These explain most of the score amplification.

## Relationship to the confirmed abandonment bug

Before deletion the only abandoned quest is 387. Its only route and trace action before abandonment is acceptance; its dungeon is not entered, no objective is performed, and it gives no kill or reward XP. It is abandoned at level 30, time **62146.701891112396**, trace event **574**. There is no performed work for its abandonment to erase.

After deletion the only abandoned quest is 97914, a delivery quest with no objectives. Its trace contains an accept and an abandonment, without objective work. The independently demonstrated last-beneficiary work-loss bug therefore has no demonstrated contribution to the two captured routes' cleanup delta. It remains a real separate defect and may affect other candidate search trajectories; that possibility does not make it this pair's cause.

## Source-based next options, without an implementation

Keep the dungeon/quest consistency rule. Keeping an impossible dungeon quest merely for its training side effect would preserve the symptom's score but violate the guide requirement.

The narrowest conceptual counterfactual is to remove quest 387 while preserving its actual training service. The present stop representation cannot express that independently: there is no `Kind::Train`. Two design directions warrant evaluation after the diagnostic capture:

1. Represent a class-trainer visit explicitly, so cleanup can retain an incidental service independently of the quest it removes. A finite oracle should remove an optional accept-only quest near a trainer, preserve the genuine trainer visit, and verify subsequent combat gates and quest consistency. This makes the service visible to search and cleanup rather than depending on a dummy quest.
2. Let cleanup preserve needed incidental services through a dedicated service-visit representation. This is closely related to the first option; a score guard alone is insufficient because it cannot keep the invalid quest. The captured before/after pair supplies an exact fixed-route oracle once such a representation exists.

Considering class trainers actually passed during travel could also avoid missed opportunities. However, current training is stop-based, and extending it to path opportunities changes ordinary simulation more broadly and needs a verified definition of "passed". It is a wider policy change than preserving a specifically observed training visit. The evidence supports evaluating explicit service preservation first; it does not yet establish the best implementation or prove that one restored visit eliminates every downstream loss without a counterfactual replay.

The late timed-gate behavior is another finite diagnostic target: compare the same timed accept just below and just above a spell-training level, while controlling gear and training. Do not change gate or training policy solely from this source interpretation. No counterfactual route, new stop kind, or production correction was implemented here.
