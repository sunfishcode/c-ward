use crate::convert_res;
use alloc::vec::Vec;
use errno::{set_errno, Errno};
use libc::c_int;
use rustix::event::{PollFd, PollFlags, Timespec};
use rustix::fd::{AsFd, AsRawFd, BorrowedFd};

#[deprecated]
#[no_mangle]
unsafe extern "C" fn select(
    nfds: c_int,
    readfds: *mut libc::fd_set,
    writefds: *mut libc::fd_set,
    exceptfds: *mut libc::fd_set,
    timeout: *mut libc::timeval,
) -> c_int {
    libc!(libc::select(nfds, readfds, writefds, exceptfds, timeout));

    if nfds < 0 || nfds > libc::FD_SETSIZE as c_int {
        set_errno(Errno(libc::EINVAL));
        return -1;
    };

    let mut poll_fds = Vec::new();

    for fd in 0..nfds {
        let mut events = PollFlags::empty();

        if set_contains(readfds, fd) {
            events |= PollFlags::IN;
            libc::FD_CLR(fd, readfds);
        }
        if set_contains(writefds, fd) {
            events |= PollFlags::OUT;
            libc::FD_CLR(fd, writefds);
        }
        if set_contains(exceptfds, fd) {
            events |= PollFlags::ERR;
            libc::FD_CLR(fd, exceptfds);
        }

        if !events.is_empty() {
            let event = PollFd::from_borrowed_fd(BorrowedFd::borrow_raw(fd), events);
            poll_fds.push(event);
        }
    }

    let timeout = if timeout.is_null() {
        None
    } else {
        Some(Timespec {
            tv_sec: (*timeout).tv_sec.into(),
            tv_nsec: (*timeout).tv_usec * 1000 as rustix::time::Nsecs,
        })
    };
    // TODO: use rustix::event::select
    let res = match convert_res(rustix::event::poll(&mut poll_fds, timeout.as_ref())) {
        Some(res) => res,
        None => return -1,
    };

    for event in &poll_fds[..res] {
        let fd = event.as_fd().as_raw_fd();
        if event.revents().contains(PollFlags::IN) {
            libc::FD_SET(fd, readfds);
        }
        if event.revents().contains(PollFlags::OUT) {
            libc::FD_SET(fd, writefds);
        }
        if event.revents().contains(PollFlags::ERR) {
            libc::FD_SET(fd, exceptfds);
        }
    }
    res as c_int
}

unsafe fn set_contains(fds: *mut libc::fd_set, fd: i32) -> bool {
    if fds.is_null() {
        return false;
    }

    libc::FD_ISSET(fd, fds)
}
