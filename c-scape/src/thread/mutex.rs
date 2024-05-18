use rustix_futex_sync::lock_api::{RawMutex as _, RawReentrantMutex};
use rustix_futex_sync::{RawCondvar, RawMutex};

use core::mem::{size_of, ManuallyDrop};
use core::ptr;
use core::sync::atomic::{AtomicU32, Ordering};
use core::time::Duration;
use libc::c_int;

use crate::GetThreadId;

#[allow(non_camel_case_types)]
#[repr(C)]
union pthread_mutex_u {
    normal: ManuallyDrop<RawMutex>,
    reentrant: ManuallyDrop<RawReentrantMutex<RawMutex, GetThreadId>>,
}

#[allow(non_camel_case_types)]
#[repr(C)]
struct PthreadMutexT {
    kind: AtomicU32,
    u: pthread_mutex_u,
    pad0: usize,
    #[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
    pad1: usize,
}
libc_type!(PthreadMutexT, pthread_mutex_t);

#[allow(non_camel_case_types)]
#[repr(C)]
struct PthreadMutexattrT {
    kind: AtomicU32,
    #[cfg(target_arch = "aarch64")]
    pad0: u32,
}
libc_type!(PthreadMutexattrT, pthread_mutexattr_t);

#[no_mangle]
unsafe extern "C" fn pthread_mutexattr_destroy(attr: *mut PthreadMutexattrT) -> c_int {
    libc!(libc::pthread_mutexattr_destroy(checked_cast!(attr)));
    ptr::drop_in_place(attr);
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_mutexattr_init(attr: *mut PthreadMutexattrT) -> c_int {
    libc!(libc::pthread_mutexattr_init(checked_cast!(attr)));
    ptr::write(
        attr,
        PthreadMutexattrT {
            kind: AtomicU32::new(libc::PTHREAD_MUTEX_DEFAULT as u32),
            #[cfg(target_arch = "aarch64")]
            pad0: 0,
        },
    );
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_mutexattr_settype(attr: *mut PthreadMutexattrT, kind: c_int) -> c_int {
    libc!(libc::pthread_mutexattr_settype(checked_cast!(attr), kind));

    match kind {
        libc::PTHREAD_MUTEX_NORMAL
        | libc::PTHREAD_MUTEX_ERRORCHECK
        | libc::PTHREAD_MUTEX_RECURSIVE => {}
        _ => return libc::EINVAL,
    }

    (*attr).kind.store(kind as u32, Ordering::SeqCst);
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_mutexattr_gettype(
    attr: *mut PthreadMutexattrT,
    kind: *mut c_int,
) -> c_int {
    //libc!(libc::pthread_mutexattr_gettype(checked_cast!(attr), kind));

    *kind = (*attr).kind.load(Ordering::SeqCst) as c_int;
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_mutex_destroy(mutex: *mut PthreadMutexT) -> c_int {
    libc!(libc::pthread_mutex_destroy(checked_cast!(mutex)));
    match (*mutex).kind.load(Ordering::SeqCst) as i32 {
        libc::PTHREAD_MUTEX_NORMAL => ManuallyDrop::drop(&mut (*mutex).u.normal),
        libc::PTHREAD_MUTEX_RECURSIVE => ManuallyDrop::drop(&mut (*mutex).u.reentrant),
        libc::PTHREAD_MUTEX_ERRORCHECK => todo!("PTHREAD_MUTEX_ERRORCHECK"),
        other => unimplemented!("unsupported pthread mutex kind {}", other),
    }
    (*mutex).kind.store(!0, Ordering::SeqCst);
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_mutex_init(
    mutex: *mut PthreadMutexT,
    mutexattr: *const PthreadMutexattrT,
) -> c_int {
    libc!(libc::pthread_mutex_init(
        checked_cast!(mutex),
        checked_cast!(mutexattr)
    ));
    let kind = if mutexattr.is_null() {
        libc::PTHREAD_MUTEX_DEFAULT as u32
    } else {
        (*mutexattr).kind.load(Ordering::SeqCst)
    };

    match kind as i32 {
        libc::PTHREAD_MUTEX_NORMAL => {
            ptr::write(&mut (*mutex).u.normal, ManuallyDrop::new(RawMutex::INIT))
        }
        libc::PTHREAD_MUTEX_RECURSIVE => ptr::write(
            &mut (*mutex).u.reentrant,
            ManuallyDrop::new(RawReentrantMutex::INIT),
        ),
        libc::PTHREAD_MUTEX_ERRORCHECK => todo!("PTHREAD_MUTEX_ERRORCHECK"),
        other => unimplemented!("unsupported pthread mutex kind {}", other),
    }
    (*mutex).kind.store(kind, Ordering::SeqCst);
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_mutex_lock(mutex: *mut PthreadMutexT) -> c_int {
    libc!(libc::pthread_mutex_lock(checked_cast!(mutex)));
    match (*mutex).kind.load(Ordering::SeqCst) as i32 {
        libc::PTHREAD_MUTEX_NORMAL => (*mutex).u.normal.lock(),
        libc::PTHREAD_MUTEX_RECURSIVE => (*mutex).u.reentrant.lock(),
        libc::PTHREAD_MUTEX_ERRORCHECK => todo!("PTHREAD_MUTEX_ERRORCHECK"),
        other => unimplemented!("unsupported pthread mutex kind {}", other),
    }
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_mutex_trylock(mutex: *mut PthreadMutexT) -> c_int {
    libc!(libc::pthread_mutex_trylock(checked_cast!(mutex)));
    if match (*mutex).kind.load(Ordering::SeqCst) as i32 {
        libc::PTHREAD_MUTEX_NORMAL => (*mutex).u.normal.try_lock(),
        libc::PTHREAD_MUTEX_RECURSIVE => (*mutex).u.reentrant.try_lock(),
        libc::PTHREAD_MUTEX_ERRORCHECK => todo!("PTHREAD_MUTEX_ERRORCHECK"),
        other => unimplemented!("unsupported pthread mutex kind {}", other),
    } {
        0
    } else {
        libc::EBUSY
    }
}

#[no_mangle]
unsafe extern "C" fn pthread_mutex_unlock(mutex: *mut PthreadMutexT) -> c_int {
    libc!(libc::pthread_mutex_unlock(checked_cast!(mutex)));

    let mutex = &*mutex;
    match mutex.kind.load(Ordering::SeqCst) as i32 {
        libc::PTHREAD_MUTEX_NORMAL => {
            if !mutex.u.normal.is_locked() {
                return libc::EPERM;
            }
            mutex.u.normal.unlock()
        }
        libc::PTHREAD_MUTEX_RECURSIVE => {
            if !mutex.u.reentrant.is_locked() {
                return libc::EPERM;
            }
            mutex.u.reentrant.unlock()
        }
        libc::PTHREAD_MUTEX_ERRORCHECK => todo!("PTHREAD_MUTEX_ERRORCHECK"),
        other => unimplemented!("unsupported pthread mutex kind {}", other),
    }
    0
}

const SIZEOF_PTHREAD_COND_T: usize = 48;

#[cfg_attr(target_arch = "x86", repr(C, align(4)))]
#[cfg_attr(not(target_arch = "x86"), repr(C, align(8)))]
struct PthreadCondT {
    inner: RawCondvar,
    attr: PthreadCondattrT,
    pad: [u8; SIZEOF_PTHREAD_COND_T - size_of::<RawCondvar>() - size_of::<PthreadCondattrT>()],
}

libc_type!(PthreadCondT, pthread_cond_t);

#[cfg(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "arm",
    all(target_arch = "aarch64", target_pointer_width = "32"),
    target_arch = "riscv64",
))]
const SIZEOF_PTHREAD_CONDATTR_T: usize = 4;

#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
const SIZEOF_PTHREAD_CONDATTR_T: usize = 8;

#[repr(C, align(4))]
struct PthreadCondattrT {
    pad: [u8; SIZEOF_PTHREAD_CONDATTR_T],
}

impl Default for PthreadCondattrT {
    fn default() -> Self {
        Self {
            pad: [0_u8; SIZEOF_PTHREAD_CONDATTR_T],
        }
    }
}

libc_type!(PthreadCondattrT, pthread_condattr_t);

#[no_mangle]
unsafe extern "C" fn pthread_condattr_destroy(attr: *mut PthreadCondattrT) -> c_int {
    libc!(libc::pthread_condattr_destroy(checked_cast!(attr)));
    ptr::drop_in_place(attr);
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_condattr_init(attr: *mut PthreadCondattrT) -> c_int {
    libc!(libc::pthread_condattr_init(checked_cast!(attr)));
    ptr::write(attr, PthreadCondattrT::default());
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_condattr_setclock(
    attr: *mut PthreadCondattrT,
    clock_id: c_int,
) -> c_int {
    libc!(libc::pthread_condattr_setclock(
        checked_cast!(attr),
        clock_id
    ));
    let _ = attr;

    if clock_id == libc::CLOCK_PROCESS_CPUTIME_ID || clock_id == libc::CLOCK_THREAD_CPUTIME_ID {
        return libc::EINVAL;
    }

    rustix::io::write(
        rustix::stdio::stderr(),
        b"unimplemented: pthread_condattr_setclock\n",
    )
    .ok();
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_cond_broadcast(cond: *mut PthreadCondT) -> c_int {
    libc!(libc::pthread_cond_broadcast(checked_cast!(cond)));
    (*cond).inner.notify_all();
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_cond_destroy(cond: *mut PthreadCondT) -> c_int {
    libc!(libc::pthread_cond_destroy(checked_cast!(cond)));
    ptr::drop_in_place(cond);
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_cond_init(
    cond: *mut PthreadCondT,
    attr: *const PthreadCondattrT,
) -> c_int {
    libc!(libc::pthread_cond_init(
        checked_cast!(cond),
        checked_cast!(attr)
    ));
    let attr = if attr.is_null() {
        PthreadCondattrT::default()
    } else {
        ptr::read(attr)
    };
    ptr::write(
        cond,
        PthreadCondT {
            inner: RawCondvar::new(),
            attr,
            pad: [0_u8;
                SIZEOF_PTHREAD_COND_T - size_of::<RawCondvar>() - size_of::<PthreadCondattrT>()],
        },
    );
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_cond_signal(cond: *mut PthreadCondT) -> c_int {
    libc!(libc::pthread_cond_signal(checked_cast!(cond)));
    (*cond).inner.notify_one();
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_cond_wait(cond: *mut PthreadCondT, lock: *mut PthreadMutexT) -> c_int {
    libc!(libc::pthread_cond_wait(
        checked_cast!(cond),
        checked_cast!(lock)
    ));
    match (*lock).kind.load(Ordering::SeqCst) as i32 {
        libc::PTHREAD_MUTEX_NORMAL => (*cond).inner.wait(&(*lock).u.normal),
        libc::PTHREAD_MUTEX_RECURSIVE => todo!("PTHREAD_MUTEX_RECURSIVE"),
        libc::PTHREAD_MUTEX_ERRORCHECK => todo!("PTHREAD_MUTEX_ERRORCHECK"),
        other => unimplemented!("unsupported pthread mutex kind {}", other),
    }
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_cond_timedwait(
    cond: *mut PthreadCondT,
    lock: *mut PthreadMutexT,
    abstime: *const libc::timespec,
) -> c_int {
    libc!(libc::pthread_cond_timedwait(
        checked_cast!(cond),
        checked_cast!(lock),
        abstime,
    ));
    let abstime = ptr::read(abstime);
    let abstime = Duration::new(
        abstime.tv_sec.try_into().unwrap(),
        abstime.tv_nsec.try_into().unwrap(),
    );
    let now = rustix::time::clock_gettime(rustix::time::ClockId::Realtime);
    let now = Duration::new(
        now.tv_sec.try_into().unwrap(),
        now.tv_nsec.try_into().unwrap(),
    );
    let reltime = abstime.saturating_sub(now);
    match (*lock).kind.load(Ordering::SeqCst) as i32 {
        libc::PTHREAD_MUTEX_NORMAL => {
            if (*cond).inner.wait_timeout(&(*lock).u.normal, reltime) {
                0
            } else {
                libc::ETIMEDOUT
            }
        }
        libc::PTHREAD_MUTEX_RECURSIVE => todo!("PTHREAD_MUTEX_RECURSIVE"),
        libc::PTHREAD_MUTEX_ERRORCHECK => todo!("PTHREAD_MUTEX_ERRORCHECK"),
        other => unimplemented!("unsupported pthread mutex kind {}", other),
    }
}
