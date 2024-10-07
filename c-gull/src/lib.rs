#![doc = include_str!("../README.md")]
#![feature(sync_unsafe_cell)]
#![feature(strict_provenance)]
#![deny(fuzzy_provenance_casts, lossy_provenance_casts)]
#![cfg_attr(not(feature = "std"), no_std)]
// Don't warn if `try_into()` is fallible on some targets.
#![allow(unreachable_patterns)]
// Don't warn if `try_into()` is fallible on some targets.
#![allow(irrefutable_let_patterns)]

#[cfg(feature = "std")]
extern crate alloc;
#[allow(unused_extern_crates)]
extern crate c_scape;

// Re-export the c_scape crate's API, which includes the libc API. This allows
// users to depend on the c-scape crate in place of libc.
pub use c_scape::*;

#[macro_use]
mod use_libc;

#[cfg(feature = "std")]
mod nss;
#[cfg(feature = "std")]
mod resolve;
#[cfg(feature = "std")]
mod sysconf;
#[cfg(feature = "std")]
mod system;
#[cfg(feature = "std")]
mod termios_;
#[cfg(feature = "std")]
mod time;
#[cfg(feature = "std")]
#[cfg(not(target_env = "musl"))]
mod utmp;

#[cfg(feature = "std")]
#[cold]
#[no_mangle]
unsafe extern "C" fn __assert_fail(
    expr: *const c_char,
    file: *const c_char,
    line: c_int,
    func: *const c_char,
) -> ! {
    use std::ffi::CStr;
    //libc!(libc::__assert_fail(expr, file, line, func));

    eprintln!(
        "Assertion failed: {:?} ({:?}:{}: {:?})",
        CStr::from_ptr(expr),
        CStr::from_ptr(file),
        line,
        CStr::from_ptr(func)
    );
    std::process::abort();
}

// utilities

/// Convert a rustix `Result` into an `Option` with the error stored
/// in `errno`.
#[cfg(feature = "std")]
fn convert_res<T>(result: Result<T, rustix::io::Errno>) -> Option<T> {
    use errno::{set_errno, Errno};
    result
        .map_err(|err| set_errno(Errno(err.raw_os_error())))
        .ok()
}
