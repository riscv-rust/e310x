use std::path::Path;
use std::process::Command;

fn main() {
    let _status = Command::new("sh")
        .arg("./build.sh")
        .status()
        .expect("failed to execute ./build.sh");

    assert!(Path::new("src/lib.rs").exists());
    // not required to succeed if src/lib.rs is available
    // svd2rust or rustfmt may not be in path
    // assert!(_status.success());

    println!("cargo:rerun-if-changed=e310x.svd");
    println!("cargo:rerun-if-changed=build.sh");
}
