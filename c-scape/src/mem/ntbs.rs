//! Library routines working with Nul-Terminated Byte Sequences (NTBS).

use core::cell::SyncUnsafeCell;
use core::ffi::CStr;
use core::ptr;
use libc::{c_char, c_int, c_uchar, malloc, memcpy, size_t};

use crate::sync_ptr::SyncMutPtr;

const NUL: c_char = 0;

#[no_mangle]
unsafe extern "C" fn stpcpy(mut d: *mut c_char, mut s: *const c_char) -> *mut c_char {
    libc!(libc::stpcpy(d, s));

    loop {
        *d = *s;

        if *d == NUL {
            break;
        }

        d = d.add(1);
        s = s.add(1);
    }

    d
}

#[no_mangle]
unsafe extern "C" fn stpncpy(
    mut d: *mut c_char,
    mut s: *const c_char,
    mut n: size_t,
) -> *mut c_char {
    libc!(libc::stpncpy(d, s, n));

    while n > 0 {
        *d = *s;

        if *d == NUL {
            break;
        }

        n -= 1;
        d = d.add(1);
        s = s.add(1);
    }

    libc::memset(d.cast(), 0, n);

    d
}

#[no_mangle]
unsafe extern "C" fn strcat(d: *mut c_char, s: *const c_char) -> *mut c_char {
    libc!(libc::strcat(d, s));

    strcpy(strchr(d, 0), s);
    d
}

#[no_mangle]
unsafe extern "C" fn strchr(s: *const c_char, c: c_int) -> *mut c_char {
    libc!(libc::strchr(s, c));

    let mut s = s.cast_mut();
    loop {
        if *s == c as _ {
            return s;
        }
        if *s == NUL {
            break;
        }
        s = s.add(1);
    }

    ptr::null_mut()
}

#[no_mangle]
unsafe extern "C" fn strchrnul(s: *const c_char, c: c_int) -> *mut c_char {
    libc!(libc::strchrnul(s, c));

    let mut s = s.cast_mut();
    loop {
        if *s == c as _ {
            break;
        }
        if *s == NUL {
            break;
        }
        s = s.add(1);
    }
    s
}

#[no_mangle]
unsafe extern "C" fn strcmp(mut s1: *const c_char, mut s2: *const c_char) -> c_int {
    libc!(libc::strcmp(s1, s2));

    while *s1 == *s2 && *s1 != NUL {
        s1 = s1.add(1);
        s2 = s2.add(1);
    }

    *s1 as c_uchar as c_int - *s2 as c_uchar as c_int
}

// enum for strverscmp state
// internal so no surface
// Tracks the current state of the comparison
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum StrverscmpState {
    /// Normal string comparison
    Normal,
    /// compare whole numbers: 1<2<10
    Integral,
    /// compare fractional 01 < 011 < 02
    Fractional,
    /// compare leading zeros 000 < 00
    LeadingZeros,
}

enum CharType {
    Zero,
    Digit,
    NonNumeric,
}

impl CharType {
    fn from_char(c: c_char) -> Self {
        match c as c_uchar {
            // ASCII 0
            b'0' => Self::Zero,
            // ASCII 1-9
            b'1'..=b'9' => Self::Digit,
            // non numeric ASCII
            _ => Self::NonNumeric,
        }
    }
}

impl StrverscmpState {
    fn transition(&mut self, s: CharType) {
        match s {
            CharType::Zero => {
                if self == &Self::Normal {
                    *self = Self::LeadingZeros;
                }
            }
            // ASCII 1-9
            CharType::Digit => {
                if self == &Self::Normal {
                    *self = Self::Integral;
                }
                if self == &Self::LeadingZeros {
                    *self = Self::Fractional;
                }
            }
            // non numeric ASCII
            CharType::NonNumeric => *self = Self::Normal,
        }
    }

    unsafe fn exit(&mut self, mut s1: *const c_char, mut s2: *const c_char) -> c_int {
        let chartype1 = CharType::from_char(*s1);
        let chartype2 = CharType::from_char(*s2);
        match (self, chartype1, chartype2) {
            // compare the strings based on length of the numeric part
            (StrverscmpState::Normal, CharType::Digit, CharType::Digit)
            | (StrverscmpState::Integral, CharType::Digit, CharType::Digit)
            | (StrverscmpState::Integral, CharType::Digit, CharType::Zero)
            | (StrverscmpState::Integral, CharType::Zero, CharType::Digit)
            | (StrverscmpState::Integral, CharType::Zero, CharType::Zero) => {
                let diff = *s1 as c_uchar as c_int - *s2 as c_uchar as c_int;
                loop {
                    let chartype1 = CharType::from_char(*s1);
                    let chartype2 = CharType::from_char(*s2);
                    match (chartype1, chartype2) {
                        (CharType::Zero, CharType::NonNumeric)
                        | (CharType::Digit, CharType::NonNumeric) => return 1,
                        (CharType::NonNumeric, CharType::Zero)
                        | (CharType::NonNumeric, CharType::Digit) => return -1,
                        (CharType::NonNumeric, CharType::NonNumeric) => break,
                        (_, _) => {
                            s1 = s1.add(1);
                            s2 = s2.add(1);
                        }
                    }
                }
                diff
            }

            (StrverscmpState::Integral, CharType::Zero, CharType::NonNumeric)
            | (StrverscmpState::Integral, CharType::Digit, CharType::NonNumeric)
            | (StrverscmpState::LeadingZeros, CharType::NonNumeric, CharType::Zero)
            | (StrverscmpState::LeadingZeros, CharType::NonNumeric, CharType::Digit) => 1,

            (StrverscmpState::Integral, CharType::NonNumeric, CharType::Zero)
            | (StrverscmpState::Integral, CharType::NonNumeric, CharType::Digit)
            | (StrverscmpState::LeadingZeros, CharType::Zero, CharType::NonNumeric)
            | (StrverscmpState::LeadingZeros, CharType::Digit, CharType::NonNumeric) => -1,

            // Compare the strings the same as strcmp
            (_, _, _) => *s1 as c_uchar as c_int - *s2 as c_uchar as c_int,
        }
    }
}

#[no_mangle]
unsafe extern "C" fn strverscmp(mut s1: *const c_char, mut s2: *const c_char) -> c_int {
    // libc!(libc::strverscmp(s1, s2));
    let mut state = StrverscmpState::Normal;
    while *s1 == *s2 && *s1 != NUL {
        state.transition(CharType::from_char(*s1));
        s1 = s1.add(1);
        s2 = s2.add(1);
    }

    state.exit(s1, s2)
}

#[no_mangle]
unsafe extern "C" fn strcpy(d: *mut c_char, s: *const c_char) -> *mut c_char {
    libc!(libc::strcpy(d, s));

    stpcpy(d, s);
    d
}

#[no_mangle]
unsafe extern "C" fn strncpy(d: *mut c_char, s: *const c_char, n: size_t) -> *mut c_char {
    libc!(libc::strncpy(d, s, n));

    stpncpy(d, s, n);
    d
}

#[no_mangle]
unsafe extern "C" fn strcspn(s: *const c_char, m: *const c_char) -> size_t {
    libc!(libc::strspn(s, m));

    let mut w = s;
    while *w != NUL {
        let mut m = m;
        while *m != NUL {
            if *w == *m {
                break;
            }
            m = m.add(1);
        }

        if *m != NUL {
            break;
        }

        w = w.add(1);
    }

    w.offset_from(s) as size_t
}

#[no_mangle]
unsafe extern "C" fn strdup(s: *const c_char) -> *mut c_char {
    libc!(libc::strdup(s));

    let len = libc::strlen(s);
    let d = malloc(len + 1);
    if !d.is_null() {
        memcpy(d, s.cast(), len + 1);
    }
    d.cast()
}

#[cfg(feature = "define-mem-functions")]
#[no_mangle]
unsafe extern "C" fn strlen(s: *const c_char) -> size_t {
    libc!(libc::strlen(s));

    #[cfg(feature = "use-compiler-builtins")]
    {
        compiler_builtins::mem::strlen(s)
    }

    #[cfg(not(feature = "use-compiler-builtins"))]
    {
        let mut s = s;
        let mut n = 0;
        while *s != 0 {
            n += 1;
            s = s.add(1);
            core::arch::asm!("");
        }
        n
    }
}

#[no_mangle]
unsafe extern "C" fn strncat(d: *mut c_char, mut s: *const c_char, mut n: size_t) -> *mut c_char {
    libc!(libc::strncat(d, s, n));

    let mut w = strchr(d, 0);

    while n > 0 && *s != NUL {
        n -= 1;

        *w = *s;

        w = w.add(1);
        s = s.add(1);
    }
    *w = 0;

    d
}

#[no_mangle]
unsafe extern "C" fn strncmp(mut s1: *const c_char, mut s2: *const c_char, mut n: size_t) -> c_int {
    libc!(libc::strncmp(s1, s2, n));

    loop {
        if n == 0 {
            return 0;
        }
        n -= 1;

        if *s1 != *s2 || *s1 == NUL {
            break;
        }

        s1 = s1.add(1);
        s2 = s2.add(1);
    }

    *s1 as c_uchar as c_int - *s2 as c_uchar as c_int
}

#[no_mangle]
unsafe extern "C" fn strndup(s: *const c_char, n: size_t) -> *mut c_char {
    libc!(libc::strndup(s, n));

    let len = strnlen(s, n);
    let d = malloc(len + 1);
    if !d.is_null() {
        memcpy(d, s.cast(), len);
    }

    let ret = d.cast::<c_char>();
    *ret.add(len) = 0;
    ret
}

#[no_mangle]
unsafe extern "C" fn strnlen(s: *const c_char, mut n: size_t) -> size_t {
    libc!(libc::strnlen(s, n));

    let mut w = s;
    while n > 0 && *w != NUL {
        n -= 1;
        w = w.add(1);
    }

    w.offset_from(s) as size_t
}

#[no_mangle]
unsafe extern "C" fn strnlen_s(s: *const c_char, n: size_t) -> size_t {
    //libc!(libc::strnlen_s(s, n));

    if s.is_null() {
        0
    } else {
        strnlen(s, n)
    }
}

#[no_mangle]
unsafe extern "C" fn strpbrk(s: *const c_char, m: *const c_char) -> *mut c_char {
    libc!(libc::strpbrk(s, m));

    let s = s.add(strcspn(s, m)).cast_mut();

    if *s != NUL {
        return s;
    }

    ptr::null_mut()
}

#[no_mangle]
unsafe extern "C" fn strrchr(s: *const c_char, c: c_int) -> *mut c_char {
    libc!(libc::strrchr(s, c));

    libc::memrchr(s.cast(), c, libc::strlen(s) + 1).cast()
}

#[no_mangle]
unsafe extern "C" fn strspn(s: *const c_char, m: *const c_char) -> size_t {
    libc!(libc::strspn(s, m));

    let mut w = s;
    while *w != NUL {
        let mut m = m;
        while *m != NUL {
            if *w == *m {
                break;
            }
            m = m.add(1);
        }

        if *m == NUL {
            break;
        }

        w = w.add(1);
    }

    w.offset_from(s) as size_t
}

#[no_mangle]
unsafe extern "C" fn strtok(s: *mut c_char, m: *const c_char) -> *mut c_char {
    libc!(libc::strtok(s, m));

    static STORAGE: SyncUnsafeCell<SyncMutPtr<c_char>> =
        SyncUnsafeCell::new(unsafe { SyncMutPtr::new(ptr::null_mut()) });

    strtok_r(s, m, SyncUnsafeCell::get(&STORAGE) as *mut *mut c_char)
}

#[no_mangle]
unsafe extern "C" fn strtok_r(
    s: *mut c_char,
    m: *const c_char,
    p: *mut *mut c_char,
) -> *mut c_char {
    libc!(libc::strtok_r(s, m, p));

    let mut s = if s.is_null() { *p } else { s };

    if s.is_null() {
        return ptr::null_mut();
    }

    s = s.add(strspn(s, m));
    if *s == NUL {
        *p = ptr::null_mut();
        return ptr::null_mut();
    }

    let t = s.add(strcspn(s, m));
    if *t != NUL {
        *t = NUL;
        *p = t.add(1);
    } else {
        *p = ptr::null_mut();
    }

    s
}

#[no_mangle]
unsafe extern "C" fn strcasecmp(mut s1: *const c_char, mut s2: *const c_char) -> c_int {
    libc!(libc::strcasecmp(s1, s2));

    while *s1 != NUL && *s2 != NUL {
        if libc::tolower(*s1 as c_uchar as c_int) != libc::tolower(*s2 as c_uchar as c_int) {
            break;
        }

        s1 = s1.add(1);
        s2 = s2.add(1);
    }

    libc::tolower(*s1 as c_uchar as c_int) - libc::tolower(*s2 as c_uchar as c_int)
}

#[no_mangle]
unsafe extern "C" fn strncasecmp(
    mut s1: *const c_char,
    mut s2: *const c_char,
    mut n: size_t,
) -> c_int {
    libc!(libc::strncasecmp(s1, s2, n));

    loop {
        if n == 0 {
            return 0;
        }
        n -= 1;

        if libc::tolower(*s1 as c_uchar as c_int) != libc::tolower(*s2 as c_uchar as c_int)
            || *s1 == NUL
        {
            break;
        }

        s1 = s1.add(1);
        s2 = s2.add(1);
    }

    libc::tolower(*s1 as c_uchar as c_int) - libc::tolower(*s2 as c_uchar as c_int)
}

#[no_mangle]
unsafe extern "C" fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char {
    libc!(libc::strstr(haystack, needle));

    if *needle == 0 {
        return haystack.cast_mut();
    }

    let mut haystack = haystack;
    loop {
        if *haystack == 0 {
            break;
        }
        let mut len = 0;
        for n in CStr::from_ptr(needle).to_bytes() {
            let h = *haystack.add(len);
            if h != *n as c_char {
                break;
            }
            len += 1;
        }
        if *needle.add(len) == 0 {
            return haystack.cast_mut();
        }
        haystack = haystack.add(1);
    }
    ptr::null_mut()
}

#[no_mangle]
unsafe extern "C" fn strcasestr(haystack: *const c_char, needle: *const c_char) -> *mut c_char {
    libc!(libc::strcasestr(haystack, needle));

    if *needle == 0 {
        return haystack.cast_mut();
    }

    let mut haystack = haystack;
    loop {
        if *haystack == 0 {
            break;
        }
        let mut len = 0;
        for n in CStr::from_ptr(needle).to_bytes() {
            let h = *haystack.add(len);
            if libc::tolower(h as _) != libc::tolower(*n as _) {
                break;
            }
            len += 1;
        }
        if *needle.add(len) == 0 {
            return haystack.cast_mut();
        }
        haystack = haystack.add(1);
    }
    ptr::null_mut()
}

#[no_mangle]
unsafe extern "C" fn index(s: *const c_char, c: c_int) -> *mut c_char {
    //libc!(libc::index(s, c));

    strchr(s, c)
}

#[no_mangle]
unsafe extern "C" fn rindex(s: *const c_char, c: c_int) -> *mut c_char {
    //libc!(libc::rindex(s, c));

    strrchr(s, c)
}

#[no_mangle]
unsafe extern "C" fn strsep(str_: *mut *mut c_char, sep: *const c_char) -> *mut c_char {
    //libc!(libc::strsep(str_, sep));

    let s = *str_;
    if s.is_null() {
        return ptr::null_mut();
    }
    let mut end = s.add(strcspn(s, sep));
    if *end != 0 {
        *end = 0;
        end = end.add(1);
    } else {
        end = ptr::null_mut();
    }
    *str_ = end;
    s
}

#[no_mangle]
unsafe extern "C" fn strlcpy(dst: *mut c_char, src: *const c_char, limit: size_t) -> size_t {
    //libc!(libc::strlcpy(dst, src, limit));

    let src_len = libc::strlen(src);

    if src_len < limit {
        libc::memcpy(dst.cast(), src.cast(), src_len + 1);
    } else if limit > 0 {
        libc::memcpy(dst.cast(), src.cast(), limit);
        *dst.add(limit - 1) = 0;
    }

    src_len
}

#[no_mangle]
unsafe extern "C" fn strlcat(dst: *mut c_char, src: *const c_char, limit: size_t) -> size_t {
    //libc!(libc::strlcat(dst, src, limit));

    let len = strnlen(dst, limit);
    if len == limit {
        return libc::strlen(src) + len;
    }

    strlcpy(dst.add(len), src, limit - len) + len
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strverscmp() {
        unsafe {
            assert!(strverscmp("000\0".as_ptr().cast(), "00\0".as_ptr().cast()) < 0);
            assert!(strverscmp("00\0".as_ptr().cast(), "01\0".as_ptr().cast()) < 0);
            assert!(strverscmp("01\0".as_ptr().cast(), "010\0".as_ptr().cast()) < 0);
            assert!(strverscmp("010\0".as_ptr().cast(), "09\0".as_ptr().cast()) < 0);
            assert!(strverscmp("09\0".as_ptr().cast(), "0\0".as_ptr().cast()) < 0);
            assert!(strverscmp("0\0".as_ptr().cast(), "1\0".as_ptr().cast()) < 0);
            assert!(strverscmp("1\0".as_ptr().cast(), "9\0".as_ptr().cast()) < 0);
            assert!(strverscmp("9\0".as_ptr().cast(), "10\0".as_ptr().cast()) < 0);
        }
    }
}
