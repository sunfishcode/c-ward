//! Minimal implementation of locales.
//!
//! This currently only supports the C/POSIX locale.

use core::ptr::{addr_of, null_mut};
use libc::{c_char, c_int, lconv};

static EMPTY_STR: [c_char; 1] = [b'\0' as _];
static C_STR: [c_char; 2] = [b'C' as _, b'\0' as _];
static POSIX_STR: [c_char; 6] = [
    b'P' as _, b'O' as _, b'S' as _, b'I' as _, b'X' as _, b'\0' as _,
];
static DOT_STR: [c_char; 2] = [b'.' as _, b'\0' as _];

struct SyncLconv(lconv);

// SAFETY: The `lconv` instance below is not mutable.
unsafe impl Sync for SyncLconv {}

static THE_C_GLOBAL_LOCALE: SyncLconv = SyncLconv(lconv {
    currency_symbol: EMPTY_STR.as_ptr().cast_mut(),
    decimal_point: DOT_STR.as_ptr().cast_mut(),
    frac_digits: c_char::MAX,
    grouping: EMPTY_STR.as_ptr().cast_mut(),
    int_curr_symbol: EMPTY_STR.as_ptr().cast_mut(),
    int_frac_digits: c_char::MAX,
    int_n_cs_precedes: c_char::MAX,
    int_n_sep_by_space: c_char::MAX,
    int_n_sign_posn: c_char::MAX,
    int_p_cs_precedes: c_char::MAX,
    int_p_sep_by_space: c_char::MAX,
    int_p_sign_posn: c_char::MAX,
    mon_decimal_point: EMPTY_STR.as_ptr().cast_mut(),
    mon_grouping: EMPTY_STR.as_ptr().cast_mut(),
    mon_thousands_sep: EMPTY_STR.as_ptr().cast_mut(),
    negative_sign: EMPTY_STR.as_ptr().cast_mut(),
    n_cs_precedes: c_char::MAX,
    n_sep_by_space: c_char::MAX,
    n_sign_posn: c_char::MAX,
    positive_sign: EMPTY_STR.as_ptr().cast_mut(),
    p_cs_precedes: c_char::MAX,
    p_sep_by_space: c_char::MAX,
    p_sign_posn: c_char::MAX,
    thousands_sep: EMPTY_STR.as_ptr().cast_mut(),
});

#[no_mangle]
unsafe extern "C" fn setlocale(_category: c_int, locale: *const c_char) -> *mut c_char {
    libc!(libc::setlocale(_category, locale));

    if locale.is_null()
        || libc::strcmp(locale, C_STR.as_ptr()) == 0
        || libc::strcmp(locale, POSIX_STR.as_ptr()) == 0
    {
        return C_STR.as_ptr().cast_mut();
    }

    null_mut()
}

#[no_mangle]
unsafe extern "C" fn localeconv() -> *mut lconv {
    libc!(libc::localeconv());

    addr_of!(THE_C_GLOBAL_LOCALE.0).cast_mut()
}

#[no_mangle]
unsafe extern "C" fn strcoll(l: *const c_char, r: *const c_char) -> c_int {
    libc!(libc::strcoll(l, r));

    libc::strcmp(l, r)
}
