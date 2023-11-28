//! Environment variable setting.
//!
//! This implementation is not thread-safe.
//!
//! Enable the "threadsafe-setenv" feature to use the thread-safe
//! implementation in set_thread.rs instead.

use crate::{set_errno, Errno};
use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use core::cell::SyncUnsafeCell;
use core::ptr::null_mut;
use core::slice;
use libc::{c_char, c_int};
use rustix::ffi::{CStr, CString};

// The global `environ` pointer.
#[no_mangle]
static mut environ: *mut *mut c_char = null_mut();

pub(crate) unsafe fn load_environ() -> *mut *mut c_char {
    environ
}

#[cfg(not(target_os = "wasi"))]
pub(super) unsafe fn init_from_envp(envp: *mut *mut c_char) {
    environ = envp;
}

/// If we haven't read `environ` yet, or if it changed out from underneath
/// us, read `environ`.
unsafe fn sync_environ(environ_vecs: &mut EnvironVecs) {
    let mut ptr = load_environ();

    if environ_vecs.ptrs.is_empty() || environ_vecs.ptrs.as_ptr() != ptr {
        let mut vecs = EnvironVecs::new();

        loop {
            let env = *ptr;
            if env.is_null() {
                break;
            }
            let owned = CStr::from_ptr(env).to_owned();
            vecs.ptrs.push(owned.as_ptr().cast_mut());
            vecs.allocs.push(owned);
            ptr = ptr.add(1);
        }

        vecs.ptrs.push(null_mut());

        *environ_vecs = vecs;
        environ = environ_vecs.ptrs.as_mut_ptr();
    }
}

#[no_mangle]
unsafe extern "C" fn setenv(key: *const c_char, value: *const c_char, overwrite: c_int) -> c_int {
    libc!(libc::setenv(key, value, overwrite));

    if key.is_null() || value.is_null() {
        set_errno(Errno(libc::EINVAL));
        return -1;
    }

    let key = CStr::from_ptr(key);
    let key_bytes = key.to_bytes();

    let value = CStr::from_ptr(value);
    let value_bytes = value.to_bytes();

    if key_bytes.is_empty() || key_bytes.contains(&b'=') {
        set_errno(Errno(libc::EINVAL));
        return -1;
    }

    // Construct the new "key=value" string.
    let mut owned = Vec::new();
    owned.extend_from_slice(key_bytes);
    owned.extend_from_slice(b"=");
    owned.extend_from_slice(value_bytes);
    let owned = CString::new(owned).unwrap();

    let environ_vecs = ENVIRON_VECS.get_mut();
    sync_environ(environ_vecs);

    // Search for the key.
    let start = load_environ();
    let mut ptr = start;
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
            // We found it.
            if overwrite != 0 {
                let index = ptr.offset_from(start) as usize;
                environ_vecs.ptrs[index] = owned.as_ptr().cast_mut();
                environ_vecs.allocs[index] = owned;
            }
            return 0;
        }
        ptr = ptr.add(1);
    }

    // We didn't find the key; append it.
    environ_vecs.ptrs.pop();
    environ_vecs.ptrs.push(owned.as_ptr().cast_mut());
    environ_vecs.ptrs.push(null_mut());
    environ_vecs.allocs.push(owned);

    environ = environ_vecs.ptrs.as_mut_ptr();

    0
}

#[no_mangle]
unsafe extern "C" fn unsetenv(key: *const c_char) -> c_int {
    libc!(libc::unsetenv(key));

    if key.is_null() {
        set_errno(Errno(libc::EINVAL));
        return -1;
    }

    let key = CStr::from_ptr(key);
    let key_bytes = key.to_bytes();

    if key_bytes.is_empty() || key_bytes.contains(&b'=') {
        set_errno(Errno(libc::EINVAL));
        return -1;
    }

    let environ_vecs = ENVIRON_VECS.get_mut();
    sync_environ(environ_vecs);

    // Search for the key.
    let start = load_environ();
    let mut ptr = start;
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
            // We found it.
            let index = ptr.offset_from(start) as usize;
            environ_vecs.ptrs.pop();
            environ_vecs.ptrs.swap_remove(index);
            environ_vecs.ptrs.push(null_mut());
            environ_vecs.allocs.swap_remove(index);
            environ = environ_vecs.ptrs.as_mut_ptr();
            break;
        }
        ptr = ptr.add(1);
    }

    0
}

#[no_mangle]
unsafe extern "C" fn putenv(key_value: *mut c_char) -> c_int {
    libc!(libc::putenv(key_value));

    let key_value_cstr = CStr::from_ptr(key_value);
    let key_value_bytes = key_value_cstr.to_bytes();

    let eq = key_value_bytes.iter().position(|x| *x == b'=').unwrap();
    let key_bytes = &key_value_bytes[..eq];

    let environ_vecs = ENVIRON_VECS.get_mut();
    sync_environ(environ_vecs);

    // Search for the key.
    let start = load_environ();
    let mut ptr = start;
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
            // We found it.
            let index = ptr.offset_from(start) as usize;
            environ_vecs.ptrs[index] = key_value;
            return 0;
        }
        ptr = ptr.add(1);
    }

    // We didn't find the key; append it.
    environ_vecs.ptrs.pop();
    environ_vecs.ptrs.push(key_value);
    environ_vecs.ptrs.push(null_mut());

    environ = environ_vecs.ptrs.as_mut_ptr();

    0
}

struct EnvironVecs {
    ptrs: Vec<*mut c_char>,
    allocs: Vec<CString>,
}

impl EnvironVecs {
    const fn new() -> Self {
        Self {
            ptrs: Vec::new(),
            allocs: Vec::new(),
        }
    }
}

static mut ENVIRON_VECS: SyncUnsafeCell<EnvironVecs> = SyncUnsafeCell::new(EnvironVecs::new());
