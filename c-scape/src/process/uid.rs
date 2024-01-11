use libc::{c_int, uid_t};

#[no_mangle]
unsafe extern "C" fn getuid() -> uid_t {
    libc!(libc::getuid());
    rustix::process::getuid().as_raw()
}
