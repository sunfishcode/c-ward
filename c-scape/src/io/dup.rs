use rustix::fd::{BorrowedFd, FromRawFd, IntoRawFd, OwnedFd};

use libc::c_int;

use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn dup(fd: c_int) -> c_int {
    libc!(libc::dup(fd));

    match convert_res(rustix::io::dup(BorrowedFd::borrow_raw(fd))) {
        Some(fd) => fd.into_raw_fd(),
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn dup2(fd: c_int, to: c_int) -> c_int {
    libc!(libc::dup2(fd, to));

    // `dup2` requires an `OwnedFd` since it closes the old fd before reusing
    // the index for the new fd.
    let mut to = OwnedFd::from_raw_fd(to);

    let result = convert_res(rustix::io::dup2(BorrowedFd::borrow_raw(fd), &mut to));

    // Convert it back into a raw fd, so that we don't drop it.
    let to = to.into_raw_fd();

    match result {
        Some(()) => to,
        None => -1,
    }
}
