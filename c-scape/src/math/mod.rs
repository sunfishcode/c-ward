#[no_mangle]
unsafe extern "C" fn acos(x: f64) -> f64 {
    libm::acos(x)
}

#[no_mangle]
unsafe extern "C" fn acosf(x: f32) -> f32 {
    libm::acosf(x)
}

#[no_mangle]
unsafe extern "C" fn acosh(x: f64) -> f64 {
    libm::acosh(x)
}

#[no_mangle]
unsafe extern "C" fn acoshf(x: f32) -> f32 {
    libm::acoshf(x)
}

#[no_mangle]
unsafe extern "C" fn asin(x: f64) -> f64 {
    libm::asin(x)
}

#[no_mangle]
unsafe extern "C" fn asinf(x: f32) -> f32 {
    libm::asinf(x)
}

#[no_mangle]
unsafe extern "C" fn asinh(x: f64) -> f64 {
    libm::asinh(x)
}

#[no_mangle]
unsafe extern "C" fn asinhf(x: f32) -> f32 {
    libm::asinhf(x)
}

#[no_mangle]
unsafe extern "C" fn atan(x: f64) -> f64 {
    libm::atan(x)
}

#[no_mangle]
unsafe extern "C" fn atan2(x: f64, y: f64) -> f64 {
    libm::atan2(x, y)
}

#[no_mangle]
unsafe extern "C" fn atan2f(x: f32, y: f32) -> f32 {
    libm::atan2f(x, y)
}

#[no_mangle]
unsafe extern "C" fn atanf(x: f32) -> f32 {
    libm::atanf(x)
}

#[no_mangle]
unsafe extern "C" fn atanh(x: f64) -> f64 {
    libm::atanh(x)
}

#[no_mangle]
unsafe extern "C" fn atanhf(x: f32) -> f32 {
    libm::atanhf(x)
}

#[no_mangle]
unsafe extern "C" fn cbrt(x: f64) -> f64 {
    libm::cbrt(x)
}

#[no_mangle]
unsafe extern "C" fn cbrtf(x: f32) -> f32 {
    libm::cbrtf(x)
}

#[no_mangle]
unsafe extern "C" fn ceil(x: f64) -> f64 {
    libm::ceil(x)
}

#[no_mangle]
unsafe extern "C" fn ceilf(x: f32) -> f32 {
    libm::ceilf(x)
}

#[no_mangle]
unsafe extern "C" fn copysign(x: f64, y: f64) -> f64 {
    libm::copysign(x, y)
}

#[no_mangle]
unsafe extern "C" fn copysignf(x: f32, y: f32) -> f32 {
    libm::copysignf(x, y)
}

#[no_mangle]
unsafe extern "C" fn cos(x: f64) -> f64 {
    libm::cos(x)
}

#[no_mangle]
unsafe extern "C" fn cosf(x: f32) -> f32 {
    libm::cosf(x)
}

#[no_mangle]
unsafe extern "C" fn cosh(x: f64) -> f64 {
    libm::cosh(x)
}

#[no_mangle]
unsafe extern "C" fn coshf(x: f32) -> f32 {
    libm::coshf(x)
}

#[no_mangle]
unsafe extern "C" fn erf(x: f64) -> f64 {
    libm::erf(x)
}

#[no_mangle]
unsafe extern "C" fn erfc(x: f64) -> f64 {
    libm::erfc(x)
}

#[no_mangle]
unsafe extern "C" fn erfcf(x: f32) -> f32 {
    libm::erfcf(x)
}

#[no_mangle]
unsafe extern "C" fn erff(x: f32) -> f32 {
    libm::erff(x)
}

#[no_mangle]
unsafe extern "C" fn exp(x: f64) -> f64 {
    libm::exp(x)
}

#[no_mangle]
unsafe extern "C" fn exp2(x: f64) -> f64 {
    libm::exp2(x)
}

#[no_mangle]
unsafe extern "C" fn exp2f(x: f32) -> f32 {
    libm::exp2f(x)
}

#[no_mangle]
unsafe extern "C" fn exp10(x: f64) -> f64 {
    libm::exp10(x)
}

#[no_mangle]
unsafe extern "C" fn exp10f(x: f32) -> f32 {
    libm::exp10f(x)
}

#[no_mangle]
unsafe extern "C" fn expf(x: f32) -> f32 {
    libm::expf(x)
}

#[no_mangle]
unsafe extern "C" fn expm1(x: f64) -> f64 {
    libm::expm1(x)
}

#[no_mangle]
unsafe extern "C" fn expm1f(x: f32) -> f32 {
    libm::expm1f(x)
}

#[no_mangle]
unsafe extern "C" fn fabs(x: f64) -> f64 {
    libm::fabs(x)
}

#[no_mangle]
unsafe extern "C" fn fabsf(x: f32) -> f32 {
    libm::fabsf(x)
}

#[no_mangle]
unsafe extern "C" fn fdim(x: f64, y: f64) -> f64 {
    libm::fdim(x, y)
}

#[no_mangle]
unsafe extern "C" fn fdimf(x: f32, y: f32) -> f32 {
    libm::fdimf(x, y)
}

#[no_mangle]
unsafe extern "C" fn floor(x: f64) -> f64 {
    libm::floor(x)
}

#[no_mangle]
unsafe extern "C" fn floorf(x: f32) -> f32 {
    libm::floorf(x)
}

#[no_mangle]
unsafe extern "C" fn fma(x: f64, y: f64, z: f64) -> f64 {
    libm::fma(x, y, z)
}

#[no_mangle]
unsafe extern "C" fn fmaf(x: f32, y: f32, z: f32) -> f32 {
    libm::fmaf(x, y, z)
}

#[no_mangle]
unsafe extern "C" fn fmax(x: f64, y: f64) -> f64 {
    libm::fmax(x, y)
}

#[no_mangle]
unsafe extern "C" fn fmaxf(x: f32, y: f32) -> f32 {
    libm::fmaxf(x, y)
}

#[no_mangle]
unsafe extern "C" fn fmin(x: f64, y: f64) -> f64 {
    libm::fmin(x, y)
}

#[no_mangle]
unsafe extern "C" fn fminf(x: f32, y: f32) -> f32 {
    libm::fminf(x, y)
}

#[no_mangle]
unsafe extern "C" fn fmod(x: f64, y: f64) -> f64 {
    libm::fmod(x, y)
}

#[no_mangle]
unsafe extern "C" fn fmodf(x: f32, y: f32) -> f32 {
    libm::fmodf(x, y)
}

#[no_mangle]
unsafe extern "C" fn frexp(x: f64, y: *mut i32) -> f64 {
    let (a, b) = libm::frexp(x);
    *y = b;
    a
}

#[no_mangle]
unsafe extern "C" fn frexpf(x: f32, y: *mut i32) -> f32 {
    let (a, b) = libm::frexpf(x);
    *y = b;
    a
}

#[no_mangle]
unsafe extern "C" fn hypot(x: f64, y: f64) -> f64 {
    libm::hypot(x, y)
}

#[no_mangle]
unsafe extern "C" fn hypotf(x: f32, y: f32) -> f32 {
    libm::hypotf(x, y)
}

#[no_mangle]
unsafe extern "C" fn ilogb(x: f64) -> i32 {
    libm::ilogb(x)
}

#[no_mangle]
unsafe extern "C" fn ilogbf(x: f32) -> i32 {
    libm::ilogbf(x)
}

#[no_mangle]
unsafe extern "C" fn j0(x: f64) -> f64 {
    libm::j0(x)
}

#[no_mangle]
unsafe extern "C" fn j0f(x: f32) -> f32 {
    libm::j0f(x)
}

#[no_mangle]
unsafe extern "C" fn j1(x: f64) -> f64 {
    libm::j1(x)
}

#[no_mangle]
unsafe extern "C" fn j1f(x: f32) -> f32 {
    libm::j1f(x)
}

#[no_mangle]
unsafe extern "C" fn jn(x: i32, y: f64) -> f64 {
    libm::jn(x, y)
}

#[no_mangle]
unsafe extern "C" fn jnf(x: i32, y: f32) -> f32 {
    libm::jnf(x, y)
}

#[no_mangle]
unsafe extern "C" fn ldexp(x: f64, y: i32) -> f64 {
    libm::ldexp(x, y)
}

#[no_mangle]
unsafe extern "C" fn ldexpf(x: f32, y: i32) -> f32 {
    libm::ldexpf(x, y)
}

#[no_mangle]
unsafe extern "C" fn lgamma_r(x: f64, y: *mut i32) -> f64 {
    let (a, b) = libm::lgamma_r(x);
    *y = b;
    a
}

#[no_mangle]
unsafe extern "C" fn lgammaf_r(x: f32, y: *mut i32) -> f32 {
    let (a, b) = libm::lgammaf_r(x);
    *y = b;
    a
}

#[no_mangle]
unsafe extern "C" fn log(x: f64) -> f64 {
    libm::log(x)
}

#[no_mangle]
unsafe extern "C" fn log1p(x: f64) -> f64 {
    libm::log1p(x)
}

#[no_mangle]
unsafe extern "C" fn log1pf(x: f32) -> f32 {
    libm::log1pf(x)
}

#[no_mangle]
unsafe extern "C" fn log2(x: f64) -> f64 {
    libm::log2(x)
}

#[no_mangle]
unsafe extern "C" fn log2f(x: f32) -> f32 {
    libm::log2f(x)
}

#[no_mangle]
unsafe extern "C" fn log10(x: f64) -> f64 {
    libm::log10(x)
}

#[no_mangle]
unsafe extern "C" fn log10f(x: f32) -> f32 {
    libm::log10f(x)
}

#[no_mangle]
unsafe extern "C" fn logf(x: f32) -> f32 {
    libm::logf(x)
}

#[no_mangle]
unsafe extern "C" fn modf(x: f64, y: *mut f64) -> f64 {
    let (a, b) = libm::modf(x);
    *y = b;
    a
}

#[no_mangle]
unsafe extern "C" fn modff(x: f32, y: *mut f32) -> f32 {
    let (a, b) = libm::modff(x);
    *y = b;
    a
}

#[no_mangle]
unsafe extern "C" fn nextafter(x: f64, y: f64) -> f64 {
    libm::nextafter(x, y)
}

#[no_mangle]
unsafe extern "C" fn nextafterf(x: f32, y: f32) -> f32 {
    libm::nextafterf(x, y)
}

#[no_mangle]
unsafe extern "C" fn pow(x: f64, y: f64) -> f64 {
    libm::pow(x, y)
}

#[no_mangle]
unsafe extern "C" fn powf(x: f32, y: f32) -> f32 {
    libm::powf(x, y)
}

#[no_mangle]
unsafe extern "C" fn remainder(x: f64, y: f64) -> f64 {
    libm::remainder(x, y)
}

#[no_mangle]
unsafe extern "C" fn drem(x: f64, y: f64) -> f64 {
    remainder(x, y)
}

#[no_mangle]
unsafe extern "C" fn remainderf(x: f32, y: f32) -> f32 {
    libm::remainderf(x, y)
}

#[no_mangle]
unsafe extern "C" fn dremf(x: f32, y: f32) -> f32 {
    remainderf(x, y)
}

#[no_mangle]
unsafe extern "C" fn remquo(x: f64, y: f64, z: *mut i32) -> f64 {
    let (a, b) = libm::remquo(x, y);
    *z = b;
    a
}

#[no_mangle]
unsafe extern "C" fn remquof(x: f32, y: f32, z: *mut i32) -> f32 {
    let (a, b) = libm::remquof(x, y);
    *z = b;
    a
}

#[no_mangle]
unsafe extern "C" fn round(x: f64) -> f64 {
    libm::round(x)
}

#[no_mangle]
unsafe extern "C" fn roundf(x: f32) -> f32 {
    libm::roundf(x)
}

#[no_mangle]
unsafe extern "C" fn scalbn(x: f64, y: i32) -> f64 {
    libm::scalbn(x, y)
}

#[no_mangle]
unsafe extern "C" fn scalbnf(x: f32, y: i32) -> f32 {
    libm::scalbnf(x, y)
}

#[no_mangle]
unsafe extern "C" fn scalbln(x: f64, y: libc::c_long) -> f64 {
    let y = y.clamp(libc::c_int::MIN.into(), libc::c_int::MAX.into()) as _;
    scalbn(x, y)
}

#[no_mangle]
unsafe extern "C" fn scalblnf(x: f32, y: libc::c_long) -> f32 {
    let y = y.clamp(libc::c_int::MIN.into(), libc::c_int::MAX.into()) as _;
    scalbnf(x, y)
}

#[no_mangle]
unsafe extern "C" fn scalb(x: f64, exp: f64) -> f64 {
    if x.is_nan() {
        x - 0.0
    } else if exp.is_nan() {
        exp - 0.0
    } else if !exp.is_finite() {
        if exp > 0.0 {
            x * exp
        } else {
            x / -exp
        }
    } else if rint(exp) != exp {
        f64::NAN
    } else if exp > 65000.0 {
        scalbn(x, 65000)
    } else if -exp > 65000.0 {
        scalbn(x, -65000)
    } else {
        scalbn(x, exp as i32)
    }
}

#[no_mangle]
unsafe extern "C" fn scalbf(x: f32, exp: f32) -> f32 {
    if x.is_nan() {
        x - 0.0
    } else if exp.is_nan() {
        exp - 0.0
    } else if !exp.is_finite() {
        if exp > 0.0 {
            x * exp
        } else {
            x / -exp
        }
    } else if rintf(exp) != exp {
        f32::NAN
    } else if exp > 65000.0 {
        scalbnf(x, 65000)
    } else if -exp > 65000.0 {
        scalbnf(x, -65000)
    } else {
        scalbnf(x, exp as i32)
    }
}

#[no_mangle]
unsafe extern "C" fn sin(x: f64) -> f64 {
    libm::sin(x)
}

#[no_mangle]
unsafe extern "C" fn sincos(x: f64, y: *mut f64, z: *mut f64) {
    let (a, b) = libm::sincos(x);
    *y = a;
    *z = b;
}

#[no_mangle]
unsafe extern "C" fn sincosf(x: f32, y: *mut f32, z: *mut f32) {
    let (a, b) = libm::sincosf(x);
    *y = a;
    *z = b;
}

#[no_mangle]
unsafe extern "C" fn sinf(x: f32) -> f32 {
    libm::sinf(x)
}

#[no_mangle]
unsafe extern "C" fn sinh(x: f64) -> f64 {
    libm::sinh(x)
}

#[no_mangle]
unsafe extern "C" fn sinhf(x: f32) -> f32 {
    libm::sinhf(x)
}

#[no_mangle]
unsafe extern "C" fn sqrt(x: f64) -> f64 {
    libm::sqrt(x)
}

#[no_mangle]
unsafe extern "C" fn sqrtf(x: f32) -> f32 {
    libm::sqrtf(x)
}

#[no_mangle]
unsafe extern "C" fn tan(x: f64) -> f64 {
    libm::tan(x)
}

#[no_mangle]
unsafe extern "C" fn tanf(x: f32) -> f32 {
    libm::tanf(x)
}

#[no_mangle]
unsafe extern "C" fn tanh(x: f64) -> f64 {
    libm::tanh(x)
}

#[no_mangle]
unsafe extern "C" fn tanhf(x: f32) -> f32 {
    libm::tanhf(x)
}

#[no_mangle]
unsafe extern "C" fn tgamma(x: f64) -> f64 {
    libm::tgamma(x)
}

#[no_mangle]
unsafe extern "C" fn tgammaf(x: f32) -> f32 {
    libm::tgammaf(x)
}

#[no_mangle]
unsafe extern "C" fn trunc(x: f64) -> f64 {
    libm::trunc(x)
}

#[no_mangle]
unsafe extern "C" fn truncf(x: f32) -> f32 {
    libm::truncf(x)
}

#[no_mangle]
unsafe extern "C" fn y0(x: f64) -> f64 {
    libm::y0(x)
}

#[no_mangle]
unsafe extern "C" fn y0f(x: f32) -> f32 {
    libm::y0f(x)
}

#[no_mangle]
unsafe extern "C" fn y1(x: f64) -> f64 {
    libm::y1(x)
}

#[no_mangle]
unsafe extern "C" fn y1f(x: f32) -> f32 {
    libm::y1f(x)
}

#[no_mangle]
unsafe extern "C" fn yn(x: i32, y: f64) -> f64 {
    libm::yn(x, y)
}

#[no_mangle]
unsafe extern "C" fn ynf(x: i32, y: f32) -> f32 {
    libm::ynf(x, y)
}

#[no_mangle]
unsafe extern "C" fn rint(x: f64) -> f64 {
    libm::rint(x)
}

#[no_mangle]
unsafe extern "C" fn rintf(x: f32) -> f32 {
    libm::rintf(x)
}

#[no_mangle]
unsafe extern "C" fn logb(x: f64) -> f64 {
    if x.is_nan() {
        x - 0.0
    } else if x == 0.0 {
        -f64::INFINITY
    } else if fabs(x) == f64::INFINITY {
        f64::INFINITY
    } else {
        ilogb(x) as f64
    }
}

#[no_mangle]
unsafe extern "C" fn logbf(x: f32) -> f32 {
    if x.is_nan() {
        x - 0.0
    } else if x == 0.0 {
        -f32::INFINITY
    } else if fabsf(x) == f32::INFINITY {
        f32::INFINITY
    } else {
        ilogbf(x) as f32
    }
}

// The libm crate doesn't have `lrint` etc., but we can implement them with
// `rint` etc. and casting, because we don't support floating-point exceptions,
// so don't worry about it `FE_INEXACT`.
#[no_mangle]
unsafe extern "C" fn lrint(x: f64) -> libc::c_long {
    rint(x) as libc::c_long
}

#[no_mangle]
unsafe extern "C" fn lrintf(x: f32) -> libc::c_long {
    rintf(x) as libc::c_long
}

#[no_mangle]
unsafe extern "C" fn llrint(x: f64) -> libc::c_longlong {
    rint(x) as libc::c_longlong
}

#[no_mangle]
unsafe extern "C" fn llrintf(x: f32) -> libc::c_longlong {
    rintf(x) as libc::c_longlong
}

#[no_mangle]
unsafe extern "C" fn lround(x: f64) -> libc::c_long {
    round(x) as libc::c_long
}

#[no_mangle]
unsafe extern "C" fn lroundf(x: f32) -> libc::c_long {
    roundf(x) as libc::c_long
}

#[no_mangle]
unsafe extern "C" fn llround(x: f64) -> libc::c_longlong {
    round(x) as libc::c_longlong
}

#[no_mangle]
unsafe extern "C" fn llroundf(x: f32) -> libc::c_longlong {
    roundf(x) as libc::c_longlong
}

// `nearbyint` differs from `rint` in that it doesn't raise
// `FE_INEXACT`. But we don't support floating-point exceptions
// anyway, so don't worry about it.
#[no_mangle]
unsafe extern "C" fn nearbyint(x: f64) -> f64 {
    libm::rint(x)
}

#[no_mangle]
unsafe extern "C" fn nearbyintf(x: f32) -> f32 {
    libm::rintf(x)
}

#[no_mangle]
unsafe extern "C" fn finite(x: f64) -> i32 {
    x.is_finite() as i32
}
#[no_mangle]
unsafe extern "C" fn finitef(x: f32) -> i32 {
    x.is_finite() as i32
}

#[no_mangle]
unsafe extern "C" fn isnan(x: f64) -> i32 {
    x.is_nan() as i32
}

#[no_mangle]
unsafe extern "C" fn isnanf(x: f32) -> i32 {
    x.is_nan() as i32
}

#[no_mangle]
unsafe extern "C" fn isinf(x: f64) -> i32 {
    if x == f64::INFINITY {
        1
    } else if x == f64::NEG_INFINITY {
        -1
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "C" fn isinff(x: f32) -> i32 {
    if x == f32::INFINITY {
        1
    } else if x == f32::NEG_INFINITY {
        -1
    } else {
        0
    }
}

#[no_mangle]
static mut signgam: i32 = 0;

#[no_mangle]
unsafe extern "C" fn lgamma(x: f64) -> f64 {
    let (a, b) = libm::lgamma_r(x);
    signgam = b;
    a
}

#[no_mangle]
unsafe extern "C" fn lgammaf(x: f32) -> f32 {
    let (res, sgn) = libm::lgammaf_r(x);
    signgam = sgn;
    res
}

// Enable support for complex numbers only on architectures where the builtin
// C complex type has the same calling convention rules as a struct containing
// two scalars. Notably, this excludes 32-bit "x86".
#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "riscv64",
    target_arch = "x86_64"
))]
mod complex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_finite_test() {
        unsafe {
            assert_eq!(finite(1.), 1);
            assert_eq!(finite(f64::NAN), 0);
            assert_eq!(finite(f64::INFINITY), 0);
            assert_eq!(finite(f64::NEG_INFINITY), 0);

            assert_eq!(isnan(1.), 0);
            assert_eq!(isnan(f64::NAN), 1);
            assert_eq!(isnan(f64::INFINITY), 0);
            assert_eq!(isnan(f64::NEG_INFINITY), 0);

            assert_eq!(isinf(1.), 0);
            assert_eq!(isinf(f64::NAN), 0);
            assert_eq!(isinf(f64::INFINITY), 1);
            assert_eq!(isinf(f64::NEG_INFINITY), -1);

            assert_eq!(finitef(1.), 1);
            assert_eq!(finitef(f32::NAN), 0);
            assert_eq!(finitef(f32::INFINITY), 0);
            assert_eq!(finitef(f32::NEG_INFINITY), 0);

            assert_eq!(isnanf(1.), 0);
            assert_eq!(isnanf(f32::NAN), 1);
            assert_eq!(isnanf(f32::INFINITY), 0);
            assert_eq!(isnanf(f32::NEG_INFINITY), 0);

            assert_eq!(isinff(1.), 0);
            assert_eq!(isinff(f32::NAN), 0);
            assert_eq!(isinff(f32::INFINITY), 1);
            assert_eq!(isinff(f32::NEG_INFINITY), -1);
        }
    }
}
