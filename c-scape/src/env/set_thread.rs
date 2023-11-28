//! Thread-safe environment variable setting.
//!
//! This implementation achieves thread safety by having `setenv` and friends
//! always leak the old memory. Leaking isn't great, but we may assume that
//! most programs aren't calling `setenv` etc. very many times, and for any
//! that are, there are probably better ways to do what needs to be done
//! anyway. Leaking allows us to avoid pulling the rug out from underneath a
//! pointer returned by `getenv`.
//!
//! That said, this behavior is optional, and disabling the "threadsafe-setenv"
//! feature disabled this behavior and switches to the non-threadsafe but
//! also non-leaking implementation in set.rs.

use crate::{set_errno, Errno};
use alloc::vec;
use alloc::vec::Vec;
use core::ptr::null_mut;
use core::slice;
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;
use libc::{c_char, c_int};
use rustix::ffi::{CStr, CString};

// The global `environ` pointer. We declare this as an `AtomicPtr` rather than
// a `*mut T`; it has the same in-memory representation as a `*mut T`, and
// using `AtomicPtr` allows us to atomically update it.
#[no_mangle]
static environ: AtomicPtr<*mut c_char> = AtomicPtr::new(null_mut());

pub(crate) unsafe fn load_environ() -> *mut *mut c_char {
    environ.load(Ordering::Relaxed)
}

#[cfg(not(target_os = "wasi"))]
pub(super) fn init_from_envp(envp: *mut *mut c_char) {
    environ.store(envp, Ordering::Relaxed);
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
    let leaked = owned.into_raw();

    // Read the existing `environ` contents.
    let mut environ_vecs = EnvironVecs::read();

    // Search for the key.
    let start = environ_vecs.ptrs.as_mut_ptr();
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
                environ_vecs.ptrs[index] = leaked;

                environ_vecs.install();
            }

            return 0;
        }
        ptr = ptr.add(1);
    }

    // We didn't find the key; append it (preserving the terminating null).
    environ_vecs.ptrs.pop();
    environ_vecs.ptrs.push(leaked);
    environ_vecs.ptrs.push(null_mut());

    environ_vecs.install();

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

    // Read the existing `environ` contents.
    let mut environ_vecs = EnvironVecs::read();

    // Search for the key.
    let start = environ_vecs.ptrs.as_mut_ptr();
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
            break;
        }
        ptr = ptr.add(1);
    }

    environ_vecs.install();

    0
}

#[no_mangle]
unsafe extern "C" fn putenv(key_value: *mut c_char) -> c_int {
    libc!(libc::putenv(key_value));

    let key_value_cstr = CStr::from_ptr(key_value);
    let key_value_bytes = key_value_cstr.to_bytes();

    let eq = key_value_bytes.iter().position(|x| *x == b'=').unwrap();
    let key_bytes = &key_value_bytes[..eq];

    // Read the existing `environ` contents.
    let mut environ_vecs = EnvironVecs::read();

    // Search for the key.
    let start = environ_vecs.ptrs.as_mut_ptr();
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

            environ_vecs.install();

            return 0;
        }
        ptr = ptr.add(1);
    }

    // We didn't find the key; append it (preserving the terminating null).
    environ_vecs.ptrs.pop();
    environ_vecs.ptrs.push(key_value);
    environ_vecs.ptrs.push(null_mut());

    environ_vecs.install();

    0
}

#[no_mangle]
unsafe extern "C" fn clearenv() -> c_int {
    libc!(libc::clearenv());

    EnvironVecs {
        ptrs: vec![null_mut()],
    }
    .install();
    0
}

struct EnvironVecs {
    ptrs: Vec<*mut c_char>,
}

impl EnvironVecs {
    unsafe fn read() -> Self {
        let mut ptrs = Vec::new();
        let mut ptr = load_environ();
        loop {
            let env = *ptr;
            ptrs.push(env);
            if env.is_null() {
                break;
            }
            ptr = ptr.add(1);
        }

        Self { ptrs }
    }

    // Install this `EnvironVecs` data as the new global `environ`.
    fn install(mut self) {
        // Leak the memory, so that future `setenv`/`unsetenv` calls don't
        // invalidate it, because `getenv` calls return raw pointers into it.
        self.ptrs.shrink_to_fit();
        let leaked: &'static mut [*mut c_char] = self.ptrs.leak();

        // Update the global `environ` pointer.
        environ.store(leaked.as_mut_ptr(), Ordering::Relaxed);
    }
}
