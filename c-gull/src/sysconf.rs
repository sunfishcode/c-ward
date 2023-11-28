use libc::c_int;

#[no_mangle]
unsafe extern "C" fn get_nprocs_conf() -> c_int {
    //libc!(libc::get_nprocs_conf());

    match std::thread::available_parallelism() {
        Ok(n) => n.get().try_into().unwrap_or(c_int::MAX),
        Err(_) => 1,
    }
}

#[no_mangle]
unsafe extern "C" fn get_nprocs() -> c_int {
    //libc!(libc::get_nprocs());

    get_nprocs_conf()
}
