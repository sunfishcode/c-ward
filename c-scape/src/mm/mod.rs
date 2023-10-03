use rustix::fd::BorrowedFd;
use rustix::mm::{MapFlags, MprotectFlags, MremapFlags, ProtFlags};

use core::ffi::c_void;
use errno::{set_errno, Errno};
use libc::{c_int, c_uint, off64_t, off_t, size_t};

use crate::convert_res;

#[no_mangle]
unsafe extern "C" fn mmap(
    addr: *mut c_void,
    length: size_t,
    prot: c_int,
    flags: c_int,
    fd: c_int,
    offset: off_t,
) -> *mut c_void {
    libc!(libc::mmap(addr, length, prot, flags, fd, offset));

    mmap64(addr, length, prot, flags, fd, offset as off64_t)
}

#[no_mangle]
unsafe extern "C" fn mmap64(
    addr: *mut c_void,
    length: size_t,
    prot: c_int,
    flags: c_int,
    fd: c_int,
    offset: off64_t,
) -> *mut c_void {
    libc!(libc::mmap64(addr, length, prot, flags, fd, offset));

    let anon = flags & libc::MAP_ANONYMOUS == libc::MAP_ANONYMOUS;
    let prot = ProtFlags::from_bits(prot as _).unwrap();
    let flags = MapFlags::from_bits((flags & !libc::MAP_ANONYMOUS) as _).unwrap();
    match convert_res(if anon {
        rustix::mm::mmap_anonymous(addr, length, prot, flags)
    } else {
        rustix::mm::mmap(
            addr,
            length,
            prot,
            flags,
            BorrowedFd::borrow_raw(fd),
            offset as _,
        )
    }) {
        Some(ptr) => ptr,
        None => libc::MAP_FAILED,
    }
}

#[no_mangle]
unsafe extern "C" fn munmap(ptr: *mut c_void, len: size_t) -> c_int {
    libc!(libc::munmap(ptr, len));

    match convert_res(rustix::mm::munmap(ptr, len)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn mremap(
    old_address: *mut c_void,
    old_size: size_t,
    new_size: size_t,
    flags: c_int,
    mut args: ...
) -> *mut c_void {
    if (flags & libc::MREMAP_FIXED) == libc::MREMAP_FIXED {
        let new_address = args.arg::<*mut c_void>();
        libc!(libc::mremap(
            old_address,
            old_size,
            new_size,
            flags,
            new_address
        ));

        let flags = flags & !libc::MREMAP_FIXED;
        let flags = MremapFlags::from_bits(flags as _).unwrap();
        match convert_res(rustix::mm::mremap_fixed(
            old_address,
            old_size,
            new_size,
            flags,
            new_address,
        )) {
            Some(new_address) => new_address,
            None => libc::MAP_FAILED,
        }
    } else {
        libc!(libc::mremap(old_address, old_size, new_size, flags));

        let flags = MremapFlags::from_bits(flags as _).unwrap();
        match convert_res(rustix::mm::mremap(old_address, old_size, new_size, flags)) {
            Some(new_address) => new_address,
            None => libc::MAP_FAILED,
        }
    }
}

#[no_mangle]
unsafe extern "C" fn mprotect(addr: *mut c_void, length: size_t, prot: c_int) -> c_int {
    libc!(libc::mprotect(addr, length, prot));

    let prot = MprotectFlags::from_bits(prot as _).unwrap();
    match convert_res(rustix::mm::mprotect(addr, length, prot)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn madvise(addr: *mut c_void, length: size_t, advice: c_int) -> c_int {
    libc!(libc::madvise(addr, length, advice));

    use rustix::mm::Advice;

    let advice = match advice {
        libc::MADV_NORMAL => Advice::Normal,
        libc::MADV_SEQUENTIAL => Advice::Sequential,
        libc::MADV_RANDOM => Advice::Random,
        libc::MADV_WILLNEED => Advice::WillNeed,
        libc::MADV_DONTNEED => Advice::LinuxDontNeed,
        libc::MADV_FREE => Advice::LinuxFree,
        libc::MADV_REMOVE => Advice::LinuxRemove,
        libc::MADV_DONTFORK => Advice::LinuxDontFork,
        libc::MADV_DOFORK => Advice::LinuxDoFork,
        libc::MADV_HWPOISON => Advice::LinuxHwPoison,
        #[cfg(not(any(
            target_arch = "mips",
            target_arch = "mips32r6",
            target_arch = "mips64",
            target_arch = "mips64r6"
        )))]
        libc::MADV_SOFT_OFFLINE => Advice::LinuxSoftOffline,
        libc::MADV_MERGEABLE => Advice::LinuxMergeable,
        libc::MADV_UNMERGEABLE => Advice::LinuxUnmergeable,
        libc::MADV_HUGEPAGE => Advice::LinuxHugepage,
        libc::MADV_NOHUGEPAGE => Advice::LinuxNoHugepage,
        libc::MADV_DONTDUMP => Advice::LinuxDontDump,
        libc::MADV_DODUMP => Advice::LinuxDoDump,
        libc::MADV_WIPEONFORK => Advice::LinuxWipeOnFork,
        libc::MADV_KEEPONFORK => Advice::LinuxKeepOnFork,
        libc::MADV_COLD => Advice::LinuxCold,
        libc::MADV_PAGEOUT => Advice::LinuxPageOut,
        libc::MADV_POPULATE_READ => Advice::LinuxPopulateRead,
        libc::MADV_POPULATE_WRITE => Advice::LinuxPopulateWrite,
        libc::MADV_DONTNEED_LOCKED => Advice::LinuxDontneedLocked,
        _ => {
            set_errno(Errno(libc::EINVAL));
            return -1;
        }
    };
    match convert_res(rustix::mm::madvise(addr, length, advice)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn mlock(addr: *mut c_void, len: size_t) -> c_int {
    libc!(libc::mlock(addr, len));

    match convert_res(rustix::mm::mlock(addr as *mut c_void, len)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn mlock2(addr: *const c_void, len: size_t, flags: c_uint) -> c_int {
    libc!(libc::mlock2(addr, len, flags));

    let flags = rustix::mm::MlockFlags::from_bits_retain(flags);
    match convert_res(rustix::mm::mlock_with(addr as *mut c_void, len, flags)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn munlock(addr: *const c_void, len: size_t) -> c_int {
    libc!(libc::munlock(addr, len));

    match convert_res(rustix::mm::munlock(addr as *mut c_void, len)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn msync(addr: *mut c_void, len: size_t, flags: c_int) -> c_int {
    libc!(libc::msync(addr, len, flags));

    let flags = rustix::mm::MsyncFlags::from_bits_retain(flags as _);
    match convert_res(rustix::mm::msync(addr, len, flags)) {
        Some(()) => 0,
        None => -1,
    }
}
