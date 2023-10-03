use crate::convert_res;
use libc::{c_int, off_t};
use rustix::fd::BorrowedFd;
use rustix::fs::FallocateFlags;

#[no_mangle]
unsafe extern "C" fn fallocate(fd: c_int, mode: c_int, offset: off_t, len: off_t) -> c_int {
    libc!(libc::fallocate(fd, mode, offset, len));

    let fd = BorrowedFd::borrow_raw(fd);
    let mode = FallocateFlags::from_bits_retain(mode as _);
    match convert_res(rustix::fs::fallocate(fd, mode, offset as _, len as _)) {
        Some(()) => 0,
        None => -1,
    }
}
