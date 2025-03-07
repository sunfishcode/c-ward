use rustix::process::{Pid, Signal};

use errno::{set_errno, Errno};
use libc::{c_int, pid_t};

use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn kill(pid: pid_t, sig: c_int) -> c_int {
    libc!(libc::kill(pid, sig));

    if sig == 0 {
        let res = if pid < 0 {
            rustix::process::test_kill_process_group(Pid::from_raw(-pid as _).unwrap())
        } else if let Some(pid) = Pid::from_raw(pid as _) {
            rustix::process::test_kill_process(pid)
        } else {
            rustix::process::test_kill_current_process_group()
        };

        return match convert_res(res) {
            Some(()) => 0,
            None => -1,
        };
    }

    let sig = Signal::from_raw_unchecked(sig);

    let res = if pid < 0 {
        rustix::process::kill_process_group(Pid::from_raw(-pid as _).unwrap(), sig)
    } else if let Some(pid) = Pid::from_raw(pid as _) {
        rustix::process::kill_process(pid, sig)
    } else {
        rustix::process::kill_current_process_group(sig)
    };

    match convert_res(res) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn killpg(pgid: pid_t, sig: c_int) -> c_int {
    libc!(libc::killpg(pgid, sig));

    if pgid < 0 {
        set_errno(Errno(libc::EINVAL));
        return -1;
    }

    kill(-pgid, sig)
}
