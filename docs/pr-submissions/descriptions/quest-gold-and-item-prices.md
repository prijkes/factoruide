Quest rewards and item prices are now available to the planner, including an `fg gold --mangos ...` importer. Guides show projected quest income, training costs and flight spending, with each turn-in carrying its money reward. Starting copper is retained and already completed quests are not paid again. Databases without the new columns continue to load with zero/unknown amounts.

The packed data retains every current upstream nonmonetary record and all 27 newly added Forever quests. Only the imported monetary columns and their provenance tokens are overlaid; VMaNGOS attribution is retained.

Validation: starting/earned copper and exported turn-in money, completed-quest replay, and old/new database column loading are covered by regressions. Rust lint/type checks and frontend build/tests/lint/translations pass. The selected Rust packages passed 510 tests, with two ignored tests and the known Windows Lua round-trip newline fixture excluded.
