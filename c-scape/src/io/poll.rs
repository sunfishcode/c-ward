use core::slice;
use libc::c_int;

use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn poll(fds: *mut libc::pollfd, nfds: libc::nfds_t, timeout: c_int) -> c_int {
    libc!(libc::poll(fds, nfds, timeout));

    let pollfds: *mut rustix::event::PollFd<'_> = checked_cast!(fds);

    let fds = slice::from_raw_parts_mut(pollfds, nfds.try_into().unwrap());
    let timeout = if timeout < 0 {
        None
    } else {
        Some(rustix::event::Timespec {
            tv_sec: i64::from(timeout) / 1000,
            tv_nsec: (i64::from(timeout) % 1000) * 1_000_000,
        })
    };
    match convert_res(rustix::event::poll(fds, timeout.as_ref())) {
        Some(num) => num.try_into().unwrap(),
        None => -1,
    }
}
