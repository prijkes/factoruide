The allocator module is available on WASI and Unix, but its integration test references it on every platform. Apply the same platform guard to the test crate so Windows can compile the test target while supported platforms retain their stress coverage.

Validation:

- On upstream, `cargo test --profile fast -p fg-web --test alloc_stress --no-run` fails on Windows with two E0433 errors because `fg_web::alloc` is configured out.
- After the guard, `cargo test --profile fast -p fg-web --test alloc_stress` passes with zero tests on Windows, as intended. Unix/WASI execution was not available in this Windows run.
- Only the two-line platform guard changes.
