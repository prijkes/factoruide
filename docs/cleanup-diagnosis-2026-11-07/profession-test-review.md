# Scoped review: first profession trainer visit tests

Decision: **Approve**. No Critical, Important or Minor findings in this test-only adjustment.

I reviewed the diff in `tests/it/routes.rs` and `plan/tests/profession_trainers.rs`, plus `plan/profession_trainers.rs`, the real stop/level-gate training path, `export/writer.rs` and the profession text templates. Comparing the relevant trainer and export production files against `e73af02` produced no changes. I ran no builds, tests or planners and edited no production or test source. This approval concerns test validity; the coordinator will supply execution results.

## Why the changed expectation matches existing behavior

`profession_visit` computes every unlearned rank currently open from the character's level and the configured skill curve. The selected trainer teaches the intersection of those bits with its supported ranks. The method does not restrict a first visit to apprentice alone. `train_profession_at` charges one `train_time`, records one event containing the learned bit set, and marks all those ranks learned.

The writer exports the highest cap among the event's learned bits. Independently, it uses the initial `profession.learn` instruction whenever the apprentice bit is present; otherwise it uses `profession.rank`. Therefore a first visit with bits `0b11` correctly exports cap 150 and `Learn <profession> from <trainer>`. Requiring a cap-75 row was an assumption about which route happened to win, rather than a necessary condition for learning the profession under the existing simulator model.

## The initial-learning guarantee remains enforced

The integration test now finds the **first** trainer step for each exported profession and requires the explicit initial-learning text for that profession. This rejects both a missing visit and a next-rank-only first visit. It retains the disabled-option empty result, added route-time assertion, membership in the guide's skill lines, allowed caps, NPC target and world/map location, generic training instruction, and strictly increasing caps per profession. It does not merely relax the first visit to any available cap.

The focused regression independently ties that text to actual simulator state: a new character starts with zero ranks; executing a real level-gated accept stop reaches level 15/skill 50; the visit learns exactly `0b11`; exactly one lesson is charged and one `ProfTrain` event is emitted. Export then contains exactly one profession lesson with the correct skill line, cap 150, initial Tailoring-learning instruction, NPC target and world location. It does not manufacture learned ranks or a training event to satisfy the exporter assertion.

The old one-rank test still exercises a cap-75 trainer and one-time apprentice learning. Together, the tests cover both valid first-visit forms rather than removing apprentice coverage.

## Scope

The adjustment documents existing support for learning multiple open ranks during one visit. It neither changes profession-training behavior nor proves exact in-game skill progression between those rank lessons; that broader simulator model is unchanged. The supplied observation of Leatherworking first appearing at level 17 with cap 150 is consistent with the reviewed code, but I did not independently rerun that route. No accounting production adjustment is needed for this test correction.
