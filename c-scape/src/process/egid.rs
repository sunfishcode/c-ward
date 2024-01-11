use libc::{c_int, gid_t};

#[no_mangle]
unsafe extern "C" fn getegid() -> gid_t {
    libc!(libc::getegid());
    rustix::process::getegid().as_raw()
}
