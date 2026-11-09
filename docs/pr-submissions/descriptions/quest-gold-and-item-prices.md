**AI disclosure:** This contribution was created with AI. The submitter reports in-game testing of the combined fork only and has not reviewed the source code. These split branches have not been individually verified in-game. Automated checks below were run by AI; no human source-code review is claimed.

Quest rewards and item prices are now available to the planner, including an `fg gold --mangos ...` importer. Guides show projected quest income, training costs and flight spending, with each turn-in carrying its money reward. Starting copper is retained and already completed quests are not paid again. Databases without the new columns continue to load with zero/unknown amounts.

The packed data retains every current upstream nonmonetary record and all 27 newly added Forever quests. Only the imported monetary columns and their provenance tokens are overlaid; VMaNGOS attribution is retained.

AI-run automated checks: starting/earned copper and exported turn-in money, completed-quest replay, and old/new database column loading are covered by regressions. Rust lint/type checks and frontend build/tests/lint/translations pass. The selected Rust packages passed 510 tests, with two ignored tests and the known Windows Lua round-trip newline fixture excluded.

Part of the [22-topic contribution series](https://github.com/prijkes/factoruide/blob/pr/submission-plan/docs/pr-submissions/README.md).
