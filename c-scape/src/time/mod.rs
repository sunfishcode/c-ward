use errno::{set_errno, Errno};
use libc::{c_int, c_uint};

use crate::convert_res;

fn rustix_timespec_to_libc_timespec(
    rustix_time: rustix::time::Timespec,
) -> Result<libc::timespec, core::num::TryFromIntError> {
    // SAFETY: libc structs can be zero-initialized freely
    let mut time: libc::timespec = unsafe { core::mem::zeroed() };
    time.tv_sec = rustix_time.tv_sec.try_into()?;
    time.tv_nsec = rustix_time.tv_nsec.try_into()?;
    Ok(time)
}

#[no_mangle]
unsafe extern "C" fn clock_gettime(id: c_int, tp: *mut libc::timespec) -> c_int {
    libc!(libc::clock_gettime(id, tp));

    let id = match id {
        libc::CLOCK_MONOTONIC => {
            rustix::time::DynamicClockId::Known(rustix::time::ClockId::Monotonic)
        }
        libc::CLOCK_REALTIME => {
            rustix::time::DynamicClockId::Known(rustix::time::ClockId::Realtime)
        }
        libc::CLOCK_BOOTTIME => rustix::time::DynamicClockId::Boottime,
        libc::CLOCK_MONOTONIC_COARSE => {
            rustix::time::DynamicClockId::Known(rustix::time::ClockId::MonotonicCoarse)
        }
        libc::CLOCK_REALTIME_COARSE => {
            rustix::time::DynamicClockId::Known(rustix::time::ClockId::RealtimeCoarse)
        }
        libc::CLOCK_MONOTONIC_RAW => {
            rustix::time::DynamicClockId::Known(rustix::time::ClockId::MonotonicRaw)
        }
        libc::CLOCK_THREAD_CPUTIME_ID => {
            rustix::time::DynamicClockId::Known(rustix::time::ClockId::ThreadCPUTime)
        }
        _ => panic!("unimplemented clock_gettime({})", id),
    };

    let rustix_time = match convert_res(rustix::time::clock_gettime_dynamic(id)) {
        Some(rustix_time) => rustix_time,
        None => return -1,
    };

    match rustix_timespec_to_libc_timespec(rustix_time) {
        Ok(t) => {
            *tp = t;
            0
        }
        Err(_) => {
            set_errno(Errno(libc::EOVERFLOW));
            -1
        }
    }
}

#[no_mangle]
unsafe extern "C" fn clock_getres(id: c_int, tp: *mut libc::timespec) -> c_int {
    libc!(libc::clock_getres(id, tp));

    let id = match id {
        libc::CLOCK_MONOTONIC => rustix::time::ClockId::Monotonic,
        libc::CLOCK_REALTIME => rustix::time::ClockId::Realtime,
        _ => panic!("unimplemented clock_getres({})", id),
    };

    let rustix_time = rustix::time::clock_getres(id);

    match rustix_timespec_to_libc_timespec(rustix_time) {
        Ok(t) => {
            *tp = t;
            0
        }
        Err(_) => {
            set_errno(Errno(libc::EOVERFLOW));
            -1
        }
    }
}

#[no_mangle]
unsafe extern "C" fn time(t: *mut libc::time_t) -> libc::time_t {
    libc!(libc::time(t));

    let mut ts: libc::timespec = { core::mem::zeroed() };
    if clock_gettime(libc::CLOCK_REALTIME, &mut ts) == -1 {
        return -1;
    }

    if !t.is_null() {
        *t = ts.tv_sec;
    }

    ts.tv_sec
}

#[cfg(not(target_env = "musl"))]
#[no_mangle]
unsafe extern "C" fn gettimeofday(t: *mut libc::timeval, _tz: *mut libc::timezone) -> c_int {
    libc!(libc::gettimeofday(t, _tz));
    _gettimeofday(t)
}

#[cfg(target_env = "musl")]
#[no_mangle]
unsafe extern "C" fn gettimeofday(t: *mut libc::timeval, _tz: *mut libc::c_void) -> c_int {
    libc!(libc::gettimeofday(t, _tz));
    _gettimeofday(t)
}

unsafe fn _gettimeofday(t: *mut libc::timeval) -> c_int {
    if t.is_null() {
        return 0;
    }

    let mut ts: libc::timespec = { core::mem::zeroed() };
    if clock_gettime(libc::CLOCK_REALTIME, &mut ts) == -1 {
        return -1;
    }

    if !t.is_null() {
        (*t).tv_sec = ts.tv_sec;
        (*t).tv_usec = ts.tv_nsec / 1000;
    }

    0
}

#[no_mangle]
unsafe extern "C" fn nanosleep(req: *const libc::timespec, rem: *mut libc::timespec) -> c_int {
    libc!(libc::nanosleep(req, rem));

    let req = rustix::time::Timespec {
        tv_sec: (*req).tv_sec.into(),
        tv_nsec: (*req).tv_nsec as _,
    };
    match rustix::thread::nanosleep(&req) {
        rustix::thread::NanosleepRelativeResult::Ok => 0,
        rustix::thread::NanosleepRelativeResult::Interrupted(remaining) => {
            if !rem.is_null() {
                *rem = libc::timespec {
                    tv_sec: remaining.tv_sec.try_into().unwrap(),
                    tv_nsec: remaining.tv_nsec as _,
                };
            }
            set_errno(Errno(libc::EINTR));
            -1
        }
        rustix::thread::NanosleepRelativeResult::Err(err) => {
            set_errno(Errno(err.raw_os_error()));
            -1
        }
    }
}

#[no_mangle]
unsafe extern "C" fn clock_nanosleep(
    clockid: libc::clockid_t,
    flags: c_int,
    req: *const libc::timespec,
    rem: *mut libc::timespec,
) -> c_int {
    libc!(libc::clock_nanosleep(clockid, flags, req, rem));

    let clockid = match clockid {
        libc::CLOCK_MONOTONIC => rustix::thread::ClockId::Monotonic,
        libc::CLOCK_REALTIME => rustix::thread::ClockId::Realtime,
        libc::CLOCK_PROCESS_CPUTIME_ID => rustix::thread::ClockId::ProcessCPUTime,
        libc::CLOCK_THREAD_CPUTIME_ID => rustix::thread::ClockId::ThreadCPUTime,
        libc::CLOCK_REALTIME_COARSE => rustix::thread::ClockId::RealtimeCoarse,
        libc::CLOCK_MONOTONIC_COARSE => rustix::thread::ClockId::MonotonicCoarse,
        libc::CLOCK_MONOTONIC_RAW => rustix::thread::ClockId::MonotonicRaw,
        _ => return libc::EINVAL,
    };

    let req = rustix::time::Timespec {
        tv_sec: (*req).tv_sec.into(),
        tv_nsec: (*req).tv_nsec as _,
    };
    if flags == libc::TIMER_ABSTIME {
        match convert_res(rustix::thread::clock_nanosleep_absolute(clockid, &req)) {
            Some(()) => 0,
            None => -1,
        }
    } else if flags == 0 {
        match rustix::thread::clock_nanosleep_relative(clockid, &req) {
            rustix::thread::NanosleepRelativeResult::Ok => 0,
            rustix::thread::NanosleepRelativeResult::Interrupted(remaining) => {
                if !rem.is_null() {
                    *rem = libc::timespec {
                        tv_sec: remaining.tv_sec.try_into().unwrap(),
                        tv_nsec: remaining.tv_nsec as _,
                    };
                }
                libc::EINTR
            }
            rustix::thread::NanosleepRelativeResult::Err(err) => err.raw_os_error(),
        }
    } else {
        libc::EINVAL
    }
}

#[no_mangle]
unsafe extern "C" fn sleep(seconds: c_uint) -> c_uint {
    libc!(libc::sleep(seconds));

    let req = rustix::time::Timespec {
        tv_sec: seconds.into(),
        tv_nsec: 0,
    };
    match rustix::thread::nanosleep(&req) {
        rustix::thread::NanosleepRelativeResult::Ok => 0,
        rustix::thread::NanosleepRelativeResult::Interrupted(remaining) => remaining.tv_sec as _,
        rustix::thread::NanosleepRelativeResult::Err(_err) => unreachable!(),
    }
}

#[no_mangle]
unsafe extern "C" fn usleep(usec: libc::useconds_t) -> c_int {
    libc!(libc::usleep(usec));

    let usec: i64 = usec.into();
    let req = rustix::time::Timespec {
        tv_sec: usec / 1000000,
        tv_nsec: (usec % 1000000) * 1000,
    };
    match rustix::thread::nanosleep(&req) {
        rustix::thread::NanosleepRelativeResult::Ok => 0,
        rustix::thread::NanosleepRelativeResult::Interrupted(_remaining) => {
            set_errno(Errno(libc::EINTR));
            -1
        }
        rustix::thread::NanosleepRelativeResult::Err(err) => {
            set_errno(Errno(err.raw_os_error()));
            -1
        }
    }
}

#[no_mangle]
unsafe extern "C" fn clock_settime(id: c_int, tp: *mut libc::timespec) -> c_int {
    libc!(libc::clock_settime(id, tp));

    let id = match id {
        libc::CLOCK_MONOTONIC => rustix::time::ClockId::Monotonic,
        libc::CLOCK_REALTIME => rustix::time::ClockId::Realtime,
        _ => panic!("unimplemented clock({})", id),
    };

    let timespec = rustix::time::Timespec {
        tv_sec: (*tp).tv_sec.into(),
        tv_nsec: (*tp).tv_nsec as _,
    };

    match convert_res(rustix::time::clock_settime(id, timespec)) {
        Some(()) => 0,
        None => -1,
    }
}

#[no_mangle]
unsafe extern "C" fn difftime(time1: libc::time_t, time0: libc::time_t) -> f64 {
    libc!(libc::difftime(time1, time0));

    (time1 as i128 - time0 as i128) as f64
}

#[no_mangle]
unsafe extern "C" fn clock() -> libc::clock_t {
    //libc!(libc::clock());

    let time = rustix::time::clock_gettime(rustix::time::ClockId::ProcessCPUTime);

    time.tv_sec
        .checked_mul(1_000_000)
        .map(|usec| usec + time.tv_nsec / 1000)
        .unwrap_or(-1)
        .try_into()
        .unwrap_or(-1)
}
