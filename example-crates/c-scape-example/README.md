This crate demonstrates the use of c-scape in "take-charge" mode with `no_main`
and `no_std`.

This version uses `-nostartfiles` and origin is in control from the very
beginning.

This crate uses stub `#[panic_handler]` and `#[lang = "eh_personality"]`
functions which are simple and have small code size, but which don't support
unwinding. See the [c-scape-unwinding] crate for an example that includes
unwinding support.

[c-scape-unwinding]: https://github.com/sunfishcode/c-ward/tree/main/example-crates/c-scape-unwinding#readme
