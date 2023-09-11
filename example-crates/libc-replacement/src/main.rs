fn main() {
    println!("Hello, world! uid={}", unsafe { libc::getuid() });
    unsafe { libc::printf("Hello world with printf! gid=%u\n\0".as_ptr().cast(), libc::getgid()); }
}
