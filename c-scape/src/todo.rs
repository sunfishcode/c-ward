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
