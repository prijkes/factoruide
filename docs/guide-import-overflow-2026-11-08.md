# Guide import integer overflow — 8 November 2026

Fixed the addon error `integer overflow attempting to store 8589934770` in
Factoruide Guide 0.16.2. Factoruide Profile remains 0.16.1; guide format remains 3.

The latest saved v3 guide contains that number in the race-restriction mask for
Additional Runecloth. Its duration is 15,945,962 seconds (displayed as 4,429 hours),
which fits within a signed 32-bit integer. The failure occurred when saving the
decoded guide: the importer's `%d` formatter converted its wider mask through a
client integer type.

The importer now uses `%.0f` for exact integral Lua numbers below `2^53`, matching
the existing profile exporter. Masks retain their complete numeric value and
type; fractional values retain the existing lossless formatting path. This change
does not alter route calculations or strip reputation metadata.

## Verification

- A compressed import regression first reproduced the exact overflow under a
  signed-32-bit formatter shim, then passed with the fix. It checks the reported
  mask, positive/negative 32-bit boundaries, `2^53 - 1`, a wide numeric table key
  and fractional values through import, saving and reload.
- The user's saved v3 route was rendered to Lua in scratch storage. The original
  importer reproduced the exact error; the fixed importer passed compressed
  import/save/reload with all 3,077 steps and 68,230 numeric values unchanged.
- Full Lua smoke, all three released guide fixtures, profile-only smoke and
  formatting passed. Independent review approved the narrow change.

Original guides and character snapshots were read only. The corrected addon is
installed in the authorized Forever AddOns directory, preserving existing guides,
configuration and SavedVariables. Reload the UI with `/reload` and paste the same
guide again; regeneration is unnecessary.
