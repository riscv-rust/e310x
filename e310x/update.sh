#!/usr/bin/env bash
set -x
set -e

# used svd2rust 8b809ac2c1e1a13f30af59ee41f4d66a4995d625 (unreleased)
rm -rf src
mkdir src
../../svd2rust/target/release/svd2rust --target riscv --settings settings.yaml -g -i e310x.svd
mv generic.rs src/
form -i lib.rs -o src/
rm lib.rs
cargo fmt

# combine generated device.x with memory.x
mv device.x ints.x
cat memory.x ints.x > device.x
rm -f ints.x