#[no_mangle]
static error_message_count: libc::c_uint = 0;
#[no_mangle]
static error_one_per_line: libc::c_int = 0;
#[no_mangle]
static error_print_progname: UnsafeSendSyncVoidStar = UnsafeSendSyncVoidStar(core::ptr::null());

/// A type for `_error_print_progname`. It should be a function pointer, but
/// Rust doesn't permit those to be null. We'd instead use a `*const c_void`,
/// except that's not `Send` or `Sync`. So we use a wrapper.
#[repr(transparent)]
struct UnsafeSendSyncVoidStar(*const core::ffi::c_void);
unsafe impl Send for UnsafeSendSyncVoidStar {}
unsafe impl Sync for UnsafeSendSyncVoidStar {}
