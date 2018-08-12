set -euxo pipefail

main() {
    cargo check --target $TARGET

    cargo check --target $TARGET --features riscv-rt
}

main
