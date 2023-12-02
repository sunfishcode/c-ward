use crate::READ_BUFFER;
use core::cmp::min;
use core::ptr::copy_nonoverlapping;
use core::slice;
use errno::{set_errno, Errno};
use libc::{c_int, c_void, iovec, off64_t, off_t, size_t, ssize_t};
use rustix::fd::BorrowedFd;
use rustix::io::IoSliceMut;

use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn read(fd: c_int, ptr: *mut c_void, len: usize) -> isize {
    libc!(libc::read(fd, ptr, len));

    if fd == -1 {
        set_errno(Errno(libc::EBADF));
        return -1;
    }

    // `slice::from_raw_parts_mut` assumes that the memory is initialized,
    // which our C API here doesn't guarantee. Since rustix currently requires
    // a slice, use a temporary copy.
    match convert_res(rustix::io::read(
        BorrowedFd::borrow_raw(fd),
        &mut READ_BUFFER[..min(len, READ_BUFFER.len())],
    )) {
        Some(nread) => {
            copy_nonoverlapping(READ_BUFFER.as_ptr(), ptr.cast::<u8>(), nread);
            nread as isize
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn readv(fd: c_int, iov: *const iovec, iovcnt: c_int) -> isize {
    libc!(libc::readv(fd, iov, iovcnt));

    if fd == -1 {
        set_errno(Errno(libc::EBADF));
        return -1;
    }

    if iovcnt < 0 {
        set_errno(Errno(libc::EINVAL));
        return -1;
    }

    let iov: *const IoSliceMut<'_> = checked_cast!(iov);

    // Note that rustix's `readv` takes a `&mut`, however it doesn't
    // mutate the `IoSliceMut` instances themselves, so it's safe to
    // cast away the `const` here.
    match convert_res(rustix::io::readv(
        BorrowedFd::borrow_raw(fd),
        slice::from_raw_parts_mut(iov.cast_mut(), iovcnt as usize),
    )) {
        Some(nread) => nread as isize,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn pread(fd: c_int, ptr: *mut c_void, len: usize, offset: off_t) -> isize {
    libc!(libc::pread(fd, ptr, len, offset));
    pread64(fd, ptr, len, offset as off64_t)
}

#[no_mangle]
unsafe extern "C" fn pread64(fd: c_int, ptr: *mut c_void, len: usize, offset: off64_t) -> isize {
    libc!(libc::pread64(fd, ptr, len, offset));

    if fd == -1 {
        set_errno(Errno(libc::EBADF));
        return -1;
    }

    // `slice::from_raw_parts_mut` assumes that the memory is initialized,
    // which our C API here doesn't guarantee. Since rustix currently requires
    // a slice, use a temporary copy.
    match convert_res(rustix::io::pread(
        BorrowedFd::borrow_raw(fd),
        &mut READ_BUFFER[..min(len, READ_BUFFER.len())],
        offset as u64,
    )) {
        Some(nread) => {
            copy_nonoverlapping(READ_BUFFER.as_ptr(), ptr.cast::<u8>(), nread);
            nread as isize
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn preadv(fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t) -> isize {
    libc!(libc::preadv(fd, iov, iovcnt, offset));
    preadv64(fd, iov, iovcnt, offset as off64_t)
}

#[no_mangle]
unsafe extern "C" fn preadv64(
    fd: c_int,
    iov: *const iovec,
    iovcnt: c_int,
    offset: off64_t,
) -> isize {
    libc!(libc::preadv64(fd, iov, iovcnt, offset));

    if fd == -1 {
        set_errno(Errno(libc::EBADF));
        return -1;
    }

    let iov: *const IoSliceMut<'_> = checked_cast!(iov);

    // Note that rustix's `readv` takes a `&mut`, however it doesn't
    // mutate the `IoSliceMut` instances themselves, so it's safe to
    // cast away the `const` here.
    match convert_res(rustix::io::preadv(
        BorrowedFd::borrow_raw(fd),
        slice::from_raw_parts_mut(iov.cast_mut(), iovcnt as usize),
        offset as u64,
    )) {
        Some(nwritten) => nwritten as isize,
        None => -1,
    }
}

// `__*_chk` functions that have to live in c-gull because they depend on
// C functions not in the libc crate, due to `VaList` being unstable.

extern "C" {
    #[cold]
    fn __chk_fail() -> !;
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---pread-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __pread_chk(
    fd: c_int,
    buf: *mut c_void,
    nbytes: size_t,
    offset: off_t,
    buflen: size_t,
) -> ssize_t {
    //libc!(libc::__pread_chk(fd, buf, nbytes, offset, buflen));

    if nbytes > buflen {
        __chk_fail();
    }

    pread(fd, buf, nbytes, offset)
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---pread64-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __pread64_chk(
    fd: c_int,
    buf: *mut c_void,
    nbytes: size_t,
    offset: off64_t,
    buflen: size_t,
) -> ssize_t {
    //libc!(libc::__pread64_chk(fd, buf, nbytes, offset, buflen));

    if nbytes > buflen {
        __chk_fail();
    }

    pread64(fd, buf, nbytes, offset)
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---read-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __read_chk(
    fd: c_int,
    buf: *mut c_void,
    nbytes: size_t,
    buflen: size_t,
) -> ssize_t {
    //libc!(libc::__read_chk(fd, buf, nbytes, buflen));

    if nbytes > buflen {
        __chk_fail();
    }

    read(fd, buf, nbytes)
}
