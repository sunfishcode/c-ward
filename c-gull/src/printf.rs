//! The `printf` family of functions.
//!
//! This code is highly experimental.
//!
//! This uses the `printf_compat` crate, which [has differences with glibc].
//!
//! The `fprintf` implementation doesn't have a real `FILE` type yet.
//!
//! Caution is indicated.
//!
//! [has differences with glibc]: https://docs.rs/printf-compat/0.1.1/printf_compat/output/fn.fmt_write.html#differences

use errno::{set_errno, Errno};
use libc::{c_char, c_int, c_uchar, c_void, size_t};
use printf_compat::{format, output};
use rustix::fd::{FromRawFd, IntoRawFd};
use std::cmp::min;
use std::ffi::{CStr, VaList};
use std::io::{stderr as rust_stderr, stdout as rust_stdout, Write};
use std::ptr::copy_nonoverlapping;

extern "C" {
    fn __chk_fail();
}

#[no_mangle]
unsafe extern "C" fn printf(fmt: *const c_char, mut args: ...) -> c_int {
    let va_list = args.as_va_list();
    vprintf(fmt, va_list)
}

#[no_mangle]
unsafe extern "C" fn vprintf(fmt: *const c_char, va_list: VaList) -> c_int {
    //libc!(libc::vprintf(fmt, va_list));

    format(fmt, va_list, output::io_write(&mut rust_stdout()))
}

#[no_mangle]
unsafe extern "C" fn sprintf(ptr: *mut c_char, fmt: *const c_char, mut args: ...) -> c_int {
    let va_list = args.as_va_list();
    vsprintf(ptr, fmt, va_list)
}

#[no_mangle]
unsafe extern "C" fn vsprintf(ptr: *mut c_char, fmt: *const c_char, va_list: VaList) -> c_int {
    //libc!(libc::vsprintf(ptr, fmt, va_list));

    let mut out = String::new();
    let num_bytes = format(fmt, va_list, output::fmt_write(&mut out));
    copy_nonoverlapping(out.as_ptr(), ptr.cast(), num_bytes as usize);
    num_bytes
}

#[no_mangle]
unsafe extern "C" fn snprintf(
    ptr: *mut c_char,
    len: usize,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    let va_list = args.as_va_list();
    vsnprintf(ptr, len, fmt, va_list)
}

#[no_mangle]
unsafe extern "C" fn vsnprintf(
    ptr: *mut c_char,
    len: usize,
    fmt: *const c_char,
    va_list: VaList,
) -> c_int {
    //libc!(libc::vsnprintf(ptr, len, fmt, va_list));

    let mut out = String::new();
    let num_bytes = format(fmt, va_list, output::fmt_write(&mut out));

    if num_bytes >= 0 {
        out.push('\0');

        copy_nonoverlapping(out.as_ptr(), ptr.cast(), min(num_bytes as usize + 1, len));
    }

    num_bytes
}

#[no_mangle]
unsafe extern "C" fn dprintf(fd: c_int, fmt: *const c_char, mut args: ...) -> c_int {
    let va_list = args.as_va_list();
    vdprintf(fd, fmt, va_list)
}

#[no_mangle]
unsafe extern "C" fn vdprintf(fd: c_int, fmt: *const c_char, va_list: VaList) -> c_int {
    //libc!(libc::vdprintf(fd, fmt, va_list));

    let mut tmp = std::fs::File::from_raw_fd(fd);
    let num_bytes = format(fmt, va_list, output::io_write(&mut tmp));
    let _ = tmp.into_raw_fd();
    num_bytes
}

#[no_mangle]
unsafe extern "C" fn fprintf(file: *mut c_void, fmt: *const c_char, mut args: ...) -> c_int {
    let va_list = args.as_va_list();
    vfprintf(file, fmt, va_list)
}

#[no_mangle]
unsafe extern "C" fn vfprintf(file: *mut c_void, fmt: *const c_char, va_list: VaList) -> c_int {
    //libc!(libc::vfprintf(file, fmt, va_list));

    if file == stdout {
        vprintf(fmt, va_list)
    } else if file == stderr {
        format(fmt, va_list, output::io_write(&mut rust_stderr()))
    } else {
        unimplemented!("vfprintf to a destination other than stdout or stderr")
    }
}

// <http://refspecs.linux-foundation.org/LSB_4.0.0/LSB-Core-generic/LSB-Core-generic/libc---snprintf-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __snprintf_chk(
    ptr: *mut c_char,
    len: size_t,
    flag: c_int,
    slen: size_t,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    if slen < len {
        __chk_fail();
    }

    if flag > 0 {
        unimplemented!("__USE_FORTIFY_LEVEL > 1");
    }

    let va_list = args.as_va_list();
    vsnprintf(ptr, len, fmt, va_list)
}

// <https://refspecs.linuxbase.org/LSB_4.0.0/LSB-Core-generic/LSB-Core-generic/libc---vsnprintf-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __vsnprintf_chk(
    ptr: *mut c_char,
    len: size_t,
    flag: c_int,
    slen: size_t,
    fmt: *const c_char,
    va_list: VaList,
) -> c_int {
    if slen < len {
        __chk_fail();
    }

    if flag > 0 {
        unimplemented!("__USE_FORTIFY_LEVEL > 1");
    }

    vsnprintf(ptr, len, fmt, va_list)
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

#[no_mangle]
unsafe extern "C" fn perror(user_message: *const c_char) {
    libc!(libc::perror(user_message));

    let errno_message = CStr::from_ptr(libc::strerror(errno::errno().0));
    let user_message = if user_message.is_null() {
        CStr::from_ptr(rustix::cstr!("").as_ptr())
    } else {
        CStr::from_ptr(user_message)
    };

    if user_message.to_bytes().is_empty() {
        eprintln!("{:?}", errno_message);
    } else {
        eprintln!("{:?}: {:?}", user_message, errno_message);
    }
}

#[no_mangle]
unsafe extern "C" fn fputc(c: c_int, file: *mut c_void) -> c_int {
    //libc!(libc::fputc(c, file));
    let r = fprintf(file, rustix::cstr!("%c").as_ptr(), c);
    if r == 1 {
        c as c_uchar as c_int
    } else {
        libc::EOF
    }
}

#[no_mangle]
unsafe extern "C" fn fputs(s: *const c_char, file: *mut c_void) -> c_int {
    //libc!(libc::fputs(s, file));
    fprintf(file, rustix::cstr!("%s").as_ptr(), s)
}

#[no_mangle]
unsafe extern "C" fn putc(c: c_int, file: *mut c_void) -> c_int {
    //libc!(libc::putc(c, file));
    fputc(c, file)
}

#[no_mangle]
unsafe extern "C" fn putchar(c: c_int) -> c_int {
    libc!(libc::putchar(c));
    fputc(c, stdout)
}

#[no_mangle]
unsafe extern "C" fn puts(s: *const c_char) -> c_int {
    libc!(libc::puts(s));
    printf(rustix::cstr!("%s\n").as_ptr(), s)
}

#[no_mangle]
unsafe extern "C" fn fwrite(
    ptr: *const c_void,
    size: size_t,
    nmemb: size_t,
    file: *mut c_void,
) -> size_t {
    //libc!(libc::fwrite(ptr, size, nmemb, file));

    // Overflow would be UB here.
    let len = nmemb * size;
    let buf = core::slice::from_raw_parts(ptr.cast::<u8>(), len);

    if file == stdout {
        match rust_stdout().write(&buf) {
            Ok(n) => n / size,
            Err(_err) => 0,
        }
    } else if file == stderr {
        match rust_stderr().write(&buf) {
            Ok(n) => n / size,
            Err(_err) => 0,
        }
    } else {
        unimplemented!("fwrite to a destination other than stdout or stderr")
    }
}

#[no_mangle]
unsafe extern "C" fn fflush(file: *mut c_void) -> c_int {
    //libc!(libc::fflush(file);

    if file == stdout {
        match rust_stdout().flush() {
            Ok(()) => 0,
            Err(err) => {
                set_errno(Errno(err.raw_os_error().unwrap_or(libc::EIO)));
                libc::EOF
            }
        }
    } else if file == stderr {
        // Stderr is not buffered; nothing to do.
        0
    } else {
        unimplemented!("fflush to a destination other than stdout or stderr")
    }
}

#[no_mangle]
#[allow(non_upper_case_globals)]
static mut stdin: *mut c_void = unsafe { THE_STDIN.as_mut_ptr().cast() };
#[no_mangle]
#[allow(non_upper_case_globals)]
static mut stdout: *mut c_void = unsafe { THE_STDOUT.as_mut_ptr().cast() };
#[no_mangle]
#[allow(non_upper_case_globals)]
static mut stderr: *mut c_void = unsafe { THE_STDERR.as_mut_ptr().cast() };

static mut THE_STDIN: [u8; 1] = [0];
static mut THE_STDOUT: [u8; 1] = [1];
static mut THE_STDERR: [u8; 1] = [2];
