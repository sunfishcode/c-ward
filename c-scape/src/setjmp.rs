use libc::{c_int, c_void};

// Placeholder types while we don't yet support `longjmp`.
#[allow(non_camel_case_types)]
type jmp_buf = *const c_void;
#[allow(non_camel_case_types)]
type sigjmp_buf = *const c_void;

#[no_mangle]
unsafe extern "C" fn setjmp(_env: jmp_buf) -> c_int {
    //libc!(libc::setjmp(env));

    // We don't support `longjmp` yet, so just do the first-time return of 0.
    0
}

#[no_mangle]
unsafe extern "C" fn _setjmp(env: jmp_buf) -> c_int {
    //libc!(libc::_setjmp(jmp_buf));

    setjmp(env)
}

#[no_mangle]
unsafe extern "C" fn sigsetjmp(_env: sigjmp_buf, _savesigs: c_int) -> c_int {
    //libc!(libc::sigsetjmp(env, savesigs));

    // As in `setjmp`, just do the first-time return.
    0
}

#[no_mangle]
unsafe extern "C" fn __sigsetjmp(env: sigjmp_buf, savesigs: c_int) -> c_int {
    //libc!(libc::__sigsetjmp(env, savesigs));

    sigsetjmp(env, savesigs)
}
