use crate::convert_res;
use libc::c_int;
use rustix::fd::{BorrowedFd, IntoRawFd};

#[no_mangle]
unsafe extern "C" fn posix_openpt(flags: c_int) -> c_int {
    libc!(libc::posix_openpt(flags));

    let flags = rustix::pty::OpenptFlags::from_bits_retain(flags as _);
    match convert_res(rustix::pty::openpt(flags)) {
        Some(fd) => fd.into_raw_fd(),
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn grantpt(fd: c_int) -> c_int {
    libc!(libc::grantpt(fd));

    let fd = BorrowedFd::borrow_raw(fd);
    match convert_res(rustix::pty::grantpt(fd)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn unlockpt(fd: c_int) -> c_int {
    libc!(libc::unlockpt(fd));

    let fd = BorrowedFd::borrow_raw(fd);
    match convert_res(rustix::pty::unlockpt(fd)) {
        Some(()) => 0,
        None => -1,
    }
}
