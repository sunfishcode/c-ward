use crate::convert_res;
use libc::c_int;
use rustix::fd::BorrowedFd;
use rustix::fs::FlockOperation;

#[no_mangle]
unsafe extern "C" fn flock(fd: c_int, operation: c_int) -> c_int {
    libc!(libc::flock(fd, operation));

    let fd = BorrowedFd::borrow_raw(fd);
    let operation = if operation == libc::LOCK_SH {
        FlockOperation::LockShared
    } else if operation == libc::LOCK_EX {
        FlockOperation::LockExclusive
    } else if operation == libc::LOCK_UN {
        FlockOperation::Unlock
    } else if operation == libc::LOCK_SH | libc::LOCK_NB {
        FlockOperation::NonBlockingLockShared
    } else if operation == libc::LOCK_EX | libc::LOCK_NB {
        FlockOperation::NonBlockingLockExclusive
    } else if operation == libc::LOCK_UN | libc::LOCK_NB {
        FlockOperation::NonBlockingUnlock
    } else {
        unreachable!()
    };

    match convert_res(rustix::fs::flock(fd, operation)) {
        Some(()) => 0,
        None => -1,
    }
}
