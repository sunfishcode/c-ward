//! Run the programs in the `example-crates` directory and compare their
//! outputs with expected outputs.

#![feature(cfg_target_abi)]

use std::sync::OnceLock;

fn test_crate(
    name: &str,
    args: &[&str],
    envs: &[(&str, &str)],
    stdout: &'static str,
    stderr: &'static str,
    code: Option<i32>,
) {
    use assert_cmd::Command;

    #[cfg(target_arch = "x86_64")]
    let arch = "x86_64";
    #[cfg(target_arch = "aarch64")]
    let arch = "aarch64";
    #[cfg(target_arch = "riscv64")]
    let arch = "riscv64gc";
    #[cfg(target_arch = "x86")]
    let arch = "i686";
    #[cfg(target_arch = "arm")]
    let arch = "armv5te";
    #[cfg(target_env = "gnueabi")]
    let env = "gnueabi";
    #[cfg(all(target_env = "gnu", target_abi = "eabi"))]
    let env = "gnueabi";
    #[cfg(all(target_env = "gnu", not(target_abi = "eabi")))]
    let env = "gnu";

    let mut command = Command::new("cargo");
    command.arg("run").arg("--quiet");
    command.arg(&format!("--target={}-unknown-linux-{}", arch, env));
    command.args(args);

    command.envs(envs.iter().cloned());
    command.current_dir(format!("example-crates/{}", name));
    let assert = command.assert();
    let assert = assert.stdout(stdout).stderr(stderr);
    if let Some(code) = code {
        assert.code(code);
    } else {
        assert.success();
    }
}

#[test]
fn example_crate_libc_replacement() {
    static EXPECTED: OnceLock<String> = OnceLock::new();
    let expected = EXPECTED.get_or_init(|| {
        let uid = unsafe { libc::getuid() };
        let gid = unsafe { libc::getgid() };
        format!(
            "Hello, world! uid={}\nHello world with printf! gid={}\n",
            uid, gid
        )
    });

    test_crate("libc-replacement", &[], &[], &expected, "", None);
}

#[test]
fn example_crate_c_gull_example() {
    test_crate(
        "c-gull-example",
        &[],
        &[],
        "Hello world using Rust `println!`!\nHello world using libc `printf`!\n",
        "",
        None,
    );
}

#[test]
fn example_crate_c_gull_lto() {
    test_crate(
        "c-gull-lto",
        &["--release"],
        &[],
        "Hello world using Rust `println!`!\nHello world using libc `printf`!\n",
        "",
        None,
    );
}

#[test]
fn example_crate_custom_allocator() {
    test_crate(
        "custom-allocator",
        &[],
        &[],
        "Hello world using Rust `println!`!\nHello world using libc `printf`!\n",
        "",
        None,
    );
}

#[test]
fn example_crate_c_scape_example() {
    test_crate("c-scape-example", &[], &[], "Hello, world!\n", "", None);
}
