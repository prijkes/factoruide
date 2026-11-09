**AI disclosure:** This contribution was created with AI. The submitter reports in-game testing of the combined fork only and has not reviewed the source code. These split branches have not been individually verified in-game. Automated checks below were run by AI; no human source-code review is claimed.

The allocator module is available on WASI and Unix, but its integration test references it on every platform. Apply the same platform guard to the test crate so Windows can compile the test target while supported platforms retain their stress coverage.

AI-run automated checks:

- On upstream, `cargo test --profile fast -p fg-web --test alloc_stress --no-run` fails on Windows with two E0433 errors because `fg_web::alloc` is configured out.
- After the guard, `cargo test --profile fast -p fg-web --test alloc_stress` passes with zero tests on Windows, as intended. Unix/WASI execution was not available in this Windows run.
- Only the two-line platform guard changes.

Part of the [22-topic contribution series](https://github.com/prijkes/factoruide/blob/pr/submission-plan/docs/pr-submissions/README.md).
