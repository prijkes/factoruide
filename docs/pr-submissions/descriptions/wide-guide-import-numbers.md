Pasted guide imports can fail when WoW's integer formatter is limited to signed 32-bit values. Serialize exact integer Lua numbers with a floating-point integer format so wide masks and numeric table keys survive import and reload.

Validation: the regression reproduces overflow before the fix and passes afterward, covering signed 32-bit boundaries, wide masks, numeric keys and fractional values. The full guide smoke suite passes with a scratch LF-normalized settings-comparison probe; the native guide suite retains upstream's Windows CRLF comparison failure. Native profile smoke and repository-configured StyLua checks pass.

The test uses upstream's current guide format 2. This change does not bump addon versions.

The profile-export JSON parsing subcheck was skipped because `python3` is unavailable in this Windows environment.
