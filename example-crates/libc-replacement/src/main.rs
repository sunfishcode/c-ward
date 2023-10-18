fn main() {
    println!("Hello, world! uid={}", unsafe { libc::getuid() });
    unsafe { libc::printf("Hello world with printf! gid=%u\n\0".as_ptr().cast(), libc::getgid()); }
    unsafe { libc::atexit(atexit_func); }
}

extern "C" fn atexit_func() {
    unsafe { libc::printf("Hello world from `atexit_func`\n\0".as_ptr().cast()); }
}
