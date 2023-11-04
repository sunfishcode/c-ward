#[no_mangle]
static mut error_message_count: libc::c_uint = 0;
#[no_mangle]
static mut error_one_per_line: libc::c_int = 0;
#[no_mangle]
static mut error_print_progname: Option<unsafe extern "C" fn()> = None;
