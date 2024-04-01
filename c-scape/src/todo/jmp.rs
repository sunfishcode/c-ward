//! Support for `setjmp`/`longjmp` and `setcontext`/`getcontext`/etc.
//!
//! Rust in general doesn't support `setjmp`/`longjmp` or
//! `setcontext`/`getcontext`/etc.

#[no_mangle]
unsafe extern "C" fn __longjmp_chk() {
    todo!("__longjmp_chk")
}
#[no_mangle]
unsafe extern "C" fn getcontext() {
    todo!("getcontext")
}
#[no_mangle]
unsafe extern "C" fn setcontext() {
    todo!("setcontext")
}
#[no_mangle]
unsafe extern "C" fn makecontext() {
    todo!("makecontext")
}
#[no_mangle]
unsafe extern "C" fn swapcontext() {
    todo!("swapcontext")
}
