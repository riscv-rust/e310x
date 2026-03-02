fn main() {
    if std::env::var_os("CARGO_FEATURE_ASYNC").is_some() {
        println!("cargo:rustc-env=RISCV_RT_BASE_ISA=rv32i");
    }
}
