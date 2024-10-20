This crate demonstrates the use of c-scape in "take-charge" mode with `no_main`
and `no_std`, and includes unwinding panic support.

It's the same as [c-scape-example], but enables the "eh-personality" and
"panic-handler" features instead of defining stub `#[panic_handler]` and
`#[lang = "eh_personality"]` functions, and it performs an unwind.

[c-scape-example]: https://github.com/sunfishcode/c-ward/tree/main/example-crates/c-scape-example#readme
