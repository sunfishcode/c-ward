#![doc = include_str!("../README.md")]
#![no_std]

/// All the functionality of Eyra is factored out into separate libraries. This
/// `extern crate` line is needed to ensure that libraries that intercept C
/// library symbols get linked in.
extern crate c_gull;
