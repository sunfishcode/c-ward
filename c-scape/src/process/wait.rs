use core::mem::zeroed;
use errno::{set_errno, Errno};
use libc::{c_int, id_t, idtype_t, pid_t, siginfo_t};
use rustix::fd::BorrowedFd;
use rustix::process::{Pid, WaitId, WaitOptions, WaitidOptions};

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

#[no_mangle]
unsafe extern "C" fn waitid(
    idtype: idtype_t,
    id: id_t,
    infop: *mut siginfo_t,
    options: c_int,
) -> c_int {
    libc!(libc::waitid(idtype, id, infop, options));

    let id = match idtype {
        libc::P_PID => {
            if let Some(pid) = Pid::from_raw(id as _) {
                WaitId::Pid(pid)
            } else {
                set_errno(Errno(libc::EINVAL));
                return -1;
            }
        }
        libc::P_PIDFD => WaitId::PidFd(BorrowedFd::borrow_raw(id as _)),
        libc::P_PGID => WaitId::Pgid(Pid::from_raw(id as _)),
        libc::P_ALL => WaitId::All,
        _ => {
            set_errno(Errno(libc::EINVAL));
            return -1;
        }
    };

    let options = WaitidOptions::from_bits(options as _).unwrap();

    match convert_res(rustix::process::waitid(id, options)) {
        Some(Some(new_info)) => {
            *infop = zeroed();
            (*infop).si_signo = new_info.as_raw().__bindgen_anon_1.__bindgen_anon_1.si_signo;
            (*infop).si_errno = new_info.as_raw().__bindgen_anon_1.__bindgen_anon_1.si_errno;
            (*infop).si_code = new_info.as_raw().__bindgen_anon_1.__bindgen_anon_1.si_code;
            0
        }
        Some(None) => 0,
        None => -1,
    }
}
