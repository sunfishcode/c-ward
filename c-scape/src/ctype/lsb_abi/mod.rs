mod ctype_b_loc;
mod ctype_tolower_loc;
mod ctype_toupper_loc;

#[no_mangle]
extern "C" fn __ctype_get_mb_cur_max() -> libc::size_t {
    //libc!(libc::__ctype_get_mb_cur_max());

    // Just C/POSIX.
    1
}
