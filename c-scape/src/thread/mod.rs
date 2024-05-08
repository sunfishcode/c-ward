mod key;
mod rwlock;
mod spinlock;

use crate::GetThreadId;
use alloc::boxed::Box;
use alloc::format;
use core::ffi::c_void;
use core::mem::{align_of, size_of, transmute, zeroed, ManuallyDrop, MaybeUninit};
use core::ptr::{self, copy_nonoverlapping, null_mut, NonNull};
use core::slice;
use core::sync::atomic::Ordering::SeqCst;
use core::sync::atomic::AtomicU32;
use core::time::Duration;
use origin::thread::{self, Thread};
use rustix::fs::{Mode, OFlags};
use rustix_futex_sync::lock_api::{RawMutex as _, RawReentrantMutex};
use rustix_futex_sync::{Once, RawCondvar, RawMutex};

use libc::{c_char, c_int, size_t};

// In Linux, `pthread_t` is usually `unsigned long`, but we make it a pointer
// type so that it preserves provenance.
#[allow(non_camel_case_types)]
type PthreadT = *mut c_void;
libc_type!(PthreadT, pthread_t);

bitflags::bitflags! {
    /// Flags for use with [`PthreadAttrT`].
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct PthreadAttrFlags: usize {
        const DETACHSTATE = 0x1;

        /// <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>
        const _ = !0;
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone)]
struct PthreadAttrT {
    stack_addr: *mut c_void,
    stack_size: usize,
    guard_size: usize,
    flags: PthreadAttrFlags,
    pad0: usize,
    pad1: usize,
    pad2: usize,
    #[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
    pad3: usize,
    #[cfg(target_arch = "x86")]
    pad4: usize,
}
libc_type!(PthreadAttrT, pthread_attr_t);

impl Default for PthreadAttrT {
    fn default() -> Self {
        Self {
            stack_addr: null_mut(),
            stack_size: thread::default_stack_size(),
            guard_size: thread::default_guard_size(),
            flags: PthreadAttrFlags::empty(),
            pad0: 0,
            pad1: 0,
            pad2: 0,
            #[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
            pad3: 0,
            #[cfg(target_arch = "x86")]
            pad4: 0,
        }
    }
}

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
unsafe extern "C" fn pthread_self() -> PthreadT {
    libc!(ptr::with_exposed_provenance_mut(libc::pthread_self() as _));
    thread::current().to_raw().cast()
}

#[no_mangle]
unsafe extern "C" fn pthread_getattr_np(thread: PthreadT, attr: *mut PthreadAttrT) -> c_int {
    libc!(libc::pthread_getattr_np(
        thread.expose_provenance() as _,
        checked_cast!(attr)
    ));
    let (stack_addr, stack_size, guard_size) = thread::stack(Thread::from_raw(thread.cast()));
    ptr::write(
        attr,
        PthreadAttrT {
            stack_addr,
            stack_size,
            guard_size,
            flags: PthreadAttrFlags::empty(),
            pad0: 0,
            pad1: 0,
            pad2: 0,
            #[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
            pad3: 0,
            #[cfg(target_arch = "x86")]
            pad4: 0,
        },
    );
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_attr_init(attr: *mut PthreadAttrT) -> c_int {
    libc!(libc::pthread_attr_init(checked_cast!(attr)));
    ptr::write(attr, PthreadAttrT::default());
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_attr_destroy(attr: *mut PthreadAttrT) -> c_int {
    libc!(libc::pthread_attr_destroy(checked_cast!(attr)));
    ptr::drop_in_place(attr);
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_attr_getstack(
    attr: *const PthreadAttrT,
    stackaddr: *mut *mut c_void,
    stacksize: *mut usize,
) -> c_int {
    libc!(libc::pthread_attr_getstack(
        checked_cast!(attr),
        stackaddr,
        stacksize
    ));
    *stackaddr = (*attr).stack_addr;
    *stacksize = (*attr).stack_size;
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_attr_setstack(
    attr: *mut PthreadAttrT,
    stackaddr: *mut c_void,
    stacksize: usize,
) -> c_int {
    //libc!(libc::pthread_attr_setstack(checked_cast!(attr), stackaddr, stacksize));

    if stacksize < libc::PTHREAD_STACK_MIN {
        return libc::EINVAL;
    }

    (*attr).stack_addr = stackaddr;
    (*attr).stack_size = stacksize;
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_attr_setdetachstate(
    attr: *mut PthreadAttrT,
    detachstate: c_int,
) -> c_int {
    libc!(libc::pthread_attr_setdetachstate(
        checked_cast!(attr),
        detachstate
    ));
    let value = match detachstate {
        libc::PTHREAD_CREATE_DETACHED => true,
        libc::PTHREAD_CREATE_JOINABLE => false,
        _ => return libc::EINVAL,
    };
    (*attr).flags.set(PthreadAttrFlags::DETACHSTATE, value);
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_attr_getdetachstate(
    attr: *const PthreadAttrT,
    detachstate: *mut c_int,
) -> c_int {
    //libc!(libc::pthread_attr_getdetachstate(checked_cast!(attr), detachstate));
    let newstate = if (*attr).flags.contains(PthreadAttrFlags::DETACHSTATE) {
        libc::PTHREAD_CREATE_DETACHED
    } else {
        libc::PTHREAD_CREATE_JOINABLE
    };
    *detachstate = newstate;
    0
}

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

    (*attr).kind.store(kind as u32, SeqCst);
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_mutexattr_gettype(
    attr: *mut PthreadMutexattrT,
    kind: *mut c_int,
) -> c_int {
    //libc!(libc::pthread_mutexattr_gettype(checked_cast!(attr), kind));

    *kind = (*attr).kind.load(SeqCst) as c_int;
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_mutex_destroy(mutex: *mut PthreadMutexT) -> c_int {
    libc!(libc::pthread_mutex_destroy(checked_cast!(mutex)));
    match (*mutex).kind.load(SeqCst) as i32 {
        libc::PTHREAD_MUTEX_NORMAL => ManuallyDrop::drop(&mut (*mutex).u.normal),
        libc::PTHREAD_MUTEX_RECURSIVE => ManuallyDrop::drop(&mut (*mutex).u.reentrant),
        libc::PTHREAD_MUTEX_ERRORCHECK => todo!("PTHREAD_MUTEX_ERRORCHECK"),
        other => unimplemented!("unsupported pthread mutex kind {}", other),
    }
    (*mutex).kind.store(!0, SeqCst);
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
        (*mutexattr).kind.load(SeqCst)
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
    (*mutex).kind.store(kind, SeqCst);
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_mutex_lock(mutex: *mut PthreadMutexT) -> c_int {
    libc!(libc::pthread_mutex_lock(checked_cast!(mutex)));
    match (*mutex).kind.load(SeqCst) as i32 {
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
    if match (*mutex).kind.load(SeqCst) as i32 {
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
    match mutex.kind.load(SeqCst) as i32 {
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

#[no_mangle]
unsafe extern "C" fn pthread_attr_getguardsize(
    attr: *const PthreadAttrT,
    guardsize: *mut usize,
) -> c_int {
    libc!(libc::pthread_attr_getguardsize(
        checked_cast!(attr),
        guardsize
    ));
    *guardsize = (*attr).guard_size;
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_attr_setguardsize(attr: *mut PthreadAttrT, guardsize: usize) -> c_int {
    // TODO: libc!(libc::pthread_attr_setguardsize(attr, guardsize));
    (*attr).guard_size = guardsize;
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
    match (*lock).kind.load(SeqCst) as i32 {
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
    match (*lock).kind.load(SeqCst) as i32 {
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

#[no_mangle]
unsafe extern "C" fn pthread_create(
    pthread: *mut PthreadT,
    attr: *const PthreadAttrT,
    fn_: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    arg: *mut c_void,
) -> c_int {
    libc!(libc::pthread_create(
        pthread as _,
        checked_cast!(attr),
        core::mem::transmute(fn_),
        arg
    ));

    let PthreadAttrT {
        stack_addr,
        stack_size,
        guard_size,
        flags,
        pad0: _,
        pad1: _,
        pad2: _,
        #[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
            pad3: _,
        #[cfg(target_arch = "x86")]
            pad4: _,
    } = if attr.is_null() {
        PthreadAttrT::default()
    } else {
        ptr::read(attr)
    };
    assert!(
        stack_addr.is_null(),
        "custom thread stacks not supported yet"
    );

    let args = [NonNull::new(fn_ as *mut c_void), NonNull::new(arg)];

    // `create_thread` takes a bare function pointer, and it's not
    // `extern "C"`, so we have to wrap the user's `fn_`.
    unsafe fn call(args: &mut [Option<NonNull<c_void>>]) -> Option<NonNull<c_void>> {
        let fn_ = match args[0] {
            Some(fn_) => fn_.as_ptr(),
            None => null_mut(),
        };
        let fn_: unsafe extern "C" fn(*mut c_void) -> *mut c_void = transmute(fn_);

        let arg = match args[1] {
            Some(arg) => arg.as_ptr(),
            None => null_mut(),
        };

        let return_value = fn_(arg);

        NonNull::new(return_value)
    }

    // Create the thread.
    let thread = match thread::create(call, &args, stack_size, guard_size) {
        Ok(thread) => thread,
        Err(e) => return e.raw_os_error(),
    };

    // In theory we could optimize this by adding an argument to origin's
    // `create_thread` to initialize the thread in the detached state,
    // however this seems adequate for now.
    if flags.contains(PthreadAttrFlags::DETACHSTATE) {
        thread::detach(thread);
    }

    pthread.write(thread.to_raw().cast());
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_detach(pthread: PthreadT) -> c_int {
    libc!(libc::pthread_detach(pthread.expose_provenance() as _));
    thread::detach(Thread::from_raw(pthread.cast()));
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_join(pthread: PthreadT, retval: *mut *mut c_void) -> c_int {
    libc!(libc::pthread_join(pthread.expose_provenance() as _, retval));

    let return_value = thread::join(Thread::from_raw(pthread.cast()));

    if !retval.is_null() {
        *retval = match return_value {
            Some(return_value) => return_value.as_ptr(),
            None => null_mut(),
        };
    }

    0
}

#[no_mangle]
unsafe extern "C" fn pthread_equal(a: libc::pthread_t, b: libc::pthread_t) -> c_int {
    //libc!(libc::pthread_equal(a, b));

    i32::from(a == b)
}

#[no_mangle]
unsafe extern "C" fn pthread_sigmask(
    how: c_int,
    set: *const libc::sigset_t,
    oldset: *mut libc::sigset_t,
) -> c_int {
    libc!(libc::pthread_sigmask(how, set, oldset));

    let how = match how {
        libc::SIG_BLOCK => rustix::runtime::How::BLOCK,
        libc::SIG_UNBLOCK => rustix::runtime::How::UNBLOCK,
        libc::SIG_SETMASK => rustix::runtime::How::SETMASK,
        _ => return libc::EINVAL,
    };

    if !oldset.is_null() {
        oldset.write(zeroed());
    }

    assert!(size_of::<rustix::runtime::Sigset>() <= size_of::<libc::sigset_t>());
    assert!(align_of::<rustix::runtime::Sigset>() <= align_of::<libc::sigset_t>());
    let set: *const rustix::runtime::Sigset = set.cast();
    let oldset: *mut rustix::runtime::Sigset = oldset.cast();

    let set = if set.is_null() { None } else { Some(&*set) };

    match rustix::runtime::sigprocmask(how, set) {
        Ok(old) => {
            if !oldset.is_null() {
                oldset.write(old);
            }
            0
        }
        Err(e) => e.raw_os_error(),
    }
}

#[no_mangle]
unsafe extern "C" fn pthread_attr_getstacksize(
    attr: *const PthreadAttrT,
    stacksize: *mut usize,
) -> c_int {
    libc!(libc::pthread_attr_getstacksize(
        checked_cast!(attr),
        stacksize
    ));
    *stacksize = (*attr).stack_size;
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_attr_setstacksize(attr: *mut PthreadAttrT, stacksize: usize) -> c_int {
    libc!(libc::pthread_attr_setstacksize(
        checked_cast!(attr),
        stacksize
    ));

    if stacksize < libc::PTHREAD_STACK_MIN {
        return libc::EINVAL;
    }

    (*attr).stack_size = stacksize;
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_atfork(
    prepare: Option<unsafe extern "C" fn()>,
    parent: Option<unsafe extern "C" fn()>,
    child: Option<unsafe extern "C" fn()>,
) -> c_int {
    libc!(libc::pthread_atfork(prepare, parent, child));
    crate::at_fork::at_fork(prepare, parent, child);
    0
}

#[no_mangle]
unsafe extern "C" fn pthread_getname_np(
    pthread: PthreadT,
    name: *mut c_char,
    len: size_t,
) -> c_int {
    libc!(libc::pthread_getname_np(
        pthread.expose_provenance() as _,
        name,
        len
    ));

    if len < 16 {
        return libc::ERANGE;
    }

    let origin_thread = Thread::from_raw(pthread.cast());

    if origin_thread == thread::current() {
        let prctl_name = match rustix::thread::name() {
            Ok(prctl_name) => prctl_name,
            Err(err) => return err.raw_os_error(),
        };

        let bytes = prctl_name.to_bytes_with_nul();

        debug_assert!(bytes.len() <= len);

        copy_nonoverlapping(bytes.as_ptr().cast(), name, bytes.len());
        return 0;
    }

    let path = format!(
        "/proc/self/task/{}/comm",
        thread::id(origin_thread).unwrap().as_raw_nonzero()
    );
    let fd = match rustix::fs::open(
        path,
        OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOCTTY,
        Mode::empty(),
    ) {
        Ok(fd) => fd,
        Err(err) => return err.raw_os_error(),
    };

    loop {
        let buf = slice::from_raw_parts_mut(name.cast::<MaybeUninit<u8>>(), len);
        match rustix::io::read_uninit(&fd, buf) {
            Ok((init, _uninit)) if init.is_empty() => return libc::EIO,
            Ok((init, _uninit)) if init.len() <= len => {
                *name.add(init.len() - 1) = 0;
                break;
            }
            Ok(_) => return libc::EIO,
            Err(rustix::io::Errno::INTR) => continue,
            Err(err) => return err.raw_os_error(),
        }
    }
    0
}

#[cfg(target_os = "linux")]
#[no_mangle]
unsafe extern "C" fn pthread_setname_np(pthread: PthreadT, name: *const libc::c_char) -> c_int {
    libc!(libc::pthread_setname_np(
        pthread.expose_provenance() as _,
        name
    ));

    let name = core::ffi::CStr::from_ptr(name);
    let bytes = name.to_bytes();

    if bytes.len() >= 16 {
        return libc::ERANGE;
    }

    let origin_thread = Thread::from_raw(pthread.cast());

    if origin_thread == thread::current() {
        return match rustix::thread::set_name(name) {
            Ok(()) => 0,
            Err(err) => err.raw_os_error(),
        };
    }

    let path = format!(
        "/proc/self/task/{}/comm",
        thread::id(origin_thread).unwrap().as_raw_nonzero()
    );
    let fd = match rustix::fs::open(
        path,
        OFlags::WRONLY | OFlags::CLOEXEC | OFlags::NOCTTY,
        Mode::empty(),
    ) {
        Ok(fd) => fd,
        Err(err) => return err.raw_os_error(),
    };

    loop {
        match rustix::io::write(&fd, bytes) {
            Ok(n) if n == bytes.len() => return 0,
            Ok(_) => return libc::EIO,
            Err(rustix::io::Errno::INTR) => continue,
            Err(err) => return err.raw_os_error(),
        }
    }
}

// TODO: See comment on `pthread_clean_push` about the
// ordering guarantees that programs expect.
#[no_mangle]
unsafe extern "C" fn __cxa_thread_atexit_impl(
    func: unsafe extern "C" fn(*mut c_void),
    obj: *mut c_void,
    _dso_symbol: *mut c_void,
) -> c_int {
    // TODO: libc!(libc::__cxa_thread_atexit_impl(func, obj, _dso_symbol));
    thread::at_exit(Box::new(move || func(obj)));
    0
}

#[cfg(feature = "thread")]
#[no_mangle]
unsafe extern "C" fn __tls_get_addr(p: &[usize; 2]) -> *mut c_void {
    //libc!(libc::__tls_get_addr(p));
    let [module, offset] = *p;
    // Offset 0 is the generation field, and we don't support dynamic linking,
    // so we should only ever see 1 here.
    assert_eq!(module, 1);
    thread::current_tls_addr(offset)
}

#[cfg(target_arch = "x86")]
#[no_mangle]
unsafe extern "C" fn ___tls_get_addr() {
    //libc!(libc::___tls_get_addr());
    todo!("___tls_get_addr")
}

#[no_mangle]
unsafe extern "C" fn pthread_once(
    once_control: *mut libc::pthread_once_t,
    init_routine: extern "C" fn(),
) -> c_int {
    libc!(libc::pthread_once(once_control, init_routine));

    // Assert that `PTHREAD_ONCE_INIT` is zero, just like
    // `rustix_futex_sync::Once::new()` is documented to be.
    debug_assert_eq!(libc::PTHREAD_ONCE_INIT, transmute(Once::new()));
    debug_assert_eq!(size_of::<libc::pthread_once_t>(), size_of::<Once>());
    debug_assert_eq!(align_of::<libc::pthread_once_t>(), align_of::<Once>());

    // Cast the `*mut pthread_once_t` to `*mut Once`, which we can do since
    // `rustix_futex_sync` is documented to be a `repr(transparent)` wrapper
    // around `AtomicU32`.
    (*once_control.cast::<Once>()).call_once(move || {
        init_routine();
    });

    0
}
