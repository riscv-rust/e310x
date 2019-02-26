#!/bin/sh
set -e
set -o pipefail
svd2rust --target riscv -i e310x.svd
mv lib.rs src/
rustfmt src/lib.rs || exit 0
