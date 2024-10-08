use rustix_futex_sync::lock_api::RawRwLock as _;
use rustix_futex_sync::RawRwLock;

use core::ptr;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use libc::c_int;

#[allow(non_camel_case_types)]
#[repr(C)]
struct PthreadRwlockT {
    lock: RawRwLock,
    exclusive: AtomicBool,
    pad0: usize,
    pad1: usize,
    pad2: usize,
    pad3: usize,
    pad4: usize,
}
libc_type!(PthreadRwlockT, pthread_rwlock_t);

#[allow(non_camel_case_types)]
#[cfg_attr(
    any(target_env = "musl", target_env = "ohos", target_pointer_width = "32"),
    repr(align(4))
)]
#[cfg_attr(
    all(
        not(target_env = "musl"),
        not(target_env = "ohos"),
        target_pointer_width = "64"
    ),
    repr(align(8))
)]
struct PthreadRwlockattrT {
    kind: AtomicU32,
    _pad0: u32,
}
libc_type!(PthreadRwlockattrT, pthread_rwlockattr_t);

#[no_mangle]
unsafe extern "C" fn pthread_rwlock_init(
    rwlock: *mut PthreadRwlockT,
    rwlockattr: *const PthreadRwlockattrT,
) -> c_int {
    libc!(libc::pthread_rwlock_init(
        checked_cast!(rwlock),
        checked_cast!(rwlockattr)
    ));
    let _ = (*rwlockattr).kind.load(Ordering::SeqCst);
    ptr::write(&mut (*rwlock).lock, RawRwLock::INIT);
    (*rwlock).exclusive.store(false, Ordering::SeqCst);

    0
}

#[no_mangle]
unsafe extern "C" fn pthread_rwlock_destroy(rwlock: *mut PthreadRwlockT) -> c_int {
    libc!(libc::pthread_rwlock_destroy(checked_cast!(rwlock)));
    ptr::drop_in_place(rwlock);
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_rwlock_wrlock(rwlock: *mut PthreadRwlockT) -> c_int {
    libc!(libc::pthread_rwlock_wrlock(checked_cast!(rwlock)));
    (*rwlock).lock.lock_exclusive();
    (*rwlock).exclusive.store(true, Ordering::SeqCst);
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_rwlockattr_init(attr: *mut PthreadRwlockattrT) -> c_int {
    libc!(libc::pthread_rwlockattr_init(checked_cast!(attr)));

    attr.write(PthreadRwlockattrT {
        kind: AtomicU32::new(0),
        _pad0: 0,
    });
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_rwlockattr_destroy(_attr: *mut PthreadRwlockattrT) -> c_int {
    libc!(libc::pthread_rwlockattr_destroy(checked_cast!(_attr)));
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_rwlock_tryrdlock(rwlock: *mut PthreadRwlockT) -> c_int {
    libc!(libc::pthread_rwlock_tryrdlock(checked_cast!(rwlock)));
    let result = (*rwlock).lock.try_lock_shared();
    if result {
        (*rwlock).exclusive.store(false, Ordering::SeqCst);
        0
    } else {
        libc::EBUSY
    }
}

#[no_mangle]
unsafe extern "C" fn pthread_rwlock_trywrlock(rwlock: *mut PthreadRwlockT) -> c_int {
    libc!(libc::pthread_rwlock_trywrlock(checked_cast!(rwlock)));
    let result = (*rwlock).lock.try_lock_exclusive();
    if result {
        (*rwlock).exclusive.store(true, Ordering::SeqCst);
        0
    } else {
        libc::EBUSY
    }
}

#[no_mangle]
unsafe extern "C" fn pthread_rwlock_rdlock(rwlock: *mut PthreadRwlockT) -> c_int {
    libc!(libc::pthread_rwlock_rdlock(checked_cast!(rwlock)));
    (*rwlock).lock.lock_shared();
    (*rwlock).exclusive.store(false, Ordering::SeqCst);
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_rwlock_unlock(rwlock: *mut PthreadRwlockT) -> c_int {
    libc!(libc::pthread_rwlock_unlock(checked_cast!(rwlock)));

    let rwlock = &*rwlock;
    if rwlock.exclusive.load(Ordering::SeqCst) {
        if !rwlock.lock.is_locked_exclusive() {
            return libc::EPERM;
        }
        rwlock.lock.unlock_exclusive();
    } else {
        if !rwlock.lock.is_locked() {
            return libc::EPERM;
        }
        rwlock.lock.unlock_shared();
    }
    0
}
