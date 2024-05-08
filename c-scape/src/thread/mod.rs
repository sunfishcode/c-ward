mod key;
mod mutex;
mod once;
mod rwlock;
mod spinlock;

use alloc::boxed::Box;
use alloc::format;
use core::ffi::c_void;
use core::mem::{align_of, size_of, transmute, zeroed, MaybeUninit};
use core::ptr::{self, copy_nonoverlapping, null_mut, NonNull};
use core::slice;
use origin::thread::{self, Thread};
use rustix::fs::{Mode, OFlags};

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
