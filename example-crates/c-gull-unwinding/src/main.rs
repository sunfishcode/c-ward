//! A simple example using `std::panic::catch_unwind`.

fn main() {
    // Panic and catch it.
    std::panic::catch_unwind(|| call_do_panic()).unwrap_err();

    println!("Hello, world!");
    unsafe {
        libc::printf("Hello world using libc `printf`!\n\0".as_ptr().cast());
    }
}

fn call_do_panic() {
    do_panic()
}

fn do_panic() {
    panic!("catch me!");
}
