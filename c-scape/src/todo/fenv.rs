//! Support for dynamic floating-point rounding modes and floating-point
//! exception flags.
//!
//! Rust in general doesn't support dynamic rounding modes or floating-point
//! expecting handling.

#[no_mangle]
unsafe extern "C" fn fegetenv() {
    todo!("fegetenv")
}
#[no_mangle]
unsafe extern "C" fn fesetenv() {
    todo!("fesetenv")
}
#[no_mangle]
unsafe extern "C" fn fegetround() {
    todo!("fegetround")
}
#[no_mangle]
unsafe extern "C" fn fesetround() {
    todo!("fesetround")
}
#[no_mangle]
unsafe extern "C" fn feclearexcept() {
    todo!("feclearexcept")
}
#[no_mangle]
unsafe extern "C" fn feraiseexcept() {
    todo!("feraiseexcept")
}
#[no_mangle]
unsafe extern "C" fn fetestexcept() {
    todo!("fetestexcept")
}
#[no_mangle]
unsafe extern "C" fn fegetexceptflag() {
    todo!("fegetexceptflag")
}
#[no_mangle]
unsafe extern "C" fn fesetexceptflag() {
    todo!("fesetexceptflag")
}
#[no_mangle]
unsafe extern "C" fn feholdexcept() {
    todo!("feholdexcept")
}
#[no_mangle]
unsafe extern "C" fn feupdateenv() {
    todo!("feupdateenv")
}
