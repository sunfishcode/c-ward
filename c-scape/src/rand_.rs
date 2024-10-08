use crate::convert_res;
use core::mem::MaybeUninit;
use core::slice;
#[cfg(not(target_env = "musl"))]
use errno::{set_errno, Errno};
use libc::c_void;

#[cfg(any(target_os = "android", target_os = "linux"))]
#[no_mangle]
unsafe extern "C" fn getrandom(ptr: *mut c_void, len: usize, flags: u32) -> isize {
    libc!(libc::getrandom(ptr, len, flags));

    if len == 0 {
        return 0;
    }

    let flags = rustix::rand::GetRandomFlags::from_bits_retain(flags);
    let buf = slice::from_raw_parts_mut(ptr.cast::<MaybeUninit<u8>>(), len);

    match convert_res(rustix::rand::getrandom_uninit(buf, flags)) {
        Some((init, _uninit)) => init.len() as isize,
        None => -1,
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[cfg(not(target_env = "musl"))]
#[no_mangle]
unsafe extern "C" fn getentropy(ptr: *mut c_void, len: usize) -> i32 {
    libc!(libc::getentropy(ptr, len));

    if len == 0 {
        return 0;
    }

    if len > 256 {
        set_errno(Errno(libc::EIO));
        return -1;
    }

    let flags = rustix::rand::GetRandomFlags::empty();
    let buf = slice::from_raw_parts_mut(ptr.cast::<MaybeUninit<u8>>(), len);

    let mut filled = 0usize;

    while filled < buf.len() {
        match rustix::rand::getrandom_uninit(&mut buf[filled..], flags) {
            Ok((init, _uninit)) => filled += init.len(),
            Err(rustix::io::Errno::INTR) => {}
            Err(err) => {
                set_errno(Errno(err.raw_os_error()));
                return -1;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(any(target_os = "android", target_os = "linux"))]
    #[test]
    fn test_getentropy() {
        unsafe {
            let mut buf = [0; 257];
            assert_eq!(getentropy(buf.as_mut_ptr().cast(), 257), -1);
            assert_eq!(errno::errno().0, libc::EIO);

            let mut buf = [0; 257];
            assert_eq!(getentropy(buf.as_mut_ptr().cast(), 256), 0);
            assert!(buf.iter().any(|b| *b != 0));
        }
    }
}
