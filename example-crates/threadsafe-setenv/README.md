c-scape has a "threadsafe-setenv" feature which makes `setenv` threadsafe,
fixing a [Rust soundness bug]. This example demonstrates it in use on an
exampled linked to from that issue.

[Rust soundness bug]: https://github.com/rust-lang/rust/issues/27970
