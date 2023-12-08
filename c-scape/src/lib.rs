#![doc = include_str!("../README.md")]
#![no_std]
#![feature(thread_local)] // for `__errno_location`
#![feature(c_variadic)] // for `ioctl` etc.
#![cfg_attr(feature = "use-compiler-builtins", feature(rustc_private))]
#![feature(strict_provenance)]
#![feature(inline_const)]
#![feature(sync_unsafe_cell)]
#![feature(linkage)]
#![deny(fuzzy_provenance_casts, lossy_provenance_casts)]

// Check that our features were used as we intend.
#[cfg(all(feature = "coexist-with-libc", feature = "take-charge"))]
compile_error!("Enable only one of \"coexist-with-libc\" and \"take-charge\".");
#[cfg(all(not(feature = "coexist-with-libc"), not(feature = "take-charge")))]
compile_error!("Enable one \"coexist-with-libc\" and \"take-charge\".");

extern crate alloc;
#[cfg(feature = "use-compiler-builtins")]
extern crate compiler_builtins;

// Re-export the libc crate's API. This allows users to depend on the c-scape
// crate in place of libc.
pub use libc::*;

#[macro_use]
mod use_libc;

#[cfg(not(target_os = "wasi"))]
mod at_fork;
mod error_str;
mod sync_ptr;

// Selected libc-compatible interfaces.
//
// The goal here isn't necessarily to build a complete libc; it's primarily
// to provide things that `std` and possibly popular crates are currently
// using.
//
// This effectively undoes the work that `rustix` does: it calls `rustix` and
// translates it back into a C-like ABI. Ideally, Rust code should just call
// the `rustix` APIs directly, which are safer, more ergonomic, and skip this
// whole layer.

#[cfg(feature = "take-charge")]
use core::ptr::addr_of;
use errno::{set_errno, Errno};

mod brk;
mod ctype;
mod env;
mod fs;
mod io;
mod shm;

#[cfg(feature = "take-charge")]
mod malloc;

mod math;
mod mem;

#[cfg(not(target_os = "wasi"))]
mod mm;

mod net;

#[cfg(not(target_os = "wasi"))]
mod process;

mod rand;
mod rand48;
#[cfg(not(target_os = "wasi"))]
#[cfg(feature = "take-charge")]
mod signal;
mod termios_;

#[cfg(feature = "thread")]
#[cfg(feature = "take-charge")]
mod thread;

mod atoi;
mod errno_;
mod error;
mod exec;
#[cfg(feature = "take-charge")]
mod exit;
mod glibc_versioning;
mod int;
mod locale;
mod mkostemps;
mod nss;
mod pause;
mod posix_spawn;
mod process_;
mod pty;
mod rand_;
mod regex;
mod setjmp;
mod sort;
mod stdio;
mod strtol;
mod syscall;
mod time;

#[cfg(feature = "deprecated-and-unimplemented")]
mod deprecated;
#[cfg(feature = "todo")]
mod todo;

/// An ABI-conforming `__dso_handle`.
#[cfg(feature = "take-charge")]
#[no_mangle]
static __dso_handle: UnsafeSendSyncVoidStar =
    UnsafeSendSyncVoidStar(addr_of!(__dso_handle) as *const _);

/// A type for `__dso_handle`.
///
/// `*const c_void` isn't `Send` or `Sync` because a raw pointer could point to
/// arbitrary data which isn't thread-safe, however `__dso_handle` is used as
/// an opaque cookie value, and it always points to itself.
///
/// Note that in C, `__dso_handle`'s type is usually `void *` which would
/// correspond to `*mut c_void`, however we can assume the pointee is never
/// actually mutated.
#[repr(transparent)]
#[cfg(feature = "take-charge")]
struct UnsafeSendSyncVoidStar(*const core::ffi::c_void);
#[cfg(feature = "take-charge")]
unsafe impl Send for UnsafeSendSyncVoidStar {}
#[cfg(feature = "take-charge")]
unsafe impl Sync for UnsafeSendSyncVoidStar {}

/// This function is called by Origin.
///
/// SAFETY: `argc`, `argv`, and `envp` describe incoming program
/// command-line arguments and environment variables.
#[cfg(feature = "take-charge")]
#[cfg(feature = "call-main")]
#[no_mangle]
unsafe fn origin_main(argc: usize, argv: *mut *mut u8, envp: *mut *mut u8) -> i32 {
    extern "C" {
        fn main(argc: i32, argv: *const *const u8, envp: *const *const u8) -> i32;
    }
    main(argc as _, argv as _, envp as _)
}

// utilities

/// Convert a rustix `Result` into an `Option` with the error stored
/// in `errno`.
fn convert_res<T>(result: Result<T, rustix::io::Errno>) -> Option<T> {
    result
        .map_err(|err| set_errno(Errno(err.raw_os_error())))
        .ok()
}

/// A thread-local buffer for reading into, when the user-supplied buffer
/// may not be initialized.
#[thread_local]
static mut READ_BUFFER: [u8; libc::PIPE_BUF] = [0_u8; libc::PIPE_BUF];

/// A type that implements `lock_api::GetThreadId` for use with
/// `lock_api::RawReentrantMutex`.
#[cfg(feature = "thread")]
pub(crate) struct GetThreadId;

#[cfg(feature = "thread")]
unsafe impl rustix_futex_sync::lock_api::GetThreadId for GetThreadId {
    const INIT: Self = Self;

    #[inline]
    fn nonzero_thread_id(&self) -> core::num::NonZeroUsize {
        // Use the current thread "raw" value, which origin guarantees uniquely
        // identifies a thread. `thread::current_id` would also work, but would
        // be slightly slower on some architectures.
        origin::thread::current().to_raw_non_null().addr()
    }
}
