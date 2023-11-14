//! Environment variable handling.

use core::ptr::null_mut;
use core::slice;
use libc::{c_char, c_int};
use rustix::ffi::CStr;

#[no_mangle]
unsafe extern "C" fn getenv(key: *const c_char) -> *mut c_char {
    libc!(libc::getenv(key));

    let key = CStr::from_ptr(key.cast());
    let key_bytes = key.to_bytes();

    _getenv(key_bytes)
}

#[no_mangle]
unsafe extern "C" fn secure_getenv(key: *const c_char) -> *mut c_char {
    //libc!(libc::secure_getenv(key));

    if rustix::runtime::linux_secure() {
        return null_mut();
    }

    let key = CStr::from_ptr(key.cast());
    let key_bytes = key.to_bytes();

    _getenv(key_bytes)
}

pub(crate) unsafe fn _getenv(key_bytes: &[u8]) -> *mut c_char {
    let mut ptr = super::set::load_environ();

    loop {
        let env = *ptr;
        if env.is_null() {
            break;
        }
        let mut c = env;
        while *c != (b'=' as c_char) {
            c = c.add(1);
        }
        if key_bytes
            == slice::from_raw_parts(env.cast::<u8>(), c.offset_from(env).try_into().unwrap())
        {
            return c.add(1);
        }
        ptr = ptr.add(1);
    }

    null_mut()
}

#[no_mangle]
unsafe extern "C" fn getlogin() -> *mut c_char {
    libc!(libc::getlogin());

    _getenv(b"LOGNAME")
}

/// GLIBC and origin pass argc, argv, and envp to functions in .init_array, as
/// a non-standard extension. Use priority 98 so that we run before any
/// normal user-defined constructor functions and our own functions which
/// depend on `getenv` working.
#[cfg(any(target_env = "gnu", feature = "take-charge"))]
#[link_section = ".init_array.00098"]
#[used]
static INIT_ARRAY: unsafe extern "C" fn(c_int, *mut *mut c_char, *mut *mut c_char) = {
    unsafe extern "C" fn function(_argc: c_int, _argv: *mut *mut c_char, envp: *mut *mut c_char) {
        super::set::init_from_envp(envp);
    }
    function
};

#[cfg(not(any(target_env = "gnu", feature = "take-charge")))]
static INIT_ARRAY: Unimplemented = Unimplemented::new();
