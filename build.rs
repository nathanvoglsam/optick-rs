extern crate cmake;

use std::path::PathBuf;

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

    let mut config = cmake::Config::new(path);
    config.define("OPTICK_STATIC", "ON");
    config.static_crt(static_crt());

    let out = config.build().join("lib");
    let out = out.to_str().unwrap();
    println!("cargo:rustc-link-search=native={}", &out);

    println!("cargo:rustc-link-lib=static=OptickCored");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=ws2_32");
    println!("cargo:rustc-link-lib=dbghelp");
}
