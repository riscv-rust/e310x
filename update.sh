#!/usr/bin/env bash
set -x
set -e

# used svd2rust v0.19.0
rm -rf src/common
mkdir src/common
svd2rust --target riscv -i e310x.svd
form -i lib.rs -o src/common
rm lib.rs

# Convert library into module
mv src/common/lib.rs src/common/mod.rs
cargo fmt
rustfmt src/common/mod.rs

# Strip crate-level things
# update this if svd2rust changes things
tail -n+30 src/common/mod.rs > src/common/_mod.rs
echo -en "use core::marker::PhantomData;\nuse core::ops::Deref;\n" > src/common/mod.rs
cat src/common/_mod.rs >> src/common/mod.rs
rm src/common/_mod.rs
mv src/common/generic.rs src/
mv src/common/interrupt.rs src/
