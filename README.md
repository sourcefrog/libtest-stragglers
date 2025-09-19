# libtest-stragglers

Demonstration of the value of fail-fast within individual test targets.

You can now say `cargo +nightly test -- --include-ignored -Zunstable-options --fail-fast` and the test suite will complete faster.

See <https://github.com/rust-lang/rust/issues/142859>.
