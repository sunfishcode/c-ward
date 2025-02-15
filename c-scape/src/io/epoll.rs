use crate::convert_res;
use alloc::vec::Vec;
use errno::{set_errno, Errno};
use libc::c_int;
use rustix::buffer::spare_capacity;
use rustix::event::epoll::{add, delete, modify, CreateFlags, Event, EventData, EventFlags};
use rustix::fd::{BorrowedFd, IntoRawFd};

#[no_mangle]
unsafe extern "C" fn epoll_create(size: c_int) -> c_int {
    libc!(libc::epoll_create(size));

    if size <= 0 {
        set_errno(Errno(libc::EINVAL));
        return -1;
    }

    epoll_create1(0)
}

#[no_mangle]
unsafe extern "C" fn epoll_create1(flags: c_int) -> c_int {
    libc!(libc::epoll_create1(flags));

    let flags = CreateFlags::from_bits(flags as _).unwrap();

    match convert_res(rustix::event::epoll::create(flags)) {
        Some(epoll) => epoll.into_raw_fd(),
        None => -1,
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[no_mangle]
unsafe extern "C" fn epoll_ctl(
    epfd: c_int,
    op: c_int,
    fd: c_int,
    event: *mut libc::epoll_event,
) -> c_int {
    libc!(libc::epoll_ctl(epfd, op, fd, event));

    let epfd = BorrowedFd::borrow_raw(epfd);
    let fd = BorrowedFd::borrow_raw(fd);
    let res = match op {
        libc::EPOLL_CTL_ADD => {
            let libc::epoll_event { events, r#u64 } = event.read();
            let events = EventFlags::from_bits(events).unwrap();
            add(epfd, fd, EventData::new_u64(r#u64), events)
        }
        libc::EPOLL_CTL_MOD => {
            let libc::epoll_event { events, r#u64 } = event.read();
            let events = EventFlags::from_bits(events).unwrap();
            modify(epfd, fd, EventData::new_u64(r#u64), events)
        }
        libc::EPOLL_CTL_DEL => delete(epfd, fd),
        _ => {
            set_errno(Errno(libc::EINVAL));
            return -1;
        }
    };

    match convert_res(res) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn epoll_wait(
    epfd: c_int,
    events: *mut libc::epoll_event,
    maxevents: c_int,
    timeout: c_int,
) -> c_int {
    libc!(libc::epoll_wait(epfd, events, maxevents, timeout));

    if maxevents <= 0 {
        set_errno(Errno(libc::EINVAL));
        return -1;
    }

    let timeout = if timeout < 0 {
        None
    } else {
        Some(rustix::event::Timespec {
            tv_sec: i64::from(timeout) / 1000,
            tv_nsec: (i64::from(timeout) % 1000) * 1_000_000,
        })
    };

    // TODO: We should use `Vec::from_raw_parts` to allow `epoll_wait`
    // to write events directly into the user's buffer, rather then allocating
    // and copying here.
    let mut events_vec = Vec::with_capacity(maxevents as usize);
    match convert_res(rustix::event::epoll::wait(
        BorrowedFd::borrow_raw(epfd),
        spare_capacity(&mut events_vec),
        timeout.as_ref(),
    )) {
        Some(_) => {
            let mut events = events;
            for Event { flags, data } in events_vec.iter().copied() {
                events.write(libc::epoll_event {
                    events: flags.bits(),
                    r#u64: data.u64(),
                });
                events = events.add(1);
            }
            events_vec.len() as c_int
        }
        None => -1,
    }
}
