use core::sync::atomic::{AtomicU32, Ordering};

use libc::c_int;

type PthreadSpinlockT = AtomicU32;
libc_type!(PthreadSpinlockT, pthread_spinlock_t);

#[no_mangle]
unsafe extern "C" fn pthread_spin_destroy(lock: *mut PthreadSpinlockT) -> c_int {
    libc!(libc::pthread_spin_destroy(checked_cast!(lock)));
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_spin_init(lock: *mut PthreadSpinlockT, pshared: c_int) -> c_int {
    libc!(libc::pthread_spin_init(checked_cast!(lock), pshared));

    let lock = &*lock;

    lock.store(0, Ordering::Release);
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_spin_lock(lock: *mut PthreadSpinlockT) -> c_int {
    libc!(libc::pthread_spin_lock(checked_cast!(lock)));

    let lock = &*lock;

    while lock.swap(1, Ordering::Acquire) == 1 {
        core::hint::spin_loop();
    }

    0
}
#[no_mangle]
unsafe extern "C" fn pthread_spin_trylock(lock: *mut PthreadSpinlockT) -> c_int {
    libc!(libc::pthread_spin_trylock(checked_cast!(lock)));

    let lock = &*lock;

    if lock.swap(1, Ordering::Acquire) == 1 {
        libc::EBUSY
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "C" fn pthread_spin_unlock(lock: *mut PthreadSpinlockT) -> c_int {
    libc!(libc::pthread_spin_unlock(checked_cast!(lock)));

    let lock = &*lock;

    lock.store(0, Ordering::Release);
    0
}
