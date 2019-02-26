use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Put the linker script somewhere the linker can find it.
fn memory_linker_script() {
    let out_dir = env::var("OUT_DIR").expect("No out dir");
    let dest_path = Path::new(&out_dir);
    let mut f = File::create(&dest_path.join("memory.x"))
        .expect("Could not create file");

    f.write_all(include_bytes!("memory.x"))
        .expect("Could not write file");

    println!("cargo:rustc-link-search={}", dest_path.display());
    println!("cargo:rerun-if-changed=memory.x");
}

fn external_clock_frequency() {
    let out_dir = env::var("OUT_DIR").expect("No out dir");
    let dest_path = Path::new(&out_dir).join("constants.rs");
    let mut f = File::create(&dest_path).expect("Could not create file");

    let hfxosc_freq = option_env!("BOARD_HFXOSC_FREQ").unwrap_or("16000000");

    let hfxosc_freq: u32 = str::parse(hfxosc_freq)
        .expect("Could not parse BOARD_HFXOSC_FREQ");

    writeln!(&mut f, "const BOARD_HFXOSC_FREQ: u32 = {};", hfxosc_freq)
        .expect("Could not write file");

    println!("cargo:rerun-if-env-changed=BOARD_HFXOSC_FREQ");
}

fn main() {
    memory_linker_script();
    external_clock_frequency();
    println!("cargo:rerun-if-changed=build.rs");
}
