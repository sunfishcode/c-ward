//! Linux's system calls for these functions only set the IDs for one thread,
//! and we need to set the IDs for all threads in a process.
//!
//! This would typically entail taking a lock that prevents thread creation,
//! setting the IDs for each thread manually by sending signals to them and
//! having signal handlers that perform the set operation, waiting for all
//! the handlers to run, and then releasing the lock.

#[no_mangle]
unsafe extern "C" fn seteuid() {
    todo!("seteuid")
}
#[no_mangle]
unsafe extern "C" fn setegid() {
    todo!("setegid")
}
#[no_mangle]
unsafe extern "C" fn setreuid() {
    todo!("setreuid")
}
#[no_mangle]
unsafe extern "C" fn setregid() {
    todo!("setregid")
}
#[no_mangle]
unsafe extern "C" fn setresuid() {
    todo!("setresuid")
}
#[no_mangle]
unsafe extern "C" fn setresgid() {
    todo!("setresgid")
}
