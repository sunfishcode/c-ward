//! A simple example using `no_main`, `no_std`, and "take-charge" mode.

#![no_std]
#![no_main]
#![allow(internal_features)]
#![feature(lang_items)]
#![feature(core_intrinsics)]

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    core::intrinsics::abort()
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[global_allocator]
static GLOBAL_ALLOCATOR: rustix_dlmalloc::GlobalDlmalloc = rustix_dlmalloc::GlobalDlmalloc;

#[no_mangle]
unsafe extern "C" fn main(_argc: i32, _argv: *const *const u8, _envp: *const *const u8) -> i32 {
    // Call functions declared in the `libc` crate, which will be resolved by
    // c-scape. c-scape doesn't have `printf`, so we do it by hand.
    let message = b"Hello, world!\n";
    let mut remaining = &message[..];
    while !remaining.is_empty() {
        match libc::write(1, message.as_ptr().cast(), message.len()) {
            -1 => match errno::errno().0 {
                libc::EINTR => continue,
                _ => panic!(),
            }
            n => remaining = &remaining[n as usize..],
        }
    }
    libc::exit(0);
}
