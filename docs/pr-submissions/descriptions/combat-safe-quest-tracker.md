The quest tracker can inherit protection from its secure quest-item buttons on newer WoW clients. Refreshing protected lines and tabs during combat then triggers blocked UI actions. Keep their geometry and visibility stable during combat, defer new rows and secure buttons, and apply the layout and zoom after combat.

Validation: the source regression fails against upstream and passes with the fix. The full guide smoke suite passes using a scratch probe that normalizes only the generated-settings CRLF comparison; the unmodified native guide run retains upstream's Windows SettingsSchema byte-comparison failure. Native profile smoke and StyLua (Windows checkout line endings) pass. No in-game client validation was run.

The profile-export JSON parsing subcheck was skipped because `python3` is unavailable in this Windows environment.
