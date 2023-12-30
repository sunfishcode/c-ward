use libc::{c_char, c_float, c_double};
use core::ptr::null_mut;

#[cfg(feature = "todo")]
#[no_mangle]
unsafe extern "C" fn atof(nptr: *const c_char) -> c_double {
    libc!(libc::atof(nptr));

    strtod(nptr, null_mut())
}

#[cfg(feature = "todo")]
#[no_mangle]
unsafe extern "C" fn strtof(nptr: *const c_char, endptr: *mut *mut c_char) -> c_float {
    libc!(libc::strtof(nptr, endptr));

    todo!("strtof")
}

#[cfg(feature = "todo")]
#[no_mangle]
unsafe extern "C" fn strtod(nptr: *const c_char, endptr: *mut *mut c_char) -> c_double {
    libc!(libc::strtod(nptr, endptr));

    todo!("strtod")
}
