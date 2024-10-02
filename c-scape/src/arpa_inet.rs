use alloc::string::ToString;
use core::cell::SyncUnsafeCell;
use core::ffi::CStr;
use core::net::{Ipv4Addr, Ipv6Addr};
use core::ptr::{addr_of, copy_nonoverlapping, null};
use errno::{set_errno, Errno};
use libc::{c_char, c_int, c_void, in_addr, in_addr_t, socklen_t};

#[no_mangle]
unsafe extern "C" fn inet_aton(cp: *const c_char, inp: *mut in_addr) -> c_int {
    //libc!(libc::inet_aton(cp, inp));

    inet_pton(libc::AF_INET, cp, inp.cast::<c_void>())
}

#[no_mangle]
unsafe extern "C" fn inet_ntoa(in_: in_addr) -> *const c_char {
    //libc!(libc::inet_ntoa(in_));

    static BUFFER: SyncUnsafeCell<[c_char; 16]> = SyncUnsafeCell::new([0; 16]);
    let buffer = &mut *BUFFER.get();

    inet_ntop(
        libc::AF_INET,
        addr_of!(in_).cast::<c_void>(),
        buffer.as_mut_ptr(),
        buffer.len() as _,
    )
}

#[no_mangle]
unsafe extern "C" fn inet_pton(domain: c_int, src: *const c_char, dest: *mut c_void) -> c_int {
    //libc!(libc::inet_pton(in_));

    let src = match CStr::from_ptr(src).to_str() {
        Err(_) => return 0,
        Ok(src) => src,
    };
    match domain {
        libc::AF_INET => {
            let addr: Ipv4Addr = match src.parse() {
                Ok(addr) => addr,
                Err(_) => return 0,
            };
            let octets = addr.octets();
            copy_nonoverlapping(octets.as_ptr(), dest.cast::<u8>(), octets.len());
            1
        }
        libc::AF_INET6 => {
            let addr: Ipv6Addr = match src.parse() {
                Ok(addr) => addr,
                Err(_) => return 0,
            };
            let octets = addr.octets();
            copy_nonoverlapping(octets.as_ptr(), dest.cast::<u8>(), octets.len());
            1
        }
        _ => {
            set_errno(Errno(libc::EAFNOSUPPORT));
            -1
        }
    }
}

#[no_mangle]
unsafe extern "C" fn inet_ntop(
    domain: c_int,
    src: *const c_void,
    dest: *mut c_char,
    size: socklen_t,
) -> *const c_char {
    match domain {
        libc::AF_INET => {
            let addr = Ipv4Addr::from(*src.cast::<[u8; 4]>());
            let s = addr.to_string();
            if s.len() >= size as usize {
                set_errno(Errno(libc::ENOSPC));
                return null();
            }
            copy_nonoverlapping(s.as_ptr(), dest.cast::<u8>(), s.len());
            *dest.cast::<u8>().add(s.len()) = b'\0';
            dest
        }
        libc::AF_INET6 => {
            let addr = Ipv6Addr::from(*src.cast::<[u8; 16]>());
            let s = addr.to_string();
            if s.len() >= size as usize {
                set_errno(Errno(libc::ENOSPC));
                return null();
            }
            copy_nonoverlapping(s.as_ptr(), dest.cast::<u8>(), s.len());
            *dest.cast::<u8>().add(s.len()) = b'\0';
            dest
        }
        _ => {
            set_errno(Errno(libc::EAFNOSUPPORT));
            null()
        }
    }
}

#[no_mangle]
unsafe extern "C" fn inet_addr(cp: *const c_char) -> in_addr_t {
    let mut val: in_addr = in_addr { s_addr: 0 };

    if inet_aton(cp, &mut val) > 0 {
        val.s_addr
    } else {
        libc::INADDR_NONE
    }
}

#[no_mangle]
unsafe extern "C" fn inet_network(cp: *mut c_char) -> in_addr_t {
    u32::from_be(inet_addr(cp))
}

#[no_mangle]
pub extern "C" fn inet_lnaof(input: in_addr) -> in_addr_t {
    if input.s_addr >> 24 < 128 {
        input.s_addr & 0xff_ffff
    } else if input.s_addr >> 24 < 192 {
        input.s_addr & 0xffff
    } else {
        input.s_addr & 0xff
    }
}

#[no_mangle]
pub extern "C" fn inet_makeaddr(net: in_addr_t, host: in_addr_t) -> in_addr {
    let mut output: in_addr = in_addr { s_addr: 0 };

    if net < 256 {
        output.s_addr = host | net << 24;
    } else if net < 65536 {
        output.s_addr = host | net << 16;
    } else {
        output.s_addr = host | net << 8;
    }

    output
}

#[no_mangle]
pub extern "C" fn inet_netof(input: in_addr) -> in_addr_t {
    if input.s_addr >> 24 < 128 {
        input.s_addr & 0xff_ffff
    } else if input.s_addr >> 24 < 192 {
        input.s_addr & 0xffff
    } else {
        input.s_addr & 0xff
    }
}
