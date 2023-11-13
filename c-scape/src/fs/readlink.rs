use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::slice;
use rustix::fd::BorrowedFd;

use libc::{c_char, c_int};

use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn readlink(pathname: *const c_char, buf: *mut c_char, bufsiz: usize) -> isize {
    libc!(libc::readlink(pathname, buf, bufsiz));

    readlinkat(libc::AT_FDCWD, pathname, buf, bufsiz)
}

#[no_mangle]
unsafe extern "C" fn readlinkat(
    fd: c_int,
    pathname: *const c_char,
    buf: *mut c_char,
    bufsiz: usize,
) -> isize {
    libc!(libc::readlinkat(fd, pathname, buf, bufsiz));

    let buf = slice::from_raw_parts_mut(buf.cast::<MaybeUninit<u8>>(), bufsiz);
    let (yes, _no) = match convert_res(rustix::fs::readlinkat_raw(
        BorrowedFd::borrow_raw(fd),
        CStr::from_ptr(pathname.cast()),
        buf,
    )) {
        Some(slices) => slices,
        None => return -1,
    };
    yes.len() as isize
}
