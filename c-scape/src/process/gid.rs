use libc::{c_int, gid_t};

#[no_mangle]
unsafe extern "C" fn getgid() -> gid_t {
    libc!(libc::getgid());
    rustix::process::getgid().as_raw()
}

#[no_mangle]
unsafe extern "C" fn setgid(_gid: gid_t) -> c_int {
    libc!(libc::setgid(_gid));

    // rustix has a `set_thread_gid` function, but it just wraps the Linux
    // syscall which sets a per-thread GID rather than the whole process GID.
    // Linux expects libc's to have logic to set the GID for all the threads.
    todo!("setgid")
}
