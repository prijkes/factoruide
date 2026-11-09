# Automatic guide selection for the current character

The user wants the addon to load the correct guide automatically for the logged-in
character. A Tauren Warrior's saved selection currently names an Undead Rogue
guide, although a compatible Warrior guide is imported. Quest eligibility in the
planner is correct; restoring the addon's saved choice bypasses compatibility.

## Behavior

- On login, import refresh, or `FollowChoice`, keep a saved choice only when
  `Import.Fits` matches the current race, class and faction, including group members.
- Otherwise use the existing automatic selection: the unique compatible guide,
  then a compatible legacy character assignment. With several matching guides and
  no valid choice/assignment, leave selection to the existing guide list.
- Reject incompatible explicit `Guide:Choose` calls without changing the active
  guide; disable their Follow buttons in the guide list.
- Preserve progress through the existing per-character, per-route `SwitchProgress`
  mechanism. A rejected selection must not clear or transfer progress.
- Apply these checks at runtime. Do not rewrite the user's SavedVariables or
  imported guide data, change planner eligibility, or add a new matching schema.

## Approaches

Use the existing `Import.Fits` predicate at each selection boundary. This is the
smallest correction and preserves matching/group behavior already used by the
automatic selector. Adding a separate profile-to-guide mapping would require
new export metadata and conflict rules; choosing the newest matching guide could
override an intentional selection. Neither is needed for the reported case.

The user requested automatic profile matching and previously delegated design
review/approval. This focused correction proceeds under that authority. Keeping
the existing ambiguity behavior is the implementation choice stated to the user.

## Verification and delivery

Exercise real imports, login/refresh, explicit selection, grouped profiles,
ambiguous choices and saved progress in the Lua smoke suite. Run both full addon
suites, formatting and independent review. Update both addon changelogs and install
the changed addon files to the existing WoW directory, preserving generated guide
files and saved game state. The running client can load the correction with
`/reload`. The desktop app checks the addon's version rather than embedding its
source, so this change does not require rebuilding the planner executable.
