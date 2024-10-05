<div align="center">
  <h1><code>c-gull</code></h1>

  <p>
    <strong>A libc implementation in Rust</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/c-ward/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/c-ward/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://bytecodealliance.zulipchat.com/#narrow/stream/206238-general"><img src="https://img.shields.io/badge/zulip-join_chat-brightgreen.svg" alt="zulip chat" /></a>
    <a href="https://crates.io/crates/c-gull"><img src="https://img.shields.io/crates/v/c-gull.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/c-gull"><img src="https://docs.rs/c-gull/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

c-gull is a libc implementation. It is an implementation of the ABI described
by the [libc] crate.

It is implemented in terms of crates written in Rust, such as [c-scape],
[rustix], [origin], [libm], [realpath-ext], [tz-rs], [printf-compat],
[num-complex], and [posix-regex].

Currently it only supports `*-*-linux-gnu` ABIs, though other ABIs could be
added in the future. And currently this mostly focused on features needed by
Rust programs, so it doesn't have all the C-idiomatic things like `qsort` yet,
but they could be added in the future.

The goal is to have very little code in c-gull itself, by factoring out all of
the significant functionality into independent crates with more Rust-idiomatic
APIs, with c-gull just wrapping those APIs to implement the C ABIs.

This is currently highly experimental, incomplete, and some things aren't
optimized. And it depends on Nightly Rust.

## c-gull's two modes

c-gull has two main cargo features: "take-charge" and "coexist-with-libc". One
of these must be enabled.

In "take-charge" mode, c-gull takes charge of the process, handling program
startup (via Origin) providing `malloc` (via c-scape), and other things. This
requires some additional setup; see the [c-gull-example] example crate for
more details.

In "coexist-with-libc" mode, c-gull can be used as a drop-in (partial) libc
replacement. To use it, just change your typical libc dependency in Cargo.toml
to this:

```toml
libc = { version = "<c-gull version>", package = "c-gull", features = ["coexist-with-libc"] }
```

and c-gull will replace as many of the system libc implementation with its own
implementations as it can. In particular, it can't replace `malloc` or any of
the pthread functions in this configuration, but it can replace many other
things. See the [libc-replacement example] for more details.

[libc-replacement example]: https://github.com/sunfishcode/c-ward/blob/main/test-crates/libc-replacement/README.md
[c-scape]: https://crates.io/crates/c-scape
[rustix]: https://crates.io/crates/rustix
[origin]: https://crates.io/crates/origin
[libm]: https://crates.io/crates/libm
[libc]: https://crates.io/crates/libc
[realpath-ext]: https://crates.io/crates/realpath-ext
[tz-rs]: https://crates.io/crates/tz-rs
[printf-compat]: https://crates.io/crates/printf-compat
[num-complex]: https://crates.io/crates/num-complex
[posix-regex]: https://crates.io/crates/posix-regex
[c-gull-example]: https://github.com/sunfishcode/c-ward/blob/main/example-crates/c-gull-example
