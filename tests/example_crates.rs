//! Run the programs in the `example-crates` directory and compare their
//! outputs with expected outputs.

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

    let mut command = Command::new("cargo");
    command.arg("run").arg("--quiet");
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
