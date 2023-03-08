use std::path::PathBuf;
use std::{env, fs};

fn main() {
    // Put the linker script somewhere the linker can find it
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());
    fs::copy("fe310x-interrupt.x", out_dir.join("fe310x-interrupt.x")).unwrap();
    println!("cargo:rerun-if-changed=fe310x-interrupt.x");
}
