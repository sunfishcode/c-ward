use crate::convert_res;
use core::ptr::null_mut;
use errno::{set_errno, Errno};
use libc::{c_char, c_int};
use rand::{Rng, TryRngCore};
use rand_core::OsRng;
use rustix::fd::IntoRawFd;
use rustix::fs::MemfdFlags;

#[no_mangle]
unsafe extern "C" fn mkstemp(template: *mut c_char) -> c_int {
    libc!(libc::mkstemp(template));

    mkstemp64(template)
}

#[no_mangle]
unsafe extern "C" fn mkstemp64(template: *mut c_char) -> c_int {
    //libc!(libc::mkstemp64(template));

    mkostemps(template, 0, 0)
}

#[no_mangle]
unsafe extern "C" fn mkostemp(template: *mut c_char, flags: c_int) -> c_int {
    libc!(libc::mkostemp(template, flags));

    mkostemps(template, 0, flags)
}

#[no_mangle]
unsafe extern "C" fn mkstemps(template: *mut c_char, suffixlen: c_int) -> c_int {
    libc!(libc::mkstemps(template, suffixlen));

    mkostemps(template, suffixlen, 0)
}

#[deprecated]
#[no_mangle]
unsafe extern "C" fn mktemp(template: *mut c_char) -> *mut c_char {
    //libc!(libc::mktemp(template));

    let fd = mkstemp(template);
    if fd < 0 {
        return null_mut();
    }
    libc::close(fd);
    libc::unlink(template);
    template
}

#[no_mangle]
unsafe extern "C" fn tmpfile64() -> *mut libc::FILE {
    libc!(libc::tmpfile64());

    let fd = match convert_res(rustix::fs::memfd_create(
        c"libc::tmpfile",
        MemfdFlags::empty(),
    )) {
        Some(fd) => fd,
        None => return null_mut(),
    };
    let fd = fd.into_raw_fd();
    libc::fdopen(fd, c"w+".as_ptr())
}

#[no_mangle]
unsafe extern "C" fn tmpfile() -> *mut libc::FILE {
    libc!(libc::tmpfile());

    tmpfile64()
}

#[no_mangle]
unsafe extern "C" fn mkostemps(template: *mut c_char, suffixlen: c_int, flags: c_int) -> c_int {
    libc!(libc::mkostemps(template, suffixlen, flags));

    const XXXXXX: &[u8; 6] = b"XXXXXX";
    const ALNUM: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

    let flags = (flags & !libc::O_ACCMODE) | libc::O_RDWR | libc::O_CREAT | libc::O_EXCL;

    let len = libc::strlen(template);
    let suffixlen = suffixlen as usize;
    let template = template.cast::<c_char>();

    if len < 6 || suffixlen > len - 6 {
        set_errno(Errno(libc::EINVAL));
        return -1;
    }

    if libc::memcmp(
        template.add(len - suffixlen - 6).cast(),
        XXXXXX.as_ptr().cast(),
        XXXXXX.len(),
    ) != 0
    {
        set_errno(Errno(libc::EINVAL));
        return -1;
    }

    for _ in 0..(ALNUM.len() * ALNUM.len() * ALNUM.len()) {
        for i in 0..XXXXXX.len() {
            let r = OsRng.unwrap_err().random_range(0..ALNUM.len());
            *template.add(len - suffixlen - 6 + i) = ALNUM[r] as c_char;
        }

        let fd = libc::open(template, flags, 0o600);
        if fd >= 0 {
            return fd;
        }
    }

    set_errno(Errno(libc::EEXIST));
    -1
}
