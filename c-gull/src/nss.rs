//! Name Service Switch functions.
//!
//! In order to avoid implementing `dlopen`, while still correctly implementing
//! fully /etc/nsswitch.conf-respecting NSS functionality, we invoke the
//! `getent` command and parse its output.
//!
//! This file doesn't yet implement enumeration, but the `getent` command does,
//! so it's theoretically doable.

use core::cell::SyncUnsafeCell;
use core::ffi::CStr;
use core::mem::{align_of, zeroed};
use core::ptr::{addr_of_mut, copy_nonoverlapping, null, null_mut, write};
use core::str;
use core::str::FromStr;
use errno::{set_errno, Errno};
use libc::{c_char, c_int, c_void, gid_t, group, passwd, size_t, uid_t};
use rustix::path::DecInt;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::process::Command;

#[no_mangle]
unsafe extern "C" fn getpwnam_r(
    name: *const c_char,
    pwd: *mut passwd,
    buf: *mut c_char,
    buflen: usize,
    result: *mut *mut passwd,
) -> c_int {
    libc!(libc::getpwnam_r(name, pwd, buf, buflen, result));

    let name = OsStr::from_bytes(CStr::from_ptr(name).to_bytes());
    let mut command = Command::new("getent");
    command.arg("passwd").arg(name);
    getpw_r(command, pwd, buf, buflen, result)
}

#[no_mangle]
unsafe extern "C" fn getpwuid_r(
    uid: uid_t,
    pwd: *mut passwd,
    buf: *mut c_char,
    buflen: usize,
    result: *mut *mut passwd,
) -> c_int {
    libc!(libc::getpwuid_r(uid, pwd, buf, buflen, result));

    let dec_int = DecInt::new(uid);
    let name = OsStr::from_bytes(dec_int.as_bytes());
    let mut command = Command::new("getent");
    command.arg("passwd").arg(name);
    getpw_r(command, pwd, buf, buflen, result)
}

#[no_mangle]
unsafe extern "C" fn getgrnam_r(
    name: *const c_char,
    grp: *mut group,
    buf: *mut c_char,
    buflen: usize,
    result: *mut *mut group,
) -> c_int {
    libc!(libc::getgrnam_r(name, grp, buf, buflen, result));

    let name = OsStr::from_bytes(CStr::from_ptr(name).to_bytes());
    let mut command = Command::new("getent");
    command.arg("group").arg(name);
    getgr_r(command, grp, buf, buflen, result)
}

#[no_mangle]
unsafe extern "C" fn getgrgid_r(
    gid: gid_t,
    grp: *mut group,
    buf: *mut c_char,
    buflen: usize,
    result: *mut *mut group,
) -> c_int {
    libc!(libc::getgrgid_r(gid, grp, buf, buflen, result));

    let dec_int = DecInt::new(gid);
    let name = OsStr::from_bytes(dec_int.as_bytes());
    let mut command = Command::new("getent");
    command.arg("group").arg(name);
    getgr_r(command, grp, buf, buflen, result)
}

unsafe fn getpw_r(
    command: Command,
    pwd: *mut passwd,
    buf: *mut c_char,
    buflen: usize,
    result: *mut *mut passwd,
) -> c_int {
    let mut command = command;
    let output = match command.output() {
        Ok(output) => output,
        Err(_err) => return parse_error(result.cast()),
    };

    match output.status.code() {
        Some(0) => {}
        Some(2) => return success(result.cast(), null_mut()),
        Some(r) => panic!("unexpected exit status from `getent passwd`: {}", r),
        None => return parse_error(result.cast()),
    }

    let stdout = match str::from_utf8(&output.stdout) {
        Ok(stdout) => stdout,
        Err(_err) => return parse_error(result.cast()),
    };
    let stdout = match stdout.strip_suffix('\n') {
        Some(stdout) => stdout,
        None => return parse_error(result.cast()),
    };

    let mut parts = stdout.split(':');
    let mut buf = buf;
    let mut buflen = buflen;

    let part = match parts.next() {
        Some(part) => part,
        None => return parse_error(result.cast()),
    };
    if part.len() > buflen {
        return buffer_exhausted(result.cast());
    }
    let pw_name = buf;
    copy_nonoverlapping(part.as_ptr(), buf.cast(), part.len());
    buf = buf.add(part.len());
    write(buf, 0);
    buf = buf.add(1);
    buflen -= part.len() + 1;

    let part = match parts.next() {
        Some(part) => part,
        None => return parse_error(result.cast()),
    };
    if part.len() > buflen {
        return buffer_exhausted(result.cast());
    }
    let pw_passwd = buf;
    copy_nonoverlapping(part.as_ptr(), buf.cast(), part.len());
    buf = buf.add(part.len());
    write(buf, 0);
    buf = buf.add(1);
    buflen -= part.len() + 1;

    let part = match parts.next() {
        Some(part) => part,
        None => return parse_error(result.cast()),
    };
    let pw_uid = match part.parse() {
        Ok(pw_uid) => pw_uid,
        Err(_err) => return parse_error(result.cast()),
    };

    let part = match parts.next() {
        Some(part) => part,
        None => return parse_error(result.cast()),
    };
    let pw_gid = match part.parse() {
        Ok(pw_gid) => pw_gid,
        Err(_err) => return parse_error(result.cast()),
    };

    let part = match parts.next() {
        Some(part) => part,
        None => return parse_error(result.cast()),
    };
    if part.len() > buflen {
        return buffer_exhausted(result.cast());
    }
    let pw_gecos = buf;
    copy_nonoverlapping(part.as_ptr(), buf.cast(), part.len());
    buf = buf.add(part.len());
    write(buf, 0);
    buf = buf.add(1);
    buflen -= part.len() + 1;

    let part = match parts.next() {
        Some(part) => part,
        None => return parse_error(result.cast()),
    };
    if part.len() > buflen {
        return buffer_exhausted(result.cast());
    }
    let pw_dir = buf;
    copy_nonoverlapping(part.as_ptr(), buf.cast(), part.len());
    buf = buf.add(part.len());
    write(buf, 0);
    buf = buf.add(1);
    buflen -= part.len() + 1;

    let part = match parts.next() {
        Some(part) => part,
        None => return parse_error(result.cast()),
    };
    if part.len() > buflen {
        return buffer_exhausted(result.cast());
    }
    let pw_shell = buf;
    copy_nonoverlapping(part.as_ptr(), buf.cast(), part.len());
    buf = buf.add(part.len());
    write(buf, 0);

    write(
        pwd,
        passwd {
            pw_name,
            pw_passwd,
            pw_uid,
            pw_gid,
            pw_gecos,
            pw_dir,
            pw_shell,
        },
    );
    success(result.cast(), pwd.cast())
}

unsafe fn getgr_r(
    command: Command,
    grp: *mut group,
    buf: *mut c_char,
    buflen: usize,
    result: *mut *mut group,
) -> c_int {
    let mut command = command;
    let output = match command.output() {
        Ok(output) => output,
        Err(_err) => return parse_error(result.cast()),
    };

    match output.status.code() {
        Some(0) => {}
        Some(2) => return success(result.cast(), null_mut()),
        Some(r) => panic!("unexpected exit status from `getent group`: {}", r),
        None => return parse_error(result.cast()),
    }

    let stdout = match str::from_utf8(&output.stdout) {
        Ok(stdout) => stdout,
        Err(_err) => return parse_error(result.cast()),
    };
    let stdout = match stdout.strip_suffix('\n') {
        Some(stdout) => stdout,
        None => return parse_error(result.cast()),
    };

    let mut parts = stdout.split(':');
    let mut buf = buf;
    let mut buflen = buflen;

    let part = match parts.next() {
        Some(part) => part,
        None => return parse_error(result.cast()),
    };
    if part.len() > buflen {
        return buffer_exhausted(result.cast());
    }
    let gr_name = buf;
    copy_nonoverlapping(part.as_ptr(), buf.cast(), part.len());
    buf = buf.add(part.len());
    write(buf, 0);
    buf = buf.add(1);
    buflen -= part.len() + 1;

    let part = match parts.next() {
        Some(part) => part,
        None => return parse_error(result.cast()),
    };
    if part.len() > buflen {
        return buffer_exhausted(result.cast());
    }
    let gr_passwd = buf;
    copy_nonoverlapping(part.as_ptr(), buf.cast(), part.len());
    buf = buf.add(part.len());
    write(buf, 0);
    buf = buf.add(1);
    buflen -= part.len() + 1;

    let part = match parts.next() {
        Some(part) => part,
        None => return parse_error(result.cast()),
    };
    let gr_gid = match part.parse() {
        Ok(pw_gid) => pw_gid,
        Err(_err) => return parse_error(result.cast()),
    };

    let part = match parts.next() {
        Some(part) => part,
        None => return parse_error(result.cast()),
    };
    if part.len() > buflen {
        return buffer_exhausted(result.cast());
    }

    let num_members = if part.is_empty() {
        0
    } else {
        part.split(',').count()
    };
    let pad = align_of::<*const c_char>() - (buf.addr()) % align_of::<*const c_char>();
    buf = buf.add(pad);
    buflen -= pad;
    let gr_mem = buf.cast::<*mut c_char>();
    buf = gr_mem.add(num_members + 1).cast::<c_char>();
    buflen -= buf.addr() - gr_mem.addr();

    let mut cur_mem = gr_mem;
    if num_members != 0 {
        for member in part.split(',') {
            *cur_mem = buf;
            cur_mem = cur_mem.add(1);
            copy_nonoverlapping(member.as_ptr(), buf.cast(), member.len());
            buf = buf.add(member.len());
            write(buf, 0);
            buf = buf.add(1);
            buflen -= member.len() + 1;
        }
    }
    write(cur_mem, null_mut());

    write(
        grp,
        group {
            gr_name,
            gr_passwd,
            gr_gid,
            gr_mem,
        },
    );
    success(result.cast(), grp.cast())
}

#[cold]
unsafe fn buffer_exhausted(result: *mut *mut c_void) -> c_int {
    *result = null_mut();
    // It isn't documented that the `_r` functions set `errno` in addition to
    // returning it, but other popular implementations do, so set it.
    set_errno(Errno(libc::ERANGE));
    libc::ERANGE
}

#[cold]
unsafe fn parse_error(result: *mut *mut c_void) -> c_int {
    *result = null_mut();
    // As above, also set `errno`.
    set_errno(Errno(libc::EIO));
    libc::EIO
}

unsafe fn success(result: *mut *mut c_void, value: *mut c_void) -> c_int {
    *result = value;
    // As above, also set `errno`. Explicitly set it to zero in case any
    // intermediate operations failed.
    set_errno(Errno(0));
    0
}

struct StaticPasswd {
    record: passwd,
    buf: *mut c_char,
    len: usize,
}
// The C contract is that it's the caller's responsibility to ensure that
// we don't implicitly send this across threads.
unsafe impl Sync for StaticPasswd {}
static STATIC_PASSWD: SyncUnsafeCell<StaticPasswd> = SyncUnsafeCell::new(StaticPasswd {
    record: passwd {
        pw_name: null_mut(),
        pw_passwd: null_mut(),
        pw_uid: 0,
        pw_gid: 0,
        pw_gecos: null_mut(),
        pw_dir: null_mut(),
        pw_shell: null_mut(),
    },
    buf: null_mut(),
    len: 0,
});

struct StaticGroup {
    record: group,
    buf: *mut c_char,
    len: usize,
}
// The C contract is that it's the caller's responsibility to ensure that
// we don't implicitly send this across threads.
unsafe impl Sync for StaticGroup {}
static STATIC_GROUP: SyncUnsafeCell<StaticGroup> = SyncUnsafeCell::new(StaticGroup {
    record: group {
        gr_name: null_mut(),
        gr_passwd: null_mut(),
        gr_gid: 0,
        gr_mem: null_mut(),
    },
    buf: null_mut(),
    len: 0,
});

#[no_mangle]
unsafe extern "C" fn getpwnam(name: *const c_char) -> *mut libc::passwd {
    libc!(libc::getpwnam(name));

    let static_passwd = &mut *STATIC_PASSWD.get();
    let mut ptr: *mut libc::passwd = &mut static_passwd.record;

    loop {
        if static_passwd.len == 0 {
            static_passwd.len = 1024;
        } else {
            static_passwd.len *= 2;
            libc::free(static_passwd.buf.cast());
        }

        static_passwd.buf = libc::malloc(static_passwd.len).cast();
        if static_passwd.buf.is_null() {
            static_passwd.len = 0;
            set_errno(Errno(libc::ENOMEM));
            return null_mut();
        }

        let r = getpwnam_r(
            name,
            &mut static_passwd.record,
            static_passwd.buf,
            static_passwd.len,
            &mut ptr,
        );
        if r == 0 {
            return ptr;
        }
        if r != libc::ERANGE {
            return null_mut();
        }
    }
}

#[no_mangle]
unsafe extern "C" fn getpwuid(uid: uid_t) -> *mut libc::passwd {
    libc!(libc::getpwuid(uid));

    let static_passwd = &mut *STATIC_PASSWD.get();
    let mut ptr: *mut libc::passwd = &mut static_passwd.record;

    loop {
        if static_passwd.len == 0 {
            static_passwd.len = 1024;
        } else {
            static_passwd.len *= 2;
            libc::free(static_passwd.buf.cast());
        }

        static_passwd.buf = libc::malloc(static_passwd.len).cast();
        if static_passwd.buf.is_null() {
            static_passwd.len = 0;
            set_errno(Errno(libc::ENOMEM));
            return null_mut();
        }

        let r = getpwuid_r(
            uid,
            &mut static_passwd.record,
            static_passwd.buf,
            static_passwd.len,
            &mut ptr,
        );
        if r == 0 {
            return ptr;
        }
        if r != libc::ERANGE {
            return null_mut();
        }
    }
}

#[no_mangle]
unsafe extern "C" fn getgrnam(name: *const c_char) -> *mut libc::group {
    libc!(libc::getgrnam(name));

    let static_group = &mut *STATIC_GROUP.get();
    let mut ptr: *mut libc::group = &mut static_group.record;

    loop {
        if static_group.len == 0 {
            static_group.len = 1024;
        } else {
            static_group.len *= 2;
            libc::free(static_group.buf.cast());
        }

        static_group.buf = libc::malloc(static_group.len).cast();
        if static_group.buf.is_null() {
            static_group.len = 0;
            set_errno(Errno(libc::ENOMEM));
            return null_mut();
        }

        let r = getgrnam_r(
            name,
            &mut static_group.record,
            static_group.buf,
            static_group.len,
            &mut ptr,
        );
        if r == 0 {
            return ptr;
        }
        if r != libc::ERANGE {
            return null_mut();
        }
    }
}

#[no_mangle]
unsafe extern "C" fn getgrgid(gid: gid_t) -> *mut libc::group {
    libc!(libc::getgrgid(gid));

    let static_group = &mut *STATIC_GROUP.get();
    let mut ptr: *mut libc::group = &mut static_group.record;

    loop {
        if static_group.len == 0 {
            static_group.len = 1024;
        } else {
            static_group.len *= 2;
            libc::free(static_group.buf.cast());
        }

        static_group.buf = libc::malloc(static_group.len).cast();
        if static_group.buf.is_null() {
            static_group.len = 0;
            set_errno(Errno(libc::ENOMEM));
            return null_mut();
        }

        let r = getgrgid_r(
            gid,
            &mut static_group.record,
            static_group.buf,
            static_group.len,
            &mut ptr,
        );
        if r == 0 {
            return ptr;
        }
        if r != libc::ERANGE {
            return null_mut();
        }
    }
}

#[no_mangle]
unsafe extern "C" fn getgrouplist(
    user: *const c_char,
    group: gid_t,
    groups: *mut gid_t,
    ngroups: *mut c_int,
) -> c_int {
    libc!(libc::getgrouplist(user, group, groups, ngroups));

    let user = OsStr::from_bytes(CStr::from_ptr(user).to_bytes());
    let mut groups = groups;

    let mut command = Command::new("getent");
    command.arg("initgroups").arg(user);

    let output = match command.output() {
        Ok(output) => output,
        Err(_err) => return -1,
    };

    match output.status.code() {
        Some(0) => {}
        Some(r) => panic!("unexpected exit status from `getent initgroups`: {}", r),
        None => return -1,
    }

    let stdout = match str::from_utf8(&output.stdout) {
        Ok(stdout) => stdout,
        Err(_err) => return -1,
    };
    let stdout = match stdout.strip_suffix('\n') {
        Some(stdout) => stdout,
        None => return -1,
    };

    let mut parts = stdout.split_ascii_whitespace();
    match parts.next() {
        Some(part) => {
            if part != user {
                return -1;
            }
        }
        None => return -1,
    };

    let ngroups_in = ngroups.read();
    let mut ngroups_out = 0;

    if ngroups_out == ngroups_in {
        return -1;
    }
    ngroups_out += 1;
    groups.write(group);
    groups = groups.add(1);

    for part in parts {
        let gid: u32 = match part.parse() {
            Ok(gid) => gid,
            Err(_) => return -1,
        };
        if gid == group {
            continue;
        }
        if ngroups_out == ngroups_in {
            return -1;
        }
        ngroups_out += 1;
        groups.write(gid);
        groups = groups.add(1);
    }

    ngroups.write(ngroups_out);
    ngroups_out
}

#[no_mangle]
unsafe extern "C" fn getservbyport_r(
    port: c_int,
    proto: *const c_char,
    result_buf: *mut libc::servent,
    buf: *mut c_char,
    buflen: size_t,
    result: *mut *mut libc::servent,
) -> c_int {
    //libc!(libc::getservbyport_r(
    //port, proto, result_buf, buf, buflen, result
    //));

    let mut command = Command::new("getent");
    command
        .arg("services")
        .arg(DecInt::new(u16::from_be(port as u16)).as_ref());
    getserv_r(command, null(), proto, result_buf, buf, buflen, result)
}

#[no_mangle]
unsafe extern "C" fn getservbyname_r(
    name: *const c_char,
    proto: *const c_char,
    result_buf: *mut libc::servent,
    buf: *mut c_char,
    buflen: size_t,
    result: *mut *mut libc::servent,
) -> c_int {
    //libc!(libc::getservbyname_r(
    //name, proto, result_buf, buf, buflen, result
    //));

    let arg_name = OsStr::from_bytes(CStr::from_ptr(name).to_bytes());
    let mut command = Command::new("getent");
    command.arg("services").arg(arg_name);
    getserv_r(command, name, proto, result_buf, buf, buflen, result)
}

unsafe fn getserv_r(
    command: Command,
    name: *const c_char,
    proto: *const c_char,
    result_buf: *mut libc::servent,
    buf: *mut c_char,
    buflen: size_t,
    result: *mut *mut libc::servent,
) -> c_int {
    // glibc returns all the aliases but doesn't include the protocol name, and
    // musl returns just the protocol name as the alias list. The intersection
    // of these two that portable code is obliged to assume is an empty list.
    static mut STATIC_SERVENT_ALIASES: *mut c_char = null_mut();
    let s_aliases = &mut *addr_of_mut!(STATIC_SERVENT_ALIASES);

    let mut command = command;
    let output = match command.output() {
        Ok(output) => output,
        Err(_err) => {
            *result = null_mut();
            return libc::EIO;
        }
    };

    match output.status.code() {
        Some(0) => {}
        Some(2) => {
            *result = null_mut();
            return libc::ENOENT;
        }
        Some(r) => panic!("unexpected exit status from `getent services`: {}", r),
        None => {
            *result = null_mut();
            return libc::EIO;
        }
    }

    let stdout = match str::from_utf8(&output.stdout) {
        Ok(stdout) => stdout,
        Err(_err) => {
            *result = null_mut();
            return libc::EIO;
        }
    };
    let stdout = match stdout.strip_suffix('\n') {
        Some(stdout) => stdout,
        None => {
            *result = null_mut();
            return libc::EIO;
        }
    };

    // Parse eg. "http                  80/tcp www".
    let mut parts = stdout.split_ascii_whitespace();

    let s_name = match parts.next() {
        Some(check_name) => {
            if name.is_null() {
                if check_name.len() > buflen {
                    return libc::ERANGE;
                }
                copy_nonoverlapping(check_name.as_ptr(), buf.cast(), check_name.len());
                buf.add(check_name.len()).write(0);
                buf
            } else {
                name.cast_mut()
            }
        }
        None => {
            *result = null_mut();
            return libc::EIO;
        }
    };

    let port_protocol = match parts.next() {
        Some(port_protocol) => port_protocol,
        None => {
            *result = null_mut();
            return libc::EIO;
        }
    };

    // The rest of `pars.next()` contains aliases, but per the comment above,
    // we ignore them.

    // Parse eg. "443/tcp".
    let (port, protocol) = match port_protocol.split_once('/') {
        Some(port_protocol) => port_protocol,
        None => {
            *result = null_mut();
            return libc::EIO;
        }
    };

    // Parse the port number.
    let s_port: i32 = match u16::from_str(port) {
        Ok(port) => port.to_be().into(),
        Err(_) => {
            *result = null_mut();
            return libc::EIO;
        }
    };

    // Check the protocol, and if needed, translate the protocol string to
    // a static string so that it lives at least as long as the `servent`.
    let s_proto = if !proto.is_null() {
        if protocol.as_bytes() != CStr::from_ptr(proto).to_bytes() {
            *result = null_mut();
            return libc::EIO;
        }
        proto
    } else if protocol == "tcp" {
        c"tcp".as_ptr()
    } else if protocol == "udp" {
        c"udp".as_ptr()
    } else {
        return libc::EINVAL;
    }
    .cast_mut();

    *result_buf = libc::servent {
        s_name,
        s_aliases,
        s_port,
        s_proto,
    };
    *result = result_buf;
    0
}

// The C contract is that it's the caller's responsibility to ensure that
// we don't implicitly send this across threads.
static mut STATIC_SERVENT: libc::servent = unsafe { zeroed() };

#[no_mangle]
unsafe extern "C" fn getservbyname(
    name: *const c_char,
    proto: *const c_char,
) -> *mut libc::servent {
    libc!(libc::getservbyname(name, proto));

    let mut result = null_mut();
    if getservbyname_r(
        name,
        proto,
        addr_of_mut!(STATIC_SERVENT),
        null_mut(),
        0,
        &mut result,
    ) == 0
    {
        result
    } else {
        null_mut()
    }
}

#[no_mangle]
unsafe extern "C" fn getservbyport(port: c_int, proto: *const c_char) -> *mut libc::servent {
    libc!(libc::getservbyport(port, proto));

    let mut buf = [0; 32];
    let mut result = null_mut();
    if getservbyport_r(
        port,
        proto,
        addr_of_mut!(STATIC_SERVENT),
        buf.as_mut_ptr(),
        buf.len(),
        &mut result,
    ) == 0
    {
        result
    } else {
        null_mut()
    }
}
