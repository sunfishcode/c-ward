use core::ptr::null_mut;
use libc::{c_int, c_void, size_t};

#[no_mangle]
unsafe extern "C" fn memchr(s: *const c_void, c: c_int, len: size_t) -> *mut c_void {
    libc!(libc::memchr(s, c, len));

    // It's tempting to use the [memchr crate] to optimize this. However its
    // API requires a Rust slice, which requires that all `len` bytes of the
    // buffer be accessible, while the C `memchr` API guarantees that it stops
    // accessing memory at the first byte that matches.
    //
    // [memchr crate](https://crates.io/crates/memchr)
    for i in 0..len {
        if *s.cast::<u8>().add(i) == c as u8 {
            return s.cast::<u8>().add(i).cast::<c_void>().cast_mut();
        }
    }
    null_mut()
}

// Extension: GNU
#[no_mangle]
unsafe extern "C" fn memrchr(s: *const c_void, c: c_int, len: size_t) -> *mut c_void {
    libc!(libc::memrchr(s, c, len));

    // As above, it's tempting to use the memchr crate, but the C API here has
    // requirements that we can't meet here.
    for i in 0..len {
        if *s.cast::<u8>().add(len - i - 1) == c as u8 {
            return s.cast::<u8>().add(len - i - 1).cast::<c_void>().cast_mut();
        }
    }
    null_mut()
}

#[cfg(feature = "define-mem-functions")]
#[no_mangle]
unsafe extern "C" fn memcmp(a: *const c_void, b: *const c_void, len: size_t) -> c_int {
    libc!(libc::memcmp(a, b, len));

    #[cfg(feature = "use-compiler-builtins")]
    {
        compiler_builtins::mem::memcmp(a.cast(), b.cast(), len)
    }

    #[cfg(not(feature = "use-compiler-builtins"))]
    {
        let a = a.cast::<u8>();
        let b = b.cast::<u8>();
        let mut i = 0;
        while i < len {
            let a = *a.add(i);
            let b = *b.add(i);
            if a != b {
                return a as i32 - b as i32;
            }
            i += 1;
            core::arch::asm!("");
        }
        0
    }
}

// Obsolescent
#[cfg(feature = "define-mem-functions")]
#[no_mangle]
unsafe extern "C" fn bcmp(a: *const c_void, b: *const c_void, len: size_t) -> c_int {
    //libc!(libc::bcmp(a, b, len));

    memcmp(a, b, len)
}

#[cfg(feature = "define-mem-functions")]
#[no_mangle]
unsafe extern "C" fn memcpy(dst: *mut c_void, src: *const c_void, len: size_t) -> *mut c_void {
    libc!(libc::memcpy(dst, src, len));

    #[cfg(feature = "use-compiler-builtins")]
    {
        compiler_builtins::mem::memcpy(dst.cast(), src.cast(), len).cast()
    }

    #[cfg(not(feature = "use-compiler-builtins"))]
    {
        let start = dst;
        let mut dst = dst.cast::<u8>();
        let mut src = src.cast::<u8>();
        let dst_end = dst.add(len);
        while dst < dst_end {
            *dst = *src;
            dst = dst.add(1);
            src = src.add(1);
            core::arch::asm!("");
        }
        start
    }
}

#[cfg(feature = "define-mem-functions")]
#[no_mangle]
unsafe extern "C" fn memmove(dst: *mut c_void, src: *const c_void, len: size_t) -> *mut c_void {
    libc!(libc::memmove(dst, src, len));

    #[cfg(feature = "use-compiler-builtins")]
    {
        compiler_builtins::mem::memmove(dst.cast(), src.cast(), len).cast()
    }

    #[cfg(not(feature = "use-compiler-builtins"))]
    {
        let start = dst;
        let mut dst = dst.cast::<u8>();
        let mut src = src.cast::<u8>();
        let delta = (dst.addr()).wrapping_sub(src.addr());
        if delta >= len {
            let dst_end = dst.add(len);
            while dst < dst_end {
                *dst = *src;
                dst = dst.add(1);
                src = src.add(1);
                core::arch::asm!("");
            }
        } else {
            let dst_start = dst;
            let mut dst = dst.add(len);
            let mut src = src.add(len);
            while dst > dst_start {
                dst = dst.sub(1);
                src = src.sub(1);
                *dst = *src;
                core::arch::asm!("");
            }
        }
        start
    }
}

#[cfg(feature = "define-mem-functions")]
#[no_mangle]
unsafe extern "C" fn memset(dst: *mut c_void, fill: c_int, len: size_t) -> *mut c_void {
    libc!(libc::memset(dst, fill, len));

    #[cfg(feature = "use-compiler-builtins")]
    {
        compiler_builtins::mem::memset(dst.cast(), fill, len).cast()
    }

    #[cfg(not(feature = "use-compiler-builtins"))]
    {
        let mut s = dst.cast::<u8>();
        let end = s.add(len);
        while s < end {
            *s = fill as _;
            s = s.add(1);
            core::arch::asm!("");
        }
        dst
    }
}

#[no_mangle]
unsafe extern "C" fn bzero(dst: *mut c_void, len: size_t) {
    //libc!(libc::bzero(dst, len));

    libc::memset(dst, 0, len);
}

#[no_mangle]
unsafe extern "C" fn explicit_bzero(dst: *mut c_void, len: size_t) {
    libc!(libc::explicit_bzero(dst, len));

    bzero(dst, len);
    core::arch::asm!("# {}, {}", in(reg) dst, in(reg) len, options(nostack, preserves_flags));
}

#[no_mangle]
unsafe extern "C" fn mempcpy(dst: *mut c_void, src: *const c_void, len: size_t) -> *mut c_void {
    //libc!(libc::mempcpy(dst, src, len));

    // `mempcpy` is the same as `memcpy` except it returns the pointer at the
    // end instead of the beginning.
    libc::memcpy(dst, src, len).cast::<u8>().add(len).cast()
}

#[no_mangle]
unsafe extern "C" fn memmem(
    haystack: *const c_void,
    haystacklen: size_t,
    needle: *const c_void,
    needlelen: size_t,
) -> *mut c_void {
    libc!(libc::memmem(haystack, haystacklen, needle, needlelen));

    let haystack = haystack.cast::<u8>();
    let needle = needle.cast::<u8>();

    if haystacklen == 0 || needlelen == 0 || haystacklen < needlelen {
        return null_mut();
    }

    let last = haystack.add(haystacklen).sub(needlelen);
    let mut p = haystack;
    while p <= last {
        if p.read() == needle.read() && libc::memcmp(p.cast(), needle.cast(), needlelen) == 0 {
            return p.cast::<c_void>().cast_mut();
        }
        p = p.add(1);
    }

    null_mut()
}

#[no_mangle]
unsafe extern "C" fn memccpy(
    dst: *mut c_void,
    src: *const c_void,
    c: c_int,
    len: size_t,
) -> *mut c_void {
    //libc!(libc::memccpy(dst, src, c, len));

    let dst = dst.cast::<u8>();
    let src = src.cast::<u8>();

    for i in 0..len {
        let b = src.add(i).read();
        if b == c as u8 {
            break;
        }
        dst.add(i).write(b);
    }

    dst.cast()
}
