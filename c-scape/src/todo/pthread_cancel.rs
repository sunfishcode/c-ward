//! Unimplemented pthread cancellation functions.
//!
//! Thread cancellation is not used by Rust code, and more broadly, it adds a
//! lot of complexity to a lot of things without adding being proportionately
//! valuable.

use libc::c_int;

#[no_mangle]
unsafe extern "C" fn pthread_exit() -> c_int {
    todo!("pthread_exit")
}
#[no_mangle]
unsafe extern "C" fn pthread_cancel() -> c_int {
    todo!("pthread_cancel")
}
#[no_mangle]
unsafe extern "C" fn __pthread_register_cancel() {
    todo!("__pthread_register_cancel")
}
#[no_mangle]
unsafe extern "C" fn __pthread_unregister_cancel() {
    todo!("__pthread_unregister_cancel")
}
#[no_mangle]
unsafe extern "C" fn pthread_cleanup_push() -> c_int {
    todo!("pthread_cleanup_push")
}
#[no_mangle]
unsafe extern "C" fn pthread_cleanup_pop() -> c_int {
    todo!("pthread_cleanup_pop")
}
#[no_mangle]
unsafe extern "C" fn pthread_setcancelstate() -> c_int {
    todo!("pthread_setcancelstate")
}
#[no_mangle]
unsafe extern "C" fn pthread_setcanceltype() -> c_int {
    todo!("pthread_setcanceltype")
}
#[no_mangle]
unsafe extern "C" fn pthread_testcancel() -> c_int {
    todo!("pthread_testcancel")
}
