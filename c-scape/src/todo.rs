//! Unimplemented stub functions. These may help porting programs which
//! need these functions to link but don't (always) call them at runtime.

mod aio;
mod cat;
mod dl;
mod fenv;
mod jmp;
mod locale;
mod long_double;
mod long_double_complex;
mod pthread_cancel;
mod set_id;
mod sysv;
mod wchar;

// Functions that are implemented in rustix, so we just need to implement
// C-compatible wrappers for them.

#[no_mangle]
unsafe extern "C" fn sysinfo() {
    todo!("sysinfo")
}
#[no_mangle]
unsafe extern "C" fn signalfd() {
    todo!("signalfd")
}
#[no_mangle]
unsafe extern "C" fn mount() {
    todo!("mount")
}
#[no_mangle]
unsafe extern "C" fn umount() {
    todo!("umount")
}
#[no_mangle]
unsafe extern "C" fn umount2() {
    todo!("umount2")
}
#[no_mangle]
unsafe extern "C" fn unshare() {
    todo!("unshare")
}
#[no_mangle]
unsafe extern "C" fn timerfd_gettime() {
    todo!("timerfd_gettime")
}

// `_chk` versions of functions we have implemented, so we just need to add
// wrappers with extra checks.

#[no_mangle]
unsafe extern "C" fn __realpath_chk() {
    todo!("__realpath_chk")
}
#[no_mangle]
unsafe extern "C" fn __fread_chk() {
    todo!("__fread_chk")
}
#[no_mangle]
unsafe extern "C" fn __getgroups_chk() {
    todo!("__getgroups_chk")
}
#[no_mangle]
unsafe extern "C" fn __readlink_chk() {
    todo!("__readlink_chk")
}
#[no_mangle]
unsafe extern "C" fn __readlinkat_chk() {
    todo!("__readlinkat_chk")
}

// BSD formatted error functions. These can be implemented using other
// libc functions.

#[no_mangle]
unsafe extern "C" fn err() {
    todo!("err")
}
#[no_mangle]
unsafe extern "C" fn errx() {
    todo!("errx")
}
#[no_mangle]
unsafe extern "C" fn warn() {
    todo!("warn")
}
#[no_mangle]
unsafe extern "C" fn warnx() {
    todo!("warnx")
}
#[no_mangle]
unsafe extern "C" fn verr() {
    todo!("verr")
}
#[no_mangle]
unsafe extern "C" fn verrx() {
    todo!("verrx")
}
#[no_mangle]
unsafe extern "C" fn vwarn() {
    todo!("vwarn")
}
#[no_mangle]
unsafe extern "C" fn vwarnx() {
    todo!("vwarnx")
}

// NSS functions. Currently we're implementing NSS functions in c-gull by
// invoking the `getent` command and parsing its output.

#[no_mangle]
unsafe extern "C" fn setnetent() {
    todo!("setnetent")
}
#[no_mangle]
unsafe extern "C" fn getnetent() {
    todo!("getnetent")
}
#[no_mangle]
unsafe extern "C" fn endnetent() {
    todo!("endnetent")
}
#[no_mangle]
unsafe extern "C" fn setprotoent() {
    todo!("setprotoent")
}
#[no_mangle]
unsafe extern "C" fn getprotoent() {
    todo!("getprotoent")
}
#[no_mangle]
unsafe extern "C" fn endprotoent() {
    todo!("endprotoent")
}
#[no_mangle]
unsafe extern "C" fn setservent() {
    todo!("setservent")
}
#[no_mangle]
unsafe extern "C" fn getservent() {
    todo!("getservent")
}
#[no_mangle]
unsafe extern "C" fn endservent() {
    todo!("endservent")
}
#[no_mangle]
unsafe extern "C" fn sethostent() {
    todo!("sethostent")
}
#[no_mangle]
unsafe extern "C" fn gethostent() {
    todo!("gethostent")
}
#[no_mangle]
unsafe extern "C" fn endhostent() {
    todo!("endhostent")
}
#[no_mangle]
unsafe extern "C" fn setpwent() {
    todo!("setpwent")
}
#[no_mangle]
unsafe extern "C" fn getpwent() {
    todo!("getpwent")
}
#[no_mangle]
unsafe extern "C" fn endpwent() {
    todo!("endpwent")
}
#[no_mangle]
unsafe extern "C" fn setmntent() {
    todo!("setmntent")
}
#[no_mangle]
unsafe extern "C" fn getmntent() {
    todo!("getmntent")
}
#[no_mangle]
unsafe extern "C" fn endmntent() {
    todo!("endmntent")
}
#[no_mangle]
unsafe extern "C" fn hasmntopt() {
    todo!("hasmntopt")
}
#[no_mangle]
unsafe extern "C" fn setgrent() {
    todo!("setgrent")
}
#[no_mangle]
unsafe extern "C" fn getgrent() {
    todo!("getgrent")
}
#[no_mangle]
unsafe extern "C" fn endgrent() {
    todo!("endgrent")
}
#[no_mangle]
unsafe extern "C" fn gethostbyaddr() {
    todo!("gethostbyaddr")
}
#[no_mangle]
unsafe extern "C" fn getnetbyname() {
    todo!("getnetbyname")
}
#[no_mangle]
unsafe extern "C" fn getprotobyname() {
    todo!("getprotobyname")
}
#[no_mangle]
unsafe extern "C" fn getprotobynumber() {
    todo!("getprotobynumber")
}
#[no_mangle]
unsafe extern "C" fn getifaddrs() {
    todo!("getifaddrs")
}
#[no_mangle]
unsafe extern "C" fn freeifaddrs() {
    todo!("freeifaddrs")
}
#[no_mangle]
unsafe extern "C" fn getnameinfo() {
    todo!("getnameinfo")
}
#[no_mangle]
unsafe extern "C" fn gethostbyname() {
    todo!("gethostbyname")
}
#[no_mangle]
unsafe extern "C" fn getgrent_r() {
    todo!("getgrent_r")
}
#[no_mangle]
unsafe extern "C" fn getpwent_r() {
    todo!("getpwent_r")
}
#[no_mangle]
unsafe extern "C" fn getspent_r() {
    todo!("getspent_r")
}
#[no_mangle]
unsafe extern "C" fn gethostbyname_r() {
    todo!("gethostbyname_r")
}
#[no_mangle]
unsafe extern "C" fn initgroups() {
    todo!("initgroups")
}
#[no_mangle]
unsafe extern "C" fn if_indextoname() {
    todo!("if_indextoname")
}
#[no_mangle]
unsafe extern "C" fn if_nametoindex() {
    todo!("if_nametoindex")
}
#[no_mangle]
unsafe extern "C" fn if_nameindex() {
    todo!("if_nameindex")
}
#[no_mangle]
unsafe extern "C" fn if_freenameindex() {
    todo!("if_freenameindex")
}

// Additional functions.

#[no_mangle]
unsafe extern "C" fn alarm() {
    todo!("alarm")
}
#[no_mangle]
unsafe extern "C" fn process_vm_writev() {
    todo!("process_vm_writev")
}
#[no_mangle]
unsafe extern "C" fn process_vm_readv() {
    todo!("process_vm_readv")
}
#[no_mangle]
unsafe extern "C" fn setfsuid() {
    todo!("setfsuid")
}
#[no_mangle]
unsafe extern "C" fn setfsgid() {
    todo!("setfsgid")
}
#[no_mangle]
unsafe extern "C" fn strftime() {
    todo!("strftime")
}
#[no_mangle]
unsafe extern "C" fn wcsftime() {
    todo!("wcsftime")
}
#[no_mangle]
unsafe extern "C" fn __isoc99_scanf() {
    todo!("__isoc99_scanf")
}
#[no_mangle]
unsafe extern "C" fn __isoc99_sscanf() {
    todo!("__isoc99_sscanf")
}
#[no_mangle]
unsafe extern "C" fn pthread_kill() {
    todo!("pthread_kill")
}
#[no_mangle]
unsafe extern "C" fn __fdelt_chk() {
    todo!("__fdelt_chk")
}
#[no_mangle]
unsafe extern "C" fn acct() {
    todo!("acct")
}
#[no_mangle]
unsafe extern "C" fn clock_getcpuclockid() {
    todo!("clock_getcpuclockid")
}
#[no_mangle]
unsafe extern "C" fn getresgid() {
    todo!("getresgid")
}
#[no_mangle]
unsafe extern "C" fn getresuid() {
    todo!("getresuid")
}
#[no_mangle]
unsafe extern "C" fn ppoll() {
    todo!("ppoll")
}
#[no_mangle]
unsafe extern "C" fn __ppoll_chk() {
    todo!("__ppoll_chk")
}
#[no_mangle]
unsafe extern "C" fn pselect() {
    todo!("pselect")
}
#[no_mangle]
unsafe extern "C" fn epoll_pwait() {
    todo!("epoll_pwait")
}
#[no_mangle]
unsafe extern "C" fn epoll_pwait2() {
    todo!("epoll_pwait2")
}
#[no_mangle]
unsafe extern "C" fn ptrace() {
    todo!("ptrace")
}
#[no_mangle]
unsafe extern "C" fn forkpty() {
    todo!("forkpty")
}
#[no_mangle]
unsafe extern "C" fn recvmmsg() {
    todo!("recvmmsg")
}
#[no_mangle]
unsafe extern "C" fn sendmmsg() {
    todo!("sendmmsg")
}
#[no_mangle]
unsafe extern "C" fn timer_create() {
    todo!("timer_create")
}
#[no_mangle]
unsafe extern "C" fn timer_delete() {
    todo!("timer_delete")
}
#[no_mangle]
unsafe extern "C" fn timer_gettime() {
    todo!("timer_gettime")
}
#[no_mangle]
unsafe extern "C" fn timer_settime() {
    todo!("timer_settime")
}
#[no_mangle]
unsafe extern "C" fn timer_getoverrun() {
    todo!("timer_getoverrun")
}
#[no_mangle]
unsafe extern "C" fn truncate() {
    todo!("truncate")
}
#[no_mangle]
unsafe extern "C" fn truncate64() {
    todo!("truncate64")
}
#[no_mangle]
unsafe extern "C" fn personality() {
    todo!("personality")
}
#[no_mangle]
unsafe extern "C" fn ctermid() {
    todo!("ctermid")
}
#[no_mangle]
unsafe extern "C" fn cuserid() {
    todo!("cuserid")
}
#[no_mangle]
unsafe extern "C" fn __getdelim() {
    todo!("__getdelim")
}
#[no_mangle]
unsafe extern "C" fn __getline() {
    todo!("__getline")
}
#[no_mangle]
unsafe extern "C" fn getrusage() {
    todo!("getrusage")
}
#[no_mangle]
unsafe extern "C" fn pclose() {
    todo!("pclose")
}
#[no_mangle]
unsafe extern "C" fn popen() {
    todo!("popen")
}
#[no_mangle]
unsafe extern "C" fn tempnam() {
    todo!("tempnam")
}
#[no_mangle]
unsafe extern "C" fn tmpnam() {
    todo!("tmpnam")
}
#[no_mangle]
unsafe extern "C" fn ungetc() {
    todo!("ungetc")
}
#[no_mangle]
unsafe extern "C" fn vfscanf() {
    todo!("vfscanf")
}
#[no_mangle]
unsafe extern "C" fn vscanf() {
    todo!("vscanf")
}
#[no_mangle]
unsafe extern "C" fn vsscanf() {
    todo!("vsscanf")
}
#[no_mangle]
unsafe extern "C" fn error() {
    todo!("error")
}
#[no_mangle]
unsafe extern "C" fn error_at_line() {
    todo!("error_at_line")
}
#[no_mangle]
unsafe extern "C" fn fopencookie() {
    todo!("fopencookie")
}
#[no_mangle]
unsafe extern "C" fn getdelim() {
    todo!("getdelim")
}
#[no_mangle]
unsafe extern "C" fn getline() {
    todo!("getline")
}
#[no_mangle]
unsafe extern "C" fn getopt_long() {
    todo!("getopt_long")
}
#[no_mangle]
unsafe extern "C" fn getopt_long_only() {
    todo!("getopt_long_only")
}
#[no_mangle]
unsafe extern "C" fn ns_get16() {
    todo!("ns_get16")
}
#[no_mangle]
unsafe extern "C" fn ns_get32() {
    todo!("ns_get32")
}
#[no_mangle]
unsafe extern "C" fn ns_put16() {
    todo!("ns_put16")
}
#[no_mangle]
unsafe extern "C" fn ns_put32() {
    todo!("ns_put32")
}
#[no_mangle]
unsafe extern "C" fn open_memstream() {
    todo!("open_memstream")
}
#[no_mangle]
unsafe extern "C" fn optarg() {
    todo!("optarg")
}
#[no_mangle]
unsafe extern "C" fn optind() {
    todo!("optind")
}
#[no_mangle]
unsafe extern "C" fn optopt() {
    todo!("optopt")
}
#[no_mangle]
unsafe extern "C" fn opterr() {
    todo!("opterr")
}
#[no_mangle]
unsafe extern "C" fn posix_spawn() {
    todo!("posix_spawn")
}
#[no_mangle]
unsafe extern "C" fn pthread_attr_getinheritsched() {
    todo!("pthread_attr_getinheritsched")
}
#[no_mangle]
unsafe extern "C" fn pthread_attr_getschedparam() {
    todo!("pthread_attr_getschedparam")
}
#[no_mangle]
unsafe extern "C" fn pthread_attr_getschedpolicy() {
    todo!("pthread_attr_getschedpolicy")
}
#[no_mangle]
unsafe extern "C" fn pthread_attr_getscope() {
    todo!("pthread_attr_getscope")
}
#[no_mangle]
unsafe extern "C" fn pthread_attr_setinheritsched() {
    todo!("pthread_attr_setinheritsched")
}
#[no_mangle]
unsafe extern "C" fn pthread_attr_setschedparam() {
    todo!("pthread_attr_setschedparam")
}
#[no_mangle]
unsafe extern "C" fn pthread_attr_setschedpolicy() {
    todo!("pthread_attr_setschedpolicy")
}
#[no_mangle]
unsafe extern "C" fn pthread_attr_setscope() {
    todo!("pthread_attr_setscope")
}
#[no_mangle]
unsafe extern "C" fn pthread_barrierattr_destroy() {
    todo!("pthread_barrierattr_destroy")
}
#[no_mangle]
unsafe extern "C" fn pthread_barrierattr_getpshared() {
    todo!("pthread_barrierattr_getpshared")
}
#[no_mangle]
unsafe extern "C" fn pthread_barrierattr_init() {
    todo!("pthread_barrierattr_init")
}
#[no_mangle]
unsafe extern "C" fn pthread_barrierattr_setpshared() {
    todo!("pthread_barrierattr_setpshared")
}
#[no_mangle]
unsafe extern "C" fn pthread_barrier_destroy() {
    todo!("pthread_barrier_destroy")
}
#[no_mangle]
unsafe extern "C" fn pthread_barrier_init() {
    todo!("pthread_barrier_init")
}
#[no_mangle]
unsafe extern "C" fn pthread_barrier_wait() {
    todo!("pthread_barrier_wait")
}
#[no_mangle]
unsafe extern "C" fn pthread_condattr_getclock() {
    todo!("pthread_condattr_getclock")
}
#[no_mangle]
unsafe extern "C" fn pthread_condattr_getpshared() {
    todo!("pthread_condattr_getpshared")
}
#[no_mangle]
unsafe extern "C" fn pthread_condattr_setpshared() {
    todo!("pthread_condattr_setpshared")
}
#[no_mangle]
unsafe extern "C" fn pthread_getschedparam() {
    todo!("pthread_getschedparam")
}
#[no_mangle]
unsafe extern "C" fn pthread_mutexattr_getprotocol() {
    todo!("pthread_mutexattr_getprotocol")
}
#[no_mangle]
unsafe extern "C" fn pthread_mutexattr_getpshared() {
    todo!("pthread_mutexattr_getpshared")
}
#[no_mangle]
unsafe extern "C" fn pthread_mutexattr_getrobust() {
    todo!("pthread_mutexattr_getrobust")
}
#[no_mangle]
unsafe extern "C" fn pthread_mutexattr_setprotocol() {
    todo!("pthread_mutexattr_setprotocol")
}
#[no_mangle]
unsafe extern "C" fn pthread_mutexattr_setpshared() {
    todo!("pthread_mutexattr_setpshared")
}
#[no_mangle]
unsafe extern "C" fn pthread_mutexattr_setrobust() {
    todo!("pthread_mutexattr_setrobust")
}
#[no_mangle]
unsafe extern "C" fn pthread_rwlockattr_getpshared() {
    todo!("pthread_rwlockattr_getpshared")
}
#[no_mangle]
unsafe extern "C" fn pthread_rwlockattr_setpshared() {
    todo!("pthread_rwlockattr_setpshared")
}
#[no_mangle]
unsafe extern "C" fn pthread_setschedparam() {
    todo!("pthread_setschedparam")
}
#[cfg(not(feature = "std"))]
#[no_mangle]
unsafe extern "C" fn system() {
    todo!("system")
}
#[no_mangle]
unsafe extern "C" fn crypt() {
    todo!("crypt")
}
#[no_mangle]
unsafe extern "C" fn dn_expand() {
    todo!("dn_expand")
}
#[no_mangle]
unsafe extern "C" fn fmemopen() {
    todo!("fmemopen")
}
#[no_mangle]
unsafe extern "C" fn fnmatch() {
    todo!("fnmatch")
}
#[no_mangle]
unsafe extern "C" fn getopt() {
    todo!("getopt")
}
#[no_mangle]
unsafe extern "C" fn hcreate() {
    todo!("hcreate")
}
#[no_mangle]
unsafe extern "C" fn hdestroy() {
    todo!("hdestroy")
}
#[no_mangle]
unsafe extern "C" fn hsearch() {
    todo!("hsearch")
}
#[no_mangle]
unsafe extern "C" fn iconv() {
    todo!("iconv")
}
#[no_mangle]
unsafe extern "C" fn iconv_close() {
    todo!("iconv_close")
}
#[no_mangle]
unsafe extern "C" fn iconv_open() {
    todo!("iconv_open")
}
#[no_mangle]
unsafe extern "C" fn __isoc99_fscanf() {
    todo!("__isoc99_fscanf")
}
#[no_mangle]
unsafe extern "C" fn __isoc99_fwscanf() {
    todo!("__isoc99_fwscanf")
}
#[no_mangle]
unsafe extern "C" fn lfind() {
    todo!("lfind")
}
#[no_mangle]
unsafe extern "C" fn lsearch() {
    todo!("lsearch")
}
#[no_mangle]
unsafe extern "C" fn tsearch() {
    todo!("tsearch")
}
#[no_mangle]
unsafe extern "C" fn tfind() {
    todo!("tfind")
}
#[no_mangle]
unsafe extern "C" fn tdelete() {
    todo!("tdelete")
}
#[no_mangle]
unsafe extern "C" fn twalk() {
    todo!("twalk")
}
#[no_mangle]
unsafe extern "C" fn posix_spawn_file_actions_addclose() {
    todo!("posix_spawn_file_actions_addclose")
}
#[no_mangle]
unsafe extern "C" fn pthread_mutex_consistent() {
    todo!("pthread_mutex_consistent")
}
#[no_mangle]
unsafe extern "C" fn pthread_mutex_timedlock() {
    todo!("pthread_mutex_timedlock")
}
#[no_mangle]
unsafe extern "C" fn insque() {
    todo!("insque")
}
#[no_mangle]
unsafe extern "C" fn remque() {
    todo!("remque")
}
#[no_mangle]
unsafe extern "C" fn sem_close() {
    todo!("sem_close")
}
#[no_mangle]
unsafe extern "C" fn sem_destroy() {
    todo!("sem_destroy")
}
#[no_mangle]
unsafe extern "C" fn sem_getvalue() {
    todo!("sem_getvalue")
}
#[no_mangle]
unsafe extern "C" fn sem_init() {
    todo!("sem_init")
}
#[no_mangle]
unsafe extern "C" fn sem_open() {
    todo!("sem_open")
}
#[no_mangle]
unsafe extern "C" fn sem_post() {
    todo!("sem_post")
}
#[no_mangle]
unsafe extern "C" fn sem_timedwait() {
    todo!("sem_timedwait")
}
#[no_mangle]
unsafe extern "C" fn sem_trywait() {
    todo!("sem_trywait")
}
#[no_mangle]
unsafe extern "C" fn sem_unlink() {
    todo!("sem_unlink")
}
#[no_mangle]
unsafe extern "C" fn sem_wait() {
    todo!("sem_wait")
}
#[no_mangle]
unsafe extern "C" fn initstate() {
    todo!("initstate")
}
#[no_mangle]
unsafe extern "C" fn setstate() {
    todo!("setstate")
}
#[no_mangle]
unsafe extern "C" fn random() {
    todo!("random")
}
#[no_mangle]
unsafe extern "C" fn srandom() {
    todo!("srandom")
}
#[no_mangle]
unsafe extern "C" fn strptime() {
    todo!("strptime")
}
#[no_mangle]
unsafe extern "C" fn alphasort() {
    todo!("alphasort")
}
#[no_mangle]
unsafe extern "C" fn asctime() {
    todo!("asctime")
}
#[no_mangle]
unsafe extern "C" fn ctime() {
    todo!("ctime")
}
#[no_mangle]
unsafe extern "C" fn ctime_r() {
    todo!("ctime_r")
}
#[no_mangle]
unsafe extern "C" fn __isoc99_vsscanf() {
    todo!("__isoc99_vsscanf")
}
#[no_mangle]
unsafe extern "C" fn times() {
    todo!("times")
}
#[no_mangle]
unsafe extern "C" fn pthread_getcpuclockid() {
    todo!("pthread_getcpuclockid")
}
#[no_mangle]
unsafe extern "C" fn pthread_mutexattr_getprioceiling() {
    todo!("pthread_mutexattr_getprioceiling")
}
#[no_mangle]
unsafe extern "C" fn pthread_mutexattr_setprioceiling() {
    todo!("pthread_mutexattr_setprioceiling")
}
#[no_mangle]
unsafe extern "C" fn pthread_mutex_getprioceiling() {
    todo!("pthread_mutex_getprioceiling")
}
#[no_mangle]
unsafe extern "C" fn pthread_rwlock_timedrdlock() {
    todo!("pthread_rwlock_timedrdlock")
}
#[no_mangle]
unsafe extern "C" fn pthread_rwlock_timedwrlock() {
    todo!("pthread_rwlock_timedwrlock")
}
#[no_mangle]
unsafe extern "C" fn pthread_setaffinity_np() {
    todo!("pthread_setaffinity_np")
}
#[no_mangle]
unsafe extern "C" fn pthread_setschedprio() {
    todo!("pthread_setschedprio")
}
#[no_mangle]
unsafe extern "C" fn sched_getparam() {
    todo!("sched_getparam")
}
#[no_mangle]
unsafe extern "C" fn sched_get_priority_max() {
    todo!("sched_get_priority_max")
}
#[no_mangle]
unsafe extern "C" fn sched_get_priority_min() {
    todo!("sched_get_priority_min")
}
#[no_mangle]
unsafe extern "C" fn sched_getscheduler() {
    todo!("sched_getscheduler")
}
#[no_mangle]
unsafe extern "C" fn sched_rr_get_interval() {
    todo!("sched_rr_get_interval")
}
#[no_mangle]
unsafe extern "C" fn sched_setparam() {
    todo!("sched_setparam")
}
#[no_mangle]
unsafe extern "C" fn sched_setscheduler() {
    todo!("sched_setscheduler")
}
#[no_mangle]
unsafe extern "C" fn sigqueue() {
    todo!("sigqueue")
}
#[no_mangle]
unsafe extern "C" fn __isoc99_vfscanf() {
    todo!("__isoc99_vfscanf")
}
#[no_mangle]
unsafe extern "C" fn mincore() {
    todo!("mincore")
}
#[no_mangle]
unsafe extern "C" fn setns() {
    todo!("setns")
}
#[no_mangle]
unsafe extern "C" fn mq_close() {
    todo!("mq_close")
}
#[no_mangle]
unsafe extern "C" fn mq_getattr() {
    todo!("mq_getattr")
}
#[no_mangle]
unsafe extern "C" fn mq_notify() {
    todo!("mq_notify")
}
#[no_mangle]
unsafe extern "C" fn mq_open() {
    todo!("mq_open")
}
#[no_mangle]
unsafe extern "C" fn mq_receive() {
    todo!("mq_receive")
}
#[no_mangle]
unsafe extern "C" fn mq_send() {
    todo!("mq_send")
}
#[no_mangle]
unsafe extern "C" fn mq_setattr() {
    todo!("mq_setattr")
}
#[no_mangle]
unsafe extern "C" fn mq_timedreceive() {
    todo!("mq_timedreceive")
}
#[no_mangle]
unsafe extern "C" fn mq_timedsend() {
    todo!("mq_timedsend")
}
#[no_mangle]
unsafe extern "C" fn mq_unlink() {
    todo!("mq_unlink")
}
#[no_mangle]
unsafe extern "C" fn setitimer() {
    todo!("setitimer")
}
#[no_mangle]
unsafe extern "C" fn getitimer() {
    todo!("getitimer")
}
#[no_mangle]
unsafe extern "C" fn adjtimex() {
    todo!("adjtimex")
}
#[no_mangle]
unsafe extern "C" fn clone() {
    todo!("clone")
}
#[no_mangle]
unsafe extern "C" fn close_range() {
    todo!("close_range")
}
#[no_mangle]
unsafe extern "C" fn confstr() {
    todo!("confstr")
}
#[no_mangle]
unsafe extern "C" fn getdtablesize() {
    todo!("getdtablesize")
}
#[no_mangle]
unsafe extern "C" fn execveat() {
    todo!("execveat")
}
#[no_mangle]
unsafe extern "C" fn fanotify_init() {
    todo!("fanotify_init")
}
#[no_mangle]
unsafe extern "C" fn fanotify_mark() {
    todo!("fanotify_mark")
}
#[no_mangle]
unsafe extern "C" fn name_to_handle_at() {
    todo!("name_to_handle_at")
}
#[no_mangle]
unsafe extern "C" fn mkdtemp() {
    todo!("mkdtemp")
}
#[no_mangle]
unsafe extern "C" fn addseverity() {
    todo!("addseverity")
}
#[no_mangle]
unsafe extern "C" fn fmtmsg() {
    todo!("fmtmsg")
}
#[no_mangle]
unsafe extern "C" fn getcpu() {
    todo!("getcpu")
}
#[no_mangle]
unsafe extern "C" fn getdents64() {
    todo!("getdents64")
}
#[no_mangle]
unsafe extern "C" fn getdomainname() {
    todo!("getdomainname")
}
#[no_mangle]
unsafe extern "C" fn ioperm() {
    todo!("ioperm")
}
#[no_mangle]
unsafe extern "C" fn iopl() {
    todo!("iopl")
}
#[no_mangle]
unsafe extern "C" fn get_current_dir_name() {
    todo!("get_current_dir_name")
}
#[no_mangle]
unsafe extern "C" fn mallopt() {
    todo!("mallopt")
}
#[no_mangle]
unsafe extern "C" fn malloc_stats() {
    todo!("malloc_stats")
}
#[no_mangle]
unsafe extern "C" fn mallinfo() {
    todo!("mallinfo")
}
#[no_mangle]
unsafe extern "C" fn mallinfo2() {
    todo!("mallinfo2")
}
#[no_mangle]
unsafe extern "C" fn open_by_handle_at() {
    todo!("open_by_handle_at")
}
#[no_mangle]
unsafe extern "C" fn nftw() {
    todo!("nftw")
}
#[no_mangle]
unsafe extern "C" fn nftw64() {
    todo!("nftw64")
}
#[no_mangle]
unsafe extern "C" fn pkey_mprotect() {
    todo!("pkey_mprotect")
}
#[no_mangle]
unsafe extern "C" fn pkey_free() {
    todo!("pkey_free")
}
#[no_mangle]
unsafe extern "C" fn pkey_alloc() {
    todo!("pkey_alloc")
}
#[no_mangle]
unsafe extern "C" fn wait4() {
    todo!("wait4")
}
#[no_mangle]
unsafe extern "C" fn profil() {
    todo!("profil")
}
#[no_mangle]
unsafe extern "C" fn quotactl() {
    todo!("quotactl")
}
#[no_mangle]
unsafe extern "C" fn readahead() {
    todo!("readahead")
}
#[no_mangle]
unsafe extern "C" fn remap_file_pages() {
    todo!("remap_file_pages")
}
#[no_mangle]
unsafe extern "C" fn setdomainname() {
    todo!("setdomainname")
}
#[no_mangle]
unsafe extern "C" fn settimeofday() {
    todo!("settimeofday")
}
#[no_mangle]
unsafe extern "C" fn ulimit() {
    todo!("ulimit")
}
#[no_mangle]
unsafe extern "C" fn fts_read() {
    todo!("fts_read")
}
#[no_mangle]
unsafe extern "C" fn fts_open() {
    todo!("fts_open")
}
#[no_mangle]
unsafe extern "C" fn fts_close() {
    todo!("fts_close")
}
#[no_mangle]
unsafe extern "C" fn __open_2() {
    todo!("__open_2")
}
#[no_mangle]
unsafe extern "C" fn wordexp() {
    todo!("wordexp")
}
#[no_mangle]
unsafe extern "C" fn wordfree() {
    todo!("wordfree")
}
#[no_mangle]
unsafe extern "C" fn ftw() {
    todo!("ftw")
}
#[no_mangle]
unsafe extern "C" fn getpass() {
    todo!("getpass")
}
#[no_mangle]
unsafe extern "C" fn getusershell() {
    todo!("getusershell")
}
#[no_mangle]
unsafe extern "C" fn glob() {
    todo!("glob")
}
#[no_mangle]
unsafe extern "C" fn on_exit() {
    todo!("on_exit")
}
#[no_mangle]
unsafe extern "C" fn printf_size() {
    todo!("printf_size")
}
#[no_mangle]
unsafe extern "C" fn printf_size_info() {
    todo!("printf_size_info")
}
#[no_mangle]
unsafe extern "C" fn register_printf_function() {
    todo!("register_printf_function")
}
#[no_mangle]
unsafe extern "C" fn adjtime() {
    todo!("adjtime")
}
#[no_mangle]
unsafe extern "C" fn gamma() {
    todo!("gamma")
}
#[no_mangle]
unsafe extern "C" fn gammaf() {
    todo!("gammaf")
}
#[no_mangle]
unsafe extern "C" fn strfromf() {
    todo!("strfromf")
}
#[no_mangle]
unsafe extern "C" fn hstrerror() {
    todo!("hstrerror")
}
#[no_mangle]
unsafe extern "C" fn getdate() {
    todo!("getdate")
}
#[no_mangle]
unsafe extern "C" fn getdate_err() {
    todo!("getdate_err")
}
#[no_mangle]
unsafe extern "C" fn backtrace() {
    todo!("backtrace")
}
#[no_mangle]
unsafe extern "C" fn backtrace_symbols() {
    todo!("backtrace_symbols")
}
#[no_mangle]
unsafe extern "C" fn getwc() {
    todo!("getwc")
}
#[no_mangle]
unsafe extern "C" fn putwc() {
    todo!("putwc")
}
