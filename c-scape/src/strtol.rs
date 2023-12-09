use errno::{set_errno, Errno};
use libc::{c_char, c_int, c_long, c_longlong, c_ulong, c_ulonglong, intmax_t, uintmax_t};

#[no_mangle]
unsafe extern "C" fn strtoul(s: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong {
    libc!(libc::strtoul(s, endptr, base));

    strto(s, endptr, base, 0, c_ulong::MAX as _) as _
}

#[no_mangle]
unsafe extern "C" fn strtol(s: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long {
    libc!(libc::strtol(s, endptr, base));

    strto(s, endptr, base, c_long::MIN as _, c_long::MAX as _) as _
}

#[no_mangle]
unsafe extern "C" fn strtoull(
    s: *const c_char,
    endptr: *mut *mut c_char,
    base: c_int,
) -> c_ulonglong {
    libc!(libc::strtoull(s, endptr, base));

    strto(s, endptr, base, 0, c_ulonglong::MAX) as _
}

#[no_mangle]
unsafe extern "C" fn strtoll(
    s: *const c_char,
    endptr: *mut *mut c_char,
    base: c_int,
) -> c_longlong {
    libc!(libc::strtoll(s, endptr, base));

    strto(s, endptr, base, c_longlong::MIN, c_longlong::MAX as _) as _
}

#[no_mangle]
unsafe extern "C" fn strtoumax(
    s: *const c_char,
    endptr: *mut *mut c_char,
    base: c_int,
) -> uintmax_t {
    //libc!(libc::strtoumax(s, endptr, base));

    strto(s, endptr, base, 0, uintmax_t::MAX)
}

#[no_mangle]
unsafe extern "C" fn strtoimax(
    s: *const c_char,
    endptr: *mut *mut c_char,
    base: c_int,
) -> intmax_t {
    //libc!(libc::strtoimax(s, endptr, base));

    strto(s, endptr, base, intmax_t::MIN, intmax_t::MAX as _) as _
}

/// Helper function for `strto*` functions. `min` and `max` specify the minimum
/// and maximum values for the result type.
unsafe fn strto(
    s: *const c_char,
    endptr: *mut *mut c_char,
    base: c_int,
    min: intmax_t,
    max: uintmax_t,
) -> uintmax_t {
    if base < 0 || base > 36 {
        set_errno(Errno(libc::EINVAL));
        return max;
    }

    // Skip leading whitespace.
    let mut s = s;
    while libc::isspace(c_int::from(*s)) != 0 {
        s = s.add(1);
    }

    // Parse an optional +/- sign.
    let mut negate = false;
    if *s == b'+' as c_char {
        s = s.add(1);
    } else if *s == b'-' as c_char {
        negate = true;
        s = s.add(1);
    }

    // Parse an optional base prefix.
    let mut base: uintmax_t = base as uintmax_t;
    if base == 0 {
        if *s == b'0' as c_char {
            s = s.add(1);
            if (*s == b'x' as c_char || *s == b'X' as c_char)
                && matches!(*s.add(1) as u8, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')
            {
                s = s.add(1);
                base = 16;
            } else {
                base = 8;
            }
        } else {
            base = 10;
        }
    } else if base == 16
        && *s == b'0' as c_char
        && (*s.add(1) == b'x' as c_char || *s.add(1) == b'X' as c_char)
        && matches!(*s.add(2) as u8, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')
    {
        s = s.add(2);
    }

    // Parse the digits.
    let mut overflow = false;
    let mut num: uintmax_t = 0;
    loop {
        let digit: uintmax_t = match *s as u8 {
            x @ b'0'..=b'9' => x - b'0',
            x @ b'a'..=b'z' => x - b'a' + 10,
            x @ b'A'..=b'Z' => x - b'A' + 10,
            _ => break,
        }
        .into();
        if digit >= base {
            break;
        }

        if negate && min != 0 {
            if (num as intmax_t) < min / base as intmax_t {
                overflow = true;
            }
        } else {
            if num > max / base {
                overflow = true;
            }
        }
        num = num.wrapping_mul(base);

        if negate && min != 0 {
            if (num as intmax_t) < min + digit as intmax_t {
                overflow = true;
            }
            num = num.wrapping_sub(digit);
        } else {
            if num > max - digit {
                overflow = true;
            }
            num = num.wrapping_add(digit);
        }

        s = s.add(1);
    }

    // If requested, report the end position.
    if !endptr.is_null() {
        *endptr = s.cast_mut();
    }

    // Report overflow.
    if overflow {
        set_errno(Errno(libc::ERANGE));
        return if negate && min != 0 {
            min as uintmax_t
        } else {
            max
        };
    }

    // Perform negation if requested.
    if negate && min == 0 {
        num = num.wrapping_neg();
    }

    // Return a successful result.
    num as uintmax_t
}
