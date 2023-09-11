c-gull re-exports libc's API, so it can be used as a drop-in replacement.

This line:

```toml
libc = { path = "../../c-gull", features = ["coexist-with-libc"], package = "c-gull" }
```

tells cargo to use c-gull in place of libc. In this configuration, it doesn't
replace the malloc, pthread, or getauxval functions, but it replaces everything
it can, such as the `getuid` and `getgid` functions in the example.
