#!/usr/bin/env bash

set -eux

cargo fmt -- --check
check_args=("" "--no-default-features" "--all-features")

for args in "${check_args[@]}"; do
    cargo check ${args}
    cargo clippy ${args} -- -D warnings
done

echo PASS
