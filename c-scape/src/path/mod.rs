use libc::c_char;
use rustix::cstr;

const SLASH: c_char = b'/' as c_char;

#[no_mangle]
unsafe extern "C" fn basename(path: *mut c_char) -> *mut c_char {
    //libc!(libc::basename(path));

    // The glibc `basename` doesn't mutate the string.
    #[cfg(target_env = "gnu")]
    {
        let p = libc::strrchr(path, SLASH as _);
        if p.is_null() {
            path
        } else {
            p.add(1)
        }
    }

    // The POSIX `basename` does.
    #[cfg(not(target_env = "gnu"))]
    {
        __xpg_basename(path)
    }
}

#[no_mangle]
unsafe extern "C" fn __xpg_basename(path: *mut c_char) -> *mut c_char {
    //libc!(libc::__xpg_basename(path));

    if path.is_null() || *path == 0 {
        return cstr!(".").as_ptr().cast_mut();
    }

    // Find the last slash.
    let mut p = libc::strrchr(path, SLASH as _);
    if p.is_null() {
        return path;
    }
    if *p.add(1) != 0 {
        return p.add(1);
    }

    // Skip any extra slashes.
    while p > path && *p.sub(1) == SLASH {
        p = p.sub(1);
    }

    // If it's all slashes, return the last one.
    if p == path {
        return p.add(libc::strlen(p) - 1);
    }

    // NUL-terminate.
    *p = 0;
    p = p.sub(1);

    // Scan until we find a slash.
    while p > path && *p.sub(1) != SLASH {
        p = p.sub(1);
    }

    p
}

#[no_mangle]
unsafe extern "C" fn dirname(path: *mut c_char) -> *mut c_char {
    libc!(libc::dirname(path));

    if path.is_null() {
        return cstr!(".").as_ptr().cast_mut();
    }

    // Start at the end of the string.
    let mut i = libc::strlen(path);

    // If the string is exactly "//", then return it as "//", because this is
    // a special case in POSIX:
    //
    // > A pathname that begins with two successive slashes may be interpreted
    // > in an implementation-defined manner, although more than two leading
    // > slashes shall be treated as a single slash.
    //
    // <https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap04.html#tag_04_13>
    if i == 2 && *path == SLASH && *path.add(1) == SLASH {
        return b"//\0".as_ptr() as _;
    }

    // Walk back past any trailing slashes.
    while i >= 2 && *path.add(i - 1) == SLASH {
        i -= 1;
    }

    // Walk back past the "basename" part.
    while i >= 1 && *path.add(i - 1) != SLASH {
        i -= 1;
    }

    // Walk back past any number of slashes.
    while i >= 2 && *path.add(i - 1) == SLASH {
        i -= 1;
    }

    // If there was no dir name, return ".".
    if i == 0 {
        return cstr!(".").as_ptr().cast_mut();
    }

    // Terminate the string at the end of the dirname and return it.
    *path.add(i) = 0;
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirname_basename() {
        use core::ffi::CStr;

        fn test(input: &CStr, expected_dir: &CStr, expected_gnu: &CStr, expected_posix: &CStr) {
            unsafe {
                let mut s = input.to_bytes_with_nul().to_vec();
                let i = s.as_mut_ptr().cast();
                let o = libc::dirname(i);
                assert_eq!(CStr::from_ptr(o), expected_dir);

                let mut s = input.to_bytes_with_nul().to_vec();
                let i = s.as_mut_ptr().cast();
                let o = libc::gnu_basename(i);
                assert_eq!(CStr::from_ptr(o), expected_gnu);

                let mut s = input.to_bytes_with_nul().to_vec();
                let i = s.as_mut_ptr().cast();
                let o = libc::posix_basename(i);
                assert_eq!(CStr::from_ptr(o), expected_posix);
            }
        }

        test(cstr!("/usr/lib"), cstr!("/usr"), cstr!("lib"), cstr!("lib"));
        test(
            cstr!("/usr//lib"),
            cstr!("/usr"),
            cstr!("lib"),
            cstr!("lib"),
        );
        test(cstr!("/usr/lib/"), cstr!("/usr"), cstr!(""), cstr!("lib"));
        test(cstr!("/usr/lib//"), cstr!("/usr"), cstr!(""), cstr!("lib"));
        test(cstr!("/"), cstr!("/"), cstr!(""), cstr!("/"));
        test(cstr!("//"), cstr!("//"), cstr!(""), cstr!("/"));
        test(cstr!("///"), cstr!("/"), cstr!(""), cstr!("/"));
        test(cstr!(""), cstr!("."), cstr!(""), cstr!("."));
        test(cstr!("usr"), cstr!("."), cstr!("usr"), cstr!("usr"));
        test(cstr!("usr/"), cstr!("."), cstr!(""), cstr!("usr"));
        test(cstr!("usr//"), cstr!("."), cstr!(""), cstr!("usr"));
    }
}
