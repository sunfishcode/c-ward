//! `__*_chk` functions.

use libc::{c_char, c_void, size_t};

// <https://refspecs.linuxbase.org/LSB_4.0.0/LSB-Core-generic/LSB-Core-generic/libc---chk-fail-1.html>
#[no_mangle]
unsafe extern "C" fn __chk_fail() {
    rustix::io::write(
        rustix::stdio::stderr(),
        b"A buffer overflow has been detected.\n",
    )
    .ok();
    libc::abort();
}

// <http://refspecs.linux-foundation.org/LSB_4.0.0/LSB-Core-generic/LSB-Core-generic/libc---memcpy-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __memcpy_chk(
    dest: *mut c_void,
    src: *const c_void,
    len: size_t,
    destlen: size_t,
) -> *mut c_void {
    if destlen < len {
        __chk_fail();
    }

    libc::memcpy(dest, src, len)
}

// <https://refspecs.linuxbase.org/LSB_4.0.0/LSB-Core-generic/LSB-Core-generic/libc---strncpy-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __strncpy_chk(
    dest: *mut c_char,
    src: *const c_char,
    len: size_t,
    destlen: size_t,
) -> *mut c_char {
    if destlen < len {
        __chk_fail();
    }

    libc::strncpy(dest, src, len)
}
