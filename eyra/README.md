<div align="center">
  <h1><code>eyra</code></h1>

  <p>
    <strong>Rust programs written entirely in Rust</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/c-ward/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/c-ward/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://bytecodealliance.zulipchat.com/#narrow/stream/206238-general"><img src="https://img.shields.io/badge/zulip-join_chat-brightgreen.svg" alt="zulip chat" /></a>
    <a href="https://crates.io/crates/eyra"><img src="https://img.shields.io/crates/v/eyra.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/eyra"><img src="https://docs.rs/eyra/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

eyra is a package that supports building Rust programs implemented entirely
in Rust.

## Quick start

Check out [this hello world example].

[this hello world example]: https://github.com/sunfishcode/c-ward/tree/main/example-crates/eyra

## In detail

Eyra needs three things. First, a cargo.toml dependency:

```toml
[dependencies]
libc = { "<current-version>", package = "eyra" }
```

This uses the trick of calling the library `libc` while actually using
`eyra`. This trick isn't necessary, but it sometimes means we can skip
the next step.

The next step is to mention `libc` somewhere. If there are no other
mentions of `libc`, adding an `extern crate` is sufficient:

```rust
extern crate libc;

fn main() {
    println!("Hello, world!");
}
```

This tells Rust that eyra is actually used and the libraries should
actually be linked in.

And finally, a build.rs file to add `-nostartfiles` to the link flags to
disable the host startup code, so that eyra can provide its own. build.rs:

```rust
fn main() {
    println!("cargo:rustc-link-arg=-nostartfiles");
}
```

With these three steps, this crate prints "Hello, world!". And under the
covers, it uses [origin] to start and stop the program, [c-ward] to handle
libc calls from `std`, and [rustix] to do the printing, so it's completely
implemented in Rust.

## Background

This is similar to [mustang] and uses the same underlying code, but instead
of using a custom target and -Z build-std, it just by needs users to add
`-nostartfiles` to their link line, such as via build.rs in the example.

Like mustang, eyra currently runs on Rust Nightly on Linux on x86-64, x86,
aarch64, and riscv64. It aims to support all Linux versions [supported by Rust],
though at this time it's only tested on relatively recent versions.

[mustang]: https://github.com/sunfishcode/mustang
[origin]: https://github.com/sunfishcode/origin
[c-ward]: https://github.com/sunfishcode/c-ward
[rustix]: https://github.com/sunfishcode/rustix
[supported by Rust]: https://doc.rust-lang.org/nightly/rustc/platform-support.html
