//! A simple DNS client.

extern crate libc;

use std::net::ToSocketAddrs;

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    for arg in args {
        println!("resolving '{}:", arg);
        let addrs_iter = arg.to_socket_addrs().unwrap();
        for addr in addrs_iter {
            println!(" - {}", addr);
        }
    }
}
