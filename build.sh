#!/bin/sh
set -e
set -o pipefail
svd2rust --target riscv -i e310x.svd | (rustfmt || exit 0) | tee src/lib.rs
