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
#[no_mangle]
unsafe extern "C" fn __pthread_unregister_cancel() {
    todo!("__pthread_unregister_cancel")
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
#[deprecated]
#[no_mangle]
unsafe extern "C" fn sighold() {
    todo!("sighold")
}
#[deprecated]
#[no_mangle]
unsafe extern "C" fn sigignore() {
    todo!("sigignore")
}
#[deprecated]
#[no_mangle]
unsafe extern "C" fn sigrelse() {
    todo!("sigrelse")
}
#[deprecated]
#[no_mangle]
unsafe extern "C" fn sigset() {
    todo!("sigset")
}
#[no_mangle]
unsafe extern "C" fn __xpg_sigpause() {
    todo!("__xpg_sigpause")
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
#[no_mangle]
unsafe extern "C" fn btowc() {
    todo!("btowc")
}
#[no_mangle]
unsafe extern "C" fn clearenv() {
    todo!("clearenv")
}
#[no_mangle]
unsafe extern "C" fn copysignl() {
    todo!("copysignl")
}
#[no_mangle]
unsafe extern "C" fn crypt() {
    todo!("crypt")
}
#[no_mangle]
unsafe extern "C" fn dirname() {
    todo!("dirname")
}
#[no_mangle]
unsafe extern "C" fn dn_expand() {
    todo!("dn_expand")
}
#[no_mangle]
unsafe extern "C" fn _Exit() {
    todo!("_Exit")
}
#[no_mangle]
unsafe extern "C" fn fgetwc() {
    todo!("fgetwc")
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
unsafe extern "C" fn ftok() {
    todo!("ftok")
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
unsafe extern "C" fn initstate() {
    todo!("initstate")
}
#[no_mangle]
unsafe extern "C" fn insque() {
    todo!("insque")
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
unsafe extern "C" fn iswalnum() {
    todo!("iswalnum")
}
#[no_mangle]
unsafe extern "C" fn iswalpha() {
    todo!("iswalpha")
}
#[no_mangle]
unsafe extern "C" fn iswblank() {
    todo!("iswblank")
}
#[no_mangle]
unsafe extern "C" fn iswcntrl() {
    todo!("iswcntrl")
}
#[no_mangle]
unsafe extern "C" fn iswdigit() {
    todo!("iswdigit")
}
#[no_mangle]
unsafe extern "C" fn iswgraph() {
    todo!("iswgraph")
}
#[no_mangle]
unsafe extern "C" fn iswlower() {
    todo!("iswlower")
}
#[no_mangle]
unsafe extern "C" fn iswprint() {
    todo!("iswprint")
}
#[no_mangle]
unsafe extern "C" fn iswpunct() {
    todo!("iswpunct")
}
#[no_mangle]
unsafe extern "C" fn iswspace() {
    todo!("iswspace")
}
#[no_mangle]
unsafe extern "C" fn iswupper() {
    todo!("iswupper")
}
#[no_mangle]
unsafe extern "C" fn iswxdigit() {
    todo!("iswxdigit")
}
#[no_mangle]
unsafe extern "C" fn lfind() {
    todo!("lfind")
}
#[no_mangle]
unsafe extern "C" fn __libc_current_sigrtmin() {
    todo!("__libc_current_sigrtmin")
}
#[no_mangle]
unsafe extern "C" fn lrint() {
    todo!("lrint")
}
#[no_mangle]
unsafe extern "C" fn lrintf() {
    todo!("lrintf")
}
#[no_mangle]
unsafe extern "C" fn lsearch() {
    todo!("lsearch")
}
#[no_mangle]
unsafe extern "C" fn mbsrtowcs() {
    todo!("mbsrtowcs")
}
#[no_mangle]
unsafe extern "C" fn mkdtemp() {
    todo!("mkdtemp")
}
#[no_mangle]
unsafe extern "C" fn msgctl() {
    todo!("msgctl")
}
#[no_mangle]
unsafe extern "C" fn msgget() {
    todo!("msgget")
}
#[no_mangle]
unsafe extern "C" fn msgrcv() {
    todo!("msgrcv")
}
#[no_mangle]
unsafe extern "C" fn msgsnd() {
    todo!("msgsnd")
}
#[no_mangle]
unsafe extern "C" fn newlocale() {
    todo!("newlocale")
}
#[no_mangle]
unsafe extern "C" fn nl_langinfo() {
    todo!("nl_langinfo")
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
unsafe extern "C" fn putenv() {
    todo!("putenv")
}
#[no_mangle]
unsafe extern "C" fn random() {
    todo!("random")
}
#[no_mangle]
unsafe extern "C" fn regerror() {
    todo!("regerror")
}
#[no_mangle]
unsafe extern "C" fn remque() {
    todo!("remque")
}
#[no_mangle]
unsafe extern "C" fn scalbnl() {
    todo!("scalbnl")
}
#[no_mangle]
unsafe extern "C" fn sem_close() {
    todo!("sem_close")
}
#[no_mangle]
unsafe extern "C" fn semctl() {
    todo!("semctl")
}
#[no_mangle]
unsafe extern "C" fn sem_destroy() {
    todo!("sem_destroy")
}
#[no_mangle]
unsafe extern "C" fn semget() {
    todo!("semget")
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
unsafe extern "C" fn semop() {
    todo!("semop")
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
unsafe extern "C" fn setstate() {
    todo!("setstate")
}
#[no_mangle]
unsafe extern "C" fn shmctl() {
    todo!("shmctl")
}
#[no_mangle]
unsafe extern "C" fn srandom() {
    todo!("srandom")
}
#[no_mangle]
unsafe extern "C" fn strlcat() {
    todo!("strlcat")
}
#[no_mangle]
unsafe extern "C" fn strlcpy() {
    todo!("strlcpy")
}
#[no_mangle]
unsafe extern "C" fn strptime() {
    todo!("strptime")
}
#[no_mangle]
unsafe extern "C" fn strtold() {
    todo!("strtold")
}
#[no_mangle]
unsafe extern "C" fn strverscmp() {
    todo!("strverscmp")
}
#[no_mangle]
unsafe extern "C" fn swprintf() {
    todo!("swprintf")
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
unsafe extern "C" fn uselocale() {
    todo!("uselocale")
}
#[no_mangle]
unsafe extern "C" fn wcrtomb() {
    todo!("wcrtomb")
}
#[no_mangle]
unsafe extern "C" fn wcschr() {
    todo!("wcschr")
}
#[no_mangle]
unsafe extern "C" fn wcscmp() {
    todo!("wcscmp")
}
#[no_mangle]
unsafe extern "C" fn wcscpy() {
    todo!("wcscpy")
}
#[no_mangle]
unsafe extern "C" fn wcsncpy() {
    todo!("wcsncpy")
}
#[no_mangle]
unsafe extern "C" fn wcsstr() {
    todo!("wcsstr")
}
#[no_mangle]
unsafe extern "C" fn wctob() {
    todo!("wctob")
}
#[no_mangle]
unsafe extern "C" fn a64l() {
    todo!("a64l")
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
unsafe extern "C" fn asprintf() {
    todo!("asprintf")
}
#[no_mangle]
unsafe extern "C" fn atof() {
    todo!("atof")
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
unsafe extern "C" fn endhostent() {
    todo!("endhostent")
}
#[no_mangle]
unsafe extern "C" fn endprotoent() {
    todo!("endprotoent")
}
#[no_mangle]
unsafe extern "C" fn endservent() {
    todo!("endservent")
}
#[no_mangle]
unsafe extern "C" fn fputwc() {
    todo!("fputwc")
}
#[no_mangle]
unsafe extern "C" fn fputws() {
    todo!("fputws")
}
#[no_mangle]
unsafe extern "C" fn fwide() {
    todo!("fwide")
}
#[no_mangle]
unsafe extern "C" fn gethostbyaddr() {
    todo!("gethostbyaddr")
}
#[no_mangle]
unsafe extern "C" fn gethostent() {
    todo!("gethostent")
}
#[no_mangle]
unsafe extern "C" fn getnetbyname() {
    todo!("getnetbyname")
}
#[no_mangle]
unsafe extern "C" fn getnetent() {
    todo!("getnetent")
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
unsafe extern "C" fn getprotoent() {
    todo!("getprotoent")
}
#[no_mangle]
unsafe extern "C" fn getservent() {
    todo!("getservent")
}
#[no_mangle]
unsafe extern "C" fn __isoc99_vsscanf() {
    todo!("__isoc99_vsscanf")
}
#[no_mangle]
unsafe extern "C" fn l64a() {
    todo!("l64a")
}
#[no_mangle]
unsafe extern "C" fn lldiv() {
    todo!("lldiv")
}
#[no_mangle]
unsafe extern "C" fn opterr() {
    todo!("opterr")
}
#[no_mangle]
unsafe extern "C" fn putwchar() {
    todo!("putwchar")
}
#[no_mangle]
unsafe extern "C" fn sethostent() {
    todo!("sethostent")
}
#[no_mangle]
unsafe extern "C" fn setnetent() {
    todo!("setnetent")
}
#[no_mangle]
unsafe extern "C" fn setprotoent() {
    todo!("setprotoent")
}
#[no_mangle]
unsafe extern "C" fn setregid() {
    todo!("setregid")
}
#[no_mangle]
unsafe extern "C" fn setreuid() {
    todo!("setreuid")
}
#[no_mangle]
unsafe extern "C" fn setservent() {
    todo!("setservent")
}
#[no_mangle]
unsafe extern "C" fn strcasestr() {
    todo!("strcasestr")
}
#[no_mangle]
unsafe extern "C" fn strnlen_s() {
    todo!("strnlen_s")
}
#[no_mangle]
unsafe extern "C" fn swab() {
    todo!("swab")
}
#[no_mangle]
unsafe extern "C" fn times() {
    todo!("times")
}
#[no_mangle]
unsafe extern "C" fn towctrans() {
    todo!("towctrans")
}
#[no_mangle]
unsafe extern "C" fn ungetwc() {
    todo!("ungetwc")
}
#[no_mangle]
unsafe extern "C" fn wcscasecmp() {
    todo!("wcscasecmp")
}
#[no_mangle]
unsafe extern "C" fn wcscspn() {
    todo!("wcscspn")
}
#[no_mangle]
unsafe extern "C" fn wcsrchr() {
    todo!("wcsrchr")
}
#[no_mangle]
unsafe extern "C" fn wcstod() {
    todo!("wcstod")
}
#[no_mangle]
unsafe extern "C" fn wcstoimax() {
    todo!("wcstoimax")
}
#[no_mangle]
unsafe extern "C" fn wcstok() {
    todo!("wcstok")
}
#[no_mangle]
unsafe extern "C" fn wcstoumax() {
    todo!("wcstoumax")
}
#[no_mangle]
unsafe extern "C" fn wcswidth() {
    todo!("wcswidth")
}
#[no_mangle]
unsafe extern "C" fn wctrans() {
    todo!("wctrans")
}
#[no_mangle]
unsafe extern "C" fn wprintf() {
    todo!("wprintf")
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
unsafe extern "C" fn pthread_spin_destroy() {
    todo!("pthread_spin_destroy")
}
#[no_mangle]
unsafe extern "C" fn pthread_spin_init() {
    todo!("pthread_spin_init")
}
#[no_mangle]
unsafe extern "C" fn pthread_spin_lock() {
    todo!("pthread_spin_lock")
}
#[no_mangle]
unsafe extern "C" fn pthread_spin_trylock() {
    todo!("pthread_spin_trylock")
}
#[no_mangle]
unsafe extern "C" fn pthread_spin_unlock() {
    todo!("pthread_spin_unlock")
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
unsafe extern "C" fn sigpending() {
    todo!("sigpending")
}
#[no_mangle]
unsafe extern "C" fn sigqueue() {
    todo!("sigqueue")
}
#[no_mangle]
unsafe extern "C" fn timer_getoverrun() {
    todo!("timer_getoverrun")
}
#[no_mangle]
unsafe extern "C" fn __longjmp_chk() {
    todo!("__longjmp_chk")
}
#[no_mangle]
unsafe extern "C" fn setpgrp() {
    todo!("setpgrp")
}
#[no_mangle]
unsafe extern "C" fn __dprintf_chk() {
    todo!("__dprintf_chk")
}
#[no_mangle]
unsafe extern "C" fn endmntent() {
    todo!("endmntent")
}
#[no_mangle]
unsafe extern "C" fn __getgroups_chk() {
    todo!("__getgroups_chk")
}
#[no_mangle]
unsafe extern "C" fn hasmntopt() {
    todo!("hasmntopt")
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
unsafe extern "C" fn mount() {
    todo!("mount")
}
#[no_mangle]
unsafe extern "C" fn __realpath_chk() {
    todo!("__realpath_chk")
}
#[no_mangle]
unsafe extern "C" fn setmntent() {
    todo!("setmntent")
}
#[no_mangle]
unsafe extern "C" fn setns() {
    todo!("setns")
}
#[no_mangle]
unsafe extern "C" fn setresgid() {
    todo!("setresgid")
}
#[no_mangle]
unsafe extern "C" fn setresuid() {
    todo!("setresuid")
}
#[no_mangle]
unsafe extern "C" fn __stpcpy_chk() {
    todo!("__stpcpy_chk")
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
unsafe extern "C" fn __vasprintf_chk() {
    todo!("__vasprintf_chk")
}
#[no_mangle]
unsafe extern "C" fn __vdprintf_chk() {
    todo!("__vdprintf_chk")
}
#[no_mangle]
unsafe extern "C" fn __vsprintf_chk() {
    todo!("__vsprintf_chk")
}
#[no_mangle]
unsafe extern "C" fn __asprintf_chk() {
    todo!("__asprintf_chk")
}
#[no_mangle]
unsafe extern "C" fn __read_chk() {
    todo!("__read_chk")
}
#[no_mangle]
unsafe extern "C" fn __strncat_chk() {
    todo!("__strncat_chk")
}
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
#[no_mangle]
unsafe extern "C" fn mallopt() {
    todo!("mallopt")
}
#[no_mangle]
unsafe extern "C" fn aio_cancel() {
    todo!("aio_cancel")
}
#[no_mangle]
unsafe extern "C" fn aio_error() {
    todo!("aio_error")
}
#[no_mangle]
unsafe extern "C" fn aio_fsync() {
    todo!("aio_fsync")
}
#[no_mangle]
unsafe extern "C" fn aio_read() {
    todo!("aio_read")
}
#[no_mangle]
unsafe extern "C" fn aio_return() {
    todo!("aio_return")
}
#[no_mangle]
unsafe extern "C" fn aio_suspend() {
    todo!("aio_suspend")
}
#[no_mangle]
unsafe extern "C" fn aio_write() {
    todo!("aio_write")
}
#[no_mangle]
unsafe extern "C" fn catclose() {
    todo!("catclose")
}
#[no_mangle]
unsafe extern "C" fn catgets() {
    todo!("catgets")
}
#[no_mangle]
unsafe extern "C" fn catopen() {
    todo!("catopen")
}
#[no_mangle]
unsafe extern "C" fn getitimer() {
    todo!("getitimer")
}
#[no_mangle]
unsafe extern "C" fn lio_listio() {
    todo!("lio_listio")
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
unsafe extern "C" fn dup3() {
    todo!("dup3")
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
unsafe extern "C" fn execveat() {
    todo!("execveat")
}
#[no_mangle]
unsafe extern "C" fn posix_fadvise64() {
    todo!("posix_fadvise64")
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
unsafe extern "C" fn creat64() {
    todo!("creat64")
}
#[no_mangle]
unsafe extern "C" fn mkstemp64() {
    todo!("mkstemp64")
}
#[no_mangle]
unsafe extern "C" fn truncate64() {
    todo!("truncate64")
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
unsafe extern "C" fn __fread_chk() {
    todo!("__fread_chk")
}
#[no_mangle]
unsafe extern "C" fn fstatfs() {
    todo!("fstatfs")
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
unsafe extern "C" fn gethostbyname_r() {
    todo!("gethostbyname_r")
}
#[no_mangle]
unsafe extern "C" fn gethostid() {
    todo!("gethostid")
}
#[no_mangle]
unsafe extern "C" fn sethostid() {
    todo!("sethostid")
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
unsafe extern "C" fn __ppoll_chk() {
    todo!("__ppoll_chk")
}
#[no_mangle]
unsafe extern "C" fn wait4() {
    todo!("wait4")
}
#[no_mangle]
unsafe extern "C" fn __pread_chk() {
    todo!("__pread_chk")
}
#[no_mangle]
unsafe extern "C" fn __pread64_chk() {
    todo!("__pread64_chk")
}
#[no_mangle]
unsafe extern "C" fn preadv2() {
    todo!("preadv2")
}
#[no_mangle]
unsafe extern "C" fn pwritev2() {
    todo!("pwritev2")
}
#[no_mangle]
unsafe extern "C" fn preadv64v2() {
    todo!("preadv64v2")
}
#[no_mangle]
unsafe extern "C" fn pwritev64v2() {
    todo!("pwritev64v2")
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
unsafe extern "C" fn __readlink_chk() {
    todo!("__readlink_chk")
}
#[no_mangle]
unsafe extern "C" fn __readlinkat_chk() {
    todo!("__readlinkat_chk")
}
#[no_mangle]
unsafe extern "C" fn reboot() {
    todo!("reboot")
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
unsafe extern "C" fn utime() {
    todo!("utime")
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
unsafe extern "C" fn setlinebuf() {
    todo!("setlinebuf")
}
#[no_mangle]
unsafe extern "C" fn tcsendbreak() {
    todo!("tcsendbreak")
}
#[no_mangle]
unsafe extern "C" fn tcdrain() {
    todo!("tcdrain")
}
#[no_mangle]
unsafe extern "C" fn tcflush() {
    todo!("tcflush")
}
#[no_mangle]
unsafe extern "C" fn tcflow() {
    todo!("tcflow")
}
#[no_mangle]
unsafe extern "C" fn __memmove_chk() {
    todo!("__memmove_chk")
}
#[no_mangle]
unsafe extern "C" fn if_nameindex() {
    todo!("if_nameindex")
}
#[no_mangle]
unsafe extern "C" fn if_freenameindex() {
    todo!("if_freenameindex")
}
