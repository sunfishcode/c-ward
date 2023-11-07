use crate::convert_res;
use core::ptr::{addr_of_mut, copy_nonoverlapping};
use core::slice;
use errno::{set_errno, Errno};
use libc::{c_char, c_double, c_int};
use memoffset::span_of;

#[no_mangle]
unsafe extern "C" fn uname(buf: *mut libc::utsname) -> c_int {
    libc!(libc::uname(buf));

    let uname = rustix::system::uname();

    let sysname = uname.sysname().to_bytes_with_nul();
    assert!(sysname.len() <= span_of!(libc::utsname, sysname).len());
    copy_nonoverlapping(
        sysname.as_ptr(),
        addr_of_mut!((*buf).sysname).cast::<u8>(),
        sysname.len(),
    );

    let nodename = uname.nodename().to_bytes_with_nul();
    assert!(nodename.len() <= span_of!(libc::utsname, nodename).len());
    copy_nonoverlapping(
        nodename.as_ptr(),
        addr_of_mut!((*buf).nodename).cast::<u8>(),
        nodename.len(),
    );

    let release = uname.release().to_bytes_with_nul();
    assert!(release.len() <= span_of!(libc::utsname, release).len());
    copy_nonoverlapping(
        release.as_ptr(),
        addr_of_mut!((*buf).release).cast::<u8>(),
        release.len(),
    );

    let version = uname.version().to_bytes_with_nul();
    assert!(version.len() <= span_of!(libc::utsname, version).len());
    copy_nonoverlapping(
        version.as_ptr(),
        addr_of_mut!((*buf).version).cast::<u8>(),
        version.len(),
    );

    let machine = uname.machine().to_bytes_with_nul();
    assert!(machine.len() <= span_of!(libc::utsname, machine).len());
    copy_nonoverlapping(
        machine.as_ptr(),
        addr_of_mut!((*buf).machine).cast::<u8>(),
        machine.len(),
    );

    let domainname = uname.domainname().to_bytes_with_nul();
    assert!(domainname.len() <= span_of!(libc::utsname, domainname).len());
    copy_nonoverlapping(
        domainname.as_ptr(),
        addr_of_mut!((*buf).domainname).cast::<u8>(),
        domainname.len(),
    );

    0
}

#[no_mangle]
unsafe extern "C" fn getloadavg(a: *mut c_double, n: c_int) -> c_int {
    libc!(libc::getloadavg(a, n));

    if n == 0 {
        return 0;
    }
    if n < 0 {
        return -1;
    }

    let info = rustix::system::sysinfo();
    let mut a = a;

    for i in 0..core::cmp::min(n as usize, info.loads.len()) {
        a.write((info.loads[i] as f64) / ((1 << libc::SI_LOAD_SHIFT) as f64));
        a = a.add(1);
    }

    n
}

#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn gethostname(name: *mut c_char, len: usize) -> c_int {
    let uname = rustix::system::uname();
    let nodename = uname.nodename();
    if nodename.to_bytes().len() + 1 > len {
        set_errno(Errno(libc::ENAMETOOLONG));
        return -1;
    }
    copy_nonoverlapping(
        nodename.to_bytes().as_ptr().cast::<u8>(),
        name.cast::<u8>(),
        nodename.to_bytes().len(),
    );
    *name.add(nodename.to_bytes().len()) = 0;
    0
}

#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn sethostname(name: *mut c_char, len: usize) -> c_int {
    let slice = slice::from_raw_parts(name.cast(), len);
    match convert_res(rustix::system::sethostname(slice)) {
        Some(()) => 0,
        None => -1,
    }
}
