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

It consists of three crates:
 - [c-scape], which is `no_std`, and
 - [c-gull], which pulls in c-scape and additionally provides features
   using `std`.
 - [eyra], which provides a friendly wrapper to make it easy to
   build Rust programs entirely in Rust.

It is a goal of c-ward to be a C ABI layer on top of Rust-idomatic
libraries, rather than to have significant implementation code of
its own.

In theory c-ward could be extended to be ABI-compatible with different
platforms, however currently it is only known to be ABI-compatible with
\*-unknown-linux-gnu\* platforms.

## Similar crates

Another libc implementation is [relibc].

[c-scape]: https://github.com/sunfishcode/c-ward/tree/main/c-scape
[c-gull]: https://github.com/sunfishcode/c-ward/tree/main/c-gull
[eyra]: https://github.com/sunfishcode/c-ward/tree/main/eyra
[relibc]: https://gitlab.redox-os.org/redox-os/relibc/
[c-scape-example]: https://github.com/sunfishcode/c-ward/blob/main/example-crates/c-scape-example
[c-gull-example]: https://github.com/sunfishcode/c-ward/blob/main/example-crates/c-gull-example
