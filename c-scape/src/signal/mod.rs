use crate::convert_res;
use core::mem::{align_of, size_of, size_of_val, transmute, zeroed};
use core::ptr::{addr_of, addr_of_mut, copy_nonoverlapping};
use errno::{set_errno, Errno};
use libc::*;
use rustix::cstr;
use rustix::process::Signal;
use rustix::runtime::{How, Sigaction, Siginfo, Sigset, Stack};

#[no_mangle]
unsafe extern "C" fn signal(signal: c_int, handler: sighandler_t) -> sighandler_t {
    libc!(libc::signal(signal, handler));

    if (rustix::runtime::SIGRTMIN as i32..__libc_current_sigrtmin()).contains(&signal) {
        set_errno(Errno(libc::EINVAL));
        return SIG_ERR;
    }

    let signal = match Signal::from_raw(signal) {
        Some(signal) => signal,
        None => {
            set_errno(Errno(libc::EINVAL));
            return SIG_ERR;
        }
    };

    let mut new = zeroed::<Sigaction>();
    new.sa_handler_kernel = transmute(handler);
    new.sa_flags = SA_RESTART as _;

    match convert_res(origin::signal::sigaction(signal, Some(new))) {
        Some(old) => transmute(old.sa_handler_kernel),
        None => SIG_ERR,
    }
}

#[no_mangle]
unsafe extern "C" fn sysv_signal(signal: c_int, handler: sighandler_t) -> sighandler_t {
    //libc!(libc::sysv_signal(signal, handler));
    libc::signal(signal, handler)
}

#[no_mangle]
unsafe extern "C" fn __sysv_signal(signal: c_int, handler: sighandler_t) -> sighandler_t {
    //libc!(libc::__sysv_signal(signal, handler));
    sysv_signal(signal, handler)
}

#[no_mangle]
unsafe extern "C" fn bsd_signal(signal: c_int, handler: sighandler_t) -> sighandler_t {
    //libc!(libc::bsd_signal(signal, handler));
    libc::signal(signal, handler)
}

#[no_mangle]
unsafe extern "C" fn sigaction(signal: c_int, new: *const sigaction, old: *mut sigaction) -> c_int {
    libc!(libc::sigaction(signal, new, old));

    if (rustix::runtime::SIGRTMIN as i32..__libc_current_sigrtmin()).contains(&signal) {
        set_errno(Errno(libc::EINVAL));
        return -1;
    }

    let signal = match Signal::from_raw(signal) {
        Some(signal) => signal,
        None => {
            set_errno(Errno(libc::EINVAL));
            return -1;
        }
    };

    let new = if new.is_null() {
        None
    } else {
        let new = new.read();
        let mut sa_mask: Sigset = zeroed();
        copy_nonoverlapping(
            addr_of!(new.sa_mask).cast::<u8>(),
            addr_of_mut!(sa_mask).cast::<u8>(),
            size_of_val(&zeroed::<Sigaction>().sa_mask),
        );

        Some(Sigaction {
            sa_handler_kernel: transmute(new.sa_sigaction),
            sa_flags: new.sa_flags.try_into().unwrap(),
            #[cfg(not(target_arch = "riscv64"))]
            sa_restorer: transmute(new.sa_restorer),
            sa_mask,
        })
    };

    match convert_res(origin::signal::sigaction(signal, new)) {
        Some(old_action) => {
            if !old.is_null() {
                let mut sa_mask: sigset_t = zeroed();
                copy_nonoverlapping(
                    addr_of!(old_action.sa_mask).cast::<u8>(),
                    addr_of_mut!(sa_mask).cast::<u8>(),
                    size_of_val(&zeroed::<Sigaction>().sa_mask),
                );

                let old_action = sigaction {
                    sa_sigaction: transmute(old_action.sa_handler_kernel),
                    sa_flags: old_action.sa_flags.try_into().unwrap(),
                    #[cfg(not(target_arch = "riscv64"))]
                    sa_restorer: transmute(old_action.sa_restorer),
                    #[cfg(target_arch = "riscv64")]
                    sa_restorer: zeroed(),
                    sa_mask,
                };
                old.write(old_action);
            }
            0
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int {
    libc!(libc::sigprocmask(how, set, oldset));

    let how = match how {
        SIG_BLOCK => How::BLOCK,
        SIG_UNBLOCK => How::UNBLOCK,
        SIG_SETMASK => How::SETMASK,
        _ => {
            set_errno(Errno(EINVAL));
            return -1;
        }
    };

    if !oldset.is_null() {
        oldset.write(zeroed());
    }

    assert!(size_of::<Sigset>() <= size_of::<sigset_t>());
    assert!(align_of::<Sigset>() <= align_of::<sigset_t>());
    let set: *const Sigset = set.cast();
    let oldset: *mut Sigset = oldset.cast();

    let set = if set.is_null() { None } else { Some(&*set) };

    match convert_res(rustix::runtime::sigprocmask(how, set)) {
        Some(mut old) => {
            if !oldset.is_null() {
                // Clear out the signals reserved for libc.
                for sig in rustix::runtime::SIGRTMIN as i32..__libc_current_sigrtmin() {
                    let elem: &mut c_ulong =
                        &mut old.sig[(sig - 1) as usize / c_ulong::BITS as usize];
                    *elem &= !(1 << ((sig - 1) as u32 % c_ulong::BITS));
                }

                oldset.write(old);
            }
            0
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn sigpending(set: *mut sigset_t) -> c_int {
    libc!(libc::sigpending(set));

    let set: *mut Sigset = set.cast();

    let pending = rustix::runtime::sigpending();
    set.write(pending);
    0
}

#[no_mangle]
unsafe extern "C" fn sigaltstack(new: *const stack_t, old: *mut stack_t) -> c_int {
    libc!(libc::sigaltstack(new, old));

    let new: *const Stack = checked_cast!(new);
    let old: *mut Stack = checked_cast!(old);

    let new = if new.is_null() {
        None
    } else {
        Some(new.read())
    };

    match convert_res(rustix::runtime::sigaltstack(new)) {
        Some(old_stack) => {
            if !old.is_null() {
                old.write(old_stack);
            }
            0
        }
        None => -1,
    }
}

#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn raise(sig: c_int) -> c_int {
    libc!(libc::raise(sig));

    let sig = match Signal::from_raw(sig) {
        Some(sig) => sig,
        None => {
            set_errno(Errno(EINVAL));
            return -1;
        }
    };
    let tid = origin::thread::current_thread_id();

    // `tkill` is ordinarily considered obsolete and dangerous, because a
    // thread could exit and its thread id could get reused by another thread.
    // But in this case, we're sending the signal to ourself, so we know we
    // haven't exited.
    match convert_res(rustix::runtime::tkill(tid, sig)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn abort() {
    libc!(libc::abort());

    // The `abort` function is documented to kill the process with an abort
    // signal. As in `raise`, `tkill` is dangerous in general, but safe here.
    rustix::runtime::tkill(origin::thread::current_thread_id(), Signal::Abort).ok();

    // That ought to work, but there's a possibility that the application has
    // a handler for the abort signal and that the handler returns. We really
    // don't want to return, because our caller presumably called `abort()`
    // for a reason, so we escalate to the unhandlable signal.
    rustix::runtime::tkill(origin::thread::current_thread_id(), Signal::Kill).ok();

    // That *really* should have worked. But if we're somehow still running,
    // abruptly exit the program.
    rustix::runtime::exit_group(127)
}

#[no_mangle]
unsafe extern "C" fn sigaddset(sigset: *mut sigset_t, signum: c_int) -> c_int {
    libc!(libc::sigaddset(sigset, signum));

    if signum == 0
        || signum as usize - 1 >= size_of::<sigset_t>() * 8
        || (rustix::runtime::SIGRTMIN as i32..__libc_current_sigrtmin()).contains(&signum)
    {
        set_errno(Errno(EINVAL));
        return -1;
    }

    let sig_index = (signum - 1) as usize;
    let mut x = sigset
        .cast::<usize>()
        .add(sig_index / usize::BITS as usize)
        .read();
    x |= 1_usize << (sig_index % usize::BITS as usize);
    sigset
        .cast::<usize>()
        .add(sig_index / usize::BITS as usize)
        .write(x);
    0
}

#[no_mangle]
unsafe extern "C" fn sigdelset(sigset: *mut sigset_t, signum: c_int) -> c_int {
    libc!(libc::sigdelset(sigset, signum));

    if signum == 0
        || signum as usize - 1 >= size_of::<sigset_t>() * 8
        || (rustix::runtime::SIGRTMIN as i32..__libc_current_sigrtmin()).contains(&signum)
    {
        set_errno(Errno(EINVAL));
        return -1;
    }

    let sig_index = (signum - 1) as usize;
    let mut x = sigset
        .cast::<usize>()
        .add(sig_index / usize::BITS as usize)
        .read();
    x &= !(1_usize << (sig_index % usize::BITS as usize));
    sigset
        .cast::<usize>()
        .add(sig_index / usize::BITS as usize)
        .write(x);
    0
}

#[no_mangle]
unsafe extern "C" fn sigemptyset(sigset: *mut sigset_t) -> c_int {
    libc!(libc::sigemptyset(sigset));
    sigset.write(zeroed());
    0
}

#[no_mangle]
unsafe extern "C" fn sigfillset(sigset: *mut sigset_t) -> c_int {
    libc!(libc::sigfillset(sigset));
    for index in 0..(size_of::<sigset_t>() / size_of::<usize>()) {
        sigset.cast::<usize>().add(index).write(!0);
    }
    0
}

#[no_mangle]
unsafe extern "C" fn sigismember(sigset: *const sigset_t, signum: c_int) -> c_int {
    libc!(libc::sigismember(sigset, signum));

    if signum == 0 || signum as usize - 1 >= size_of::<sigset_t>() * 8 {
        set_errno(Errno(EINVAL));
        return -1;
    }

    let sig_index = (signum - 1) as usize;
    let x = sigset
        .cast::<usize>()
        .add(sig_index / usize::BITS as usize)
        .read();
    ((x & (1_usize << (sig_index % usize::BITS as usize))) != 0) as c_int
}

#[no_mangle]
unsafe extern "C" fn sigwait(set: *const sigset_t, sig: *mut c_int) -> c_int {
    libc!(libc::sigwait(set, sig));

    assert!(size_of::<Sigset>() <= size_of::<sigset_t>());
    assert!(align_of::<Sigset>() <= align_of::<sigset_t>());
    let set: *const Sigset = set.cast();

    match rustix::runtime::sigwait(&*set) {
        Ok(signum) => {
            sig.write(signum as _);
            0
        }
        Err(e) => e.raw_os_error(),
    }
}

#[no_mangle]
unsafe extern "C" fn sigwaitinfo(set: *const sigset_t, info: *mut siginfo_t) -> c_int {
    libc!(libc::sigwaitinfo(set, info));

    assert!(size_of::<Sigset>() <= size_of::<sigset_t>());
    assert!(align_of::<Sigset>() <= align_of::<sigset_t>());
    let set: *const Sigset = set.cast();

    let info: *mut Siginfo = checked_cast!(info);

    match convert_res(rustix::runtime::sigwaitinfo(&*set)) {
        Some(info_value) => {
            if !info.is_null() {
                info.write(info_value);
            }
            info_value.__bindgen_anon_1.__bindgen_anon_1.si_signo
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn sigtimedwait(
    set: *const sigset_t,
    info: *mut siginfo_t,
    timeout: *const timespec,
) -> c_int {
    libc!(libc::sigtimedwait(set, info, timeout));

    let timeout = if timeout.is_null() {
        None
    } else {
        Some(rustix::time::Timespec {
            tv_sec: (*timeout).tv_sec.into(),
            tv_nsec: (*timeout).tv_nsec as _,
        })
    };

    assert!(size_of::<Sigset>() <= size_of::<sigset_t>());
    assert!(align_of::<Sigset>() <= align_of::<sigset_t>());
    let set: *const Sigset = set.cast();

    let info: *mut Siginfo = checked_cast!(info);

    match convert_res(rustix::runtime::sigtimedwait(&*set, timeout)) {
        Some(info_value) => {
            if !info.is_null() {
                info.write(info_value);
            }
            info_value.__bindgen_anon_1.__bindgen_anon_1.si_signo
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn strsignal(sig: c_int) -> *mut c_char {
    libc!(libc::strsignal(sig));

    let sig = match Signal::from_raw(sig) {
        Some(sig) => sig,
        None => return cstr!("Unknown signal").as_ptr() as _,
    };

    match sig {
        Signal::Abort => cstr!("Process abort signal").as_ptr() as _,
        Signal::Alarm => cstr!("Alarm clock").as_ptr() as _,
        Signal::Bus => cstr!("Access to an undefined portion of a memory object").as_ptr() as _,
        Signal::Child => cstr!("Child process terminated, stopped, or continued").as_ptr() as _,
        Signal::Cont => cstr!("Continue executing, if stopped").as_ptr() as _,
        Signal::Fpe => cstr!("Erroneous arithmetic operation").as_ptr() as _,
        Signal::Hup => cstr!("Hangup").as_ptr() as _,
        Signal::Ill => cstr!("Illegal instruction").as_ptr() as _,
        Signal::Int => cstr!("Terminal interrupt signal").as_ptr() as _,
        Signal::Kill => cstr!("Kill (cannot be caught or ignored)").as_ptr() as _,
        Signal::Pipe => cstr!("Write on a pipe with no one to read it").as_ptr() as _,
        Signal::Quit => cstr!("Terminal quit signal").as_ptr() as _,
        Signal::Segv => cstr!("Invalid memory reference").as_ptr() as _,
        Signal::Stop => cstr!("Stop executing (cannot be caught or ignored)").as_ptr() as _,
        Signal::Term => cstr!("Termination signal").as_ptr() as _,
        Signal::Tstp => cstr!("Terminal stop signal").as_ptr() as _,
        Signal::Ttin => cstr!("Background process attempting read").as_ptr() as _,
        Signal::Ttou => cstr!("Background process attempting write").as_ptr() as _,
        Signal::Usr1 => cstr!("User-defined signal 1").as_ptr() as _,
        Signal::Usr2 => cstr!("User-defined signal 2").as_ptr() as _,
        Signal::Prof => cstr!("Profiling timer expired").as_ptr() as _,
        Signal::Sys => cstr!("Bad system call").as_ptr() as _,
        Signal::Trap => cstr!("Trace/breakpoint trap").as_ptr() as _,
        Signal::Urg => cstr!("High bandwidth data is available at a socket").as_ptr() as _,
        Signal::Vtalarm => cstr!("Virtual timer expired").as_ptr() as _,
        Signal::Xcpu => cstr!("CPU time limit exceeded").as_ptr() as _,
        Signal::Xfsz => cstr!("File size limit exceeded").as_ptr() as _,
        Signal::Io => cstr!("I/O now possible").as_ptr() as _,
        Signal::Stkflt => cstr!("Stack fault on coprocessor").as_ptr() as _,
        Signal::Winch => cstr!("Window resize signal").as_ptr() as _,
        Signal::Power => cstr!("Power failure").as_ptr() as _,
    }
}

/// This function conforms to the [LSB `__libc_current_sigrtmin`] ABI.
///
/// [LSB `__libc_current_sigrtmin`]: https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---libc-current-sigrtmin-1.html
#[no_mangle]
unsafe extern "C" fn __libc_current_sigrtmin() -> i32 {
    libc!(libc::__libc_current_sigrtmin());

    // Reserve 3 RT signals for ourselves. We don't currently implement
    // anything that uses these signals, but we might as well reserve some
    // for when we do.
    (rustix::runtime::SIGRTMIN + 3) as i32
}

/// This function conforms to the [LSB `__libc_current_sigrtmax`] ABI.
///
/// [LSB `__libc_current_sigrtmax`]: https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---libc-current-sigrtmax-1.html
#[no_mangle]
unsafe extern "C" fn __libc_current_sigrtmax() -> i32 {
    libc!(libc::__libc_current_sigrtmax());

    rustix::runtime::SIGRTMAX as i32
}
