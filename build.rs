use std::io::Write;
use std::path::PathBuf;
use std::{env, fs};

fn main() {
    // Put the linker script somewhere the linker can find it
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());
    fs::copy("interrupts.x", out_dir.join("interrupts.x")).unwrap();
    println!("cargo:rerun-if-changed=interrupts.x");
}
