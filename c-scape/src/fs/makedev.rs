#[no_mangle]
unsafe extern "C" fn gnu_dev_major(dev: libc::dev_t) -> u32 {
    libc::major(dev)
}

#[no_mangle]
unsafe extern "C" fn gnu_dev_minor(dev: libc::dev_t) -> u32 {
    libc::minor(dev)
}

#[no_mangle]
unsafe extern "C" fn gnu_dev_makedev(major: u32, minor: u32) -> libc::dev_t {
    libc::makedev(major, minor)
}
