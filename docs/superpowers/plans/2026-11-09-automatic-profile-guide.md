# Automatic profile guide implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Automatically recover from a saved guide that does not fit the logged-in character.

**Architecture:** Reuse `Import.Fits` at saved-choice, assignment and explicit-selection boundaries. Preserve the existing automatic selector and per-route progress storage.

**Tech Stack:** Lua 5.1 / LuaJIT, existing WoW addon UI and smoke harness.

**Spec:** `docs/superpowers/specs/2026-11-09-automatic-profile-guide-design.md`

## Global constraints

- Match race and class and faction, including existing group members.
- Keep a valid existing choice; do not guess among multiple matching guides.
- Preserve saved per-route progress and imported guide data.
- Do not rewrite live SavedVariables or change planner eligibility.

## Review focus

- Wrong race with the correct class must be rejected independently of wrong class.
- A same-name replacement can invalidate a previously compatible choice.
- A group guide must remain available to a matching secondary member.
- An incompatible legacy assignment cannot supply the fallback.
- Rejecting a direct selection must leave the current route and progress intact.

### Task 1: Correct guide-selection boundaries

**Files:** `addon/Factoruide_Guide/Guide.lua`, `addon/Factoruide_Guide/Options.lua`, `addon/tests/smoke.lua`, both changelogs.

**Interfaces:** Consume `Import.Fits(route)`, preserve `Guide:Choose(name)` and `Guide:FollowChoice()` and the existing `SwitchProgress` behavior.

- [x] Add regression cases using real imports and logged-in identity, observing the wrong saved guide survive before the fix. Cover all review-focus cases and the guide-list Follow state.
- [x] Require compatibility in `AutoRoute`'s legacy assignment, `PickRoute`'s saved choice and `Guide:Choose`. Disable incompatible Follow buttons.
- [x] Run `luajit addon/tests/smoke.lua` and `luajit addon/tests/smoke.lua profile`; require both suites to pass. Run StyLua and diff checks.
- [x] Obtain an independent review, address findings, and commit the focused correction.
- [x] Install the changed addon source files with hash verification and integrate to `main`. The desktop app only checks the addon version (`app/src-tauri/build.rs:95`); no planner executable rebuild is needed.
