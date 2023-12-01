use alloc::boxed::Box;
use core::ffi::CStr;
use rustix::fd::{FromRawFd, IntoRawFd, OwnedFd};
use rustix::fs::{Mode, OFlags, CWD};

use core::mem::zeroed;
use core::ptr::null_mut;
use libc::{c_char, c_int, c_void};

use super::CScapeDir;
use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn opendir(pathname: *const c_char) -> *mut c_void {
    libc!(libc::opendir(pathname).cast());

    match convert_res(rustix::fs::openat(
        CWD,
        CStr::from_ptr(pathname.cast()),
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
    )) {
        Some(fd) => fdopendir(fd.into_raw_fd()),
        None => null_mut(),
    }
}

#[no_mangle]
unsafe extern "C" fn fdopendir(fd: c_int) -> *mut c_void {
    libc!(libc::fdopendir(fd).cast());

    // Use the unsafe `Dir::new` API, because that avoids opening ".",
    // which requires additional permissions that we may not have. It's
    // up to users of `fdopendir` to ensure that the fd isn't otherwise
    // used.
    match convert_res(rustix::fs::Dir::new(OwnedFd::from_raw_fd(fd))) {
        Some(dir) => Box::into_raw(Box::new(CScapeDir {
            dir,
            storage: zeroed(),
            fd,
        }))
        .cast(),
        None => null_mut(),
    }
}

#[no_mangle]
unsafe extern "C" fn closedir(dir: *mut c_void) -> c_int {
    libc!(libc::closedir(dir.cast()));

    drop(Box::<CScapeDir>::from_raw(dir.cast()));
    0
}
