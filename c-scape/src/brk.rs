use crate::convert_res;
use core::ptr::{null_mut, without_provenance_mut};
use errno::{set_errno, Errno};
use libc::{c_int, c_void, intptr_t};

static mut CURRENT: *mut c_void = null_mut();

#[no_mangle]
unsafe extern "C" fn brk(ptr: *mut c_void) -> c_int {
    libc!(libc::brk(ptr));

    let new = match convert_res(rustix::runtime::kernel_brk(ptr)) {
        Some(new) => new,
        None => return -1,
    };

    // The `brk` syscall returns the old value if it failed.
    if ptr != new {
        set_errno(Errno(libc::ENOMEM));
        return -1;
    }

    CURRENT = new;

    0
}

#[no_mangle]
unsafe extern "C" fn sbrk(increment: intptr_t) -> *mut c_void {
    libc!(libc::sbrk(increment));

    let mut old = CURRENT;

    if old.is_null() {
        // Read the current value from the OS.
        old = match convert_res(rustix::runtime::kernel_brk(null_mut())) {
            Some(old) => old,
            None => return without_provenance_mut(!0),
        };
    }

    if increment == 0 {
        CURRENT = old;
        return old;
    }

    // Compute the desired address, and check for overflow.
    let want = old
        .cast::<u8>()
        .wrapping_add(increment as usize)
        .cast::<c_void>();
    let ok = if increment > 0 {
        want > old
    } else {
        want < old
    };
    if !ok {
        CURRENT = old;
        set_errno(Errno(libc::ENOMEM));
        return without_provenance_mut(!0);
    }

    // Install the new address.
    let new = match convert_res(rustix::runtime::kernel_brk(want)) {
        Some(new) => new,
        None => {
            CURRENT = old;
            return without_provenance_mut(!0);
        }
    };

    CURRENT = new;

    // The `brk` syscall returns the old value if it failed.
    if new != want {
        set_errno(Errno(libc::ENOMEM));
        return without_provenance_mut(!0);
    }

    old
}
