#!/usr/bin/env bash
set -x
set -e

# used svd2rust 0.34.0
rm -rf src
mkdir src
../../svd2rust/target/release/svd2rust --target riscv --settings settings.yaml -g -i e310x.svd
# svd2rust --target riscv --settings settings.yaml -g -i e310x.svd
mv generic.rs src/
form -i lib.rs -o src/
rm lib.rs
cargo fmt

# combine generated device.x with memory.x
mv device.x ints.x
cat memory.x ints.x > device.x
rm -f ints.x