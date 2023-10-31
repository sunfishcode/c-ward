use crate::convert_res;
use core::ffi::CStr;
use libc::{c_char, c_int};
use rustix::fd::BorrowedFd;

#[no_mangle]
unsafe extern "C" fn statvfs(path: *const c_char, buf: *mut libc::statvfs) -> c_int {
    libc!(libc::statvfs(path, buf));

    let path = CStr::from_ptr(path);
    let val = match convert_res(rustix::fs::statvfs(path)) {
        Some(val) => val,
        None => return -1,
    };
    rustix_to_libc(buf, val);
    0
}

#[no_mangle]
unsafe extern "C" fn fstatvfs(fd: c_int, buf: *mut libc::statvfs) -> c_int {
    libc!(libc::fstatvfs(fd, buf));

    let fd = BorrowedFd::borrow_raw(fd);
    let val = match convert_res(rustix::fs::fstatvfs(fd)) {
        Some(val) => val,
        None => return -1,
    };
    rustix_to_libc(buf, val);
    0
}

unsafe fn rustix_to_libc(buf: *mut libc::statvfs, val: rustix::fs::StatVfs) {
    let mut converted = core::mem::zeroed::<libc::statvfs>();
    converted.f_bsize = val.f_bsize as _;
    converted.f_frsize = val.f_frsize as _;
    converted.f_blocks = val.f_blocks as _;
    converted.f_bfree = val.f_bfree as _;
    converted.f_bavail = val.f_bavail as _;
    converted.f_files = val.f_files as _;
    converted.f_ffree = val.f_ffree as _;
    converted.f_favail = val.f_favail as _;
    converted.f_fsid = val.f_fsid as _;
    converted.f_flag = val.f_flag.bits() as _;
    converted.f_namemax = val.f_namemax as _;
    *buf = converted;
}
