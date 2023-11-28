use core::ffi::CStr;
use errno::{set_errno, Errno};
use libc::{c_char, c_int};
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

#[no_mangle]
unsafe extern "C" fn system(command: *const c_char) -> c_int {
    libc!(libc::system(command));

    if command.is_null() {
        (_system(OsStr::new("exit 0")) == 0).into()
    } else {
        _system(OsStr::from_bytes(CStr::from_ptr(command).to_bytes()))
    }
}

fn _system(command: &OsStr) -> c_int {
    let mut sh = std::process::Command::new("/bin/sh");
    sh.arg("-c");
    sh.arg(command);

    match sh.status() {
        Ok(status) => status.code().unwrap(),
        Err(err) => {
            set_errno(Errno(err.raw_os_error().unwrap()));
            -1
        }
    }
}
