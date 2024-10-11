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

It consists of two crates:
 - [c-scape], which is `no_std`, and
 - [c-gull], which pulls in c-scape and additionally provides features
   using `std`.

It is a goal of c-ward to be a C ABI layer on top of Rust-idomatic libraries,
rather than to have significant implementation code of its own.

In theory c-ward could be extended to be ABI-compatible with different
platforms, however currently it is only known to be ABI-compatible with
\*-unknown-linux-gnu\* platforms.

The primary way this is used is through [Mustang] and [Eyra], as their libc
implementations. It can also be used as a regular library in
["coexist-with-libc" mode].

## Runtime requirements

Resolving users and DNS records requires the execution of `getent` which
prints the entries on stdout. On a regular glibc system the `getent`
binary is provided by it and uses the NSS setup as usual.
Similar, a musl system also provides `getent` (but does not use NSS).

## Similar crates

Another libc implementation is [relibc]. [tinyrlibc] is a very minimal set of
libc functions for bare-metal embedded platforms.

## Where's the `#![no_builtins]`?

Normally, a libc implementation would use `#[no_builtins]` to prevent compilers
from noticing the bodies of libc functions implement the semantics of libc
functions and replacing them with calls, which effectively makes them uselessly
recursive calls to themselves.

However, `#[no_builtins]` is too pessimistic, because we don't need to disable
all pattern matching, just these specific cases.

So instead, c-scape and c-gull are just careful to avoid open-coding functions
which are known to get pattern-matched into builtins.

[c-scape]: https://github.com/sunfishcode/c-ward/tree/main/c-scape#readme
[c-gull]: https://github.com/sunfishcode/c-ward/tree/main/c-gull#readme
[relibc]: https://gitlab.redox-os.org/redox-os/relibc/
[tinyrlibc]: https://github.com/rust-embedded-community/tinyrlibc
[Mustang]: https://github.com/sunfishcode/mustang#readme
[Eyra]: https://github.com/sunfishcode/eyra#readme
["coexist-with-libc" mode]: https://github.com/sunfishcode/c-ward/blob/main/example-crates/libc-replacement#readme
