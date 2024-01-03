use crate::convert_res;
use core::ffi::CStr;
use errno::{set_errno, Errno};
use libc::{c_char, c_int, c_uint};
use rustix::fd::BorrowedFd;
use rustix::fs::AtFlags;

#[no_mangle]
unsafe extern "C" fn fchownat(
    dirfd: c_int,
    pathname: *const c_char,
    owner: libc::uid_t,
    group: libc::gid_t,
    flags: c_int,
) -> c_int {
    libc!(libc::fchownat(dirfd, pathname, owner, group, flags));

    let pathname = CStr::from_ptr(pathname);
    let owner = Some(rustix::process::Uid::from_raw(owner));
    let group = Some(rustix::process::Gid::from_raw(group));
    let flags = AtFlags::from_bits_retain(flags as c_uint);
    let dirfd = BorrowedFd::borrow_raw(dirfd);
    match convert_res(rustix::fs::chownat(dirfd, pathname, owner, group, flags)) {
        Some(()) => 0,
        None => -1,
    }
}
