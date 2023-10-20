extern crate alloc;

use alloc::vec::IntoIter;
use core::ffi::CStr;
use core::mem::zeroed;
use core::ptr::null_mut;
use core::str;
use core::str::FromStr;
use std::process::Command;

use errno::{set_errno, Errno};
use libc::{c_char, c_int};
use rustix::cstr;
use rustix::net::{
    IpAddr, SocketAddrAny, SocketAddrStorage, SocketAddrV4, SocketAddrV6, SocketType,
};

#[no_mangle]
unsafe extern "C" fn getaddrinfo(
    node: *const c_char,
    service: *const c_char,
    hints: *const libc::addrinfo,
    res: *mut *mut libc::addrinfo,
) -> c_int {
    libc!(libc::getaddrinfo(node, service, hints, res));

    assert!(service.is_null(), "service lookups not supported yet");
    assert!(!node.is_null(), "only name lookups are supported corrently");

    let mut socktype = None;
    if !hints.is_null() {
        let hints = &*hints;
        assert_eq!(hints.ai_flags, 0, "GAI flags hint not supported yet");
        assert_eq!(hints.ai_family, 0, "GAI family hint not supported yet");
        assert_eq!(
            hints.ai_socktype,
            SocketType::STREAM.as_raw() as _,
            "only SOCK_STREAM supported currently"
        );
        assert_eq!(hints.ai_protocol, 0, "GAI protocol hint not supported yet");
        assert_eq!(hints.ai_addrlen, 0, "GAI addrlen hint not supported yet");
        assert!(hints.ai_addr.is_null(), "GAI addr hint not supported yet");
        assert!(
            hints.ai_canonname.is_null(),
            "GAI canonname hint not supported yet"
        );
        assert!(hints.ai_next.is_null(), "GAI next hint not supported yet");
        socktype = Some(hints.ai_socktype);
    }

    let host = match CStr::from_ptr(node.cast()).to_str() {
        Ok(host) => host,
        Err(_) => {
            set_errno(Errno(libc::EILSEQ));
            return libc::EAI_SYSTEM;
        }
    };

    let layout = alloc::alloc::Layout::new::<libc::addrinfo>();
    let addr_layout = alloc::alloc::Layout::new::<SocketAddrStorage>();
    let mut first: *mut libc::addrinfo = null_mut();
    let mut prev: *mut libc::addrinfo = null_mut();
    match resolve(host, socktype) {
        Ok(addrs) => {
            for addr in addrs {
                let ptr = alloc::alloc::alloc(layout).cast::<libc::addrinfo>();
                ptr.write(zeroed());
                let info = &mut *ptr;
                match addr {
                    IpAddr::V4(v4) => {
                        // TODO: Create and write to `SocketAddrV4Storage`?
                        let storage = alloc::alloc::alloc(addr_layout).cast::<SocketAddrStorage>();
                        let len = SocketAddrAny::V4(SocketAddrV4::new(v4, 0)).write(storage);
                        info.ai_addr = storage.cast();
                        info.ai_addrlen = len.try_into().unwrap();
                    }
                    IpAddr::V6(v6) => {
                        // TODO: Create and write to `SocketAddrV6Storage`?
                        let storage = alloc::alloc::alloc(addr_layout).cast::<SocketAddrStorage>();
                        let len = SocketAddrAny::V6(SocketAddrV6::new(v6, 0, 0, 0)).write(storage);
                        info.ai_addr = storage.cast();
                        info.ai_addrlen = len.try_into().unwrap();
                    }
                }
                if !prev.is_null() {
                    (*prev).ai_next = ptr;
                }
                prev = ptr;
                if first.is_null() {
                    first = ptr;
                }
            }
            *res = first;
            0
        }
        Err(err) => return err,
    }
}

fn resolve(host: &str, socktype: Option<c_int>) -> Result<IntoIter<IpAddr>, c_int> {
    let mut command = Command::new("getent");
    command.arg("ahosts").arg(host);

    let output = match command.output() {
        Ok(output) => output,
        Err(_err) => {
            set_errno(Errno(libc::EIO));
            return Err(libc::EAI_SYSTEM);
        }
    };

    let mut hosts = Vec::new();

    match output.status.code() {
        Some(0) => {}
        Some(2) => {
            // The hostname was not found.
            return Err(libc::EAI_NONAME);
        }
        Some(r) => panic!("unexpected exit status from `getent ahosts`: {}", r),
        None => {
            set_errno(Errno(libc::EIO));
            return Err(libc::EAI_SYSTEM);
        }
    }

    let stdout = match str::from_utf8(&output.stdout) {
        Ok(stdout) => stdout,
        Err(_err) => {
            set_errno(Errno(libc::EIO));
            return Err(libc::EAI_SYSTEM);
        }
    };

    // Iterate over the lines printed by `getent`.
    for line in stdout.lines() {
        // Parse the line.
        let mut parts = line.split_ascii_whitespace();
        let addr = match parts.next() {
            Some(addr) => addr,
            None => {
                set_errno(Errno(libc::EIO));
                return Err(libc::EAI_SYSTEM);
            }
        };
        let type_ = match parts.next() {
            Some(type_) => type_,
            None => {
                set_errno(Errno(libc::EIO));
                return Err(libc::EAI_SYSTEM);
            }
        };
        // Ignore any futher parts, which would contain aliases for `host`
        // that we're uninterested in here.

        // Filter out results that don't match what's requested.
        if let Some(socktype) = socktype {
            let socktype_name = match socktype {
                libc::SOCK_STREAM => "STREAM",
                libc::SOCK_DGRAM => "DGRAM",
                libc::SOCK_RAW => "RAW",
                _ => panic!("unsupported socktype {:?}", socktype),
            };
            if type_ != socktype_name {
                continue;
            }
        }

        // Parse the address.
        let addr = match IpAddr::from_str(addr) {
            Ok(addr) => addr,
            Err(_err) => {
                set_errno(Errno(libc::EIO));
                return Err(libc::EAI_SYSTEM);
            }
        };

        hosts.push(addr);
    }

    Ok(hosts.into_iter())
}

#[no_mangle]
unsafe extern "C" fn freeaddrinfo(mut res: *mut libc::addrinfo) {
    libc!(libc::freeaddrinfo(res));

    let layout = alloc::alloc::Layout::new::<libc::addrinfo>();
    let addr_layout = alloc::alloc::Layout::new::<SocketAddrStorage>();

    while !res.is_null() {
        let addr = (*res).ai_addr;
        if !addr.is_null() {
            alloc::alloc::dealloc(addr.cast(), addr_layout);
        }
        let old = res;
        res = (*res).ai_next;
        alloc::alloc::dealloc(old.cast(), layout);
    }
}

#[no_mangle]
unsafe extern "C" fn gai_strerror(errcode: c_int) -> *const c_char {
    libc!(libc::gai_strerror(errcode));

    match errcode {
        libc::EAI_NONAME => cstr!("Name does not resolve"),
        libc::EAI_SYSTEM => cstr!("System error"),
        _ => panic!("unrecognized gai_strerror {:?}", errcode),
    }
    .as_ptr()
}

#[no_mangle]
unsafe extern "C" fn __res_init() -> c_int {
    libc!(libc::res_init());
    0
}
