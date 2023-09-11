//! A simple example using `no_main` and "take-charge" mode.

#![no_main]
#![allow(internal_features)]
#![feature(lang_items)]
#![feature(core_intrinsics)]

extern crate libc;

#[global_allocator]
static GLOBAL_ALLOCATOR: rustix_dlmalloc::GlobalDlmalloc = rustix_dlmalloc::GlobalDlmalloc;

#[no_mangle]
unsafe extern "C" fn main(_argc: i32, _argv: *const *const u8, _envp: *const *const u8) -> i32 {
    // Call functions declared in the `libc` crate, which will be resolved by
    // c-gull and c-scape.
    libc::printf("Hello, world!\n\0".as_ptr().cast());
    libc::exit(0);
}
