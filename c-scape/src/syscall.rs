use core::ptr;
use errno::{set_errno, Errno};
#[cfg(feature = "extra-syscalls")]
use libc::{c_char, size_t};
use libc::{c_int, c_long, c_void, timespec};
#[cfg(feature = "thread")]
use {crate::convert_res, core::mem::zeroed, core::ptr::null};

// `syscall` usually returns `long`, but we make it a pointer type so that it
// preserves provenance.
#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn syscall(number: c_long, mut args: ...) -> *mut c_void {
    match number {
        #[cfg(feature = "extra-syscalls")]
        libc::SYS_read => {
            let fd = args.arg::<c_int>();
            let buf = args.arg::<*mut c_void>();
            let count = args.arg::<size_t>();
            ptr::invalid_mut(libc::read(fd, buf, count) as _)
        }
        #[cfg(feature = "extra-syscalls")]
        libc::SYS_write => {
            let fd = args.arg::<c_int>();
            let buf = args.arg::<*const c_void>();
            let count = args.arg::<size_t>();
            ptr::invalid_mut(libc::write(fd, buf, count) as _)
        }
        #[cfg(feature = "extra-syscalls")]
        #[cfg(not(any(target_arch = "aarch64", target_arch = "riscv64")))]
        libc::SYS_open => {
            let path = args.arg::<*const c_char>();
            let flags = args.arg::<c_int>();
            let fd = if ((flags & libc::O_CREAT) == libc::O_CREAT)
                || ((flags & libc::O_TMPFILE) == libc::O_TMPFILE)
            {
                let mode = args.arg::<libc::mode_t>();
                libc::open(path, flags, mode)
            } else {
                libc::open(path, flags)
            };
            ptr::invalid_mut(fd as _)
        }
        #[cfg(feature = "extra-syscalls")]
        libc::SYS_openat => {
            let dirfd = args.arg::<c_int>();
            let path = args.arg::<*const c_char>();
            let flags = args.arg::<c_int>();
            let fd = if ((flags & libc::O_CREAT) == libc::O_CREAT)
                || ((flags & libc::O_TMPFILE) == libc::O_TMPFILE)
            {
                let mode = args.arg::<libc::mode_t>();
                libc::openat(dirfd, path, flags, mode)
            } else {
                libc::openat(dirfd, path, flags)
            };
            ptr::invalid_mut(fd as _)
        }
        #[cfg(feature = "extra-syscalls")]
        libc::SYS_close => {
            let fd = args.arg::<c_int>();
            ptr::invalid_mut(libc::close(fd) as _)
        }
        libc::SYS_getrandom => {
            let buf = args.arg::<*mut c_void>();
            let len = args.arg::<usize>();
            let flags = args.arg::<u32>();
            ptr::invalid_mut(libc::getrandom(buf, len, flags) as _)
        }
        #[cfg(feature = "thread")]
        libc::SYS_futex => {
            let uaddr = args.arg::<*mut u32>();
            let futex_op = args.arg::<c_int>();
            let val = args.arg::<u32>();
            let timeout = args.arg::<*const libc::timespec>();
            let uaddr2 = args.arg::<*mut u32>();
            let val3 = args.arg::<u32>();
            ptr::invalid_mut(futex(uaddr, futex_op, val, timeout, uaddr2, val3) as isize as usize)
        }
        libc::SYS_clone3 => {
            // ensure std::process uses fork as fallback code on linux
            set_errno(Errno(libc::ENOSYS));
            ptr::invalid_mut(!0)
        }
        #[cfg(feature = "extra-syscalls")]
        libc::SYS_epoll_create1 => {
            let flags = args.arg::<c_int>();
            ptr::invalid_mut(libc::epoll_create(flags) as isize as usize)
        }
        #[cfg(feature = "extra-syscalls")]
        libc::SYS_timerfd_create => {
            let clockid = args.arg::<c_int>();
            let flags = args.arg::<c_int>();
            ptr::invalid_mut(libc::timerfd_create(clockid, flags) as isize as usize)
        }
        #[cfg(feature = "extra-syscalls")]
        libc::SYS_timerfd_settime => {
            let fd = args.arg::<c_int>();
            let flags = args.arg::<c_int>();
            let new_value = args.arg::<*const libc::itimerspec>();
            let old_value = args.arg::<*mut libc::itimerspec>();
            ptr::invalid_mut(
                libc::timerfd_settime(fd, flags, new_value, old_value) as isize as usize,
            )
        }
        #[cfg(feature = "extra-syscalls")]
        libc::SYS_utimensat => {
            let fd = args.arg::<c_int>();
            let path = args.arg::<*const c_char>();
            let times = args.arg::<*const libc::timespec>();
            let flags = args.arg::<c_int>();
            ptr::invalid_mut(libc::utimensat(fd, path, times, flags) as isize as usize)
        }
        #[cfg(feature = "extra-syscalls")]
        libc::SYS_fdatasync => {
            let fd = args.arg::<c_int>();
            ptr::invalid_mut(libc::fdatasync(fd) as isize as usize)
        }
        #[cfg(feature = "extra-syscalls")]
        libc::SYS_syncfs => {
            let fd = args.arg::<c_int>();
            ptr::invalid_mut(libc::syncfs(fd) as isize as usize)
        }
        _ => unimplemented!("syscall({:?})", number),
    }
}

unsafe fn futex(
    uaddr: *mut u32,
    futex_op: c_int,
    val: u32,
    timeout: *const timespec,
    uaddr2: *mut u32,
    val3: u32,
) -> c_long {
    use rustix::thread::{futex, FutexFlags, FutexOperation};

    libc!(libc::syscall(libc::SYS_futex, uaddr, futex_op, val, timeout, uaddr2, val3) as _);
    let flags = FutexFlags::from_bits_truncate(futex_op as _);
    let op = match futex_op & (!flags.bits() as i32) {
        libc::FUTEX_WAIT => FutexOperation::Wait,
        libc::FUTEX_WAKE => FutexOperation::Wake,
        libc::FUTEX_FD => FutexOperation::Fd,
        libc::FUTEX_REQUEUE => FutexOperation::Requeue,
        libc::FUTEX_CMP_REQUEUE => FutexOperation::CmpRequeue,
        libc::FUTEX_WAKE_OP => FutexOperation::WakeOp,
        libc::FUTEX_LOCK_PI => FutexOperation::LockPi,
        libc::FUTEX_UNLOCK_PI => FutexOperation::UnlockPi,
        libc::FUTEX_TRYLOCK_PI => FutexOperation::TrylockPi,
        libc::FUTEX_WAIT_BITSET => FutexOperation::WaitBitset,
        _ => unimplemented!("unrecognized futex op {}", futex_op),
    };
    let old_timespec =
        if timeout.is_null() || !matches!(op, FutexOperation::Wait | FutexOperation::WaitBitset) {
            zeroed()
        } else {
            ptr::read(timeout)
        };
    let new_timespec = rustix::time::Timespec {
        tv_sec: old_timespec.tv_sec.into(),
        tv_nsec: old_timespec.tv_nsec as _,
    };
    let new_timespec = if timeout.is_null() {
        null()
    } else {
        &new_timespec
    };
    match convert_res(futex(uaddr, op, flags, val, new_timespec, uaddr2, val3)) {
        Some(result) => result as _,
        None => -1,
    }
}
