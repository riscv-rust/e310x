#!/bin/bash

set -euxo pipefail

cargo check --target $TARGET
cargo check --target $TARGET --features rt
cargo check --target $TARGET --features g002
cargo check --target $TARGET --features "g002 rt"
