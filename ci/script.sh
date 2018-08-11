set -euxo pipefail

main() {
    cargo check --target $TARGET

    cargo check --target $TARGET --no-default-features
}

main
