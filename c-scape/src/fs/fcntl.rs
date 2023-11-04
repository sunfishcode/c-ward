use core::ffi::VaList;
use errno::{set_errno, Errno};
use rustix::fd::{BorrowedFd, IntoRawFd};
use rustix::fs::{FdFlags, FlockOperation, OFlags};

use libc::c_int;

use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn fcntl(fd: c_int, cmd: c_int, mut args: ...) -> c_int {
    let args = args.as_va_list();
    _fcntl::<libc::flock>(fd, cmd, args)
}

#[no_mangle]
unsafe extern "C" fn fcntl64(fd: c_int, cmd: c_int, mut args: ...) -> c_int {
    let args = args.as_va_list();
    _fcntl::<libc::flock64>(fd, cmd, args)
}

unsafe fn _fcntl<FlockTy: Flock>(fd: c_int, cmd: c_int, mut args: VaList) -> c_int {
    match cmd {
        libc::F_GETFL => {
            libc!(libc::fcntl(fd, libc::F_GETFL));
            let fd = BorrowedFd::borrow_raw(fd);
            match convert_res(rustix::fs::fcntl_getfl(fd)) {
                Some(flags) => flags.bits() as _,
                None => -1,
            }
        }
        libc::F_SETFL => {
            let flags = args.arg::<c_int>();
            libc!(libc::fcntl(fd, libc::F_SETFL, flags));
            let fd = BorrowedFd::borrow_raw(fd);
            match convert_res(rustix::fs::fcntl_setfl(
                fd,
                OFlags::from_bits(flags as _).unwrap(),
            )) {
                Some(()) => 0,
                None => -1,
            }
        }
        libc::F_GETFD => {
            libc!(libc::fcntl(fd, libc::F_GETFD));
            let fd = BorrowedFd::borrow_raw(fd);
            match convert_res(rustix::fs::fcntl_getfd(fd)) {
                Some(flags) => flags.bits() as _,
                None => -1,
            }
        }
        libc::F_SETFD => {
            let flags = args.arg::<c_int>();
            libc!(libc::fcntl(fd, libc::F_SETFD, flags));
            let fd = BorrowedFd::borrow_raw(fd);
            match convert_res(rustix::fs::fcntl_setfd(
                fd,
                FdFlags::from_bits(flags as _).unwrap(),
            )) {
                Some(()) => 0,
                None => -1,
            }
        }
        libc::F_SETLK | libc::F_SETLKW => {
            let ptr = args.arg::<*mut FlockTy>();
            libc!(libc::fcntl(fd, cmd, ptr));
            let fd = BorrowedFd::borrow_raw(fd);
            let is_blocking = cmd == libc::F_SETLKW;
            let flock = &mut *ptr;
            let op = match (flock.l_type() as _, is_blocking) {
                (libc::F_RDLCK, true) => FlockOperation::LockShared,
                (libc::F_WRLCK, true) => FlockOperation::LockExclusive,
                (libc::F_UNLCK, true) => FlockOperation::Unlock,
                (libc::F_RDLCK, false) => FlockOperation::NonBlockingLockShared,
                (libc::F_WRLCK, false) => FlockOperation::NonBlockingLockExclusive,
                (libc::F_UNLCK, false) => FlockOperation::NonBlockingUnlock,
                _ => {
                    set_errno(Errno(libc::EINVAL));
                    return -1;
                }
            };
            // We currently only support whole-file locks.
            assert_eq!(flock.l_whence(), libc::SEEK_SET as _);
            assert_eq!(flock.l_start(), 0);
            assert_eq!(flock.l_len(), 0);
            match convert_res(rustix::fs::fcntl_lock(fd, op)) {
                Some(()) => {
                    flock.l_pid(-1);
                    0
                }
                None => -1,
            }
        }
        #[cfg(not(target_os = "wasi"))]
        libc::F_DUPFD_CLOEXEC => {
            let arg = args.arg::<c_int>();
            libc!(libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, arg));
            let fd = BorrowedFd::borrow_raw(fd);
            match convert_res(rustix::fs::fcntl_dupfd_cloexec(fd, arg)) {
                Some(fd) => fd.into_raw_fd(),
                None => -1,
            }
        }
        _ => todo!("unimplemented fnctl({})", cmd),
    }
}

trait Flock {
    fn l_type(&self) -> i16;
    fn l_whence(&self) -> i16;
    fn l_start(&self) -> libc::off64_t;
    fn l_len(&self) -> libc::off64_t;
    fn l_pid(&mut self, pid: libc::pid_t);
}

impl Flock for libc::flock {
    fn l_type(&self) -> i16 {
        self.l_type
    }

    fn l_whence(&self) -> i16 {
        self.l_whence
    }

    fn l_start(&self) -> libc::off64_t {
        self.l_start.into()
    }

    fn l_len(&self) -> libc::off64_t {
        self.l_len.into()
    }

    fn l_pid(&mut self, pid: libc::pid_t) {
        self.l_pid = pid;
    }
}

impl Flock for libc::flock64 {
    fn l_type(&self) -> i16 {
        self.l_type
    }

    fn l_whence(&self) -> i16 {
        self.l_whence
    }

    fn l_start(&self) -> libc::off64_t {
        self.l_start
    }

    fn l_len(&self) -> libc::off64_t {
        self.l_len
    }

    fn l_pid(&mut self, pid: libc::pid_t) {
        self.l_pid = pid;
    }
}
