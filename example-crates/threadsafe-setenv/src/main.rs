//! An example derived from
//! <https://github.com/evanj/cgogetenvcrash/blob/main/rustsetenvcrash/src/main.rs>
//! showing `setenv` unsoundness in practice. c-ward fixes this with its
//! "threadsafe-setenv" feature.

extern crate libc;

use std::net::ToSocketAddrs;

fn main() {
    const NUM_ITERATIONS: usize = 100;

    // terrible argument parsing without dependencies
    const TRY_GETENV: &str = "try_getenv";
    let program_args: Vec<String> = std::env::args().collect();
    if program_args.len() > 2 {
        eprintln!("ERROR: rustsetenvcrash only accepts one optional argument: {TRY_GETENV}; found {} args",
            program_args.len());
        std::process::exit(1);
    }
    if program_args.len() == 2 && program_args[1] != TRY_GETENV {
        eprintln!(
            "ERROR: rustsetenvcrash only accepts one optional argument: {TRY_GETENV}; found {}",
            program_args[1]
        );
        std::process::exit(1);
    }
    let try_getenv = program_args.len() == 2;

    println!("will call std::env::set_var() {NUM_ITERATIONS} times ...");

    let t = if try_getenv {
        println!(
            "spawning thread to call std::env::var (will not crash: Rust holds lock for getenv)..."
        );
        std::thread::spawn(do_getenv)
    } else {
        println!("spawning thread to lookup localhost (may crash with glibc; run with {TRY_GETENV} to call getenv instead)...");
        std::thread::spawn(lookup_localhost)
    };

    for i in 0..NUM_ITERATIONS {
        std::env::set_var(format!("ENV_VAR_{i}"), "value");
    }

    t.join().expect("BUG thread must succeed");

    println!("exiting without error");
}

fn lookup_localhost() {
    let addrs_iter = "localhost:1".to_socket_addrs().unwrap();
    for addr in addrs_iter {
        println!("localhost: ip={} port={}", addr.ip(), addr.port());
    }
}

fn do_getenv() {
    for _ in 0..1000 {
        let r = std::env::var("doesnotexist");
        assert!(r.is_ok(), "env var should not exist");
    }
}
