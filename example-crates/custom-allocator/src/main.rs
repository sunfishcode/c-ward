//! A simple example using "take-charge" mode and a custom global allocator.

#[global_allocator]
static GLOBAL_ALLOCATOR: rustix_dlmalloc::GlobalDlmalloc = rustix_dlmalloc::GlobalDlmalloc;

fn main() {
    println!("Hello world using Rust `println!`!");
    unsafe { libc::printf("Hello world using libc `printf`!\n\0".as_ptr().cast()); }
}
