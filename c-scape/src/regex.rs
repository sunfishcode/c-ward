//! POSIX regex support, using the posix-regex crate.

use alloc::borrow::Cow;
use alloc::boxed::Box;
use core::ffi::CStr;
use core::ptr::copy_nonoverlapping;
use libc::{
    c_char, c_int, regex_t, regmatch_t, size_t, REG_BADBR, REG_BADPAT, REG_BADRPT, REG_EBRACE,
    REG_EBRACK, REG_ECOLLATE, REG_ECTYPE, REG_EEND, REG_EESCAPE, REG_ENOSYS, REG_EPAREN,
    REG_ERANGE, REG_ERPAREN, REG_ESIZE, REG_ESPACE, REG_ESUBREG, REG_EXTENDED, REG_ICASE,
    REG_NEWLINE, REG_NOMATCH, REG_NOSUB, REG_NOTBOL, REG_NOTEOL,
};
use posix_regex::compile::Error;
use posix_regex::tree::Tree;
use posix_regex::{PosixRegex, PosixRegexBuilder};

#[repr(C)]
struct Regex {
    tree: *mut Tree,
    cflags: c_int,
}

#[no_mangle]
unsafe extern "C" fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int {
    libc!(libc::regcomp(preg, regex, cflags));

    if cflags & REG_EXTENDED == REG_EXTENDED {
        return REG_ENOSYS;
    }

    let preg = preg.cast::<Regex>();
    let regex = CStr::from_ptr(regex).to_bytes();

    match PosixRegexBuilder::new(regex)
        .with_default_classes()
        .compile_tokens()
    {
        Ok(tree) => {
            let tree = Box::into_raw(Box::new(tree));
            *preg = Regex { tree, cflags };
            0
        }
        Err(Error::EOF) | Err(Error::Expected(_, _)) | Err(Error::UnexpectedToken(_)) => REG_BADPAT,
        Err(Error::EmptyRepetition) | Err(Error::IntegerOverflow) | Err(Error::IllegalRange) => {
            REG_BADBR
        }
        Err(Error::InvalidBackRef(_)) => REG_ESUBREG,
        Err(Error::UnclosedRepetition) => REG_EBRACE,
        Err(Error::LeadingRepetition) => REG_BADRPT,
        Err(Error::UnknownCollation) => REG_ECOLLATE,
        Err(Error::UnknownClass(_)) => REG_ECTYPE,
    }
}

#[no_mangle]
unsafe extern "C" fn regexec(
    preg: *const regex_t,
    string: *const c_char,
    nmatch: size_t,
    pmatch: *mut regmatch_t,
    eflags: c_int,
) -> c_int {
    libc!(libc::regexec(preg, string, nmatch, pmatch, eflags));

    let preg = preg.cast::<Regex>();
    let string = CStr::from_ptr(string).to_bytes();

    let preg = &*preg;
    let tree = &*preg.tree;
    let cflags = preg.cflags;

    let nosub = cflags & REG_NOSUB == REG_NOSUB;
    let icase = cflags & REG_ICASE == REG_ICASE;
    let newline = cflags & REG_NEWLINE == REG_NEWLINE;

    let notbol = eflags & REG_NOTBOL == REG_NOTBOL;
    let noteol = eflags & REG_NOTEOL == REG_NOTEOL;

    let matches = PosixRegex::new(Cow::Borrowed(tree))
        .case_insensitive(icase)
        .newline(newline)
        .no_start(notbol)
        .no_end(noteol)
        .matches(string, Some(1));

    if let Some(first) = matches.first() {
        if !nosub {
            for i in 0..nmatch {
                let (start, end) = first.get(i).and_then(|range| *range).unwrap_or((!0, !0));
                *pmatch.add(i) = regmatch_t {
                    rm_so: start.try_into().unwrap(),
                    rm_eo: end.try_into().unwrap(),
                };
            }
        }
        0
    } else {
        REG_NOMATCH
    }
}

#[no_mangle]
unsafe extern "C" fn regerror(
    errcode: c_int,
    preg: *const regex_t,
    errbuf: *mut c_char,
    errbuf_size: size_t,
) -> size_t {
    let _preg = preg.cast::<Regex>();

    let msg = match errcode {
        0 => "Success",
        REG_NOMATCH => "regexec() failed to match",
        REG_BADPAT => "Invalid regular expression",
        REG_ECOLLATE => "Invalid collating element referenced",
        REG_ECTYPE => "Invalid character class type referenced",
        REG_EESCAPE => "Trailing '\\' in pattern",
        REG_ESUBREG => "Number in \\digit invalid or in error",
        REG_EBRACK => "\"[]\" imbalance",
        REG_EPAREN => "\"\\(\\)\" or \"()\" imbalance",
        REG_EBRACE => "\"\\{\\}\" imbalance",
        REG_BADBR => "Content of \"\\{\\}\" invalid: not a number, number too large, more than two numbers, first larger than second",
        REG_ERANGE => "Invalid endpoint in range expression",
        REG_ESPACE => "Out of memory",
        REG_BADRPT => "'?', '*', or '+' not preceded by valid regular expression",
        REG_ENOSYS => "Unsupported operation",
        REG_EEND => "Premature end",
        REG_ESIZE => "Compiled pattern bigger than 2^16 bytes",
        REG_ERPAREN => "Unmatched ) or \\); not returned from regcomp",
        _ => "Unknown error",
    };

    let len = msg.len().min(errbuf_size);
    copy_nonoverlapping(msg.as_ptr(), errbuf as *mut u8, len);
    if errbuf_size != 0 {
        *errbuf.add((msg.len() + 1).min(errbuf_size)) = 0;
    }

    msg.len() + 1
}

#[no_mangle]
unsafe extern "C" fn regfree(preg: *mut regex_t) {
    libc!(libc::regfree(preg));

    let preg = preg.cast::<Regex>();

    let _ = Box::from_raw((*preg).tree);
}
