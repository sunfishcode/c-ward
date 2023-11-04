//! Unimplemented stub functions. These may help porting programs which
//! need these functions to link but don't (always) call them at runtime.

#[no_mangle]
unsafe extern "C" fn qsort() {
    todo!("qsort")
}
#[no_mangle]
unsafe extern "C" fn bsearch() {
    todo!("bsearch")
}
#[no_mangle]
unsafe extern "C" fn freeifaddrs() {
    todo!("freeifaddrs")
}
#[no_mangle]
unsafe extern "C" fn getifaddrs() {
    todo!("getifaddrs")
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
unsafe extern "C" fn alarm() {
    todo!("alarm")
}
#[no_mangle]
unsafe extern "C" fn sysinfo() {
    todo!("sysinfo")
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
unsafe extern "C" fn waitid() {
    todo!("waitid")
}
#[no_mangle]
unsafe extern "C" fn signalfd() {
    todo!("signalfd")
}
#[no_mangle]
unsafe extern "C" fn posix_fallocate() {
    todo!("posix_fallocate")
}
#[no_mangle]
unsafe extern "C" fn daemon() {
    todo!("daemon")
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
unsafe extern "C" fn initgroups() {
    todo!("initgroups")
}
#[no_mangle]
unsafe extern "C" fn fopen() {
    todo!("fopen")
}
#[no_mangle]
unsafe extern "C" fn fopen64() {
    todo!("fopen64")
}
#[no_mangle]
unsafe extern "C" fn fread() {
    todo!("fread")
}
#[no_mangle]
unsafe extern "C" fn ftell() {
    todo!("ftell")
}
#[no_mangle]
unsafe extern "C" fn ferror() {
    todo!("ferror")
}
#[no_mangle]
unsafe extern "C" fn strftime() {
    todo!("strftime")
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
unsafe extern "C" fn pthread_once() {
    todo!("pthread_once")
}
#[no_mangle]
unsafe extern "C" fn getcontext() {
    todo!("getcontext")
}
#[no_mangle]
unsafe extern "C" fn setcontext() {
    todo!("setcontext")
}
#[no_mangle]
unsafe extern "C" fn makecontext() {
    todo!("makecontext")
}
#[no_mangle]
unsafe extern "C" fn swapcontext() {
    todo!("swapcontext")
}
#[no_mangle]
unsafe extern "C" fn select() {
    todo!("select")
}
#[no_mangle]
unsafe extern "C" fn shmat() {
    todo!("shmat")
}
#[no_mangle]
unsafe extern "C" fn shmget() {
    todo!("shmget")
}
#[no_mangle]
unsafe extern "C" fn shmdt() {
    todo!("shmdt")
}
#[no_mangle]
unsafe extern "C" fn __fdelt_chk() {
    todo!("__fdelt_chk")
}
#[no_mangle]
unsafe extern "C" fn _setjmp() {
    todo!("_setjmp")
}
#[no_mangle]
unsafe extern "C" fn _longjmp() {
    todo!("_longjmp")
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
unsafe extern "C" fn getmntent() {
    todo!("getmntent")
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
unsafe extern "C" fn clock_getres() {
    todo!("clock_getres")
}
#[no_mangle]
unsafe extern "C" fn eaccess() {
    todo!("eaccess")
}
#[no_mangle]
unsafe extern "C" fn fchownat() {
    todo!("fchownat")
}
#[no_mangle]
unsafe extern "C" fn forkpty() {
    todo!("forkpty")
}
#[no_mangle]
unsafe extern "C" fn fstatfs64() {
    todo!("fstatfs64")
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
unsafe extern "C" fn if_nametoindex() {
    todo!("if_nametoindex")
}
#[no_mangle]
unsafe extern "C" fn pause() {
    todo!("pause")
}
#[no_mangle]
unsafe extern "C" fn ppoll() {
    todo!("ppoll")
}
#[no_mangle]
unsafe extern "C" fn pselect() {
    todo!("pselect")
}
#[no_mangle]
unsafe extern "C" fn ptrace() {
    todo!("ptrace")
}
#[no_mangle]
unsafe extern "C" fn ptsname() {
    todo!("ptsname")
}
#[no_mangle]
unsafe extern "C" fn ptsname_r() {
    todo!("ptsname_r")
}
#[no_mangle]
unsafe extern "C" fn recvmmsg() {
    todo!("recvmmsg")
}
#[no_mangle]
unsafe extern "C" fn renameat2() {
    todo!("renameat2")
}
#[no_mangle]
unsafe extern "C" fn sched_getcpu() {
    todo!("sched_getcpu")
}
#[no_mangle]
unsafe extern "C" fn sched_setaffinity() {
    todo!("sched_setaffinity")
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
unsafe extern "C" fn timerfd_gettime() {
    todo!("timerfd_gettime")
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
unsafe extern "C" fn truncate() {
    todo!("truncate")
}
#[no_mangle]
unsafe extern "C" fn personality() {
    todo!("personality")
}
#[no_mangle]
unsafe extern "C" fn clearerr() {
    todo!("clearerr")
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
unsafe extern "C" fn fclose() {
    todo!("fclose")
}
#[no_mangle]
unsafe extern "C" fn fdopen() {
    todo!("fdopen")
}
#[no_mangle]
unsafe extern "C" fn feof() {
    todo!("feof")
}
#[no_mangle]
unsafe extern "C" fn fgetc() {
    todo!("fgetc")
}
#[no_mangle]
unsafe extern "C" fn fgetpos() {
    todo!("fgetpos")
}
#[no_mangle]
unsafe extern "C" fn flockfile() {
    todo!("flockfile")
}
#[no_mangle]
unsafe extern "C" fn __fpurge() {
    todo!("__fpurge")
}
#[no_mangle]
unsafe extern "C" fn freopen() {
    todo!("freopen")
}
#[no_mangle]
unsafe extern "C" fn fseeko() {
    todo!("fseeko")
}
#[no_mangle]
unsafe extern "C" fn fsetpos() {
    todo!("fsetpos")
}
#[no_mangle]
unsafe extern "C" fn ftell_locked() {
    todo!("ftell_locked")
}
#[no_mangle]
unsafe extern "C" fn ftello() {
    todo!("ftello")
}
#[no_mangle]
unsafe extern "C" fn ftrylockfile() {
    todo!("ftrylockfile")
}
#[no_mangle]
unsafe extern "C" fn funlockfile() {
    todo!("funlockfile")
}
#[no_mangle]
unsafe extern "C" fn getc() {
    todo!("getc")
}
#[no_mangle]
unsafe extern "C" fn getchar() {
    todo!("getchar")
}
#[no_mangle]
unsafe extern "C" fn getchar_unlocked() {
    todo!("getchar_unlocked")
}
#[no_mangle]
unsafe extern "C" fn getc_unlocked() {
    todo!("getc_unlocked")
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
unsafe extern "C" fn gets() {
    unimplemented!("gets")
}
#[no_mangle]
unsafe extern "C" fn getw() {
    todo!("getw")
}
#[no_mangle]
unsafe extern "C" fn inet_addr() {
    todo!("inet_addr")
}
#[no_mangle]
unsafe extern "C" fn inet_aton() {
    todo!("inet_aton")
}
#[no_mangle]
unsafe extern "C" fn inet_lnaof() {
    todo!("inet_lnaof")
}
#[no_mangle]
unsafe extern "C" fn inet_makeaddr() {
    todo!("inet_makeaddr")
}
#[no_mangle]
unsafe extern "C" fn inet_netof() {
    todo!("inet_netof")
}
#[no_mangle]
unsafe extern "C" fn inet_network() {
    todo!("inet_network")
}
#[no_mangle]
unsafe extern "C" fn inet_ntoa() {
    todo!("inet_ntoa")
}
#[no_mangle]
unsafe extern "C" fn inet_ntop() {
    todo!("inet_ntop")
}
#[no_mangle]
unsafe extern "C" fn inet_pton() {
    todo!("inet_pton")
}
#[no_mangle]
unsafe extern "C" fn mkostemp() {
    todo!("mkostemp")
}
#[no_mangle]
unsafe extern "C" fn mkostemps() {
    todo!("mkostemps")
}
#[no_mangle]
unsafe extern "C" fn mkstemp() {
    todo!("mkstemp")
}
#[no_mangle]
unsafe extern "C" fn mkstemps() {
    todo!("mkstemps")
}
#[no_mangle]
unsafe extern "C" fn mktemp() {
    todo!("mktemp")
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
unsafe extern "C" fn putchar_unlocked() {
    todo!("putchar_unlocked")
}
#[no_mangle]
unsafe extern "C" fn putc_unlocked() {
    todo!("putc_unlocked")
}
#[no_mangle]
unsafe extern "C" fn putw() {
    todo!("putw")
}
#[no_mangle]
unsafe extern "C" fn rewind() {
    todo!("rewind")
}
#[no_mangle]
unsafe extern "C" fn setbuf() {
    todo!("setbuf")
}
#[no_mangle]
unsafe extern "C" fn setvbuf() {
    todo!("setvbuf")
}
#[no_mangle]
unsafe extern "C" fn strtoimax() {
    todo!("strtoimax")
}
#[no_mangle]
unsafe extern "C" fn strtoumax() {
    todo!("strtoumax")
}
#[no_mangle]
unsafe extern "C" fn tempnam() {
    todo!("tempnam")
}
#[no_mangle]
unsafe extern "C" fn tmpfile() {
    todo!("tmpfile")
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
unsafe extern "C" fn vasprintf() {
    todo!("vasprintf")
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
unsafe extern "C" fn basename() {
    todo!("basename")
}
#[no_mangle]
unsafe extern "C" fn endgrent() {
    todo!("endgrent")
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
unsafe extern "C" fn feclearexcept() {
    todo!("feclearexcept")
}
#[no_mangle]
unsafe extern "C" fn fegetenv() {
    todo!("fegetenv")
}
#[no_mangle]
unsafe extern "C" fn fegetround() {
    todo!("fegetround")
}
#[no_mangle]
unsafe extern "C" fn feraiseexcept() {
    todo!("feraiseexcept")
}
#[no_mangle]
unsafe extern "C" fn fesetenv() {
    todo!("fesetenv")
}
#[no_mangle]
unsafe extern "C" fn fesetround() {
    todo!("fesetround")
}
#[no_mangle]
unsafe extern "C" fn fetestexcept() {
    todo!("fetestexcept")
}
#[no_mangle]
unsafe extern "C" fn ffs() {
    todo!("ffs")
}
#[no_mangle]
unsafe extern "C" fn ffsl() {
    todo!("ffsl")
}
#[no_mangle]
unsafe extern "C" fn ffsll() {
    todo!("ffsll")
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
unsafe extern "C" fn getgrent() {
    todo!("getgrent")
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
unsafe extern "C" fn getservbyname() {
    todo!("getservbyname")
}
#[no_mangle]
unsafe extern "C" fn getservbyport() {
    todo!("getservbyport")
}
#[no_mangle]
unsafe extern "C" fn if_indextoname() {
    todo!("if_indextoname")
}
#[no_mangle]
unsafe extern "C" fn longjmp() {
    todo!("longjmp")
}
#[no_mangle]
unsafe extern "C" fn mbrtoc32() {
    todo!("mbrtoc32")
}
#[no_mangle]
unsafe extern "C" fn mbrtowc() {
    todo!("mbrtowc")
}
#[no_mangle]
unsafe extern "C" fn mbstowcs() {
    todo!("mbstowcs")
}
#[no_mangle]
unsafe extern "C" fn mbtowc() {
    todo!("mbtowc")
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
unsafe extern "C" fn posix_spawn() {
    todo!("posix_spawn")
}
#[no_mangle]
unsafe extern "C" fn pthread_attr_getdetachstate() {
    todo!("pthread_attr_getdetachstate")
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
unsafe extern "C" fn pthread_attr_getstacksize() {
    todo!("pthread_attr_getstacksize")
}
#[no_mangle]
unsafe extern "C" fn pthread_attr_setdetachstate() {
    todo!("pthread_attr_setdetachstate")
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
unsafe extern "C" fn pthread_attr_setstack() {
    todo!("pthread_attr_setstack")
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
unsafe extern "C" fn pthread_getname_np() {
    todo!("pthread_getname_np")
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
unsafe extern "C" fn pthread_mutexattr_gettype() {
    todo!("pthread_mutexattr_gettype")
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
unsafe extern "C" fn __pthread_register_cancel() {
    todo!("__pthread_register_cancel")
}
#[no_mangle]
unsafe extern "C" fn pthread_rwlockattr_destroy() {
    todo!("pthread_rwlockattr_destroy")
}
#[no_mangle]
unsafe extern "C" fn pthread_rwlockattr_getpshared() {
    todo!("pthread_rwlockattr_getpshared")
}
#[no_mangle]
unsafe extern "C" fn pthread_rwlockattr_init() {
    todo!("pthread_rwlockattr_init")
}
#[no_mangle]
unsafe extern "C" fn pthread_rwlockattr_setpshared() {
    todo!("pthread_rwlockattr_setpshared")
}
#[no_mangle]
unsafe extern "C" fn pthread_setschedparam() {
    todo!("pthread_setschedparam")
}
#[no_mangle]
unsafe extern "C" fn __pthread_unregister_cancel() {
    todo!("__pthread_unregister_cancel")
}
#[no_mangle]
unsafe extern "C" fn reallocarray() {
    todo!("reallocarray")
}
#[no_mangle]
unsafe extern "C" fn regcomp() {
    todo!("regcomp")
}
#[no_mangle]
unsafe extern "C" fn regexec() {
    todo!("regexec")
}
#[no_mangle]
unsafe extern "C" fn regfree() {
    todo!("regfree")
}
#[no_mangle]
unsafe extern "C" fn sbrk() {
    todo!("sbrk")
}
#[no_mangle]
unsafe extern "C" fn setgrent() {
    todo!("setgrent")
}
#[no_mangle]
unsafe extern "C" fn setlocale() {
    todo!("setlocale")
}
#[no_mangle]
unsafe extern "C" fn sigsuspend() {
    todo!("sigsuspend")
}
#[no_mangle]
unsafe extern "C" fn strxfrm() {
    todo!("strxfrm")
}
#[no_mangle]
unsafe extern "C" fn system() {
    todo!("system")
}
#[no_mangle]
unsafe extern "C" fn tfind() {
    todo!("tfind")
}
#[no_mangle]
unsafe extern "C" fn tsearch() {
    todo!("tsearch")
}
#[no_mangle]
unsafe extern "C" fn wcsdup() {
    todo!("wcsdup")
}
#[no_mangle]
unsafe extern "C" fn wcslen() {
    todo!("wcslen")
}
#[no_mangle]
unsafe extern "C" fn wcsncasecmp() {
    todo!("wcsncasecmp")
}
#[no_mangle]
unsafe extern "C" fn wcsrtombs() {
    todo!("wcsrtombs")
}
#[no_mangle]
unsafe extern "C" fn wcstol() {
    todo!("wcstol")
}
#[no_mangle]
unsafe extern "C" fn wcstoll() {
    todo!("wcstoll")
}
#[no_mangle]
unsafe extern "C" fn wcstoul() {
    todo!("wcstoul")
}
#[no_mangle]
unsafe extern "C" fn wcstoull() {
    todo!("wcstoull")
}
#[no_mangle]
unsafe extern "C" fn wmemcmp() {
    todo!("wmemcmp")
}
#[no_mangle]
unsafe extern "C" fn __xpg_basename() {
    todo!("__xpg_basename")
}
