use core::ffi::CStr;
use rustix::fd::BorrowedFd;

use libc::{c_char, c_int, c_uint};

use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn rename(old: *const c_char, new: *const c_char) -> c_int {
    libc!(libc::rename(old, new));

    renameat(libc::AT_FDCWD, old, libc::AT_FDCWD, new)
}

#[no_mangle]
unsafe extern "C" fn renameat(
    old_fd: c_int,
    old: *const c_char,
    new_fd: c_int,
    new: *const c_char,
) -> c_int {
    libc!(libc::renameat(old_fd, old, new_fd, new));

    match convert_res(rustix::fs::renameat(
        BorrowedFd::borrow_raw(old_fd),
        CStr::from_ptr(old.cast()),
        BorrowedFd::borrow_raw(new_fd),
        CStr::from_ptr(new.cast()),
    )) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn renameat2(
    old_fd: c_int,
    old: *const c_char,
    new_fd: c_int,
    new: *const c_char,
    flags: c_uint,
) -> c_int {
    libc!(libc::renameat2(old_fd, old, new_fd, new, flags));

    match convert_res(rustix::fs::renameat_with(
        BorrowedFd::borrow_raw(old_fd),
        CStr::from_ptr(old.cast()),
        BorrowedFd::borrow_raw(new_fd),
        CStr::from_ptr(new.cast()),
        rustix::fs::RenameFlags::from_bits_retain(flags),
    )) {
        Some(()) => 0,
        None => -1,
    }
}
