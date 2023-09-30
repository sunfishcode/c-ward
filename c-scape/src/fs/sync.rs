use rustix::fd::BorrowedFd;

use libc::{c_int, c_uint, off64_t};

use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn fsync(fd: c_int) -> c_int {
    libc!(libc::fsync(fd));

    match convert_res(rustix::fs::fsync(BorrowedFd::borrow_raw(fd))) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn fdatasync(fd: c_int) -> c_int {
    libc!(libc::fdatasync(fd));

    match convert_res(rustix::fs::fdatasync(BorrowedFd::borrow_raw(fd))) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn syncfs(fd: c_int) -> c_int {
    libc!(libc::syncfs(fd));

    match convert_res(rustix::fs::syncfs(BorrowedFd::borrow_raw(fd))) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn sync() {
    libc!(libc::sync());

    rustix::fs::sync()
}

#[no_mangle]
unsafe extern "C" fn sync_file_range(
    fd: c_int,
    offset: off64_t,
    nbytes: off64_t,
    flags: c_uint,
) -> c_int {
    libc!(libc::sync_file_range(fd, offset, nbytes, flags));

    let fd = BorrowedFd::borrow_raw(fd);

    // FIXME: Add `sync_file_range` to rustix and use it.
    match convert_res(rustix::fs::fdatasync(fd)) {
        Some(()) => 0,
        None => -1,
    }
}
