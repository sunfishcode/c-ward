#[cfg(feature = "thread")]
use crate::convert_res;
#[cfg(feature = "thread")]
use core::mem::zeroed;
use core::ptr::without_provenance_mut;
#[cfg(feature = "thread")]
use core::ptr::null;
use errno::{set_errno, Errno};
#[cfg(feature = "extra-syscalls")]
use libc::{c_char, size_t};
#[cfg(feature = "thread")]
use libc::{c_int, timespec};
use libc::{c_long, c_void};

// `syscall` usually returns `long`, but we make it a pointer type so that it
// preserves provenance.
#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn syscall(number: c_long, mut args: ...) -> *mut c_void {
    match number {
        #[cfg(feature = "syscall-read")]
        libc::SYS_read => {
            let fd = args.arg::<c_int>();
            let buf = args.arg::<*mut c_void>();
            let count = args.arg::<size_t>();
            without_provenance_mut(libc::read(fd, buf, count) as _)
        }
        #[cfg(feature = "syscall-write")]
        libc::SYS_write => {
            let fd = args.arg::<c_int>();
            let buf = args.arg::<*const c_void>();
            let count = args.arg::<size_t>();
            without_provenance_mut(libc::write(fd, buf, count) as _)
        }
        #[cfg(feature = "syscall-open")]
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
            without_provenance_mut(fd as _)
        }
        #[cfg(feature = "syscall-openat")]
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
            without_provenance_mut(fd as _)
        }
        #[cfg(feature = "syscall-close")]
        libc::SYS_close => {
            let fd = args.arg::<c_int>();
            without_provenance_mut(libc::close(fd) as _)
        }
        #[cfg(feature = "syscall-getpid")]
        libc::SYS_getpid => without_provenance_mut(rustix::process::getpid().as_raw_nonzero().get() as _),
        #[cfg(feature = "syscall-statx")]
        libc::SYS_statx => {
            let dirfd = args.arg::<c_int>();
            let path = args.arg::<*const c_char>();
            let flags = args.arg::<c_int>();
            let mask = args.arg::<libc::c_uint>();
            let statxbuf = args.arg::<*mut libc::statx>();
            without_provenance_mut(libc::statx(dirfd, path, flags, mask, statxbuf) as _)
        }
        libc::SYS_getrandom => {
            let buf = args.arg::<*mut c_void>();
            let len = args.arg::<usize>();
            let flags = args.arg::<u32>();
            without_provenance_mut(libc::getrandom(buf, len, flags) as _)
        }
        #[cfg(feature = "thread")]
        libc::SYS_futex => {
            let uaddr = args.arg::<*mut u32>();
            let futex_op = args.arg::<c_int>();
            let val = args.arg::<u32>();
            let timeout = args.arg::<*const libc::timespec>();
            let uaddr2 = args.arg::<*mut u32>();
            let val3 = args.arg::<u32>();
            without_provenance_mut(futex(uaddr, futex_op, val, timeout, uaddr2, val3) as isize as usize)
        }
        libc::SYS_clone3 => {
            // ensure std::process uses fork as fallback code on linux
            set_errno(Errno(libc::ENOSYS));
            without_provenance_mut(!0)
        }
        #[cfg(feature = "syscall-epoll_create1")]
        libc::SYS_epoll_create1 => {
            let flags = args.arg::<c_int>();
            without_provenance_mut(libc::epoll_create(flags) as isize as usize)
        }
        #[cfg(feature = "syscall-timerfd_create")]
        libc::SYS_timerfd_create => {
            let clockid = args.arg::<c_int>();
            let flags = args.arg::<c_int>();
            without_provenance_mut(libc::timerfd_create(clockid, flags) as isize as usize)
        }
        #[cfg(feature = "syscall-timerfd_settime")]
        libc::SYS_timerfd_settime => {
            let fd = args.arg::<c_int>();
            let flags = args.arg::<c_int>();
            let new_value = args.arg::<*const libc::itimerspec>();
            let old_value = args.arg::<*mut libc::itimerspec>();
            without_provenance_mut(libc::timerfd_settime(fd, flags, new_value, old_value) as isize as usize)
        }
        #[cfg(feature = "syscall-utimensat")]
        libc::SYS_utimensat => {
            let fd = args.arg::<c_int>();
            let path = args.arg::<*const c_char>();
            let times = args.arg::<*const libc::timespec>();
            let flags = args.arg::<c_int>();
            // On Linux, a NULL path means `utimensat` should behave like
            // `futimens`.
            if path.is_null() {
                if flags != 0 {
                    set_errno(Errno(libc::EINVAL));
                    without_provenance_mut(-1 as isize as usize)
                } else {
                    without_provenance_mut(libc::futimens(fd, times) as isize as usize)
                }
            } else {
                without_provenance_mut(libc::utimensat(fd, path, times, flags) as isize as usize)
            }
        }
        #[cfg(feature = "syscall-fdatasync")]
        libc::SYS_fdatasync => {
            let fd = args.arg::<c_int>();
            without_provenance_mut(libc::fdatasync(fd) as isize as usize)
        }
        #[cfg(feature = "syscall-syncfs")]
        libc::SYS_syncfs => {
            let fd = args.arg::<c_int>();
            without_provenance_mut(libc::syncfs(fd) as isize as usize)
        }
        #[cfg(feature = "syscall-sync")]
        libc::SYS_sync => {
            libc::sync();
            without_provenance_mut(0)
        }
        #[cfg(feature = "syscall-pipe2")]
        libc::SYS_pipe2 => {
            let pipefd = args.arg::<*mut c_int>();
            let flags = args.arg::<c_int>();
            without_provenance_mut(libc::pipe2(pipefd, flags) as isize as usize)
        }
        _ => unimplemented!(
            "syscall({:?}); maybe try enabling the \"extra-syscalls\" feature",
            number
        ),
    }
}

#[cfg(feature = "thread")]
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
            timeout.read()
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

#[cfg(test)]
mod tests {
    #[cfg(feature = "syscall-utimensat")]
    #[test]
    fn test_syscall_utimensat() {
        use core::ptr::null_mut;
        use rustix::cstr;
        use rustix::fd::BorrowedFd;
        unsafe {
            let fd = libc::memfd_create(cstr!("test").as_ptr(), 0);
            assert_ne!(fd, -1);
            let times = [
                libc::timespec {
                    tv_sec: 43,
                    tv_nsec: 44,
                },
                libc::timespec {
                    tv_sec: 45,
                    tv_nsec: 46,
                },
            ];
            assert_eq!(
                syscall(libc::SYS_utimensat, fd, null::<u8>(), &times, 0),
                null_mut()
            );
            let stat = rustix::fs::fstat(BorrowedFd::borrow_raw(fd)).unwrap();
            assert_eq!(stat.st_mtime, times[1].tv_sec as _);
            assert_eq!(stat.st_mtime_nsec, times[1].tv_nsec as _);
        }
    }
}
