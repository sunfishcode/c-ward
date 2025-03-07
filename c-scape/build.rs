#[cfg(feature = "cc")]
use cc::Build;
use std::env::var;

fn main() {
    let take_charge = var("CARGO_FEATURE_TAKE_CHARGE").is_ok();

    // In coexist-with-libc builds, do nothing. But in take-charge builds, add
    // empty versions of libc.a and other libraries to the linker commandline,
    // to prevent the linker from finding and linking in the system versions.
    if take_charge {
        for name in &[
            "gcc", "gcc_s", "util", "rt", "pthread", "m", "dl", "c", "crypt", "xnet", "resolv",
            "unwind",
        ] {
            link_in_empty(name);
        }
    }

    let os = var("CARGO_CFG_TARGET_OS").unwrap();

    if os == "linux" || os == "l4re" || os == "android" || os == "emscripten" {
        use_feature("linux_like");
    }

    // Add some additional common target combinations.

    // Android and "regular" Linux both use the Linux kernel.
    if os == "android" || os == "linux" {
        use_feature("linux_kernel");
    }
}

fn link_in_empty(name: &str) {
    let arch = var("CARGO_CFG_TARGET_ARCH").unwrap();
    let outline_path = format!("empty/{}", arch);
    let to = format!("{}/lib{}.a", outline_path, name);
    println!("cargo:rerun-if-changed={}", to);

    // If "cc" is not enabled, use a pre-built library.
    #[cfg(not(feature = "cc"))]
    {
        println!(
            "cargo:rustc-link-search={}",
            std::fs::canonicalize(outline_path).unwrap().display()
        );
        println!("cargo:rustc-link-lib=static={}", name);
    }

    // If "cc" is enabled, build the library from source, update the pre-built
    // version, and assert that the pre-built version is checked in.
    #[cfg(feature = "cc")]
    {
        let asm_name = "empty/empty.s";
        let out_dir = var("OUT_DIR").unwrap();
        Build::new().file(&asm_name).compile(&name);
        println!("cargo:rerun-if-changed={}", asm_name);
        let from = format!("{}/lib{}.a", out_dir, name);
        let prev_metadata = std::fs::metadata(&to);
        std::fs::copy(&from, &to).unwrap();
        assert!(
            prev_metadata.is_ok(),
            "{} didn't previously exist; please inspect the new file and `git add` it",
            to
        );
        assert!(
            std::process::Command::new("git")
                .arg("diff")
                .arg("--quiet")
                .arg(&to)
                .status()
                .unwrap()
                .success(),
            "{} changed; please inspect the change and `git commit` it",
            to
        );
    }
}

fn use_feature(feature: &str) {
    println!("cargo:rustc-cfg={}", feature);
}
