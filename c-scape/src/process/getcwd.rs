use alloc::vec::Vec;
use core::ptr::{copy_nonoverlapping, null_mut};
use errno::{set_errno, Errno};
use libc::c_char;

use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn getcwd(buf: *mut c_char, len: usize) -> *mut c_char {
    libc!(libc::getcwd(buf, len));

    if len == 0 && !buf.is_null() {
        set_errno(Errno(libc::EINVAL));
        return null_mut();
    }

    match convert_res(rustix::process::getcwd(Vec::new())) {
        Some(path) => {
            let path = path.as_bytes();

            let len = if len != 0 { len } else { path.len() + 1 };

            if path.len() + 1 <= len {
                let mut buf = buf;
                if buf.is_null() {
                    buf = libc::malloc(len).cast::<c_char>();
                    if buf.is_null() {
                        return buf;
                    }
                }
                copy_nonoverlapping(path.as_ptr().cast::<u8>(), buf.cast::<u8>(), path.len());
                *buf.add(path.len()) = 0;
                buf
            } else {
                set_errno(Errno(libc::ERANGE));
                null_mut()
            }
        }
        None => null_mut(),
    }
}
