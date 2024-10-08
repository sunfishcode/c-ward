use core::ptr::null_mut;
use libc::{c_int, c_void, size_t, ssize_t};

#[no_mangle]
unsafe extern "C" fn memchr(s: *const c_void, c: c_int, len: size_t) -> *mut c_void {
    libc!(libc::memchr(s, c, len));

    // It's tempting to use the [memchr crate] to optimize this. However its
    // API requires a Rust slice, which requires that all `len` bytes of the
    // buffer be accessible, while the C `memchr` API guarantees that it stops
    // accessing memory at the first byte that matches.
    //
    // [memchr crate](https://crates.io/crates/memchr)
    let mut s = s.cast::<u8>();
    for _ in 0..len {
        if *s == c as u8 {
            return s.cast_mut().cast();
        }
        s = s.add(1);
    }
    null_mut()
}

#[no_mangle]
unsafe extern "C" fn rawmemchr(s: *const c_void, c: c_int) -> *mut c_void {
    //libc!(libc::rawmemchr(s, c));

    let mut s = s.cast::<u8>();
    loop {
        if *s == c as u8 {
            return s.cast_mut().cast();
        }
        s = s.add(1);
    }
}

// Extension: GNU
#[no_mangle]
unsafe extern "C" fn memrchr(s: *const c_void, c: c_int, len: size_t) -> *mut c_void {
    libc!(libc::memrchr(s, c, len));

    // As above, it's tempting to use the memchr crate, but the C API here has
    // requirements that we can't meet here.
    let mut s = s.cast::<u8>().add(len);
    for _ in 0..len {
        s = s.sub(1);
        if *s == c as u8 {
            return s.cast_mut().cast();
        }
    }
    null_mut()
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

    // Attempt to discourage compiler optimizations from thinking this `bzero`
    // is unnecessary.
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

    if needlelen == 0 {
        return haystack.cast_mut();
    }

    if haystacklen < needlelen {
        return null_mut();
    }

    let haystack = haystack.cast::<u8>();
    let needle = needle.cast::<u8>();

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

    let mut dst = dst.cast::<u8>();
    let mut src = src.cast::<u8>();

    for _ in 0..len {
        let b = src.read();
        dst.write(b);
        dst = dst.add(1);
        src = src.add(1);
        if b == c as u8 {
            return dst.cast();
        }
    }

    null_mut()
}

#[no_mangle]
unsafe extern "C" fn swab(from: *const c_void, to: *mut c_void, n: ssize_t) {
    //libc!(libc::swab(from, to, n));

    if n <= 0 {
        return;
    }

    let n = n as usize;
    let from = from.cast::<u16>();
    let to = to.cast::<u16>();
    for i in 0..(n / 2) {
        to.add(i)
            .write_unaligned(from.add(i).read_unaligned().swap_bytes());
    }
}
