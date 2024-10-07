use crate::convert_res;
use core::ffi::CStr;
use core::mem::{size_of, zeroed};
#[cfg(feature = "take-charge")]
use core::ptr;
use core::ptr::{addr_of, null_mut};
use errno::{set_errno, Errno};
use libc::{c_char, c_int, c_long, c_ulong, c_void};

#[no_mangle]
unsafe extern "C" fn getpagesize() -> c_int {
    //libc!(libc::getpagesize());

    __getpagesize()
}

#[no_mangle]
unsafe extern "C" fn __getpagesize() -> c_int {
    //libc!(libc::__getpagesize());

    rustix::param::page_size() as _
}

#[no_mangle]
unsafe extern "C" fn sysconf(name: c_int) -> c_long {
    libc!(libc::sysconf(name));
    _sysconf(name)
}

#[no_mangle]
unsafe extern "C" fn __sysconf(name: c_int) -> c_long {
    //libc!(libc::__sysconf(name));
    _sysconf(name)
}

unsafe fn _sysconf(name: c_int) -> c_long {
    #[cfg(feature = "std")] // These are defined in c-gull.
    #[cfg(not(target_os = "wasi"))]
    extern "C" {
        fn get_nprocs_conf() -> c_int;
        fn get_nprocs() -> c_int;
    }

    match name {
        libc::_SC_PAGESIZE => rustix::param::page_size() as _,
        libc::_SC_CLK_TCK => rustix::param::clock_ticks_per_second() as _,
        #[cfg(not(target_os = "wasi"))]
        libc::_SC_GETPW_R_SIZE_MAX | libc::_SC_GETGR_R_SIZE_MAX => -1,
        #[cfg(any(target_os = "android", target_os = "linux", target_os = "wasi"))]
        libc::_SC_SYMLOOP_MAX => 40,
        libc::_SC_HOST_NAME_MAX => 255,
        libc::_SC_NGROUPS_MAX => 32,
        #[cfg(any(target_os = "android", target_os = "linux"))]
        libc::_SC_DELAYTIMER_MAX => i32::MAX as _,
        #[cfg(feature = "std")]
        #[cfg(not(target_os = "wasi"))]
        libc::_SC_NPROCESSORS_CONF => get_nprocs_conf().into(),
        #[cfg(feature = "std")]
        #[cfg(not(target_os = "wasi"))]
        libc::_SC_NPROCESSORS_ONLN => get_nprocs().into(),
        #[cfg(not(target_os = "wasi"))]
        libc::_SC_PHYS_PAGES => get_phys_pages(),
        #[cfg(not(target_os = "wasi"))]
        libc::_SC_AVPHYS_PAGES => get_avphys_pages(),

        libc::_SC_2_C_BIND
        | libc::_SC_2_VERSION
        | libc::_SC_ADVISORY_INFO
        | libc::_SC_ASYNCHRONOUS_IO
        | libc::_SC_BARRIERS
        | libc::_SC_CLOCK_SELECTION
        | libc::_SC_CPUTIME
        | libc::_SC_FSYNC
        | libc::_SC_IPV6
        | libc::_SC_MAPPED_FILES
        | libc::_SC_MEMLOCK
        | libc::_SC_MEMLOCK_RANGE
        | libc::_SC_MEMORY_PROTECTION
        | libc::_SC_MESSAGE_PASSING
        | libc::_SC_MONOTONIC_CLOCK
        | libc::_SC_PRIORITIZED_IO
        | libc::_SC_PRIORITY_SCHEDULING
        | libc::_SC_RAW_SOCKETS
        | libc::_SC_READER_WRITER_LOCKS
        | libc::_SC_REALTIME_SIGNALS
        | libc::_SC_SEMAPHORES
        | libc::_SC_SHARED_MEMORY_OBJECTS
        | libc::_SC_SPAWN
        | libc::_SC_SPIN_LOCKS
        | libc::_SC_SYNCHRONIZED_IO
        | libc::_SC_THREAD_ATTR_STACKADDR
        | libc::_SC_THREAD_ATTR_STACKSIZE
        | libc::_SC_THREAD_CPUTIME
        | libc::_SC_THREAD_PRIO_INHERIT
        | libc::_SC_THREAD_PRIO_PROTECT
        | libc::_SC_THREAD_PRIORITY_SCHEDULING
        | libc::_SC_THREAD_PROCESS_SHARED
        | libc::_SC_THREADS
        | libc::_SC_THREAD_SAFE_FUNCTIONS
        | libc::_SC_TIMEOUTS
        | libc::_SC_TIMERS
        | libc::_SC_VERSION => 200809,

        libc::_SC_THREAD_STACK_MIN => libc::PTHREAD_STACK_MIN as _,

        _ => panic!("unrecognized sysconf({})", name),
    }
}

#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn get_phys_pages() -> c_long {
    //libc!(libc::get_phys_pages());

    let info = rustix::system::sysinfo();
    let mem_unit = if info.mem_unit == 0 {
        1
    } else {
        info.mem_unit as c_ulong
    };

    (info.totalram * mem_unit / rustix::param::page_size() as c_ulong)
        .try_into()
        .unwrap_or(c_long::MAX)
}

#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn get_avphys_pages() -> c_long {
    //libc!(libc::get_avphys_pages());

    let info = rustix::system::sysinfo();
    let mem_unit = if info.mem_unit == 0 {
        1
    } else {
        info.mem_unit as c_ulong
    };

    ((info.freeram + info.bufferram) * mem_unit / rustix::param::page_size() as c_ulong)
        .try_into()
        .unwrap_or(c_long::MAX)
}

#[no_mangle]
unsafe extern "C" fn pathconf(_path: *const c_char, name: c_int) -> c_long {
    libc!(libc::pathconf(_path, name));
    _pathconf(name)
}

#[no_mangle]
unsafe extern "C" fn fpathconf(_fd: c_int, name: c_int) -> c_long {
    libc!(libc::fpathconf(_fd, name));
    _pathconf(name)
}

fn _pathconf(name: c_int) -> c_long {
    match name {
        libc::_PC_PATH_MAX => libc::PATH_MAX as _,
        #[cfg(any(target_os = "android", target_os = "linux"))]
        libc::_PC_NAME_MAX => 255,
        _ => panic!("unrecognized pathconf({})", name),
    }
}

// `getauxval` usually returns `unsigned long`, but we make it a pointer type
// so that it preserves provenance.
//
// This is not used in coexist-with-libc configurations because libc startup
// code sometimes needs to call `getauxval` before rustix is initialized.
#[cfg(feature = "take-charge")]
#[no_mangle]
unsafe extern "C" fn getauxval(type_: c_ulong) -> *mut c_void {
    libc!(ptr::with_exposed_provenance_mut(libc::getauxval(type_) as _));
    _getauxval(type_)
}

// As with `getauxval`, this is not used in coexist-with-libc configurations
// because libc startup code sometimes needs to call `getauxval` before rustix
// is initialized.
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "take-charge")]
#[no_mangle]
unsafe extern "C" fn __getauxval(type_: c_ulong) -> *mut c_void {
    //libc!(ptr::from_exposed_addr(libc::__getauxval(type_) as _));
    _getauxval(type_)
}

#[cfg(feature = "take-charge")]
fn _getauxval(type_: c_ulong) -> *mut c_void {
    match type_ {
        libc::AT_HWCAP => ptr::without_provenance_mut(rustix::param::linux_hwcap().0),
        libc::AT_HWCAP2 => ptr::without_provenance_mut(rustix::param::linux_hwcap().1),
        libc::AT_MINSIGSTKSZ => ptr::without_provenance_mut(rustix::param::linux_minsigstksz()),
        _ => todo!("unrecognized __getauxval {}", type_),
    }
}

#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn dl_iterate_phdr(
    callback: Option<
        unsafe extern "C" fn(
            info: *mut libc::dl_phdr_info,
            size: usize,
            data: *mut c_void,
        ) -> c_int,
    >,
    data: *mut c_void,
) -> c_int {
    extern "C" {
        static mut __executable_start: c_void;
    }

    libc!(libc::dl_iterate_phdr(callback, data));

    let (phdr, _phent, phnum) = rustix::runtime::exe_phdrs();
    let mut info = libc::dl_phdr_info {
        dlpi_addr: addr_of!(__executable_start).expose_provenance() as _,
        dlpi_name: c"/proc/self/exe".as_ptr(),
        dlpi_phdr: phdr.cast(),
        dlpi_phnum: phnum.try_into().unwrap(),
        ..zeroed()
    };
    callback.unwrap()(&mut info, size_of::<libc::dl_phdr_info>(), data)
}

#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void {
    libc!(libc::dlsym(handle, symbol));

    let symbol = CStr::from_ptr(symbol.cast());

    if handle == libc::RTLD_DEFAULT {
        // `std` uses `dlsym` to dynamically detect feature availability; recognize
        // functions it asks for.
        match symbol.to_bytes() {
            #[cfg(any(target_os = "android", target_os = "linux"))]
            #[cfg(not(target_env = "musl"))]
            b"statx" => libc::statx as _,
            #[cfg(any(target_os = "android", target_os = "linux"))]
            b"getrandom" => libc::getrandom as _,
            #[cfg(any(target_os = "android", target_os = "linux"))]
            b"copy_file_range" => libc::copy_file_range as _,
            #[cfg(target_env = "gnu")]
            b"gnu_get_libc_version" => libc::gnu_get_libc_version as _,
            #[cfg(any(target_os = "android", target_os = "linux"))]
            b"epoll_create1" => libc::epoll_create1 as _,
            b"pipe2" => libc::pipe2 as _,

            // Let's just say we don't support this for now.
            #[cfg(any(target_os = "android", target_os = "linux"))]
            b"clone3" => null_mut(),
            // Let's just say we don't support this for now.
            b"__pthread_get_minstack" => null_mut(),

            _ => unimplemented!("dlsym(_, {:?})", symbol),
        }
    } else if handle == libc::RTLD_NEXT {
        // We don't support any dynamic linking, so there's no "next" dynamic
        // library.
        null_mut()
    } else {
        unimplemented!("dlsym with a handle")
    }
}

#[no_mangle]
unsafe extern "C" fn sched_yield() -> c_int {
    libc!(libc::sched_yield());

    rustix::process::sched_yield();
    0
}

#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn sched_getaffinity(
    pid: libc::pid_t,
    cpu_set_size: libc::size_t,
    mask: *mut libc::cpu_set_t,
) -> c_int {
    libc!(libc::sched_getaffinity(pid, cpu_set_size, mask.cast()));

    let pid = rustix::process::Pid::from_raw(pid as _);
    let set = match convert_res(rustix::process::sched_getaffinity(pid)) {
        Some(set) => set,
        None => return -1,
    };

    mask.write(core::mem::zeroed());
    libc::CPU_ZERO(&mut *mask);
    for i in 0..core::cmp::min(rustix::process::CpuSet::MAX_CPU, cpu_set_size * 8) {
        if set.is_set(i) {
            libc::CPU_SET(i, &mut *mask);
        }
    }
    0
}

#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn sched_setaffinity(
    pid: libc::pid_t,
    cpu_set_size: libc::size_t,
    mask: *const libc::cpu_set_t,
) -> c_int {
    libc!(libc::sched_setaffinity(pid, cpu_set_size, mask));

    let mut set = rustix::process::CpuSet::new();
    let mask = &*mask;
    for i in 0..core::cmp::min(rustix::process::CpuSet::MAX_CPU, cpu_set_size * 8) {
        if libc::CPU_ISSET(i, mask) {
            set.set(i);
        }
    }

    let pid = rustix::process::Pid::from_raw(pid as _);
    match convert_res(rustix::process::sched_setaffinity(pid, &set)) {
        Some(()) => 0,
        None => -1,
    }
}

#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn __sched_cpucount(size: libc::size_t, set: *const libc::cpu_set_t) -> c_int {
    //libc!(libc::___sched_cpucount(size, set));

    let mut count = 0;
    for i in 0..core::cmp::min(rustix::process::CpuSet::MAX_CPU, size * 8) {
        if libc::CPU_ISSET(i, &*set) {
            count += 1;
        }
    }
    count
}

#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn __sched_cpualloc(count: libc::size_t) -> *mut libc::cpu_set_t {
    //libc!(libc::___sched_cpualloc(count));

    let count = core::cmp::min(count, rustix::process::CpuSet::MAX_CPU);
    libc::malloc(libc::CPU_ALLOC_SIZE(count as _)).cast()
}

#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn __sched_cpufree(set: *mut libc::cpu_set_t) {
    //libc!(libc::___sched_cpufree(set));

    libc::free(set.cast());
}

#[cfg(not(target_os = "wasi"))]
#[no_mangle]
unsafe extern "C" fn sched_getcpu() -> c_int {
    libc!(libc::sched_getcpu());

    rustix::process::sched_getcpu() as _
}

// In Linux, `prctl`'s arguments are described as `unsigned long`, however we
// use pointer types in order to preserve provenance.
#[cfg(any(target_os = "android", target_os = "linux"))]
#[no_mangle]
unsafe extern "C" fn prctl(
    option: c_int,
    arg2: *mut c_void,
    _arg3: *mut c_void,
    _arg4: *mut c_void,
    _arg5: *mut c_void,
) -> c_int {
    libc!(libc::prctl(option, arg2, _arg3, _arg4, _arg5));
    match option {
        libc::PR_SET_NAME => {
            if arg2.is_null() {
                set_errno(Errno(libc::EFAULT));
                return -1;
            }
            match convert_res(rustix::thread::set_name(CStr::from_ptr(
                arg2.cast::<c_char>(),
            ))) {
                Some(()) => 0,
                None => -1,
            }
        }
        libc::PR_GET_PDEATHSIG => match convert_res(rustix::process::parent_process_death_signal())
        {
            Some(signal) => {
                let sig = signal.map(|s| s as u32 as c_int).unwrap_or(0);
                arg2.cast::<c_int>().write(sig);
                0
            }
            None => -1,
        },
        libc::PR_SET_PDEATHSIG => {
            let arg2_i32 =
                match convert_res(i32::try_from(arg2.addr()).map_err(|_| rustix::io::Errno::RANGE))
                {
                    Some(arg2_i32) => arg2_i32,
                    None => return -1,
                };
            // rustix converts any invalid signal to `None`, but only 0 should get mapped
            // to `None`; any other invalid signal is an error
            let sig = if arg2_i32 == 0 {
                None
            } else {
                match convert_res(
                    rustix::process::Signal::from_raw(arg2_i32).ok_or(rustix::io::Errno::RANGE),
                ) {
                    Some(s) => Some(s),
                    None => return -1,
                }
            };
            match convert_res(rustix::process::set_parent_process_death_signal(sig)) {
                Some(()) => 0,
                None => -1,
            }
        }
        libc::PR_GET_DUMPABLE => match convert_res(rustix::process::dumpable_behavior()) {
            Some(dumpable) => dumpable as i32,
            None => -1,
        },
        libc::PR_SET_DUMPABLE => {
            let arg2_i32 =
                match convert_res(i32::try_from(arg2.addr()).map_err(|_| rustix::io::Errno::RANGE))
                {
                    Some(arg2_i32) => arg2_i32,
                    None => return -1,
                };
            let dumpable = match convert_res(rustix::process::DumpableBehavior::try_from(arg2_i32))
            {
                Some(dumpable) => dumpable,
                None => return -1,
            };
            match convert_res(rustix::process::set_dumpable_behavior(dumpable)) {
                Some(()) => 0,
                None => -1,
            }
        }
        _ => unimplemented!("unrecognized prctl op {}", option),
    }
}
