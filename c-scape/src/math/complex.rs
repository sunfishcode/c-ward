use libc::{c_double, c_float};
use num_complex::{Complex32, Complex64, ComplexFloat};

#[no_mangle]
unsafe extern "C" fn creal(x: Complex64) -> c_double {
    //libc!(libc::creal(x));

    x.re
}

#[no_mangle]
unsafe extern "C" fn crealf(x: Complex32) -> c_float {
    //libc!(libc::crealf(x));

    x.re
}

#[no_mangle]
unsafe extern "C" fn cimag(x: Complex64) -> c_double {
    //libc!(libc::cimag(x));

    x.im
}

#[no_mangle]
unsafe extern "C" fn cimagf(x: Complex32) -> c_float {
    //libc!(libc::cimagf(x));

    x.im
}

#[no_mangle]
unsafe extern "C" fn cabs(x: Complex64) -> c_double {
    //libc!(libc::cabs(x));

    x.abs()
}

#[no_mangle]
unsafe extern "C" fn cabsf(x: Complex32) -> c_float {
    //libc!(libc::cabsf(x));

    x.abs()
}

#[no_mangle]
unsafe extern "C" fn carg(x: Complex64) -> c_double {
    //libc!(libc::carg(x));

    x.arg()
}

#[no_mangle]
unsafe extern "C" fn cargf(x: Complex32) -> c_float {
    //libc!(libc::cargf(x));

    x.arg()
}

#[no_mangle]
unsafe extern "C" fn cacos(x: Complex64) -> Complex64 {
    //libc!(libc::cacos(x));

    x.acos()
}

#[no_mangle]
unsafe extern "C" fn cacosf(x: Complex32) -> Complex32 {
    //libc!(libc::cacosf(x));

    x.acos()
}

#[no_mangle]
unsafe extern "C" fn casin(x: Complex64) -> Complex64 {
    //libc!(libc::casin(x));

    x.asin()
}

#[no_mangle]
unsafe extern "C" fn casinf(x: Complex32) -> Complex32 {
    //libc!(libc::casinf(x));

    x.asin()
}

#[no_mangle]
unsafe extern "C" fn catan(x: Complex64) -> Complex64 {
    //libc!(libc::catan(x));

    x.atan()
}

#[no_mangle]
unsafe extern "C" fn catanf(x: Complex32) -> Complex32 {
    //libc!(libc::catanf(x));

    x.atan()
}

#[no_mangle]
unsafe extern "C" fn ccos(x: Complex64) -> Complex64 {
    //libc!(libc::ccos(x));

    x.cos()
}

#[no_mangle]
unsafe extern "C" fn ccosf(x: Complex32) -> Complex32 {
    //libc!(libc::ccosf(x));

    x.cos()
}

#[no_mangle]
unsafe extern "C" fn csin(x: Complex64) -> Complex64 {
    //libc!(libc::csin(x));

    x.sin()
}

#[no_mangle]
unsafe extern "C" fn csinf(x: Complex32) -> Complex32 {
    //libc!(libc::csinf(x));

    x.sin()
}

#[no_mangle]
unsafe extern "C" fn ctan(x: Complex64) -> Complex64 {
    //libc!(libc::ctan(x));

    x.tan()
}

#[no_mangle]
unsafe extern "C" fn ctanf(x: Complex32) -> Complex32 {
    //libc!(libc::ctanf(x));

    x.tan()
}

#[no_mangle]
unsafe extern "C" fn cacosh(x: Complex64) -> Complex64 {
    //libc!(libc::cacosh(x));

    x.acosh()
}

#[no_mangle]
unsafe extern "C" fn cacoshf(x: Complex32) -> Complex32 {
    //libc!(libc::cacoshf(x));

    x.acosh()
}

#[no_mangle]
unsafe extern "C" fn casinh(x: Complex64) -> Complex64 {
    //libc!(libc::casinh(x));

    x.asinh()
}

#[no_mangle]
unsafe extern "C" fn casinhf(x: Complex32) -> Complex32 {
    //libc!(libc::casinhf(x));

    x.asinh()
}

#[no_mangle]
unsafe extern "C" fn catanh(x: Complex64) -> Complex64 {
    //libc!(libc::catanh(x));

    x.atanh()
}

#[no_mangle]
unsafe extern "C" fn catanhf(x: Complex32) -> Complex32 {
    //libc!(libc::catanhf(x));

    x.atanh()
}

#[no_mangle]
unsafe extern "C" fn ccosh(x: Complex64) -> Complex64 {
    //libc!(libc::ccosh(x));

    x.cosh()
}

#[no_mangle]
unsafe extern "C" fn ccoshf(x: Complex32) -> Complex32 {
    //libc!(libc::ccoshf(x));

    x.cosh()
}

#[no_mangle]
unsafe extern "C" fn csinh(x: Complex64) -> Complex64 {
    //libc!(libc::csinh(x));

    x.sinh()
}

#[no_mangle]
unsafe extern "C" fn csinhf(x: Complex32) -> Complex32 {
    //libc!(libc::csinhf(x));

    x.sinh()
}

#[no_mangle]
unsafe extern "C" fn ctanh(x: Complex64) -> Complex64 {
    //libc!(libc::ctanh(x));

    x.tanh()
}

#[no_mangle]
unsafe extern "C" fn ctanhf(x: Complex32) -> Complex32 {
    //libc!(libc::ctanhf(x));

    x.tanh()
}

#[no_mangle]
unsafe extern "C" fn cexp(x: Complex64) -> Complex64 {
    //libc!(libc::cexp(x));

    x.exp()
}

#[no_mangle]
unsafe extern "C" fn cexpf(x: Complex32) -> Complex32 {
    //libc!(libc::cexpf(x));

    x.exp()
}

#[no_mangle]
unsafe extern "C" fn clog(x: Complex64) -> Complex64 {
    //libc!(libc::clog(x));

    x.ln()
}

#[no_mangle]
unsafe extern "C" fn clogf(x: Complex32) -> Complex32 {
    //libc!(libc::clogf(x));

    x.ln()
}

#[no_mangle]
unsafe extern "C" fn clog10(x: Complex64) -> Complex64 {
    //libc!(libc::clog10(x));

    x.log10()
}

#[no_mangle]
unsafe extern "C" fn clog10f(x: Complex32) -> Complex32 {
    //libc!(libc::clog10f(x));

    x.log10()
}

#[no_mangle]
unsafe extern "C" fn csqrt(x: Complex64) -> Complex64 {
    //libc!(libc::csqrt(x));

    x.sqrt()
}

#[no_mangle]
unsafe extern "C" fn csqrtf(x: Complex32) -> Complex32 {
    //libc!(libc::csqrtf(x));

    x.sqrt()
}

#[no_mangle]
unsafe extern "C" fn conj(x: Complex64) -> Complex64 {
    //libc!(libc::conj(x));

    x.conj()
}

#[no_mangle]
unsafe extern "C" fn conjf(x: Complex32) -> Complex32 {
    //libc!(libc::conjf(x));

    x.conj()
}

#[no_mangle]
unsafe extern "C" fn cproj(x: Complex64) -> Complex64 {
    //libc!(libc::cproj(x));

    if x.re.abs() == f64::INFINITY && x.im.abs() == f64::INFINITY {
        Complex64 {
            re: f64::INFINITY,
            im: libm::copysign(0.0, x.im),
        }
    } else {
        x
    }
}

#[no_mangle]
unsafe extern "C" fn cprojf(x: Complex32) -> Complex32 {
    //libc!(libc::cprojf(x));

    if x.re.abs() == f32::INFINITY && x.im.abs() == f32::INFINITY {
        Complex32 {
            re: f32::INFINITY,
            im: libm::copysignf(0.0, x.im),
        }
    } else {
        x
    }
}

#[no_mangle]
unsafe extern "C" fn cpow(x: Complex64, y: Complex64) -> Complex64 {
    //libc!(libc::cpow(x, y));

    x.powc(y)
}

#[no_mangle]
unsafe extern "C" fn cpowf(x: Complex32, y: Complex32) -> Complex32 {
    //libc!(libc::cpowf(x, y));

    x.powc(y)
}
