use libc::{c_double, c_long, c_ushort};

use core::cell::SyncUnsafeCell;
use core::ptr::addr_of;

#[cfg(test)]
static_assertions::assert_eq_size!(c_ushort, u16);
#[cfg(test)]
static_assertions::assert_type_eq_all!(c_double, f64);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct LCongData {
    a_mult: [c_ushort; 3],
    c: c_ushort,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct RngData {
    x_subi: [c_ushort; 3],
    data: LCongData,
}

static STORAGE: SyncUnsafeCell<RngData> = SyncUnsafeCell::new(RngData {
    x_subi: [0, 0, 0],
    data: LCongData {
        a_mult: [0xe66d, 0xdeec, 0x5],
        c: 0xb,
    },
});

unsafe fn next_lcong(x_subi: *mut [c_ushort; 3], data: *const LCongData) -> u64 {
    let x: u64 = ((*x_subi)[0] as u64) | ((*x_subi)[1] as u64) << 16 | ((*x_subi)[2] as u64) << 32;
    let a: u64 = ((*data).a_mult[0] as u64)
        | ((*data).a_mult[1] as u64) << 16
        | ((*data).a_mult[2] as u64) << 32;

    let res = a.wrapping_mul(x).wrapping_add((*data).c as u64);
    (*x_subi)[0] = res as c_ushort;
    (*x_subi)[1] = (res >> 16) as c_ushort;
    (*x_subi)[2] = (res >> 32) as c_ushort;
    res & 0xffff_ffff_ffff
}

#[no_mangle]
unsafe extern "C" fn drand48() -> c_double {
    libc!(libc::drand48());

    erand48((*STORAGE.get()).x_subi.as_mut_ptr())
}

#[no_mangle]
unsafe extern "C" fn erand48(x_subi: *mut c_ushort) -> c_double {
    libc!(libc::erand48(x_subi));

    let x_subi: *mut [c_ushort; 3] = x_subi.cast();

    let next_integral = next_lcong(x_subi, addr_of!((*STORAGE.get()).data));

    f64::from_bits(16.0_f64.to_bits() + next_integral) - 16.0
}

#[no_mangle]
unsafe extern "C" fn lrand48() -> c_long {
    libc!(libc::lrand48());

    nrand48((*STORAGE.get()).x_subi.as_mut_ptr())
}

#[no_mangle]
unsafe extern "C" fn nrand48(x_subi: *mut c_ushort) -> c_long {
    libc!(libc::nrand48(x_subi));

    let x_subi: *mut [c_ushort; 3] = x_subi.cast();

    (next_lcong(x_subi, addr_of!((*STORAGE.get()).data)) >> 17) as c_long
}

#[no_mangle]
unsafe extern "C" fn mrand48() -> c_long {
    libc!(libc::mrand48());

    jrand48((*STORAGE.get()).x_subi.as_mut_ptr())
}

#[no_mangle]
unsafe extern "C" fn jrand48(x_subi: *mut c_ushort) -> c_long {
    libc!(libc::jrand48(x_subi));

    let x_subi: *mut [c_ushort; 3] = x_subi.cast();

    // Cast to i32 so the sign extension works properly
    (next_lcong(x_subi, addr_of!((*STORAGE.get()).data)) >> 16) as i32 as c_long
}

#[no_mangle]
unsafe extern "C" fn srand48(seed: c_long) {
    libc!(libc::srand48(seed));

    seed48(&mut [0x330e, seed as c_ushort, (seed >> 16) as c_ushort]);
}

#[no_mangle]
unsafe extern "C" fn seed48(seed: *mut [c_ushort; 3]) -> *mut c_ushort {
    static PREV_SEED: SyncUnsafeCell<[c_ushort; 3]> = SyncUnsafeCell::new([0, 0, 0]);

    *PREV_SEED.get() = (*STORAGE.get()).x_subi;
    (*STORAGE.get()) = RngData {
        x_subi: *seed,
        data: LCongData {
            a_mult: [0xe66d, 0xdeec, 0x5],
            c: 0xb,
        },
    };

    PREV_SEED.get().cast::<u16>()
}

#[no_mangle]
unsafe extern "C" fn lcong48(param: *mut c_ushort) {
    libc!(libc::lcong48(param));

    let param: *mut [c_ushort; 7] = param.cast();

    *STORAGE.get().cast::<[c_ushort; 7]>() = *param;
}
