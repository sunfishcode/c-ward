//! `__*_chk` functions.

use libc::{c_char, c_int, c_void, size_t};

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---chk-fail-1.html>
#[cold]
#[no_mangle]
unsafe extern "C" fn __chk_fail() -> ! {
    rustix::io::write(
        rustix::stdio::stderr(),
        b"A buffer overflow has been detected.\n",
    )
    .ok();
    libc::abort();
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---strcpy-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __strcpy_chk(
    dest: *mut c_char,
    src: *const c_char,
    destlen: size_t,
) -> *mut c_char {
    let src_strlen = libc::strlen(src);

    if src_strlen + 1 > destlen {
        __chk_fail();
    }

    libc::strcpy(dest, src)
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---memcpy-chk-1.html>
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

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---memset-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __memset_chk(
    dest: *mut c_void,
    c: c_int,
    len: size_t,
    destlen: size_t,
) -> *mut c_void {
    if destlen < len {
        __chk_fail();
    }

    libc::memset(dest, c, len)
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---strcat-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __strcat_chk(
    dest: *mut c_char,
    src: *const c_char,
    destlen: size_t,
) -> *mut c_char {
    let dest_strlen = libc::strlen(dest);
    let src_strlen = libc::strlen(src);

    if let Some(sum) = dest_strlen.checked_add(src_strlen) {
        if let Some(sum) = sum.checked_add(1) {
            if sum <= destlen {
                return libc::strcat(dest, src);
            }
        }
    }

    __chk_fail()
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---strncpy-chk-1.html>
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

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---fgets-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __fgets_chk(
    s: *mut c_char,
    size: size_t,
    strsize: c_int,
    stream: *mut c_void,
) -> *mut c_char {
    if strsize < 0 {
        __chk_fail();
    }
    if strsize as size_t > size {
        __chk_fail();
    }

    libc::fgets(s, strsize, stream.cast())
}
