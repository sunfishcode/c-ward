use crate::convert_res;
use core::num::NonZeroU64;
use errno::{set_errno, Errno};
use libc::{c_int, off64_t, off_t};
use rustix::fd::BorrowedFd;
use rustix::fs::Advice;

#[no_mangle]
unsafe extern "C" fn posix_fadvise64(
    fd: c_int,
    offset: off64_t,
    len: off64_t,
    advice: c_int,
) -> c_int {
    libc!(libc::posix_fadvise64(fd, offset, len, advice));

    let advice = match advice {
        libc::POSIX_FADV_NORMAL => Advice::Normal,
        libc::POSIX_FADV_SEQUENTIAL => Advice::Sequential,
        libc::POSIX_FADV_RANDOM => Advice::Random,
        libc::POSIX_FADV_NOREUSE => Advice::NoReuse,
        libc::POSIX_FADV_WILLNEED => Advice::WillNeed,
        libc::POSIX_FADV_DONTNEED => Advice::DontNeed,
        _ => {
            set_errno(Errno(libc::EINVAL));
            return -1;
        }
    };
    match convert_res(rustix::fs::fadvise(
        BorrowedFd::borrow_raw(fd),
        offset as u64,
        NonZeroU64::new(len as u64),
        advice,
    )) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn posix_fadvise(fd: c_int, offset: off_t, len: off_t, advice: c_int) -> c_int {
    libc!(libc::posix_fadvise(fd, offset, len, advice));

    posix_fadvise64(fd, offset.into(), len.into(), advice)
}
