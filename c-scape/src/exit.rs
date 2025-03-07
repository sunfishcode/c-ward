use alloc::boxed::Box;
use libc::{c_int, c_void};

/// Register a function to be called when `exit` is called.
///
/// This function conforms to the [LSB `__cxa_atexit`] ABI.
///
/// [LSB `__cxa_atexit`]: https://refspecs.linuxbase.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/baselib---cxa-atexit.html
#[no_mangle]
unsafe extern "C" fn __cxa_atexit(
    func: unsafe extern "C" fn(*mut c_void),
    arg: *mut c_void,
    _dso: *mut c_void,
) -> c_int {
    // Tell Rust it's ok to send `arg` across threads even though it's a
    // `*mut c_void`.
    struct SendSync(*mut c_void);
    unsafe impl Send for SendSync {}
    unsafe impl Sync for SendSync {}
    let arg = SendSync(arg);

    origin::program::at_exit(Box::new(move || {
        let arg: SendSync = arg;
        func(arg.0);
    }));

    0
}

#[no_mangle]
unsafe extern "C" fn __cxa_finalize(_d: *mut c_void) {}

/// C-compatible `atexit`. Consider using `__cxa_atexit` instead.
#[no_mangle]
unsafe extern "C" fn atexit(func: extern "C" fn()) -> c_int {
    libc!(libc::atexit(func));
    origin::program::at_exit(Box::new(move || func()));
    0
}

/// C-compatible `exit`.
///
/// Call all the registered at-exit functions, and exit the program.
#[no_mangle]
unsafe extern "C" fn exit(status: c_int) -> ! {
    libc!(libc::exit(status));
    origin::program::exit(status)
}

/// C-compatible `_Exit`.
///
/// Just exit the process.
#[no_mangle]
unsafe extern "C" fn _Exit(status: c_int) -> ! {
    //libc!(libc::_Exit(status));
    origin::program::immediate_exit(status)
}

/// POSIX-compatible `_exit`.
///
/// Just exit the process.
#[no_mangle]
unsafe extern "C" fn _exit(status: c_int) -> ! {
    libc!(libc::_exit(status));
    _Exit(status)
}

#[cold]
#[no_mangle]
unsafe extern "C" fn __stack_chk_fail() -> ! {
    let message = b"*** stack smashing detected ***";
    let _ = libc::write(libc::STDERR_FILENO, message.as_ptr().cast(), message.len());
    libc::abort()
}

// If we were building a dynamic library, this function could have a
// restricted visibility.
#[cold]
#[no_mangle]
unsafe extern "C" fn __stack_chk_fail_local() -> ! {
    __stack_chk_fail()
}
