use errno::{set_errno, Errno};
use libc::{c_int, pid_t};
use rustix::process::{Pid, WaitOptions};

use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int {
    libc!(libc::waitpid(pid, status, options));
    let options = WaitOptions::from_bits(options as _).unwrap();
    let ret_pid;
    let ret_status;
    match pid {
        -1 => match convert_res(rustix::process::wait(options)) {
            Some(Some((new_pid, new_status))) => {
                ret_pid = new_pid.as_raw_nonzero().get() as c_int;
                ret_status = new_status.as_raw() as c_int;
            }
            Some(None) => return 0,
            None => return -1,
        },
        pid if pid == pid_t::MIN => {
            set_errno(Errno(libc::ESRCH));
            return -1;
        }
        pid if pid < 0 => match convert_res(rustix::process::waitpgid(
            Pid::from_raw_unchecked(pid.wrapping_neg()),
            options,
        )) {
            Some(Some(new_status)) => {
                ret_pid = if pid == 0 {
                    rustix::process::getpid().as_raw_nonzero().get() as c_int
                } else {
                    pid
                };
                ret_status = new_status.as_raw() as c_int;
            }
            Some(None) => return 0,
            None => return -1,
        },
        pid => match convert_res(rustix::process::waitpid(Pid::from_raw(pid as _), options)) {
            Some(Some(new_status)) => {
                ret_pid = if pid == 0 {
                    rustix::process::getpid().as_raw_nonzero().get() as c_int
                } else {
                    pid
                };
                ret_status = new_status.as_raw() as c_int;
            }
            Some(None) => return 0,
            None => return -1,
        },
    }
    if !status.is_null() {
        status.write(ret_status);
    }
    ret_pid
}

#[no_mangle]
unsafe extern "C" fn wait(status: *mut c_int) -> pid_t {
    libc!(libc::wait(status));
    waitpid(-1, status, 0)
}
