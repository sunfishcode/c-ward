//! A simple example using `no_main`, `no_std`, and "take-charge" mode, using
//! the "personaliity" and "panic_handler" features to support unwinding.

#![no_std]
#![no_main]

#[global_allocator]
static GLOBAL_ALLOCATOR: rustix_dlmalloc::GlobalDlmalloc = rustix_dlmalloc::GlobalDlmalloc;

#[no_mangle]
unsafe extern "C" fn main(_argc: i32, _argv: *const *const u8, _envp: *const *const u8) -> i32 {
    // Panic and catch it.
    unwinding::panic::catch_unwind(|| call_do_panic()).unwrap_err();

    // Call functions declared in the `libc` crate, which will be resolved by
    // c-scape. c-scape doesn't have `printf`, so we do it by hand.
    let message = b"Hello, world!\n";
    let mut remaining = &message[..];
    while !remaining.is_empty() {
        match libc::write(libc::STDOUT_FILENO, message.as_ptr().cast(), message.len()) {
            -1 => match errno::errno().0 {
                libc::EINTR => continue,
                _ => panic!(),
            },
            n => remaining = &remaining[n as usize..],
        }
    }
    libc::exit(0);
}

fn call_do_panic() {
    do_panic()
}

fn do_panic() {
    panic!("catch me!");
}
