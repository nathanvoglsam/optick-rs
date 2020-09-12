extern crate cmake;

use std::path::PathBuf;
use std::env;

///
/// Are we building for the static CRT on Windows MSVC
///
fn static_crt() -> bool {
    std::env::var("CARGO_CFG_TARGET_FEATURE")
        .unwrap_or_default()
        .contains("crt-static")
}

fn main() {
    let path: PathBuf = [std::env!("CARGO_MANIFEST_DIR"), "external", "optick"].iter().collect();

    // Setup CMake build, force to build as static lib and propagate CRT requirement
    let mut config = cmake::Config::new(path);
    config.define("OPTICK_STATIC", "ON");
    config.static_crt(static_crt());

    // Perform the build and add the build output to linker search path
    let out = config.build().join("lib");
    let out = out.to_str().unwrap();
    println!("cargo:rustc-link-search=native={}", &out);

    // Handle linking on all the supported platforms
    let profile = env::var("PROFILE").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
    match (profile.as_str() ,target_os.as_str(), target_env.as_str()) {
        ("debug", "windows", "gnu") => {
            println!("cargo:rustc-link-lib=static=OptickCored");
            println!("cargo:rustc-link-lib=stdc++");
            println!("cargo:rustc-link-lib=ws2_32");
            println!("cargo:rustc-link-lib=dbghelp");
        },
        ("release", "windows", "gnu") => {
            println!("cargo:rustc-link-lib=static=OptickCore");
            println!("cargo:rustc-link-lib=stdc++");
            println!("cargo:rustc-link-lib=ws2_32");
            println!("cargo:rustc-link-lib=dbghelp");
        },
        ("debug", "windows", "msvc") => {
            println!("cargo:rustc-link-lib=static=OptickCored");
            println!("cargo:rustc-link-lib=ws2_32");
            println!("cargo:rustc-link-lib=dbghelp");
        },
        ("release", "windows", "msvc") => {
            println!("cargo:rustc-link-lib=static=OptickCore");
            println!("cargo:rustc-link-lib=ws2_32");
            println!("cargo:rustc-link-lib=dbghelp");
        },
        ("debug", "linux", _) => {
            println!("cargo:rustc-link-lib=static=OptickCored");
            println!("cargo:rustc-link-lib=stdc++");
        }
        ("release", "linux", _) => {
            println!("cargo:rustc-link-lib=static=OptickCore");
            println!("cargo:rustc-link-lib=stdc++");
        },
        ("debug", "macos", _) => {
            println!("cargo:rustc-link-lib=static=OptickCored");
            println!("cargo:rustc-link-lib=stdc++");
        }
        ("release", "macos", _) => {
            println!("cargo:rustc-link-lib=static=OptickCore");
            println!("cargo:rustc-link-lib=stdc++");
        },
        _ => {
            panic!("Unsupported platform")
        }
    }


}
