use libc::c_int;
use rustix::cstr;
use rustix::fd::{AsRawFd, FromRawFd, OwnedFd};

#[no_mangle]
unsafe extern "C" fn daemon(nochdir: c_int, noclose: c_int) -> c_int {
    libc!(libc::daemon(nochdir, noclose));

    let nochdir = nochdir != 0;
    let noclose = noclose != 0;

    if !nochdir {
        if libc::chdir(cstr!("/").as_ptr()) != 0 {
            return -1;
        }
    }

    if !noclose {
        let dev_null = libc::open(cstr!("/dev/null").as_ptr(), libc::O_RDWR);
        if dev_null < 0 {
            return -1;
        }
        let dev_null = OwnedFd::from_raw_fd(dev_null);

        if libc::dup2(dev_null.as_raw_fd(), libc::STDIN_FILENO) < 0
            || libc::dup2(dev_null.as_raw_fd(), libc::STDOUT_FILENO) < 0
            || libc::dup2(dev_null.as_raw_fd(), libc::STDERR_FILENO) < 0
        {
            return -1;
        }
    }

    match libc::fork() {
        -1 => -1,
        0 => libc::setsid(),
        _pid => libc::_exit(0),
    }
}
