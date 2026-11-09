**AI disclosure:** This fix was created with AI. The submitter reported the original failure in-game and has previously tested the combined fork in-game, but has not reviewed the source code. This fix and its standalone PR branch have not yet been confirmed in-game. Automated checks below were run by AI; no human source-code review is claimed.

A Horde rogue guide can direct the player to sell loot to Kayla Smithe (5749). Her NPC vendor flag passes the existing faction/flag filter, but opening her grimoire shop requires a warlock.

For Classic and WoW Forever, exclude the known warlock-only Demon Trainer/Master shops and Gorzeeki Wildeyes (14437, who has no title) unless every member of the group is a warlock. Ordinary merchants and legitimate trainer/vendors remain eligible under the existing faction and vendor-flag checks. TBC behavior remains unchanged because its source menus do not corroborate the same class gate.

The [VMaNGOS source snapshot](https://github.com/vmangos/core/releases/download/db_latest/db-sqlite-4641790.zip) gates merchant menus 4782 and 5818 with condition 67, a race/class condition whose class mask is 256. [The condition implementation](https://github.com/vmangos/core/blob/a88734fb88d19f5e8df6fca766729e419cf6bec0/src/game/Conditions.cpp#L236) checks that mask; the merged NPC data currently retains the vendor flag without the merchant-access condition. The Forever report corroborates Kayla's refusal; the wider shop family uses the verified Classic baseline.

AI-run automated validation:

- The new regression fails against upstream: a Classic rogue receives Kayla, Greshka and Gorzeeki as loot vendors. The TBC control already passes.
- With the fix, the complete `fg-route` suite passed **499 tests** using `cargo test --profile fast -p fg-route --locked -- --skip the_addon_reads_them_and_they_read_the_addon`. Two doctests are ignored; the one filtered test is the existing Windows newline-sensitive Lua round-trip fixture.
- The regressions cover solo rogues, solo warlocks, mixed-class parties in either leader order, all-warlock parties, a legitimate trainer/vendor, faction filtering and unchanged TBC behavior.
- `cargo clippy -p fg-route --all-targets --locked -- -D warnings`, `cargo fmt --all --check` and whitespace checks passed.

This is a standalone commit based directly on upstream `main` at `1c8e835`. It changes vendor selection, adds focused fixture/regression coverage and an Unreleased note. Packed databases, addon files and release versions are unchanged.
