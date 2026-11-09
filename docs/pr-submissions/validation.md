# Composition validation

Checks below apply to the complete prepared tree `df9fd779eef0411c26ec9acdfda484045e9f5c59`, at `pr/reputation-dungeon-quest-order` commit `4ec7b7a`. The upstream base is `1c8e835`. Validation ran on Windows; installed application and addon files were unchanged.

## Release tests

```powershell
cargo test --workspace --locked --release -- --skip the_addon_reads_them_and_they_read_the_addon
```

The command exited successfully: **888 passed, zero failed, four ignored and one filtered out**, including workspace package tests and doctests. The ignored tests are two game-client-dependent contribution/cache tests and two documentation examples. The filtered test is the known Windows line-ending-sensitive Lua round-trip fixture; its baseline failure was reproduced separately during preparation. No other tests were excluded.

Release validation used an isolated build directory. It did not replace the executable used by the installed application.

## Frontend and formatting

- `cargo clippy --workspace --all-targets --locked -- -D warnings` passed, including the desktop and CASC packages.
- `cargo fmt --all --check` passed.
- Frontend dependency installation and production build passed; the existing large-chunk warning remains.
- All 15 frontend tests passed.
- Oxlint with warnings denied passed.
- Translation validation passed for 1,246 texts. The two obsolete Dungeon guide catalog entries were removed because upstream replaced that page with objective guides.

## Addon

The complete guide smoke suite passed with a scratch wrapper that normalizes CRLF to LF only in the generated-settings comparison. It includes acceptance warnings, wide-number import, three released guide fixtures and reputation programme playback. Native profile smoke passed. StyLua 2.5.2 passed on LF-normalized snapshots using both repository ignore files; no tracked source was modified for these checks.

The unmodified guide smoke suite retains the exact upstream Windows failure at `SettingsSchema.lua differs from settings.json`. Its optional POSIX `python3` JSON-parser subcheck was skipped, and the harness printed the existing Windows missing-path messages. No in-game validation was performed as part of this submission preparation.

## Branch composition and review

Every independent branch has exactly one commit above the upstream base. Each dependent topic has exactly one commit above its declared parent. Exact heads, parents and per-topic validation appear in [the manifest](manifest.json).

All twelve cumulative snapshots passed their Rust all-target checks with the lockfile, TypeScript checks, frontend tests, translations and formatting. The eleven topic deltas also passed whitespace checks. These stage checks complement the release test of the final combined tree.

Independent review approved the extraction, the monetary-data overlay and the combined topic boundaries. Across all twelve cumulative snapshots, the three added economy regressions, the two generic dungeon/drop-rate fixes, the active audit entries and upstream objective-guide entry points remain present. All 281 upstream test names in the modified existing Rust test files remain. The complete source audit assigns all 203 original nonarchive paths and classifies all 82 authored nonmerge commits and six merges.

## Platform limits

The WASI-thread dependency probe reports no `parking_lot_core` package in the web backend graph. The allocator stress test is intentionally gated out on Windows; Unix/WASI stress execution and Miri were not run here.

Optional `cargo-deny`, ShellCheck and Zizmor tools were unavailable on this host. Packaging, installation and live game validation were outside the preparation task. Per-topic descriptions distinguish focused checks from the combined release run.
