<div align="center">
  <h1><code>Eyra</code></h1>

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

Eyra is a package that supports building Rust programs implemented entirely
in Rust.

## Quick start

Check out [this hello world example].

[this hello world example]: https://github.com/sunfishcode/c-ward/tree/main/example-crates/eyra-example

## In detail

Eyra needs three things. First, a cargo.toml dependency:

```toml
[dependencies]
libc = { version = "<current-version>", package = "eyra" }
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

This tells Rust that Eyra is actually used and the libraries should actually
be linked in.

And finally, a build.rs file to add `-nostartfiles` to the link flags to
disable the host startup code, so that Eyra can provide its own. build.rs:

```rust
fn main() {
    println!("cargo:rustc-link-arg=-nostartfiles");
}
```

With these three steps, this crate prints "Hello, world!". And under the
covers, it uses [origin] to start and stop the program, [c-ward] to handle
libc calls from `std`, and [rustix] to do the printing, so it's completely
implemented in Rust.

## Optional logging

Eyra has a `log` feature to enable Rust `log` tracing of program and thread
startup and shutdown, and an `env_logger` feature to install `env_logger`
as the logger, which can be enabled in Cargo.toml:

```toml
[dependencies]
libc = { version = "<current-version>", package = "eyra", features = ["log", "env_logger"] }
```

With this, and setting the `RUST_LOG` environment variable to "trace", the
hello world program output like this:

[TRACE origin::program] Program started
[TRACE origin::thread] Main Thread[Pid(51383)] initialized
[TRACE origin::program] Calling `.init_array`-registered function `0x55e86306bb80(1, 0x7ffd0f76aad8, 0x7ffd0f76aae8)`
[TRACE origin::program] Calling `origin_main(1, 0x7ffd0f76aad8, 0x7ffd0f76aae8)`
Hello, world!
[TRACE origin::program] `origin_main` returned `0`
[TRACE origin::thread] Thread[51383] calling `at_thread_exit`-registered function
[TRACE origin::thread] Thread[51383] calling `at_thread_exit`-registered function
[TRACE origin::program] Program exiting

## Background

Eyra is similar to [Mustang] and uses the same underlying code, but instead
of using a custom target and -Z build-std, Eyra just needs users to add
`-nostartfiles` to their link line, such as via build.rs in the example.

Like Mustang, Eyra currently runs on Rust Nightly on Linux on x86-64, x86,
aarch64, and riscv64. It aims to support all Linux versions [supported by Rust],
though at this time it's only tested on relatively recent versions.

[Mustang]: https://github.com/sunfishcode/mustang
[origin]: https://github.com/sunfishcode/origin
[c-ward]: https://github.com/sunfishcode/c-ward
[rustix]: https://github.com/sunfishcode/rustix
[supported by Rust]: https://doc.rust-lang.org/nightly/rustc/platform-support.html
