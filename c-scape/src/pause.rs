use errno::{set_errno, Errno};
use libc::c_int;

#[no_mangle]
unsafe extern "C" fn pause() -> c_int {
    libc!(libc::pause());

    rustix::event::pause();

    // `pause` sleeps until it is interrupted by a signal, so it always fails
    // with `EINTR`.
    set_errno(Errno(libc::EINTR));
    -1
}
