#!/usr/bin/env bash
set -x
set -e

# used svd2rust 0.37.0
rm -rf src
mkdir src
svd2rust --target riscv --settings settings.yaml --feature-group -g -i e310x.svd
rm -f features.toml
mv generic.rs src/
form -i lib.rs -o src/
rm lib.rs
cargo fmt

# combine generated device.x with memory.x
mv device.x ints.x
cat memory.x ints.x > device.x
rm -f ints.x