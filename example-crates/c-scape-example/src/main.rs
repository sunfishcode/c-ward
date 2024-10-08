//! A simple example using `no_main`, `no_std`, and "take-charge" mode.

#![no_std]
#![no_main]

#[no_mangle]
unsafe extern "C" fn main(_argc: i32, _argv: *const *const u8, _envp: *const *const u8) -> i32 {
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
