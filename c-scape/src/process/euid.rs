use libc::uid_t;

#[no_mangle]
unsafe extern "C" fn geteuid() -> uid_t {
    libc!(libc::geteuid());
    rustix::process::geteuid().as_raw()
}
