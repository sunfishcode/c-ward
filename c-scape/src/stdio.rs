//! The `stdio` family of functions.
//!
//! `FILE` is not currently buffered. And the `*_unlocked` functions currently
//! always lock. And the `printf` family of functions currently always call
//! `malloc`.
//!
//! The `printf` family of functions currently uses the `printf_compat` crate,
//! which [has differences with glibc]. And because we're using it in `no_std`
//! mode here, it only supports UTF-8 output.
//!
//! [has differences with glibc]: https://docs.rs/printf-compat/*/printf_compat/output/fn.fmt_write.html#differences

use crate::convert_res;
#[cfg(feature = "thread")]
use crate::GetThreadId;
use alloc::boxed::Box;
use alloc::string::String;
use core::cmp::min;
use core::ffi::{CStr, VaList};
use core::ptr::{addr_of, addr_of_mut, copy_nonoverlapping, null_mut};
use errno::{set_errno, Errno};
use libc::{c_char, c_int, c_long, c_void, off64_t, off_t, size_t};
use printf_compat::{format, output};
use rustix::cstr;
use rustix::fd::IntoRawFd;
use rustix::fs::{Mode, OFlags};
#[cfg(feature = "thread")]
use rustix_futex_sync::lock_api::RawReentrantMutex;
use rustix_futex_sync::Mutex;
#[cfg(feature = "thread")]
use rustix_futex_sync::RawMutex;

#[no_mangle]
unsafe extern "C" fn fputc(c: c_int, file: *mut libc::FILE) -> c_int {
    //libc!(libc::fputc(c, file));

    let c = c as u8;
    if fwrite([c].as_ptr().cast(), 1, 1, file) == 0 {
        libc::EOF
    } else {
        c as c_int
    }
}

#[no_mangle]
unsafe extern "C" fn fputc_unlocked(c: c_int, file: *mut libc::FILE) -> c_int {
    //libc!(libc::fputc_unlocked(c, file));

    fputc(c, file)
}

#[no_mangle]
unsafe extern "C" fn fputs(s: *const c_char, file: *mut libc::FILE) -> c_int {
    //libc!(libc::fputs(s, file));

    let len = libc::strlen(s);
    if fwrite(s.cast(), 1, len, file) != len {
        libc::EOF
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "C" fn fputs_unlocked(s: *const c_char, file: *mut libc::FILE) -> c_int {
    //libc!(libc::fputs_unlocked(s, file));

    fputs(s, file)
}

#[no_mangle]
unsafe extern "C" fn putc(c: c_int, file: *mut libc::FILE) -> c_int {
    //libc!(libc::putc(c, file));

    fputc(c, file)
}

#[no_mangle]
unsafe extern "C" fn putc_unlocked(c: c_int, file: *mut libc::FILE) -> c_int {
    //libc!(libc::putc_unlocked(c, file));

    putc(c, file)
}

#[no_mangle]
unsafe extern "C" fn putchar(c: c_int) -> c_int {
    libc!(libc::putchar(c));

    fputc(c, stdout)
}

#[no_mangle]
unsafe extern "C" fn putchar_unlocked(c: c_int) -> c_int {
    libc!(libc::putchar_unlocked(c));

    putchar(c)
}

#[no_mangle]
unsafe extern "C" fn puts(s: *const c_char) -> c_int {
    libc!(libc::puts(s));

    if fputs(s, stdout) >= 0 && fputc(b'\n' as c_int, stdout) != libc::EOF {
        0
    } else {
        libc::EOF
    }
}

#[no_mangle]
unsafe extern "C" fn fwrite(
    ptr: *const c_void,
    size: size_t,
    nmemb: size_t,
    file: *mut libc::FILE,
) -> size_t {
    //libc!(libc::fwrite(ptr, size, nmemb, file));

    // Overflow would be UB here.
    let len = nmemb * size;
    let mut file = (*file.cast::<FILE>()).locked.lock();
    let n = libc::write(file.fd, ptr, len);
    if n < 0 {
        file.error = true;
        0
    } else if size == 0 {
        0
    } else {
        n as size_t / size
    }
}

#[no_mangle]
unsafe extern "C" fn fwrite_unlocked(
    ptr: *const c_void,
    size: size_t,
    nmemb: size_t,
    file: *mut libc::FILE,
) -> size_t {
    //libc!(libc::fwrite_unlocked(ptr, size, nmemb, file));

    fwrite(ptr, size, nmemb, file)
}

#[no_mangle]
unsafe extern "C" fn fread(
    ptr: *mut c_void,
    size: size_t,
    nmemb: size_t,
    file: *mut libc::FILE,
) -> size_t {
    //libc!(libc::fread(ptr, size, nmemb, file));

    // Overflow would be UB here.
    let len = nmemb * size;

    let mut file = (*file.cast::<FILE>()).locked.lock();
    let n = libc::read(file.fd, ptr, len);
    if n == 0 {
        file.at_eof = true;
        0
    } else if n < 0 {
        file.error = true;
        0
    } else {
        n as size_t / size
    }
}

#[no_mangle]
unsafe extern "C" fn fflush(file: *mut libc::FILE) -> c_int {
    //libc!(libc::fflush(file);

    // `FILE` is not currently buffered, so we have nothing to do here.
    let _ = file;
    0
}

#[no_mangle]
unsafe extern "C" fn fgetc(file: *mut libc::FILE) -> c_int {
    //libc!(libc::fgetc(file);

    let mut file = (*file.cast::<FILE>()).locked.lock();
    let mut c = 0 as c_char;
    let n = libc::read(file.fd, addr_of_mut!(c).cast(), 1);
    if n == 0 {
        file.at_eof = true;
        libc::EOF
    } else if n < 0 {
        file.error = true;
        libc::EOF
    } else {
        c as c_int
    }
}

#[no_mangle]
unsafe extern "C" fn getc(file: *mut libc::FILE) -> c_int {
    //libc!(libc::getc(file));

    fgetc(file)
}

#[no_mangle]
unsafe extern "C" fn getchar() -> c_int {
    libc!(libc::getchar());

    getc(stdin)
}

#[no_mangle]
unsafe extern "C" fn getc_unlocked(file: *mut libc::FILE) -> c_int {
    //libc!(libc::getc_unlocked(file));

    getc(file)
}

#[no_mangle]
unsafe extern "C" fn getchar_unlocked() -> c_int {
    libc!(libc::getchar_unlocked());

    getchar()
}

#[no_mangle]
unsafe extern "C" fn fgets(s: *mut c_char, size: c_int, file: *mut libc::FILE) -> *mut c_char {
    //libc!(libc::fgets(s, size, file);

    if size < 0 {
        set_errno(Errno(libc::EINVAL));
        return null_mut();
    }

    let size = size as usize;
    let ptr = s.cast::<u8>();
    let mut num_read = 0;

    while num_read < size.saturating_sub(1) {
        let c = fgetc(file);
        if c == libc::EOF {
            break;
        }
        ptr.add(num_read).write(c as _);
        num_read += 1;
        if c == b'\n' as c_int {
            break;
        }
    }

    if num_read == 0 {
        null_mut()
    } else {
        ptr.add(num_read).write(b'\0');
        s
    }
}

#[no_mangle]
unsafe extern "C" fn fileno(file: *mut libc::FILE) -> c_int {
    //libc!(libc::fileno(file));

    let file = (*file.cast::<FILE>()).locked.lock();
    file.fd
}

#[no_mangle]
unsafe extern "C" fn clearerr(file: *mut libc::FILE) {
    libc!(libc::clearerr(file));

    let mut file = (*file.cast::<FILE>()).locked.lock();
    file.at_eof = false;
    file.error = false;
}

#[no_mangle]
unsafe extern "C" fn feof(file: *mut libc::FILE) -> c_int {
    libc!(libc::feof(file));

    let file = (*file.cast::<FILE>()).locked.lock();
    file.at_eof.into()
}

#[no_mangle]
unsafe extern "C" fn ferror(file: *mut libc::FILE) -> c_int {
    libc!(libc::ferror(file));

    let file = (*file.cast::<FILE>()).locked.lock();
    file.error.into()
}

#[no_mangle]
unsafe extern "C" fn fseek(file: *mut libc::FILE, offset: c_long, whence: c_int) -> c_int {
    //libc!(libc::fseek(file, offset, whence));

    let mut file = (*file.cast::<FILE>()).locked.lock();

    let r = libc::lseek(file.fd, offset, whence);

    if r == -1 {
        file.error = true;
        -1
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "C" fn fseeko(file: *mut libc::FILE, offset: off_t, whence: c_int) -> c_int {
    //libc!(libc::fseeko(file, offset, whence));

    let mut file = (*file.cast::<FILE>()).locked.lock();

    let r = libc::lseek(file.fd, offset, whence);

    if r == -1 {
        file.error = true;
        -1
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "C" fn fseeko64(file: *mut libc::FILE, offset: off64_t, whence: c_int) -> c_int {
    //libc!(libc::fseeko64(file, offset, whence));

    let mut file = (*file.cast::<FILE>()).locked.lock();

    let r = libc::lseek64(file.fd, offset, whence);

    if r == -1 {
        file.error = true;
        -1
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "C" fn ftell(file: *mut libc::FILE) -> c_long {
    //libc!(libc::ftell(file));

    let mut file = (*file.cast::<FILE>()).locked.lock();

    let r = libc::lseek(file.fd, 0, libc::SEEK_CUR);

    if r == -1 {
        file.error = true;
        -1
    } else if let Ok(r) = r.try_into() {
        r
    } else {
        file.error = true;
        set_errno(Errno(libc::EOVERFLOW));
        -1
    }
}

#[no_mangle]
unsafe extern "C" fn ftell_unlocked(file: *mut libc::FILE) -> c_long {
    //libc!(libc::ftell_unlocked(file));

    ftell(file)
}

#[no_mangle]
unsafe extern "C" fn ftello(file: *mut libc::FILE) -> off_t {
    //libc!(libc::ftello(file));

    let mut file = (*file.cast::<FILE>()).locked.lock();

    let r = libc::lseek(file.fd, 0, libc::SEEK_CUR);

    if r == -1 {
        file.error = true;
        -1
    } else if let Ok(r) = r.try_into() {
        r
    } else {
        file.error = true;
        set_errno(Errno(libc::EOVERFLOW));
        -1
    }
}

#[no_mangle]
unsafe extern "C" fn ftello64(file: *mut libc::FILE) -> off64_t {
    //libc!(libc::ftello64(file));

    let mut file = (*file.cast::<FILE>()).locked.lock();

    let r = libc::lseek64(file.fd, 0, libc::SEEK_CUR);

    if r == -1 {
        file.error = true;
        -1
    } else if let Ok(r) = r.try_into() {
        r
    } else {
        file.error = true;
        set_errno(Errno(libc::EOVERFLOW));
        -1
    }
}

#[no_mangle]
unsafe extern "C" fn rewind(file: *mut libc::FILE) {
    //libc!(libc::rewind(file));

    let mut file = (*file.cast::<FILE>()).locked.lock();

    let _ = libc::lseek(file.fd, 0, libc::SEEK_SET);

    file.error = false;
}

#[no_mangle]
unsafe extern "C" fn fgetpos(file: *mut libc::FILE, pos: *mut libc::fpos_t) -> c_int {
    //libc!(libc::fgetpos(file, pos));

    let new = ftell(file);
    if new == -1 {
        -1
    } else {
        *pos.cast::<off64_t>() = new.into();
        0
    }
}

#[no_mangle]
unsafe extern "C" fn fsetpos(file: *mut libc::FILE, pos: *const libc::fpos_t) -> c_int {
    //libc!(libc::fsetpos(file, pos));

    fseeko64(file, *pos.cast::<off64_t>(), libc::SEEK_SET)
}

#[no_mangle]
unsafe extern "C" fn freopen(
    path: *const c_char,
    mode: *const c_char,
    file: *mut libc::FILE,
) -> *mut libc::FILE {
    libc!(libc::freopen(path, mode, file));

    {
        let mut file = (*file.cast::<FILE>()).locked.lock();

        let oflags = match parse_oflags(mode) {
            Some(oflags) => oflags,
            None => {
                set_errno(Errno(libc::EINVAL));
                return null_mut();
            }
        };

        let path = CStr::from_ptr(path);

        let mode = Mode::RUSR | Mode::WUSR | Mode::RGRP | Mode::WGRP | Mode::ROTH | Mode::WOTH;

        let fd = match convert_res(rustix::fs::open(path, oflags, mode)) {
            Some(fd) => fd.into_raw_fd(),
            None => return null_mut(),
        };

        *file = File {
            fd,
            at_eof: false,
            error: false,
        };
    }

    file
}

#[no_mangle]
unsafe extern "C" fn fopen64(path: *const c_char, mode: *const c_char) -> *mut libc::FILE {
    libc!(libc::fopen64(path, mode));

    let oflags = match parse_oflags(mode) {
        Some(oflags) => oflags,
        None => {
            set_errno(Errno(libc::EINVAL));
            return null_mut();
        }
    };

    let path = CStr::from_ptr(path);

    let mode = Mode::RUSR | Mode::WUSR | Mode::RGRP | Mode::WGRP | Mode::ROTH | Mode::WOTH;

    let fd = match convert_res(rustix::fs::open(path, oflags, mode)) {
        Some(fd) => fd.into_raw_fd(),
        None => return null_mut(),
    };

    Box::into_raw(Box::new(FILE {
        locked: Mutex::new(File {
            fd,
            at_eof: false,
            error: false,
        }),
        #[cfg(feature = "thread")]
        flockfile_mutex: RawReentrantMutex::INIT,
    }))
    .cast()
}

#[no_mangle]
unsafe extern "C" fn fopen(path: *const c_char, mode: *const c_char) -> *mut libc::FILE {
    libc!(libc::fopen(path, mode));

    fopen64(path, mode)
}

#[no_mangle]
unsafe extern "C" fn fdopen(fd: c_int, mode: *const c_char) -> *mut libc::FILE {
    libc!(libc::fdopen(fd, mode));

    let oflags = match parse_oflags(mode) {
        Some(oflags) => oflags,
        None => {
            set_errno(Errno(libc::EINVAL));
            return null_mut();
        }
    };

    if oflags.contains(OFlags::APPEND) {
        if libc::lseek(fd, 0, libc::SEEK_END) == -1 {
            return null_mut();
        }
    }

    Box::into_raw(Box::new(FILE {
        locked: Mutex::new(File {
            fd,
            at_eof: false,
            error: false,
        }),
        #[cfg(feature = "thread")]
        flockfile_mutex: RawReentrantMutex::INIT,
    }))
    .cast()
}

#[no_mangle]
unsafe extern "C" fn fclose(file: *mut libc::FILE) -> c_int {
    libc!(libc::fclose(file));

    let file = (*file.cast::<FILE>()).locked.lock();
    rustix::io::close(file.fd);
    0
}

#[no_mangle]
unsafe extern "C" fn fpurge(file: *mut libc::FILE) -> c_int {
    //libc!(libc::fpurge(file));

    let _ = file;
    0
}

#[no_mangle]
unsafe extern "C" fn __fpurge(file: *mut libc::FILE) {
    //libc!(libc::__fpurge(file));

    let _ = file;
}

#[no_mangle]
unsafe extern "C" fn flockfile(file: *mut libc::FILE) {
    //libc!(libc::flockfile(file));

    #[cfg(feature = "thread")]
    {
        (*file.cast::<FILE>()).flockfile_mutex.lock();
    }
}

#[no_mangle]
unsafe extern "C" fn funlockfile(file: *mut libc::FILE) {
    //libc!(libc::funlockfile(file));

    #[cfg(feature = "thread")]
    {
        (*file.cast::<FILE>()).flockfile_mutex.unlock();
    }
}

#[no_mangle]
unsafe extern "C" fn ftrylockfile(file: *mut libc::FILE) -> c_int {
    //libc!(libc::ftrylockfile(file));

    #[cfg(feature = "thread")]
    {
        if (*file.cast::<FILE>()).flockfile_mutex.try_lock() {
            0
        } else {
            -1
        }
    }
}

#[no_mangle]
#[allow(non_upper_case_globals)]
pub(crate) static mut stdin: *mut libc::FILE = addr_of!(THE_STDIN).cast_mut().cast();
#[no_mangle]
#[allow(non_upper_case_globals)]
pub(crate) static mut stdout: *mut libc::FILE = addr_of!(THE_STDOUT).cast_mut().cast();
#[no_mangle]
#[allow(non_upper_case_globals)]
pub(crate) static mut stderr: *mut libc::FILE = addr_of!(THE_STDERR).cast_mut().cast();

/// The `FILE` implementation type. This is not ABI-exposed, and `libc::FILE`
/// is a zero-sized type. We cast `*mut libc::FILE` to `*mut FILE` and back as
/// needed.
#[allow(clippy::upper_case_acronyms)]
struct FILE {
    locked: Mutex<File>,

    /// An extra lock for implementing `flockfile` etc.
    #[cfg(feature = "thread")]
    flockfile_mutex: RawReentrantMutex<RawMutex, GetThreadId>,
}

/// The type that holds all the contents of a `FILE`, inside the mutex.
struct File {
    /// The raw file descriptor to do I/O on.
    fd: c_int,

    /// For [`feof`].
    at_eof: bool,

    /// For [`ferror`].
    error: bool,
}

static THE_STDIN: FILE = FILE {
    locked: Mutex::new(File {
        fd: libc::STDIN_FILENO,
        at_eof: false,
        error: false,
    }),
    #[cfg(feature = "thread")]
    flockfile_mutex: RawReentrantMutex::INIT,
};

static THE_STDOUT: FILE = FILE {
    locked: Mutex::new(File {
        fd: libc::STDOUT_FILENO,
        at_eof: false,
        error: false,
    }),
    #[cfg(feature = "thread")]
    flockfile_mutex: RawReentrantMutex::INIT,
};

static THE_STDERR: FILE = FILE {
    locked: Mutex::new(File {
        fd: libc::STDERR_FILENO,
        at_eof: false,
        error: false,
    }),
    #[cfg(feature = "thread")]
    flockfile_mutex: RawReentrantMutex::INIT,
};

/// Parse a mode string for `fopen`/`freopen`/`fdopen`.
unsafe fn parse_oflags(mode: *const c_char) -> Option<OFlags> {
    let mut bytes = mode.cast::<u8>();

    let mut oflags = match *bytes {
        b'r' => OFlags::RDONLY,
        b'w' => OFlags::WRONLY | OFlags::TRUNC | OFlags::CREATE,
        b'a' => OFlags::WRONLY | OFlags::APPEND | OFlags::CREATE,
        _ => return None,
    };
    bytes = bytes.add(1);

    if *bytes == b'+' {
        oflags &= !(OFlags::RDONLY | OFlags::WRONLY);
        oflags |= OFlags::RDWR;
        bytes = bytes.add(1);
    };

    loop {
        match *bytes {
            b'x' => oflags |= OFlags::EXCL,
            b'e' => oflags |= OFlags::CLOEXEC,
            b'\0' => break,
            _ => {}
        }
        bytes = bytes.add(1);
    }

    Some(oflags)
}

#[no_mangle]
unsafe extern "C" fn printf(fmt: *const c_char, mut args: ...) -> c_int {
    let va_list = args.as_va_list();
    vprintf(fmt, va_list)
}

#[no_mangle]
unsafe extern "C" fn vprintf(fmt: *const c_char, va_list: VaList<'_, '_>) -> c_int {
    //libc!(libc::vprintf(fmt, va_list));

    vfprintf(stdout, fmt, va_list)
}

#[no_mangle]
unsafe extern "C" fn sprintf(ptr: *mut c_char, fmt: *const c_char, mut args: ...) -> c_int {
    let va_list = args.as_va_list();
    vsprintf(ptr, fmt, va_list)
}

#[no_mangle]
unsafe extern "C" fn vsprintf(
    ptr: *mut c_char,
    fmt: *const c_char,
    va_list: VaList<'_, '_>,
) -> c_int {
    //libc!(libc::vsprintf(ptr, fmt, va_list));

    let mut out = String::new();
    let num_bytes = format(fmt, va_list, output::fmt_write(&mut out));
    if num_bytes >= 0 {
        copy_nonoverlapping(out.as_ptr(), ptr.cast(), num_bytes as usize);
    }
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
    va_list: VaList<'_, '_>,
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
unsafe extern "C" fn vdprintf(fd: c_int, fmt: *const c_char, va_list: VaList<'_, '_>) -> c_int {
    //libc!(libc::vdprintf(fd, fmt, va_list));

    let mut out = String::new();
    let num_bytes = format(fmt, va_list, output::fmt_write(&mut out));
    if num_bytes < 0 {
        return num_bytes;
    }

    let bytes = out.into_bytes();
    let mut remaining = &bytes[..];
    while !remaining.is_empty() {
        match libc::write(fd, remaining.as_ptr().cast(), remaining.len()) {
            -1 => {
                if errno::errno().0 != libc::EINTR {
                    return -1;
                }
            }
            n => remaining = &remaining[n as usize..],
        }
    }

    num_bytes
}

#[no_mangle]
unsafe extern "C" fn fprintf(file: *mut libc::FILE, fmt: *const c_char, mut args: ...) -> c_int {
    let va_list = args.as_va_list();
    vfprintf(file, fmt, va_list)
}

#[no_mangle]
unsafe extern "C" fn vfprintf(
    file: *mut libc::FILE,
    fmt: *const c_char,
    va_list: VaList<'_, '_>,
) -> c_int {
    //libc!(libc::vfprintf(file, fmt, va_list));

    let mut file = (*file.cast::<FILE>()).locked.lock();

    let r = vdprintf(file.fd, fmt, va_list);

    if r == -1 {
        file.error = true;
    }

    r
}

// `__*_chk` functions that have to live in c-gull because they depend on
// C functions not in the libc crate, due to `VaList` being unstable.

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
    va_list: VaList<'_, '_>,
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
    file: *mut libc::FILE,
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
    file: *mut libc::FILE,
    flag: c_int,
    fmt: *const c_char,
    va_list: VaList<'_, '_>,
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
unsafe extern "C" fn perror(user_message: *const c_char) {
    libc!(libc::perror(user_message));

    let mut storage: [c_char; 256] = [0; 256];
    let _ = libc::strerror_r(errno::errno().0, storage.as_mut_ptr(), storage.len());

    let errno_message = CStr::from_ptr(storage.as_mut_ptr());
    let user_message = if user_message.is_null() {
        CStr::from_ptr(rustix::cstr!("").as_ptr())
    } else {
        CStr::from_ptr(user_message)
    };

    if user_message.to_bytes().is_empty() {
        let _ = fputs(errno_message.as_ptr(), stderr);
    } else {
        let _ = fprintf(
            stderr,
            cstr!("%s: %s\n").as_ptr(),
            user_message.as_ptr(),
            errno_message.as_ptr(),
        );
    }
}

#[test]
fn test_fputs() {
    use core::ptr::null_mut;
    use rustix::cstr;
    unsafe {
        let mut buf = [0u8; 8];
        let fd = libc::memfd_create(cstr!("test").as_ptr(), 0);
        assert_ne!(fd, -1);
        let file = fdopen(fd, cstr!("w").as_ptr());
        assert_ne!(file, null_mut());

        assert!(fputs(cstr!("").as_ptr(), file) >= 0);
        assert!(fflush(file) == 0);
        assert_eq!(libc::pread(fd, buf.as_mut_ptr().cast(), buf.len(), 0), 0);
        assert_eq!(buf, [0u8; 8]);

        assert!(fputs(cstr!("hi").as_ptr(), file) >= 0);
        assert!(fflush(file) == 0);
        assert_eq!(libc::pread(fd, buf.as_mut_ptr().cast(), buf.len(), 0), 2);
        assert_eq!(&buf, b"hi\0\0\0\0\0\0");

        assert!(fputs(cstr!("hello\n").as_ptr(), file) >= 0);
        assert!(fflush(file) == 0);
        assert_eq!(libc::pread(fd, buf.as_mut_ptr().cast(), buf.len(), 2), 6);
        assert_eq!(&buf, b"hello\n\0\0");
    }
}
