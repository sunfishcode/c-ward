//! Functions which are not implemented and not expected to be.

#[no_mangle]
unsafe extern "C" fn gets() {
    unimplemented!("gets")
}
#[deprecated]
#[no_mangle]
unsafe extern "C" fn sighold() {
    unimplemented!("sighold")
}
#[deprecated]
#[no_mangle]
unsafe extern "C" fn sigignore() {
    unimplemented!("sigignore")
}
#[deprecated]
#[no_mangle]
unsafe extern "C" fn sigrelse() {
    unimplemented!("sigrelse")
}
#[deprecated]
#[no_mangle]
unsafe extern "C" fn sigset() {
    unimplemented!("sigset")
}
#[no_mangle]
unsafe extern "C" fn __xpg_sigpause() {
    unimplemented!("__xpg_sigpause")
}
