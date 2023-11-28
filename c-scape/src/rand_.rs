use crate::{convert_res, READ_BUFFER};
use core::cmp::min;
use errno::{set_errno, Errno};
use libc::c_void;

#[cfg(any(target_os = "android", target_os = "linux"))]
#[no_mangle]
unsafe extern "C" fn getrandom(buf: *mut c_void, buflen: usize, flags: u32) -> isize {
    libc!(libc::getrandom(buf, buflen, flags));

    if buflen == 0 {
        return 0;
    }

    let flags = rustix::rand::GetRandomFlags::from_bits_retain(flags);

    // `slice::from_raw_parts_mut` assumes that the memory is initialized,
    // which our C API here doesn't guarantee. Since rustix currently requires
    // a slice, use a temporary copy.
    match convert_res(rustix::rand::getrandom(
        &mut READ_BUFFER[..min(buflen, READ_BUFFER.len())],
        flags,
    )) {
        Some(num) => {
            core::ptr::copy_nonoverlapping(READ_BUFFER.as_ptr(), buf.cast::<u8>(), buflen);
            num as isize
        }
        None => -1,
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[no_mangle]
unsafe extern "C" fn getentropy(buf: *mut c_void, buflen: usize) -> i32 {
    libc!(libc::getentropy(buf, buflen));

    if buflen == 0 {
        return 0;
    }

    if buflen >= 256 {
        set_errno(Errno(libc::EIO));
        return -1;
    }
    assert!(buflen < READ_BUFFER.len());

    let flags = rustix::rand::GetRandomFlags::empty();

    let mut filled = 0usize;

    // `slice::from_raw_parts_mut` assumes that the memory is initialized,
    // which our C API here doesn't guarantee. Since rustix currently requires
    // a slice, use a temporary copy.
    while filled < buflen {
        match rustix::rand::getrandom(&mut READ_BUFFER[filled..buflen], flags) {
            Ok(num) => filled += num,
            Err(rustix::io::Errno::INTR) => {}
            Err(err) => {
                set_errno(Errno(err.raw_os_error()));
                return -1;
            }
        }
    }

    core::ptr::copy_nonoverlapping(READ_BUFFER.as_ptr(), buf.cast::<u8>(), buflen);

    0
}
