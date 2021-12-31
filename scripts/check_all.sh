#!/bin/sh

set -eux

cargo check
cargo check --no-default-features
cargo check --all-features

cargo fmt -- --check

cargo clippy -- -D warnings

echo PASS
