use core::ptr::null_mut;
use libc::{c_int, c_void, size_t};

#[no_mangle]
unsafe extern "C" fn bsearch(
    key: *const c_void,
    base: *const c_void,
    nmemb: size_t,
    width: size_t,
    compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
) -> *mut c_void {
    libc!(libc::bsearch(key, base, nmemb, width, compar));

    let compar = compar.unwrap();
    let mut base = base.cast::<u8>();
    let mut nmemb = nmemb;

    while nmemb > 0 {
        let half = nmemb / 2;
        let mid = base.add(width * half);
        let sign = compar(key, mid.cast::<c_void>());
        if sign < 0 {
            nmemb = half;
        } else if sign > 0 {
            base = mid.add(width);
            nmemb -= half + 1;
        } else {
            return mid.cast::<c_void>().cast_mut();
        }
    }

    null_mut()
}
