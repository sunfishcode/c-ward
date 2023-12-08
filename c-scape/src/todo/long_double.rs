//! Rust doesn't support ABI-compatible long double yet.

#[no_mangle]
unsafe extern "C" fn scalbnl() {
    todo!("scalbnl")
}
#[no_mangle]
unsafe extern "C" fn copysignl() {
    todo!("copysignl")
}
