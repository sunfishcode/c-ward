#[cfg(not(target_os = "wasi"))]
mod dup;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod epoll;
mod isatty;
mod pipe;
mod poll;
mod read;
mod select;
mod splice;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod timerfd;
mod write;

use rustix::event::EventfdFlags;
use rustix::fd::{BorrowedFd, IntoRawFd};

use libc::{c_int, c_long, c_uint, c_void};

use crate::convert_res;

/// Catch-all for ioctls we don't special-case below. There's no way to
/// enumerate every driver's private opcodes (DRM/GPU and friends invent their
/// own), so we just hand the opcode + arg to the kernel untouched.
struct GenericIoctl {
    opcode: rustix::ioctl::Opcode,
    arg: *mut c_void,
}

// SAFETY: opcode and arg are passed through untouched, same as the libc
// `ioctl` we're standing in for, so getting them right is on the caller. We
// mark it mutating since we can't know whether the ioctl writes back - that's
// the safe assumption.
unsafe impl rustix::ioctl::Ioctl for GenericIoctl {
    type Output = rustix::ioctl::IoctlOutput;

    const IS_MUTATING: bool = true;

    fn opcode(&self) -> rustix::ioctl::Opcode {
        self.opcode
    }

    fn as_ptr(&mut self) -> *mut c_void {
        self.arg
    }

    unsafe fn output_from_ptr(
        out: rustix::ioctl::IoctlOutput,
        _extract_output: *mut c_void,
    ) -> rustix::io::Result<rustix::ioctl::IoctlOutput> {
        Ok(out)
    }
}

#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn ioctl(fd: c_int, request: c_long, mut args: ...) -> c_int {
    const TCGETS: c_long = libc::TCGETS as c_long;
    const FIONBIO: c_long = libc::FIONBIO as c_long;
    const TIOCINQ: c_long = libc::TIOCINQ as c_long;
    const TIOCGWINSZ: c_long = libc::TIOCGWINSZ as c_long;
    const FICLONE: c_long = libc::FICLONE as c_long;
    match request {
        TCGETS => {
            libc!(libc::ioctl(fd, libc::TCGETS));
            let fd = BorrowedFd::borrow_raw(fd);
            match convert_res(rustix::termios::tcgetattr(fd)) {
                Some(x) => {
                    args.next_arg::<*mut rustix::termios::Termios>().write(x);
                    0
                }
                None => -1,
            }
        }
        FIONBIO | TIOCINQ => {
            let ptr = args.next_arg::<*mut c_int>();
            let value = *ptr != 0;
            libc!(libc::ioctl(fd, libc::FIONBIO, value as c_int));
            let fd = BorrowedFd::borrow_raw(fd);
            match convert_res(rustix::io::ioctl_fionbio(fd, value)) {
                Some(()) => 0,
                None => -1,
            }
        }
        TIOCGWINSZ => {
            libc!(libc::ioctl(fd, libc::TIOCGWINSZ));
            let fd = BorrowedFd::borrow_raw(fd);
            match convert_res(rustix::termios::tcgetwinsize(fd)) {
                Some(size) => {
                    let size = libc::winsize {
                        ws_row: size.ws_row,
                        ws_col: size.ws_col,
                        ws_xpixel: size.ws_xpixel,
                        ws_ypixel: size.ws_ypixel,
                    };
                    args.next_arg::<*mut libc::winsize>().write(size);
                    0
                }
                None => -1,
            }
        }
        FICLONE => {
            let src_fd = args.next_arg::<c_int>();
            libc!(libc::ioctl(fd, libc::FICLONE as _, src_fd));
            let fd = BorrowedFd::borrow_raw(fd);
            let src_fd = BorrowedFd::borrow_raw(src_fd);
            match convert_res(rustix::fs::ioctl_ficlone(fd, src_fd)) {
                Some(()) => 0,
                None => -1,
            }
        }
        _ => {
            // Nothing we translate above matched, so let the kernel deal with
            // it. Driver-specific ioctls (DRM/GPU etc.) end up here.
            let arg = args.next_arg::<*mut c_void>();
            let fd = BorrowedFd::borrow_raw(fd);
            let generic = GenericIoctl {
                opcode: request as rustix::ioctl::Opcode,
                arg,
            };
            match convert_res(rustix::ioctl::ioctl(fd, generic)) {
                Some(out) => out,
                None => -1,
            }
        }
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[no_mangle]
unsafe extern "C" fn eventfd(initval: c_uint, flags: c_int) -> c_int {
    libc!(libc::eventfd(initval, flags));
    let flags = EventfdFlags::from_bits(flags.try_into().unwrap()).unwrap();
    match convert_res(rustix::event::eventfd(initval, flags)) {
        Some(fd) => fd.into_raw_fd(),
        None => -1,
    }
}
