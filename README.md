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

## Similar crates

Another libc implementation is [relibc].

## Where's the `#![no_builtins]`?

Normally, a libc implementation would use `#[no_builtins]` to prevent compilers
from noticing the the bodies of libc functions implement the semantics of libc
functions and replacing them with calls, which effectively makes them uselessly
recursive calls to themselves.

However, `#[no_builtins]` is too pessimistic, because we don't need to disable
all pattern matching, just these specific cases. And, `#[no_builtins]`
[interferes with LTO optimization].

So instead, c-scape and c-gull are just careful to avoid open-coding functions
which are known to get pattern-matched into builtins, by just calling the
`compiler_builtins` implementations directly themselves. This way, we can avoid
using `#![no_builtins]`.

[c-scape]: https://github.com/sunfishcode/c-ward/tree/main/c-scape#readme
[c-gull]: https://github.com/sunfishcode/c-ward/tree/main/c-gull#readme
[relibc]: https://gitlab.redox-os.org/redox-os/relibc/
[c-scape-example]: https://github.com/sunfishcode/c-ward/blob/main/example-crates/c-scape-example
[c-gull-example]: https://github.com/sunfishcode/c-ward/blob/main/example-crates/c-gull-example
[interferes with LTO optimization]: https://github.com/rust-lang/rust/blob/72e29da3eccd3e4c1fb2c581fa33216db50fcc93/compiler/rustc_codegen_ssa/src/back/link.rs#L1264
