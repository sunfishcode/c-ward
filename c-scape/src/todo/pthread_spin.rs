//! Unimplemented pthread spin-lock functions.
//!
//! Spin locks are not widely used and are difficult to use effectively. Most
//! programs should use regular locks.

#[no_mangle]
unsafe extern "C" fn pthread_spin_destroy() {
    todo!("pthread_spin_destroy")
}
#[no_mangle]
unsafe extern "C" fn pthread_spin_init() {
    todo!("pthread_spin_init")
}
#[no_mangle]
unsafe extern "C" fn pthread_spin_lock() {
    todo!("pthread_spin_lock")
}
#[no_mangle]
unsafe extern "C" fn pthread_spin_trylock() {
    todo!("pthread_spin_trylock")
}
#[no_mangle]
unsafe extern "C" fn pthread_spin_unlock() {
    todo!("pthread_spin_unlock")
}
