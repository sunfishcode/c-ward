//! Termios APIs
//!
//! Most of the termios functions are in c-scape, but ttyname is in c-gull
//! because it depends on rustix's procfs feature, which depends on std. Rustix
//! could be changed to avoid using std here, if it becomes important.

use crate::convert_res;
use alloc::ffi::CString;
use core::cell::SyncUnsafeCell;
use core::ptr::{copy_nonoverlapping, null_mut};
use libc::{c_char, c_int, size_t};
use rustix::fd::BorrowedFd;

#[no_mangle]
unsafe extern "C" fn ttyname(fd: c_int) -> *mut c_char {
    libc!(libc::ttyname(fd));

    static STORAGE: SyncUnsafeCell<Option<CString>> = SyncUnsafeCell::new(None);

    let storage = SyncUnsafeCell::get(&STORAGE);
    let name = match convert_res(rustix::termios::ttyname(
        BorrowedFd::borrow_raw(fd),
        Vec::new(),
    )) {
        Some(name) => name,
        None => return null_mut(),
    };
    (*storage) = Some(name);

    (*storage).as_ref().unwrap().as_ptr().cast_mut()
}

#[no_mangle]
unsafe extern "C" fn ttyname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int {
    libc!(libc::ttyname_r(fd, buf, buflen));

    let name = match rustix::termios::ttyname(BorrowedFd::borrow_raw(fd), Vec::new()) {
        Ok(name) => name,
        Err(err) => return err.raw_os_error(),
    };

    let len = name.as_bytes().len();
    if len >= buflen {
        return libc::ERANGE;
    }

    copy_nonoverlapping(name.as_ptr(), buf, len);

    0
}
