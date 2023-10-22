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
use libc::{c_char, c_int, c_long, c_uchar, c_void, size_t};
use printf_compat::{format, output};
use rustix::fd::{FromRawFd, IntoRawFd};
use std::cmp::min;
use std::ffi::{CStr, VaList};
use std::io::{stderr as rust_stderr, stdin as rust_stdin, stdout as rust_stdout, Read, Write};
use std::ptr::{copy_nonoverlapping, null_mut};

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
unsafe extern "C" fn fgets(s: *mut c_char, size: c_int, file: *mut c_void) -> *mut c_char {
    //libc!(libc::fgets(s, size, file);

    if size < 0 {
        set_errno(Errno(libc::EINVAL));
        return null_mut();
    }

    let size = size as usize;
    let ptr = s.cast::<u8>();

    // Zero out the memory, since forming a slice requires initialized memory.
    core::ptr::write_bytes(ptr, 0, size);

    // Subtract one for the terminating NUL.
    let mut buf = core::slice::from_raw_parts_mut(ptr.cast::<u8>(), size - 1);

    if file == stdin {
        match rust_stdin().read(&mut buf) {
            Ok(n) => {
                buf[n] = b'\0';
                s
            }
            Err(err) => {
                set_errno(Errno(err.raw_os_error().unwrap_or(libc::EIO)));
                null_mut()
            }
        }
    } else {
        unimplemented!("fgets from a source other than stdin")
    }
}

#[no_mangle]
unsafe extern "C" fn fileno(file: *mut c_void) -> c_int {
    //libc!(libc::fileno(file));

    if file == stdin {
        0
    } else if file == stdout {
        1
    } else if file == stderr {
        2
    } else {
        unimplemented!("fileno with a file other than stdio")
    }
}

#[no_mangle]
unsafe extern "C" fn fseek(file: *mut c_void, offset: c_long, whence: c_int) -> c_int {
    //libc!(libc::fseek(file, offset, whence));

    let r = if file == stdin {
        libc::lseek(0, offset, whence)
    } else if file == stdout {
        match rust_stdout().flush() {
            Ok(()) => {}
            Err(err) => {
                set_errno(Errno(err.raw_os_error().unwrap_or(libc::EIO)));
                return -1;
            }
        }
        libc::lseek(1, offset, whence)
    } else if file == stderr {
        libc::lseek(2, offset, whence)
    } else {
        unimplemented!("fileno with a file other than stdio")
    };

    if r == -1 {
        -1
    } else {
        0
    }
}

// `__*_chk` functions that have to live in c-gull because they depend on
// C functions not in the libc crate, due to `VaList` being unstable.

extern "C" {
    fn __chk_fail();
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
    if slen < len {
        __chk_fail();
    }

    if flag > 0 {
        unimplemented!("__USE_FORTIFY_LEVEL > 0");
    }

    let va_list = args.as_va_list();
    vsnprintf(ptr, len, fmt, va_list)
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---vsnprintf-chk-1.html>
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
        unimplemented!("__USE_FORTIFY_LEVEL > 0");
    }

    vsnprintf(ptr, len, fmt, va_list)
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
    if flag > 0 {
        unimplemented!("__USE_FORTIFY_LEVEL > 0");
    }

    if strlen == 0 {
        __chk_fail();
    }

    // We can't check `sprintf` up front, so do a `vsnprintf` and check the
    // results.
    let va_list = args.as_va_list();
    let n = vsnprintf(ptr, strlen, format, va_list);
    if n >= 0 && n as size_t >= strlen {
        __chk_fail();
    }
    n
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---fprintf-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __fprintf_chk(
    file: *mut c_void,
    flag: c_int,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    if flag > 0 {
        unimplemented!("__USE_FORTIFY_LEVEL > 0");
    }

    // Our `printf` uses `printf_compat` which doesn't support `%n`.

    let va_list = args.as_va_list();
    vfprintf(file, fmt, va_list)
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---vfprintf-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __vfprintf_chk(
    file: *mut c_void,
    flag: c_int,
    fmt: *const c_char,
    va_list: VaList,
) -> c_int {
    if flag > 0 {
        unimplemented!("__USE_FORTIFY_LEVEL > 0");
    }

    // Our `printf` uses `printf_compat` which doesn't support `%n`.

    vfprintf(file, fmt, va_list)
}

// <https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---printf-chk-1.html>
#[no_mangle]
unsafe extern "C" fn __printf_chk(flag: c_int, fmt: *const c_char, mut args: ...) -> c_int {
    if flag > 0 {
        unimplemented!("__USE_FORTIFY_LEVEL > 0");
    }

    // Our `printf` uses `printf_compat` which doesn't support `%n`.

    let va_list = args.as_va_list();
    vprintf(fmt, va_list)
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
