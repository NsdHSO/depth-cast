use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");

    if env::var("TARGET").unwrap() != "thumbv6m-none-eabi" {
        return;
    }

    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::copy("memory.x", out.join("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rustc-link-arg-bin=pico-firmware=-Tlink.x");
}
