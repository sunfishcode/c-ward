mod ctype_b_loc;
mod ctype_tolower_loc;
mod ctype_toupper_loc;

#[cfg(feature = "todo")]
#[no_mangle]
extern "C" fn __ctype_get_mb_cur_max() -> libc::size_t {
    todo!("__ctype_get_mb_cur_max")
}
