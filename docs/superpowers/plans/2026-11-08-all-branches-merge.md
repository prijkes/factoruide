# All Branches Integration Plan

> **For agentic workers:** Use the executing-plans or subagent-driven-development workflow. Root owns Git staging and commits; workers own disjoint source files.

**Goal:** Integrate all local and fetched remote branches into local main while preserving the fork's reviewed features.

**Architecture:** Merge current upstream normally, resolve source conflicts semantically, and combine its updated database with the fork's money columns. Record already-rebased or equivalent remote branches with tree-preserving ancestry merges. Verify the resulting source before advancing main.

**Tech Stack:** Git, Rust, Lua/LuaJIT, TypeScript, SQLite and Python standard-library Zstandard.

**Spec:** User request to merge all branches, existing feature contracts in ASTRA_HANDOFF.md, and upstream commits through c27f59d.

## Constraints

- Preserve required reputation/gear validation, quest eligibility and skip protection, acceptance audit, gold/vendor/icon features and cache policies.
- Keep upstream quest data, XP/count observations, respawn accounting, dungeon-guide mode and forced-dungeon fixes.
- Preserve all signed/null money values and source attribution; do not discard either side of the binary database conflict.
- Do not restore deliberately superseded group-profile sharing or duplicate dependency updates.
- Preserve original saved guides and externally managed worktrees. No push requested.
- Author and committer remain `prijkes <prijkes@noreply.com>`.

## Tasks

- [ ] Audit remote branch equivalence and upstream semantic/data changes.
- [ ] Resolve four Rust conflicts and any demonstrated integration defects; preserve both behaviors and test dungeon-only normalization of fork settings.
- [ ] Resolve changelogs with fork additions under Unreleased; merge updated data with fork money columns and verify all other incoming data remains exact.
- [ ] Run focused and full Rust, frontend and addon checks, formatting and release Clippy; refresh the production build and addon archive.
- [ ] Independently review the integrated diff and database evidence; fix consequential findings and verify the corrections.
- [ ] Commit the upstream merge, record superseded/equivalent side-branch ancestry without tree changes, advance local main and verify every ref is integrated.
- [ ] Record final validation and handoff state, retaining private diagnostic evidence and backups.
