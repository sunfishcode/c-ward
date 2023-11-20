use crate::convert_res;
use errno::{set_errno, Errno};
use libc::c_int;
use rustix::fd::{BorrowedFd, FromRawFd, IntoRawFd, OwnedFd};
use rustix::io::DupFlags;

#[no_mangle]
unsafe extern "C" fn dup(fd: c_int) -> c_int {
    libc!(libc::dup(fd));

    if fd == -1 {
        set_errno(Errno(libc::EBADF));
        return -1;
    }

    match convert_res(rustix::io::dup(BorrowedFd::borrow_raw(fd))) {
        Some(fd) => fd.into_raw_fd(),
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn dup2(fd: c_int, to: c_int) -> c_int {
    libc!(libc::dup2(fd, to));

    if fd == -1 {
        set_errno(Errno(libc::EBADF));
        return -1;
    }

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

#[no_mangle]
unsafe extern "C" fn dup3(fd: c_int, to: c_int, flags: c_int) -> c_int {
    libc!(libc::dup3(fd, to, flags));

    if fd == -1 {
        set_errno(Errno(libc::EBADF));
        return -1;
    }

    let flags = DupFlags::from_bits(flags as _).unwrap();

    // `dup2` requires an `OwnedFd` since it closes the old fd before reusing
    // the index for the new fd.
    let mut to = OwnedFd::from_raw_fd(to);

    let result = convert_res(rustix::io::dup3(BorrowedFd::borrow_raw(fd), &mut to, flags));

    // Convert it back into a raw fd, so that we don't drop it.
    let to = to.into_raw_fd();

    match result {
        Some(()) => to,
        None => -1,
    }
}
