use core::ptr::null_mut;
use core::str::FromStr;
use core::{slice, str};
use errno::{set_errno, Errno};
use libc::{c_char, c_double, c_float};

#[no_mangle]
unsafe extern "C" fn atof(nptr: *const c_char) -> c_double {
    libc!(libc::atof(nptr));

    strtod(nptr, null_mut())
}

#[no_mangle]
unsafe extern "C" fn strtof(nptr: *const c_char, endptr: *mut *mut c_char) -> c_float {
    libc!(libc::strtof(nptr, endptr));

    let nptr = nptr.cast::<u8>();
    let orig = nptr;
    let (nptr, format) = scan_float(nptr);
    let s = make_str(orig, nptr);

    match format {
        Format::Hexadecimal(_any_nonzero) => {
            todo!("hexadecimal float parsing")
        }
        Format::Decimal(any_nonzero) => {
            if let Ok(f) = f32::from_str(s) {
                set_endptr(endptr, nptr);
                set_errno_f32(f, any_nonzero);
                return f;
            }
        }
        Format::Infinity => {
            set_endptr(endptr, nptr);
            return if s.starts_with('-') {
                -f32::INFINITY
            } else {
                f32::INFINITY
            };
        }
        Format::NaN(payload) => {
            let result = if s.starts_with('-') {
                libm::copysignf(f32::NAN, -1.0)
            } else {
                libm::copysignf(f32::NAN, 1.0)
            };
            if let Some(payload) = payload {
                if let Ok(payload) = u32::try_from(payload) {
                    if (libm::copysignf(result, -1.0).to_bits() & payload) == 0 {
                        set_endptr(endptr, nptr);
                        return f32::from_bits(result.to_bits() | payload);
                    }
                }
            } else {
                set_endptr(endptr, nptr);
                return result;
            }
        }
    }

    set_endptr(endptr, orig);
    0.0
}

#[no_mangle]
unsafe extern "C" fn strtod(nptr: *const c_char, endptr: *mut *mut c_char) -> c_double {
    libc!(libc::strtod(nptr, endptr));

    let nptr = nptr.cast::<u8>();
    let orig = nptr;
    let (nptr, format) = scan_float(nptr);
    let s = make_str(orig, nptr);

    match format {
        Format::Hexadecimal(_any_nonzero) => {
            todo!("hexadecimal float parsing")
        }
        Format::Decimal(any_nonzero) => {
            if let Ok(f) = f64::from_str(s) {
                set_endptr(endptr, nptr);
                set_errno_f64(f, any_nonzero);
                return f;
            }
        }
        Format::Infinity => {
            set_endptr(endptr, nptr);
            return if s.starts_with('-') {
                -f64::INFINITY
            } else {
                f64::INFINITY
            };
        }
        Format::NaN(payload) => {
            let result = if s.starts_with('-') {
                libm::copysign(f64::NAN, -1.0)
            } else {
                libm::copysign(f64::NAN, 1.0)
            };
            if let Some(payload) = payload {
                if (libm::copysign(result, -1.0).to_bits() & payload) == 0 {
                    set_endptr(endptr, nptr);
                    return f64::from_bits(result.to_bits() | payload);
                }
            } else {
                set_endptr(endptr, nptr);
                return result;
            }
        }
    }

    set_endptr(endptr, orig);
    0.0
}

unsafe fn make_str<'a>(start: *const u8, nptr: *const u8) -> &'a str {
    str::from_utf8_unchecked(slice::from_raw_parts(
        start,
        nptr.offset_from(start) as usize,
    ))
    .trim_start()
}

unsafe fn set_endptr(endptr: *mut *mut c_char, nptr: *const u8) {
    if !endptr.is_null() {
        *endptr = nptr.cast_mut().cast();
    }
}

// If we tried to parse a number but got infinity, or if the number was
// subnormal, or we saw non-zero digits but got zero, set errno.
fn set_errno_f32(f: f32, any_nonzero: bool) {
    if f.is_infinite() || f.is_subnormal() || (f == 0.0 && any_nonzero) {
        set_errno(Errno(libc::ERANGE));
    }
}

fn set_errno_f64(f: f64, any_nonzero: bool) {
    if f.is_infinite() || f.is_subnormal() || (f == 0.0 && any_nonzero) {
        set_errno(Errno(libc::ERANGE));
    }
}

unsafe fn scan_float(mut nptr: *const u8) -> (*const u8, Format) {
    while (*nptr).is_ascii_whitespace() {
        nptr = nptr.add(1);
    }

    if *nptr == b'-' || *nptr == b'+' {
        nptr = nptr.add(1);
    }

    if (*nptr).to_ascii_lowercase() == b'i'
        && (*nptr.add(1)).to_ascii_lowercase() == b'n'
        && (*nptr.add(2)).to_ascii_lowercase() == b'f'
    {
        nptr = nptr.add(3);
        if (*nptr).to_ascii_lowercase() == b'i'
            && (*nptr.add(1)).to_ascii_lowercase() == b'n'
            && (*nptr.add(2)).to_ascii_lowercase() == b'i'
            && (*nptr.add(3)).to_ascii_lowercase() == b't'
            && (*nptr.add(4)).to_ascii_lowercase() == b'y'
        {
            nptr = nptr.add(5);
        }
        return (nptr, Format::Infinity);
    }

    if (*nptr).to_ascii_lowercase() == b'n'
        && (*nptr.add(1)).to_ascii_lowercase() == b'a'
        && (*nptr.add(2)).to_ascii_lowercase() == b'n'
    {
        nptr = nptr.add(3);
        if *nptr == b'(' {
            let paren = nptr;
            nptr = nptr.add(1);
            let payload = if *nptr == b'0' && (*nptr.add(1)).to_ascii_lowercase() == b'x' {
                nptr = nptr.add(2);
                let start = nptr;
                while (*nptr).is_ascii_hexdigit() {
                    nptr = nptr.add(1);
                }
                let s = make_str(start, nptr);
                if s.is_empty() {
                    0
                } else {
                    u64::from_str_radix(s, 16).unwrap()
                }
            } else {
                let start = nptr;
                while (*nptr).is_ascii_digit() {
                    nptr = nptr.add(1);
                }
                let s = make_str(start, nptr);
                if s.is_empty() {
                    0
                } else {
                    s.parse().unwrap()
                }
            };
            if *nptr == b')' {
                nptr = nptr.add(1);
                return (nptr, Format::NaN(Some(payload)));
            }
            nptr = paren;
        }
        return (nptr, Format::NaN(None));
    }

    let mut hex = false;
    if *nptr == b'0' {
        nptr = nptr.add(1);
        if *nptr == b'x' || *nptr == b'X' {
            nptr = nptr.add(1);
            hex = true;
        }
    }

    let mut any_nonzero = false;
    if hex {
        while (*nptr).is_ascii_hexdigit() {
            if *nptr != b'0' {
                any_nonzero = true;
            }
            nptr = nptr.add(1);
        }
    } else {
        while (*nptr).is_ascii_digit() {
            if *nptr != b'0' {
                any_nonzero = true;
            }
            nptr = nptr.add(1);
        }
    }

    if *nptr == b'.' {
        nptr = nptr.add(1);

        if hex {
            while (*nptr).is_ascii_hexdigit() {
                if *nptr != b'0' {
                    any_nonzero = true;
                }
                nptr = nptr.add(1);
            }
        } else {
            while (*nptr).is_ascii_digit() {
                if *nptr != b'0' {
                    any_nonzero = true;
                }
                nptr = nptr.add(1);
            }
        }
    }

    let mut before_exp = None;
    if hex {
        if *nptr == b'p' || *nptr == b'P' {
            before_exp = Some(nptr);
            nptr = nptr.add(1);
        }
    } else {
        if *nptr == b'e' || *nptr == b'E' {
            before_exp = Some(nptr);
            nptr = nptr.add(1);
        }
    }

    if let Some(before_exp) = before_exp {
        if *nptr == b'-' || *nptr == b'+' {
            nptr = nptr.add(1);
        }

        if (*nptr).is_ascii_digit() {
            while (*nptr).is_ascii_digit() {
                nptr = nptr.add(1);
            }
        } else {
            nptr = before_exp;
        }
    }

    (
        nptr,
        if hex {
            if before_exp.is_none() {
                todo!("strtod hexadecimal format with no `p`");
            }
            Format::Hexadecimal(any_nonzero)
        } else {
            Format::Decimal(any_nonzero)
        },
    )
}

enum Format {
    Decimal(bool),
    Hexadecimal(bool),
    Infinity,
    NaN(Option<u64>),
}
