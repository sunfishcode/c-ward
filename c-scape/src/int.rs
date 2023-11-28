/// Integer math.
use libc::{c_int, c_long, c_longlong, intmax_t};

#[no_mangle]
unsafe extern "C" fn ffs(i: c_int) -> c_int {
    //libc!(libc::ffs(i));

    if i == 0 {
        0
    } else {
        i.trailing_zeros() as c_int + 1
    }
}

#[no_mangle]
unsafe extern "C" fn ffsl(i: c_long) -> c_long {
    //libc!(libc::ffsl(i));

    if i == 0 {
        0
    } else {
        i.trailing_zeros() as c_long + 1
    }
}

#[no_mangle]
unsafe extern "C" fn ffsll(i: c_longlong) -> c_longlong {
    //libc!(libc::ffsll(i));

    if i == 0 {
        0
    } else {
        i.trailing_zeros() as c_longlong + 1
    }
}

#[no_mangle]
unsafe extern "C" fn abs(i: c_int) -> c_int {
    libc!(libc::abs(i));

    i.abs()
}

#[no_mangle]
unsafe extern "C" fn labs(i: c_long) -> c_long {
    libc!(libc::labs(i));

    i.abs()
}

#[no_mangle]
unsafe extern "C" fn llabs(i: c_longlong) -> c_longlong {
    //libc!(libc::llabs(i));

    i.abs()
}

#[no_mangle]
unsafe extern "C" fn imaxabs(i: intmax_t) -> intmax_t {
    //libc!(libc::imaxabs(i));

    i.abs()
}
