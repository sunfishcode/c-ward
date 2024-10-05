<div align="center">
  <h1><code>c-scape</code></h1>

  <p>
    <strong>A layer underneath c-gull</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/c-ward/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/c-ward/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://bytecodealliance.zulipchat.com/#narrow/stream/206238-general"><img src="https://img.shields.io/badge/zulip-join_chat-brightgreen.svg" alt="zulip chat" /></a>
    <a href="https://crates.io/crates/c-scape"><img src="https://img.shields.io/crates/v/c-scape.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/c-scape"><img src="https://docs.rs/c-scape/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

c-scape is a layer underneath [c-gull]. It provides a subset of libc features,
containing only features that don't require Rust's `std` to implement. This
allows it to be used by `std` itself.

This is currently highly experimental, incomplete, and some things aren't
optimized. And it depends on Nightly Rust.

## c-scape's two modes

[Similar to c-gull], c-scape has "take-charge" and "coexist-with-libc" modes.
One of these must be enabled.

In "take-charge" mode, c-scape takes charge of the process, handling program
startup (via Origin) providing `malloc` (via c-scape), and other things. This
requires some additional setup; see the [c-scape-example] example crate for
more details.

In "coexist-with-libc" mode, c-scape can be used as a drop-in (partial) libc
replacement. To use it, just change your typical libc dependency in Cargo.toml
to this:

```toml
libc = { version = "<c-scape version>", package = "c-scape", features = ["coexist-with-libc"] }
```

[c-gull]: https://github.com/sunfishcode/c-ward/tree/main/c-gull
[c-scape-example]: https://github.com/sunfishcode/c-ward/blob/main/example-crates/c-scape-example
[Similar to c-gull]: https://github.com/sunfishcode/c-ward/tree/main/c-gull#c-gulls-two-modes
