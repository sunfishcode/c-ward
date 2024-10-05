use errno::{set_errno, Errno};
use libc::c_int;

#[no_mangle]
unsafe extern "C" fn pidfd_getpid(fd: c_int) -> c_int {
    //libc!(libc::pidfd_getpid(fd));

    // ensure std::process uses fork as fallback code on linux
    set_errno(Errno(libc::ENOSYS));
    -1
}

#[no_mangle]
unsafe extern "C" fn pidfd_spawnp(
    pid: *mut libc::c_int,
    path: *const libc::c_char,
    file_actions: *const libc::posix_spawn_file_actions_t,
    attrp: *const libc::posix_spawnattr_t,
    argv: *const *mut libc::c_char,
    envp: *const *mut libc::c_char,
) -> libc::c_int {
    //libc!(libc::pidfd_spawnp(
    //    pid,
    //    path,
    //    file_actions,
    //    attrp,
    //    argv,
    //    envp
    //));

    // ensure std::process uses fork as fallback code on linux
    set_errno(Errno(libc::ENOSYS));
    -1
}
