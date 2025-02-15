use crate::convert_res;
use core::mem::{size_of, size_of_val, transmute, zeroed};
use core::ptr::{addr_of, addr_of_mut, copy_nonoverlapping};
use errno::{set_errno, Errno};
use libc::*;
use origin::signal::{Sigaction, SigactionFlags};
use rustix::process::Signal;
use rustix::runtime::{How, KernelSigSet, Siginfo, Stack, KERNEL_SIGRTMAX, KERNEL_SIGRTMIN};

#[no_mangle]
unsafe extern "C" fn signal(signal: c_int, handler: sighandler_t) -> sighandler_t {
    libc!(libc::signal(signal, handler));

    if signal == 0 || (KERNEL_SIGRTMIN as i32..__libc_current_sigrtmin()).contains(&signal) {
        set_errno(Errno(libc::EINVAL));
        return SIG_ERR;
    }

    let signal = Signal::from_raw_unchecked(signal);

    let mut new = Sigaction::default();
    new.sa_handler_kernel = transmute(handler);
    new.sa_flags = SigactionFlags::RESTART;

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

    if (KERNEL_SIGRTMIN as i32..__libc_current_sigrtmin()).contains(&signal) {
        set_errno(Errno(libc::EINVAL));
        return -1;
    }

    let signal = Signal::from_raw_unchecked(signal);

    let new = if new.is_null() {
        None
    } else {
        let new = new.read();
        let mut sa_mask = KernelSigSet::empty();
        copy_nonoverlapping(
            addr_of!(new.sa_mask).cast::<u8>(),
            addr_of_mut!(sa_mask).cast::<u8>(),
            size_of_val(&zeroed::<Sigaction>().sa_mask),
        );

        Some(Sigaction {
            sa_handler_kernel: transmute(new.sa_sigaction),
            sa_flags: SigactionFlags::from_bits_retain(new.sa_flags as _),
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
                    sa_flags: old_action.sa_flags.bits() as _,
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

    let set = if set.is_null() {
        None
    } else {
        Some(&*set.cast::<KernelSigSet>())
    };

    match convert_res(rustix::runtime::kernel_sigprocmask(how, set)) {
        Some(mut old) => {
            if !oldset.is_null() {
                // Clear out the signals reserved for libc.
                for sig in KERNEL_SIGRTMIN..SIGRTMIN {
                    old.remove(Signal::from_raw_unchecked(sig));
                }

                oldset.write(crate::expand_sigset(old));
            }
            0
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn sigpending(set: *mut sigset_t) -> c_int {
    libc!(libc::sigpending(set));

    let pending = rustix::runtime::kernel_sigpending();
    set.write(crate::expand_sigset(pending));
    0
}

#[no_mangle]
unsafe extern "C" fn sigsuspend(set: *const sigset_t) -> c_int {
    libc!(libc::sigsuspend(set));

    match convert_res(rustix::runtime::kernel_sigsuspend(&*set.cast())) {
        Some(()) => 0,
        None => -1,
    }
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

    match convert_res(rustix::runtime::kernel_sigaltstack(new)) {
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

    let sig = Signal::from_raw_unchecked(sig);

    let tid = origin::thread::current_id();

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
unsafe extern "C" fn abort() -> ! {
    libc!(libc::abort());

    let tid = origin::thread::current_id();

    // The `abort` function is documented to kill the current thread with an
    // abort signal. As in `raise`, `tkill` is dangerous in general, but safe
    // here.
    rustix::runtime::tkill(tid, Signal::ABORT).ok();

    // That ought to work, but there's a possibility that the application has
    // a handler for the abort signal and that the handler returns.
    //
    // According to POSIX we should try unregistering any handler for
    // `Signal::Abort`, but doing that reliably requires taking a lock and
    // coordinating with various other things, so for now just switch to
    // a higher-powered way to terminate the process.
    rustix::runtime::tkill(tid, Signal::KILL).ok();

    // That *really* should have worked. But if we're somehow still running,
    // abruptly exit the program.
    origin::program::trap();
}

#[no_mangle]
unsafe extern "C" fn sigaddset(sigset: *mut sigset_t, signum: c_int) -> c_int {
    libc!(libc::sigaddset(sigset, signum));

    if signum == 0
        || signum as usize - 1 >= size_of::<sigset_t>() * 8
        || (KERNEL_SIGRTMIN as i32..__libc_current_sigrtmin()).contains(&signum)
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
        || (KERNEL_SIGRTMIN as i32..__libc_current_sigrtmin()).contains(&signum)
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

    match rustix::runtime::kernel_sigwait(&*set.cast()) {
        Ok(signum) => {
            sig.write(signum.as_raw());
            0
        }
        Err(e) => e.raw_os_error(),
    }
}

#[no_mangle]
unsafe extern "C" fn sigwaitinfo(set: *const sigset_t, info: *mut siginfo_t) -> c_int {
    libc!(libc::sigwaitinfo(set, info));

    let info: *mut Siginfo = checked_cast!(info);

    match convert_res(rustix::runtime::kernel_sigwaitinfo(&*set.cast())) {
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

    let info: *mut Siginfo = checked_cast!(info);

    match convert_res(rustix::runtime::kernel_sigtimedwait(
        &*set.cast(),
        timeout.as_ref(),
    )) {
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

    match Signal::from_raw_unchecked(sig) {
        Signal::ABORT => c"Process abort signal".as_ptr() as _,
        Signal::ALARM => c"Alarm clock".as_ptr() as _,
        Signal::BUS => c"Access to an undefined portion of a memory object".as_ptr() as _,
        Signal::CHILD => c"Child process terminated, stopped, or continued".as_ptr() as _,
        Signal::CONT => c"Continue executing, if stopped".as_ptr() as _,
        Signal::FPE => c"Erroneous arithmetic operation".as_ptr() as _,
        Signal::HUP => c"Hangup".as_ptr() as _,
        Signal::ILL => c"Invalid instruction".as_ptr() as _,
        Signal::INT => c"Terminal interrupt signal".as_ptr() as _,
        Signal::KILL => c"Kill (cannot be caught or ignored)".as_ptr() as _,
        Signal::PIPE => c"Write on a pipe with no one to read it".as_ptr() as _,
        Signal::QUIT => c"Terminal quit signal".as_ptr() as _,
        Signal::SEGV => c"Invalid memory reference".as_ptr() as _,
        Signal::STOP => c"Stop executing (cannot be caught or ignored)".as_ptr() as _,
        Signal::TERM => c"Termination signal".as_ptr() as _,
        Signal::TSTP => c"Terminal stop signal".as_ptr() as _,
        Signal::TTIN => c"Background process attempting read".as_ptr() as _,
        Signal::TTOU => c"Background process attempting write".as_ptr() as _,
        Signal::USR1 => c"User-defined signal 1".as_ptr() as _,
        Signal::USR2 => c"User-defined signal 2".as_ptr() as _,
        Signal::PROF => c"Profiling timer expired".as_ptr() as _,
        Signal::SYS => c"Bad system call".as_ptr() as _,
        Signal::TRAP => c"Trace/breakpoint trap".as_ptr() as _,
        Signal::URG => c"High bandwidth data is available at a socket".as_ptr() as _,
        Signal::VTALARM => c"Virtual timer expired".as_ptr() as _,
        Signal::XCPU => c"CPU time limit exceeded".as_ptr() as _,
        Signal::XFSZ => c"File size limit exceeded".as_ptr() as _,
        Signal::IO => c"I/O now possible".as_ptr() as _,
        Signal::STKFLT => c"Stack fault on coprocessor".as_ptr() as _,
        Signal::WINCH => c"Window resize signal".as_ptr() as _,
        Signal::POWER => c"Power failure".as_ptr() as _,
        RESERVED0 => "RESERVED0".as_ptr() as _,
        RESERVED1 => "RESERVED1".as_ptr() as _,
        RESERVED2 => "RESERVED2".as_ptr() as _,
        RT0 => "SIGRTMIN+0".as_ptr() as _,
        RT1 => "SIGRTMIN+1".as_ptr() as _,
        RT2 => "SIGRTMIN+2".as_ptr() as _,
        RT3 => "SIGRTMIN+3".as_ptr() as _,
        RT4 => "SIGRTMIN+4".as_ptr() as _,
        RT5 => "SIGRTMIN+5".as_ptr() as _,
        RT6 => "SIGRTMIN+6".as_ptr() as _,
        RT7 => "SIGRTMIN+7".as_ptr() as _,
        RT8 => "SIGRTMIN+8".as_ptr() as _,
        RT9 => "SIGRTMIN+9".as_ptr() as _,
        RT10 => "SIGRTMIN+10".as_ptr() as _,
        RT11 => "SIGRTMIN+11".as_ptr() as _,
        RT12 => "SIGRTMIN+12".as_ptr() as _,
        RT13 => "SIGRTMIN+13".as_ptr() as _,
        RT14 => "SIGRTMIN+14".as_ptr() as _,
        RT15 => "SIGRTMIN+15".as_ptr() as _,
        RT16 => "SIGRTMIN+16".as_ptr() as _,
        RT17 => "SIGRTMIN+17".as_ptr() as _,
        RT18 => "SIGRTMIN+18".as_ptr() as _,
        RT19 => "SIGRTMIN+19".as_ptr() as _,
        RT20 => "SIGRTMIN+20".as_ptr() as _,
        RT21 => "SIGRTMIN+21".as_ptr() as _,
        RT22 => "SIGRTMIN+22".as_ptr() as _,
        RT23 => "SIGRTMIN+23".as_ptr() as _,
        RT24 => "SIGRTMIN+24".as_ptr() as _,
        RT25 => "SIGRTMIN+25".as_ptr() as _,
        RT26 => "SIGRTMIN+26".as_ptr() as _,
        RT27 => "SIGRTMIN+27".as_ptr() as _,
        RT28 => "SIGRTMIN+28".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT32 => "SIGRTMIN+32".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT33 => "SIGRTMIN+33".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT34 => "SIGRTMIN+34".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT35 => "SIGRTMIN+35".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT36 => "SIGRTMIN+36".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT37 => "SIGRTMIN+37".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT38 => "SIGRTMIN+38".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT39 => "SIGRTMIN+39".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT40 => "SIGRTMIN+40".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT41 => "SIGRTMIN+41".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT42 => "SIGRTMIN+42".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT43 => "SIGRTMIN+43".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT44 => "SIGRTMIN+44".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT45 => "SIGRTMIN+45".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT46 => "SIGRTMIN+46".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT47 => "SIGRTMIN+47".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT48 => "SIGRTMIN+48".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT49 => "SIGRTMIN+49".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT50 => "SIGRTMIN+50".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT51 => "SIGRTMIN+51".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT52 => "SIGRTMIN+52".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT53 => "SIGRTMIN+53".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT54 => "SIGRTMIN+54".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT55 => "SIGRTMIN+55".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT56 => "SIGRTMIN+56".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT57 => "SIGRTMIN+57".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT58 => "SIGRTMIN+58".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT59 => "SIGRTMIN+59".as_ptr() as _,
        #[cfg(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        ))]
        RT60 => "SIGRTMIN+60".as_ptr() as _,
        _ => c"Unknown signal".as_ptr() as _,
    }
}

/// This function conforms to the [LSB `__libc_current_sigrtmin`] ABI.
///
/// [LSB `__libc_current_sigrtmin`]: https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---libc-current-sigrtmin-1.html
#[no_mangle]
unsafe extern "C" fn __libc_current_sigrtmin() -> i32 {
    libc!(libc::__libc_current_sigrtmin());

    SIGRTMIN
}

/// This function conforms to the [LSB `__libc_current_sigrtmax`] ABI.
///
/// [LSB `__libc_current_sigrtmax`]: https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---libc-current-sigrtmax-1.html
#[no_mangle]
unsafe extern "C" fn __libc_current_sigrtmax() -> i32 {
    libc!(libc::__libc_current_sigrtmax());

    SIGRTMAX
}

// Reserve 3 RT signals for ourselves. We don't currently implement
// anything that uses these signals, but we might as well reserve some
// for when we do.
const SIGRTMIN: i32 = KERNEL_SIGRTMIN as i32 + 3;
const SIGRTMAX: i32 = KERNEL_SIGRTMAX as i32;

// The three signal values we just reserved.
const RESERVED0: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 0) };
const RESERVED1: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 1) };
const RESERVED2: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 2) };

// The `SIGRTMIN+n` signals available to the user.
const RT0: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 0) };
const RT1: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 1) };
const RT2: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 2) };
const RT3: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 3) };
const RT4: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 4) };
const RT5: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 5) };
const RT6: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 6) };
const RT7: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 7) };
const RT8: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 8) };
const RT9: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 9) };
const RT10: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 10) };
const RT11: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 11) };
const RT12: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 12) };
const RT13: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 13) };
const RT14: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 14) };
const RT15: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 15) };
const RT16: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 16) };
const RT17: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 17) };
const RT18: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 18) };
const RT19: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 19) };
const RT20: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 20) };
const RT21: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 21) };
const RT22: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 22) };
const RT23: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 23) };
const RT24: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 24) };
const RT25: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 25) };
const RT26: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 26) };
const RT27: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 27) };
const RT28: Signal = unsafe { Signal::from_raw_unchecked(SIGRTMIN + 28) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT29: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 29) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT30: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 30) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT31: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 31) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT32: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 32) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT33: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 33) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT34: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 34) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT35: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 35) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT36: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 36) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT37: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 37) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT38: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 38) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT39: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 39) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT40: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 40) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT41: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 41) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT42: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 42) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT43: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 43) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT44: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 44) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT45: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 45) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT46: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 46) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT47: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 47) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT48: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 48) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT49: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 49) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT50: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 50) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT51: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 51) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT52: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 52) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT53: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 53) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT54: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 54) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT55: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 55) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT56: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 56) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT57: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 57) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT58: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 58) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT59: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 59) };
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
const RT60: Signal = unsafe { Signal::from_raw_unchecked(KERNEL_SIGRTMIN + 60) };

// Check that we have the correct number of RT signals.
/*
#[cfg(not(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
)))]
static_assertions::const_assert_eq!(linux_raw_sys::general::_NSIG, 64);
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips32r6",
    target_arch = "mips64",
    target_arch = "mips64r6"
))]
static_assertions::const_assert_eq!(linux_raw_sys::general::_NSIG, 128);
*/

#[cfg(test)]
mod tests {
    use core::mem::zeroed;
    use core::ptr::null_mut;

    #[test]
    fn test_sigaction_invalid_flags() {
        unsafe {
            let new = libc::sigaction {
                sa_sigaction: libc::SIG_DFL,
                sa_flags: !0,
                sa_mask: zeroed(),
                sa_restorer: None,
            };
            assert_eq!(libc::sigaction(libc::SIGILL, &new, null_mut()), 0);
        }
    }
}
