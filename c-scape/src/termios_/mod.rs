//! Termios APIs

use crate::{convert_res, set_errno, Errno};
use core::mem::{size_of, transmute, zeroed};
use core::ops::Index;
use libc::{c_char, c_int, pid_t, termios, winsize};
use rustix::fd::{BorrowedFd, FromRawFd, IntoRawFd, OwnedFd};
use rustix::process::Pid;
use rustix::termios::{
    Action, ControlModes, InputModes, LocalModes, OptionalActions, OutputModes, QueueSelector,
    SpecialCodeIndex, SpecialCodes, Termios,
};

#[no_mangle]
unsafe extern "C" fn cfmakeraw(termios_p: *mut libc::termios) {
    libc!(libc::cfmakeraw(termios_p));

    let termios = *termios_p;
    let mut termios = libc_to_rustix(&termios);

    rustix::termios::Termios::make_raw(&mut termios);

    (*termios_p) = rustix_to_libc(&termios);
}

#[no_mangle]
unsafe extern "C" fn cfgetospeed(termios_p: *const libc::termios) -> libc::speed_t {
    libc!(libc::cfgetospeed(termios_p));

    let termios = &*termios_p;
    let termios = libc_to_rustix(termios);

    to_libc_speed(termios.output_speed())
}

#[no_mangle]
unsafe extern "C" fn cfgetispeed(termios_p: *const libc::termios) -> libc::speed_t {
    libc!(libc::cfgetispeed(termios_p));

    let termios = &*termios_p;
    let termios = libc_to_rustix(termios);

    to_libc_speed(termios.input_speed())
}

#[no_mangle]
unsafe extern "C" fn cfsetispeed(
    termios_p: *mut libc::termios,
    speed: libc::speed_t,
) -> libc::c_int {
    libc!(libc::cfsetispeed(termios_p, speed));

    let termios = &mut *termios_p;
    let mut rustix_termios = libc_to_rustix(termios);

    let speed = match from_libc_speed(speed) {
        Some(speed) => speed,
        None => {
            set_errno(Errno(libc::EINVAL));
            return -1;
        }
    };

    match convert_res(rustix_termios.set_input_speed(speed)) {
        Some(()) => {
            *termios = rustix_to_libc(&rustix_termios);
            0
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn cfsetospeed(
    termios_p: *mut libc::termios,
    speed: libc::speed_t,
) -> libc::c_int {
    libc!(libc::cfsetospeed(termios_p, speed));

    let termios = &mut *termios_p;
    let mut rustix_termios = libc_to_rustix(termios);

    let speed = match from_libc_speed(speed) {
        Some(speed) => speed,
        None => {
            set_errno(Errno(libc::EINVAL));
            return -1;
        }
    };

    match convert_res(rustix_termios.set_output_speed(speed)) {
        Some(()) => {
            *termios = rustix_to_libc(&rustix_termios);
            0
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn cfsetspeed(
    termios_p: *mut libc::termios,
    speed: libc::speed_t,
) -> libc::c_int {
    libc!(libc::cfsetspeed(termios_p, speed));

    let termios = &mut *termios_p;
    let mut rustix_termios = libc_to_rustix(termios);

    let speed = match from_libc_speed(speed) {
        Some(speed) => speed,
        None => {
            set_errno(Errno(libc::EINVAL));
            return -1;
        }
    };

    match convert_res(rustix_termios.set_speed(speed)) {
        Some(()) => {
            *termios = rustix_to_libc(&rustix_termios);
            0
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn tcgetattr(fd: c_int, termios_p: *mut libc::termios) -> c_int {
    libc!(libc::tcgetattr(fd, termios_p));

    match convert_res(rustix::termios::tcgetattr(BorrowedFd::borrow_raw(fd))) {
        Some(termios) => {
            (*termios_p) = rustix_to_libc(&termios);
            0
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn tcsetattr(
    fd: c_int,
    optional_actions: c_int,
    termios_p: *const libc::termios,
) -> c_int {
    libc!(libc::tcsetattr(fd, optional_actions, termios_p));

    let optional_actions = match optional_actions {
        libc::TCSANOW => OptionalActions::Now,
        libc::TCSADRAIN => OptionalActions::Drain,
        libc::TCSAFLUSH => OptionalActions::Flush,
        _ => {
            set_errno(Errno(libc::EINVAL));
            return -1;
        }
    };

    let termios = *termios_p;
    let termios = libc_to_rustix(&termios);

    match convert_res(rustix::termios::tcsetattr(
        BorrowedFd::borrow_raw(fd),
        optional_actions,
        &termios,
    )) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn openpty(
    acontroller: *mut c_int,
    auser: *mut c_int,
    name: *mut c_char,
    termp: *const termios,
    winp: *const winsize,
) -> c_int {
    libc!(libc::openpty(acontroller, auser, name, termp, winp));

    // `name` is not implemented yet.
    assert!(name.is_null());

    let term = if termp.is_null() {
        None
    } else {
        Some(libc_to_rustix(&*termp))
    };
    let win = if winp.is_null() {
        None
    } else {
        let win = &*winp;
        Some(rustix::termios::Winsize {
            ws_row: win.ws_row,
            ws_col: win.ws_col,
            ws_xpixel: win.ws_xpixel,
            ws_ypixel: win.ws_ypixel,
        })
    };
    match convert_res(rustix_openpty::openpty(term.as_ref(), win.as_ref())) {
        Some(rustix_openpty::Pty { controller, user }) => {
            *acontroller = controller.into_raw_fd();
            *auser = user.into_raw_fd();
            0
        }
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn login_tty(fd: c_int) -> c_int {
    libc!(libc::login_tty(fd));

    let fd = OwnedFd::from_raw_fd(fd);
    match convert_res(rustix_openpty::login_tty(fd)) {
        Some(()) => 0,
        None => -1,
    }
}

fn libc_to_rustix(termios: &libc::termios) -> Termios {
    let mut result: Termios = unsafe { zeroed() };
    result.special_codes = unsafe {
        transmute([
            termios.c_cc[0],
            termios.c_cc[1],
            termios.c_cc[2],
            termios.c_cc[3],
            termios.c_cc[4],
            termios.c_cc[5],
            termios.c_cc[6],
            termios.c_cc[7],
            termios.c_cc[8],
            termios.c_cc[9],
            termios.c_cc[10],
            termios.c_cc[11],
            termios.c_cc[12],
            termios.c_cc[13],
            termios.c_cc[14],
            termios.c_cc[15],
            termios.c_cc[16],
            termios.c_cc[17],
            termios.c_cc[18],
        ])
    };
    result.control_modes = ControlModes::from_bits_retain(termios.c_cflag);
    result.input_modes = InputModes::from_bits_retain(termios.c_iflag);
    result.output_modes = OutputModes::from_bits_retain(termios.c_oflag);
    result.local_modes = LocalModes::from_bits_retain(termios.c_lflag);
    result.line_discipline = termios.c_line;
    result.set_input_speed(termios.c_ispeed).unwrap();
    result.set_output_speed(termios.c_ospeed).unwrap();
    result
}

fn rustix_to_libc(termios: &Termios) -> libc::termios {
    let [sc0, sc1, sc2, sc3, sc4, sc5, sc6, sc7, sc8, sc9, sc10, sc11, sc12, sc13, sc14, sc15, sc16, sc17, sc18] = unsafe {
        transmute::<
            SpecialCodes,
            [<SpecialCodes as Index<SpecialCodeIndex>>::Output;
                size_of::<SpecialCodes>() / size_of::<libc::cc_t>()],
        >(termios.special_codes.clone())
    };

    libc::termios {
        c_iflag: termios.input_modes.bits(),
        c_oflag: termios.output_modes.bits(),
        c_cflag: termios.control_modes.bits(),
        c_lflag: termios.local_modes.bits(),
        c_cc: [
            sc0, sc1, sc2, sc3, sc4, sc5, sc6, sc7, sc8, sc9, sc10, sc11, sc12, sc13, sc14, sc15,
            sc16, sc17, sc18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        c_line: termios.line_discipline,
        c_ispeed: to_libc_speed(termios.input_speed()),
        c_ospeed: to_libc_speed(termios.output_speed()),
    }
}

fn to_libc_speed(speed: u32) -> libc::speed_t {
    match speed {
        0 => libc::B0,
        50 => libc::B50,
        75 => libc::B75,
        110 => libc::B110,
        134 => libc::B134,
        150 => libc::B150,
        200 => libc::B200,
        300 => libc::B300,
        600 => libc::B600,
        1200 => libc::B1200,
        1800 => libc::B1800,
        2400 => libc::B2400,
        4800 => libc::B4800,
        9600 => libc::B9600,
        19200 => libc::B19200,
        38400 => libc::B38400,
        #[cfg(not(target_os = "aix"))]
        57600 => libc::B57600,
        #[cfg(not(target_os = "aix"))]
        115_200 => libc::B115200,
        #[cfg(not(target_os = "aix"))]
        230_400 => libc::B230400,
        #[cfg(not(any(
            apple,
            target_os = "aix",
            target_os = "dragonfly",
            target_os = "haiku",
            target_os = "openbsd"
        )))]
        460_800 => libc::B460800,
        #[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "haiku")))]
        500_000 => libc::B500000,
        #[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "haiku")))]
        576_000 => libc::B576000,
        #[cfg(not(any(
            apple,
            target_os = "aix",
            target_os = "dragonfly",
            target_os = "haiku",
            target_os = "openbsd"
        )))]
        921_600 => libc::B921600,
        #[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
        1_000_000 => libc::B1000000,
        #[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
        1_152_000 => libc::B1152000,
        #[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
        1_500_000 => libc::B1500000,
        #[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
        2_000_000 => libc::B2000000,
        #[cfg(not(any(
            target_arch = "sparc",
            target_arch = "sparc64",
            bsd,
            target_os = "aix",
            target_os = "haiku",
            target_os = "solaris",
        )))]
        2_500_000 => libc::B2500000,
        #[cfg(not(any(
            target_arch = "sparc",
            target_arch = "sparc64",
            bsd,
            target_os = "aix",
            target_os = "haiku",
            target_os = "solaris",
        )))]
        3_000_000 => libc::B3000000,
        #[cfg(not(any(
            target_arch = "sparc",
            target_arch = "sparc64",
            bsd,
            target_os = "aix",
            target_os = "haiku",
            target_os = "solaris",
        )))]
        3_500_000 => libc::B3500000,
        #[cfg(not(any(
            target_arch = "sparc",
            target_arch = "sparc64",
            bsd,
            target_os = "aix",
            target_os = "haiku",
            target_os = "solaris",
        )))]
        4_000_000 => libc::B4000000,
        _ => libc::BOTHER,
    }
}

fn from_libc_speed(speed: u32) -> Option<libc::speed_t> {
    Some(match speed {
        libc::B0 => 0,
        libc::B50 => 50,
        libc::B75 => 75,
        libc::B110 => 110,
        libc::B134 => 134,
        libc::B150 => 150,
        libc::B200 => 200,
        libc::B300 => 300,
        libc::B600 => 600,
        libc::B1200 => 1200,
        libc::B1800 => 1800,
        libc::B2400 => 2400,
        libc::B4800 => 4800,
        libc::B9600 => 9600,
        libc::B19200 => 19200,
        libc::B38400 => 38400,
        #[cfg(not(target_os = "aix"))]
        libc::B57600 => 57600,
        #[cfg(not(target_os = "aix"))]
        libc::B115200 => 115200,
        #[cfg(not(target_os = "aix"))]
        libc::B230400 => 230400,
        #[cfg(not(any(
            apple,
            target_os = "aix",
            target_os = "dragonfly",
            target_os = "haiku",
            target_os = "openbsd"
        )))]
        libc::B460800 => 460800,
        #[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "haiku")))]
        libc::B500000 => 500000,
        #[cfg(not(any(bsd, solarish, target_os = "aix", target_os = "haiku")))]
        libc::B576000 => 576000,
        #[cfg(not(any(
            apple,
            target_os = "aix",
            target_os = "dragonfly",
            target_os = "haiku",
            target_os = "openbsd"
        )))]
        libc::B921600 => 921600,
        #[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
        libc::B1000000 => 1000000,
        #[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
        libc::B1152000 => 1152000,
        #[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
        libc::B1500000 => 1500000,
        #[cfg(not(any(bsd, target_os = "aix", target_os = "haiku", target_os = "solaris")))]
        libc::B2000000 => 2000000,
        #[cfg(not(any(
            target_arch = "sparc",
            target_arch = "sparc64",
            bsd,
            target_os = "aix",
            target_os = "haiku",
            target_os = "solaris",
        )))]
        libc::B2500000 => 2500000,
        #[cfg(not(any(
            target_arch = "sparc",
            target_arch = "sparc64",
            bsd,
            target_os = "aix",
            target_os = "haiku",
            target_os = "solaris",
        )))]
        libc::B3000000 => 3000000,
        #[cfg(not(any(
            target_arch = "sparc",
            target_arch = "sparc64",
            bsd,
            target_os = "aix",
            target_os = "haiku",
            target_os = "solaris",
        )))]
        libc::B3500000 => 3500000,
        #[cfg(not(any(
            target_arch = "sparc",
            target_arch = "sparc64",
            bsd,
            target_os = "aix",
            target_os = "haiku",
            target_os = "solaris",
        )))]
        libc::B4000000 => 4000000,
        _ => return None,
    })
}

#[no_mangle]
unsafe extern "C" fn tcgetpgrp(fd: c_int) -> pid_t {
    libc!(libc::tcgetpgrp(fd));
    let fd = BorrowedFd::borrow_raw(fd);
    match convert_res(rustix::termios::tcgetpgrp(fd)) {
        Some(pid) => pid.as_raw_nonzero().get(),
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn tcsetpgrp(fd: c_int, pgrp: pid_t) -> c_int {
    libc!(libc::tcsetpgrp(fd, pgrp));
    let fd = BorrowedFd::borrow_raw(fd);
    let pgrp = Pid::from_raw(pgrp).unwrap();
    match convert_res(rustix::termios::tcsetpgrp(fd, pgrp)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn tcsendbreak(fd: c_int, duration: c_int) -> c_int {
    libc!(libc::tcsendbreak(fd, duration));

    let fd = BorrowedFd::borrow_raw(fd);
    let _ = duration;
    match convert_res(rustix::termios::tcsendbreak(fd)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn tcdrain(fd: c_int) -> c_int {
    libc!(libc::tcdrain(fd));

    let fd = BorrowedFd::borrow_raw(fd);
    match convert_res(rustix::termios::tcdrain(fd)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn tcflush(fd: c_int, queue_selector: c_int) -> c_int {
    libc!(libc::tcflush(fd, queue_selector));

    let fd = BorrowedFd::borrow_raw(fd);
    let queue_selector = match queue_selector {
        libc::TCIFLUSH => QueueSelector::IFlush,
        libc::TCOFLUSH => QueueSelector::OFlush,
        libc::TCIOFLUSH => QueueSelector::IOFlush,
        _ => {
            set_errno(Errno(libc::EINVAL));
            return -1;
        }
    };
    match convert_res(rustix::termios::tcflush(fd, queue_selector)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn tcflow(fd: c_int, action: c_int) -> c_int {
    libc!(libc::tcflow(fd, action));

    let fd = BorrowedFd::borrow_raw(fd);
    let action = match action {
        libc::TCOOFF => Action::OOff,
        libc::TCOON => Action::OOn,
        libc::TCIOFF => Action::IOff,
        libc::TCION => Action::IOn,
        _ => {
            set_errno(Errno(libc::EINVAL));
            return -1;
        }
    };
    match convert_res(rustix::termios::tcflow(fd, action)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
static mut pts_buffer: [c_char; 30] = [0; 30];

#[no_mangle]
unsafe extern "C" fn ptsname(fd: c_int) -> *mut c_char {
    if ptsname_r(fd, pts_buffer.as_mut_ptr(), pts_buffer.len()) != 0 {
        core::ptr::null_mut()
    } else {
        pts_buffer.as_mut_ptr()
    }
}

#[no_mangle]
unsafe extern "C" fn ptsname_r(fd: c_int, buf: *mut c_char, buflen: libc::size_t) -> c_int {
    libc!(libc::ptsname_r(fd, buf, buflen));
    if buf.is_null() {
        set_errno(Errno(libc::EINVAL));
        -1
    } else {
        let fd = BorrowedFd::borrow_raw(fd);
        if let Ok(name) = rustix::pty::ptsname(fd, []) {
            let len = name.as_bytes().len() + 1; // length inc null terminator
            if len > buflen {
                set_errno(Errno(libc::ERANGE));
                -1
            } else {
                // we have checked the string will fit in the buffer
                // so can use strcpy safely
                let mut d = buf;
                let mut s = name.as_ptr().cast();
                while !d.is_null() {
                    *d = *s;

                    if d.is_null() {
                        break;
                    }

                    d = d.add(1);
                    s = s.add(1);
                }
                0
            }
        } else {
            -1
        }
    }
}
