use crate::convert_res;
use core::ffi::CStr;
use libc::{c_char, c_int, mode_t};
use rustix::fd::IntoRawFd;
use rustix::fs::Mode;
use rustix::shm;

#[no_mangle]
unsafe extern "C" fn shm_open(name: *const c_char, oflags: c_int, mode: mode_t) -> c_int {
    libc!(libc::shm_open(name, oflags, mode));

    let name = CStr::from_ptr(name);
    let mode = Mode::from_bits((mode & !libc::S_IFMT) as _).unwrap();
    let oflags = shm::OFlags::from_bits(oflags as _).unwrap();
    match convert_res(shm::open(name, oflags, mode)) {
        Some(fd) => fd.into_raw_fd(),
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn shm_unlink(name: *const c_char) -> c_int {
    libc!(libc::shm_unlink(name));

    let name = CStr::from_ptr(name);
    match convert_res(shm::unlink(name)) {
        Some(()) => 0,
        None => -1,
    }
}
