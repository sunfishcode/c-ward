//! A simple example using "take-charge" mode. Origin starts the process,
//! calls `origin_main`, which transfers control to c-scape which calls the
//! C-ABI-compatible extern `main` definition, which transfers control to
//! the Rust std initialization code, which calls the user `main` function
//! here.
//!
//! The end result is that we get all of `std`, using c-gull to implement
//! all the libc calls underneath, and we can write totally normal Rust code.

fn main() {
    println!("Hello world using Rust `println!`!");
    unsafe {
        libc::printf("Hello world using libc `printf`!\n\0".as_ptr().cast());
    }
}
