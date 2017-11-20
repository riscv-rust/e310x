use std::process::Command;

fn main() {
    let status = Command::new("sh")
        .arg("./build.sh")
        .status()
        .expect("failed to execute ./build.sh");

    assert!(status.success());

    println!("cargo:rerun-if-changed=e310x.svd");
    println!("cargo:rerun-if-changed=build.sh");
}
