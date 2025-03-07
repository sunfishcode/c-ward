use crate::convert_res;
use errno::{set_errno, Errno};
use libc::{c_int, off64_t, off_t};
use rustix::fd::BorrowedFd;

#[no_mangle]
unsafe extern "C" fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    libc!(libc::lseek(fd, offset, whence));

    match lseek64(fd, off64_t::from(offset), whence).try_into() {
        Ok(v) => v,
        Err(_) => {
            set_errno(Errno(libc::EOVERFLOW));
            -1
        }
    }
}

#[no_mangle]
unsafe extern "C" fn lseek64(fd: c_int, offset: off64_t, whence: c_int) -> off64_t {
    libc!(libc::lseek64(fd, offset, whence));

    if fd == -1 {
        set_errno(Errno(libc::EBADF));
        return -1;
    }

    let seek_from = match whence {
        libc::SEEK_SET => rustix::fs::SeekFrom::Start(offset as u64),
        libc::SEEK_CUR => rustix::fs::SeekFrom::Current(offset),
        libc::SEEK_END => rustix::fs::SeekFrom::End(offset),
        #[cfg(any(apple, freebsdlike, linux_kernel, solarish))]
        libc::SEEK_DATA => rustix::fs::SeekFrom::Data(offset as u64),
        #[cfg(any(apple, freebsdlike, linux_kernel, solarish))]
        libc::SEEK_HOLE => rustix::fs::SeekFrom::Hole(offset as u64),
        _ => {
            set_errno(Errno(libc::EINVAL));
            return -1;
        }
    };
    match convert_res(rustix::fs::seek(BorrowedFd::borrow_raw(fd), seek_from)) {
        Some(offset) => offset as off64_t,
        None => -1,
    }
}
