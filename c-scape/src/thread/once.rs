use rustix_futex_sync::Once;

use libc::c_int;

libc_type!(Once, pthread_once_t);
// Assert that `PTHREAD_ONCE_INIT` is zero, just like
// `rustix_futex_sync::Once::new()` is documented to be.
#[cfg(test)]
static_assertions::const_assert_eq!(libc::PTHREAD_ONCE_INIT, unsafe {
    core::mem::transmute(Once::new())
});

#[no_mangle]
unsafe extern "C" fn pthread_once(
    once_control: *mut libc::pthread_once_t,
    init_routine: extern "C" fn(),
) -> c_int {
    libc!(libc::pthread_once(once_control, init_routine));

    // Cast the `*mut pthread_once_t` to `*mut Once`, which we can do since
    // `rustix_futex_sync` is documented to be a `repr(transparent)` wrapper
    // around `AtomicU32`.
    (*once_control.cast::<Once>()).call_once(move || {
        init_routine();
    });

    0
}
