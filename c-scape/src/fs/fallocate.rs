use crate::convert_res;
use libc::{c_int, off64_t, off_t};
use rustix::fd::BorrowedFd;
use rustix::fs::FallocateFlags;

#[no_mangle]
unsafe extern "C" fn fallocate(fd: c_int, mode: c_int, offset: off_t, len: off_t) -> c_int {
    libc!(libc::fallocate(fd, mode, offset, len));
    fallocate64(fd, mode, offset as _, len as _)
}

#[no_mangle]
unsafe extern "C" fn fallocate64(fd: c_int, mode: c_int, offset: off64_t, len: off64_t) -> c_int {
    libc!(libc::fallocate64(fd, mode, offset, len));

    let fd = BorrowedFd::borrow_raw(fd);
    let mode = FallocateFlags::from_bits_retain(mode as _);
    match convert_res(rustix::fs::fallocate(fd, mode, offset as _, len as _)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn posix_fallocate(fd: c_int, offset: off_t, len: off_t) -> c_int {
    libc!(libc::posix_fallocate(fd, offset, len));
    fallocate64(fd, 0, offset as _, len as _)
}

#[no_mangle]
unsafe extern "C" fn posix_fallocate64(fd: c_int, offset: off64_t, len: off64_t) -> c_int {
    libc!(libc::posix_fallocate64(fd, offset, len));
    fallocate64(fd, 0, offset, len)
}
