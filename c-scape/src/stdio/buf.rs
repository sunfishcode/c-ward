use core::ptr::null_mut;
use libc::{c_char, c_int, size_t, FILE};

#[no_mangle]
unsafe extern "C" fn setbuf(stream: *mut FILE, buf: *mut c_char) {
    libc!(libc::setbuf(stream, buf));

    setbuffer(stream, buf, libc::BUFSIZ as size_t)
}

#[no_mangle]
unsafe extern "C" fn setbuffer(stream: *mut FILE, buf: *mut c_char, size: size_t) {
    //libc!(libc::setbuffer(stream, buf, size));

    let mode = if buf.is_null() {
        libc::_IONBF
    } else {
        libc::_IOFBF
    };

    setvbuf(stream, buf, mode, size);
}

#[no_mangle]
unsafe extern "C" fn setlinebuf(stream: *mut FILE) {
    //libc!(libc::setlinebuf(stream));

    setvbuf(stream, null_mut(), libc::_IOLBF, 0);
}

#[no_mangle]
unsafe extern "C" fn setvbuf(
    stream: *mut FILE,
    buf: *mut c_char,
    mode: c_int,
    size: size_t,
) -> c_int {
    libc!(libc::setvbuf(stream, buf, mode, size));

    // This implementation does not current perform buffering. This is mostly
    // just a missing optimization, but it'd be observable with user-provided
    // buffers.
    match mode {
        libc::_IONBF => 0,
        libc::_IOLBF | libc::_IOFBF => {
            if !buf.is_null() && size != 0 {
                todo!("buffered I/O with a user-provided buffer");
            }
            0
        }
        _ => -1,
    }
}
