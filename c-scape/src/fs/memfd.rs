use crate::convert_res;
use core::ffi::CStr;
use libc::{c_char, c_int, c_uint};
use rustix::fd::IntoRawFd;
use rustix::fs::MemfdFlags;

#[no_mangle]
unsafe extern "C" fn memfd_create(name: *const c_char, flags: c_uint) -> c_int {
    libc!(libc::memfd_create(name, flags));

    let name = CStr::from_ptr(name);
    let flags = MemfdFlags::from_bits_retain(flags);
    match convert_res(rustix::fs::memfd_create(name, flags)) {
        Some(fd) => fd.into_raw_fd(),
        None => -1,
    }
}
