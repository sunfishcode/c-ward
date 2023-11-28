//! A simple `qsort` implementation based on the Combsort algorithm.

use libc::{c_int, c_void, size_t};

#[no_mangle]
unsafe extern "C" fn qsort(
    base: *mut c_void,
    nmemb: size_t,
    width: size_t,
    compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
) {
    libc!(libc::qsort(base, nmemb, width, compar));

    let compar = compar.unwrap();

    if nmemb <= 1 {
        return;
    }

    let base = base.cast::<u8>();
    let mut gap = nmemb;

    loop {
        gap = next_gap(gap);

        let mut any_swapped = false;
        let mut a = base;
        let mut b = base.add(gap * width);
        for _ in 0..nmemb - gap {
            if compar(a.cast(), b.cast()) > 0 {
                swap(a, b, width);
                any_swapped = true;
            }
            a = a.add(width);
            b = b.add(width);
        }

        if gap <= 1 && !any_swapped {
            break;
        }
    }
}

#[no_mangle]
unsafe extern "C" fn qsort_r(
    base: *mut c_void,
    nmemb: size_t,
    width: size_t,
    compar: Option<unsafe extern "C" fn(*const c_void, *const c_void, *mut c_void) -> c_int>,
    arg: *mut c_void,
) {
    libc!(libc::qsort_r(base, nmemb, width, compar, arg));

    let compar = compar.unwrap();

    if nmemb <= 1 {
        return;
    }

    let base = base.cast::<u8>();
    let mut gap = nmemb;

    loop {
        gap = next_gap(gap);

        let mut any_swapped = false;
        let mut a = base;
        let mut b = base.add(gap * width);
        for _ in 0..nmemb - gap {
            if compar(a.cast(), b.cast(), arg) > 0 {
                swap(a, b, width);
                any_swapped = true;
            }
            a = a.add(width);
            b = b.add(width);
        }

        if gap <= 1 && !any_swapped {
            break;
        }
    }
}

fn next_gap(gap: size_t) -> size_t {
    let gap = (gap * 10) / 13;

    if gap == 9 || gap == 10 {
        11 // apply the "rule of 11"
    } else if gap <= 1 {
        1
    } else {
        gap
    }
}

unsafe fn swap(a: *mut u8, b: *mut u8, width: size_t) {
    for i in 0..width {
        core::ptr::swap(a.add(i), b.add(i));
    }
}
