use alloc::borrow::ToOwned;
use alloc::format;
use core::cell::SyncUnsafeCell;
use core::ptr::{copy_nonoverlapping, null_mut};
use libc::{c_char, c_int};

/// Return the address of the thread-local `errno` state.
///
/// This function conforms to the [LSB `__errno_location`] ABI.
///
/// [LSB `__errno_location`]: https://refspecs.linuxfoundation.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---errno-location.html
#[no_mangle]
#[cfg(feature = "take-charge")]
unsafe extern "C" fn __errno_location() -> *mut c_int {
    libc!(libc::__errno_location());

    #[cfg(feature = "thread")]
    return origin::thread::errno_location();

    #[cfg(not(feature = "thread"))]
    {
        static mut ERRNO: i32 = 0;
        return core::ptr::addr_of_mut!(ERRNO);
    }
}

#[no_mangle]
unsafe extern "C" fn strerror(errnum: c_int) -> *mut c_char {
    libc!(libc::strerror(errnum));

    static STORAGE: SyncUnsafeCell<[c_char; 256]> = SyncUnsafeCell::new([0; 256]);

    let storage = SyncUnsafeCell::get(&STORAGE);
    __xpg_strerror_r(errnum, (*storage).as_mut_ptr(), (*storage).len());
    (*storage).as_mut_ptr()
}

// <https://refspecs.linuxfoundation.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---xpg-strerror-r.html>
#[no_mangle]
unsafe extern "C" fn __xpg_strerror_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> c_int {
    //libc!(libc::__xpg_strerror_r(errnum, buf, buflen));
    libc!(libc::strerror_r(errnum, buf, buflen));

    if buflen == 0 {
        return libc::ERANGE;
    }

    let message = if errnum == 0 {
        "Success".to_owned()
    } else {
        match crate::error_str::error_str(rustix::io::Errno::from_raw_os_error(errnum)) {
            Some(s) => s.to_owned(),
            None => format!("Unknown error {}", errnum),
        }
    };

    let min = core::cmp::min(buflen - 1, message.len());
    copy_nonoverlapping(message.as_ptr().cast(), buf, min);
    buf.add(min).write(b'\0' as libc::c_char);
    0
}

/// glibc has a non-standard return type for its `strerror_r`.
// <https://refspecs.linuxfoundation.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib-strerror-r.html>
#[cfg(target_env = "gnu")]
#[no_mangle]
unsafe extern "C" fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> *mut c_char {
    //libc!(libc::strerror_r(errnum, buf, buflen));
    if __xpg_strerror_r(errnum, buf, buflen) == 0 {
        buf
    } else {
        null_mut()
    }
}

#[cfg(not(target_env = "gnu"))]
#[no_mangle]
unsafe extern "C" fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> c_int {
    libc!(libc::strerror_r(errnum, buf, buflen));
    __xpg_strerror_r(errnum, buf, buflen)
}
