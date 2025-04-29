//! Most of the nss functions are implement in c-gull rather than c-scape.
//! But we provide a `getpwuid_r` stub definition here in no-std mode because
//! it's referenced by libstd. libstd doesn't use it for anything other than
//! a fallback for when the HOME environment variable is unset, and HOME is
//! set in any reasonable use case where this would be called, so a stub
//! suffices.

#[cfg(not(feature = "std"))]
#[cfg(not(target_os = "wasi"))]
use libc::{c_char, c_int, passwd, uid_t};

#[cfg(not(feature = "std"))] // Avoid conflicting with c-gull's more complete `getpwuid_r`.
#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn getpwuid_r(
    _uid: uid_t,
    _pwd: *mut passwd,
    _buf: *mut c_char,
    _buflen: usize,
    _result: *mut *mut passwd,
) -> c_int {
    libc!(libc::getpwuid_r(_uid, _pwd, _buf, _buflen, _result));

    // `getpwuid_r` is currently implemented in c-gull.
    unimplemented!("getpwuid_r without the \"std\" feature")
}
