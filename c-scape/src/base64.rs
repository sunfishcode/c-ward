use core::cell::SyncUnsafeCell;
use libc::{c_char, c_long};

#[no_mangle]
unsafe extern "C" fn a64l(str64: *const c_char) -> c_long {
    let str64 = str64.cast::<u8>();
    let mut x: u32 = 0;

    for i in 0..6 {
        let digit: u32 = match *str64.add(i) {
            b @ b'.'..=b'9' => b - b'.',
            b @ b'A'..=b'Z' => b - (b'A' - 12),
            b @ b'a'..=b'z' => b - (b'a' - 38),
            _ => break,
        }
        .into();
        x |= digit << (6 * i);
    }

    x as c_long
}

#[no_mangle]
unsafe extern "C" fn l64a(value: c_long) -> *mut c_char {
    static BUFFER: SyncUnsafeCell<[u8; 6 + 1]> = SyncUnsafeCell::new([0; 6 + 1]);
    let buffer = &mut *BUFFER.get();

    let mut value = value as u32;
    let mut i = 0;

    while value != 0 {
        buffer[i] = match (value & 63) as u8 {
            v @ 0..=11 => b'.' + v,
            v @ 12..=37 => b'A' - 12 + v,
            v @ 38..=63 => b'a' - 38 + v,
            _ => unreachable!(),
        };
        i += 1;
        value >>= 6;
    }
    buffer[i] = b'\0';

    buffer.as_mut_ptr().cast()
}
