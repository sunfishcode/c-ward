<div align="center">
  <h1><code>c-ward</code></h1>

  <p>
    <strong>An implementation of libc written in Rust</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/c-ward/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/c-ward/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://bytecodealliance.zulipchat.com/#narrow/stream/206238-general"><img src="https://img.shields.io/badge/zulip-join_chat-brightgreen.svg" alt="zulip chat" /></a>
    <a href="https://crates.io/crates/c-gull"><img src="https://img.shields.io/crates/v/c-gull.svg" alt="crates.io page" /></a>
    <a href="https://crates.io/crates/c-scape"><img src="https://img.shields.io/crates/v/c-scape.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/c-gull"><img src="https://docs.rs/c-gull/badge.svg" alt="docs.rs docs" /></a>
    <a href="https://docs.rs/c-scape"><img src="https://docs.rs/c-scape/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

c-ward is an implementation of the libc ABI written in Rust.

It is split into two crates:
 - [c-scape], which is `no_std`, and
 - [c-gull], which pulls in c-scape and provides additional features using `std`.

It is a goal of c-ward to be a C ABI layer on top of Rust-idomatic
libraries, rather than to have significant implementation code of
its own.

In theory c-ward could be extended to be ABI-compatible with different
platforms, however currently it is only known to be ABI-compatible with
*-unknown-linux-gnu* platforms.

## c-ward's two modes

c-ward has two main cargo features: "take-charge" and "coexist-with-libc". One
of these must be enabled.

In "take-charge" mode, c-ward takes charge of the process, handling program
startup (via origin) providing `malloc` (via c-scape), and other things.

In "coexist-with-libc" mode, c-ward can be used as a drop-in (partial) libc
replacement, provided you're using nightly Rust.

## Similar crates

Another libc implementation is [relibc].

[c-scape]: https://crates.io/crates/c-scape
[c-gull]: https://crates.io/crates/c-gull
[relibc]: https://gitlab.redox-os.org/redox-os/relibc/
