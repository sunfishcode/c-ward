// `__*_chk` functions that have to live in c-gull because they depend on
// C functions not in the libc crate, due to `VaList` being unstable.

use core::ffi::VaList;
use libc::{c_char, c_int, size_t};

extern "C" {
    #[cold]
    fn __chk_fail() -> !;
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---snprintf-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __snprintf_chk(
    ptr: *mut c_char,
    len: size_t,
    flag: c_int,
    slen: size_t,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    let va_list = args.as_va_list();
    __vsnprintf_chk(ptr, len, flag, slen, fmt, va_list)
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---vsnprintf-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __vsnprintf_chk(
    ptr: *mut c_char,
    len: size_t,
    flag: c_int,
    slen: size_t,
    fmt: *const c_char,
    va_list: VaList<'_, '_>,
) -> c_int {
    if slen < len {
        __chk_fail();
    }

    if flag > 0 {
        unimplemented!("__USE_FORTIFY_LEVEL > 0");
    }

    super::vsnprintf(ptr, len, fmt, va_list)
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---sprintf-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __sprintf_chk(
    ptr: *mut c_char,
    flag: c_int,
    strlen: size_t,
    format: *const c_char,
    mut args: ...
) -> c_int {
    let va_list = args.as_va_list();
    __vsprintf_chk(ptr, flag, strlen, format, va_list)
}

#[no_mangle]
unsafe extern "C" fn __vsprintf_chk(
    ptr: *mut c_char,
    flag: c_int,
    strlen: size_t,
    fmt: *const c_char,
    va_list: VaList<'_, '_>,
) -> c_int {
    if flag > 0 {
        unimplemented!("__USE_FORTIFY_LEVEL > 0");
    }

    if strlen == 0 {
        __chk_fail();
    }

    // We can't check `vsprintf` up front, so do a `vsnprintf` and check the
    // results.
    let n = super::vsnprintf(ptr, strlen, fmt, va_list);
    if n >= 0 && n as size_t >= strlen {
        __chk_fail();
    }
    n
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---fprintf-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __fprintf_chk(
    file: *mut libc::FILE,
    flag: c_int,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    let va_list = args.as_va_list();
    __vfprintf_chk(file, flag, fmt, va_list)
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---vfprintf-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __vfprintf_chk(
    file: *mut libc::FILE,
    flag: c_int,
    fmt: *const c_char,
    va_list: VaList<'_, '_>,
) -> c_int {
    if flag > 0 {
        unimplemented!("__USE_FORTIFY_LEVEL > 0");
    }

    // Our `printf` uses `printf_compat` which doesn't support `%n`.

    super::vfprintf(file, fmt, va_list)
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---printf-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __printf_chk(flag: c_int, fmt: *const c_char, mut args: ...) -> c_int {
    let va_list = args.as_va_list();
    __vprintf_chk(flag, fmt, va_list)
}

#[no_mangle]
unsafe extern "C" fn __vprintf_chk(
    flag: c_int,
    fmt: *const c_char,
    va_list: VaList<'_, '_>,
) -> c_int {
    if flag > 0 {
        unimplemented!("__USE_FORTIFY_LEVEL > 0");
    }

    // Our `printf` uses `printf_compat` which doesn't support `%n`.

    super::vprintf(fmt, va_list)
}

#[no_mangle]
unsafe extern "C" fn __asprintf_chk(
    strp: *mut *mut c_char,
    flag: c_int,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    let va_list = args.as_va_list();
    __vasprintf_chk(strp, flag, fmt, va_list)
}

#[no_mangle]
unsafe extern "C" fn __vasprintf_chk(
    strp: *mut *mut c_char,
    flag: c_int,
    fmt: *const c_char,
    va_list: VaList<'_, '_>,
) -> c_int {
    if flag > 0 {
        unimplemented!("__USE_FORTIFY_LEVEL > 0");
    }

    super::vasprintf(strp, fmt, va_list)
}

#[no_mangle]
unsafe extern "C" fn __dprintf_chk(
    fd: c_int,
    flag: c_int,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    let va_list = args.as_va_list();
    __vdprintf_chk(fd, flag, fmt, va_list)
}

#[no_mangle]
unsafe extern "C" fn __vdprintf_chk(
    fd: c_int,
    flag: c_int,
    fmt: *const c_char,
    va_list: VaList<'_, '_>,
) -> c_int {
    if flag > 0 {
        unimplemented!("__USE_FORTIFY_LEVEL > 0");
    }

    super::vdprintf(fd, fmt, va_list)
}
