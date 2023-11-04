#[cfg(feature = "todo")]
#[no_mangle]
unsafe extern "C" fn siglongjmp() {
    //libc!(libc::siglongjmp());
    todo!("siglongjmp")
}

#[cfg(feature = "todo")]
#[no_mangle]
unsafe extern "C" fn __sigsetjmp() {
    //libc!(libc::__sigsetjmp());
    todo!("__sigsetjmp")
}
