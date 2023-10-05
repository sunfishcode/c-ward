use core::ops::{MulAssign, Neg, SubAssign};
use libc::{c_char, c_int, c_long, c_longlong};

#[no_mangle]
unsafe extern "C" fn atoi(s: *const c_char) -> c_int {
    _atoi(s)
}

#[no_mangle]
unsafe extern "C" fn atol(s: *const c_char) -> c_long {
    _atoi(s)
}

#[no_mangle]
unsafe extern "C" fn atoll(s: *const c_char) -> c_longlong {
    _atoi(s)
}

unsafe fn _atoi<T: MulAssign + SubAssign + Neg<Output = T> + From<u8> + Default>(
    mut s: *const c_char,
) -> T {
    let mut negate = false;
    let mut n = T::default();

    // Skip leading whitespace.
    while libc::isspace((*s).into()) != 0 {
        s = s.add(1);
    }

    // Handle a sign.
    match *s as u8 {
        b'-' => {
            negate = true;
            s = s.add(1);
        }
        b'+' => {
            s = s.add(1);
        }
        _ => {}
    }

    // Handle digits.
    while libc::isdigit((*s).into()) != 0 {
        n *= T::from(10u8);
        n -= (*s as u8 - b'0').into();
        s = s.add(1);
    }

    if !negate {
        n = -n;
    }

    n
}
