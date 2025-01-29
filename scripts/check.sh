#!/usr/bin/env bash
# This scripts runs various CI-like checks in a convenient way.

set -eu
script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$script_path/.."
set -x

# Checks all tests, lints etc.
# Basically does what the CI does.

cargo +1.81 install --quiet typos-cli

export RUSTFLAGS="-D warnings"
export RUSTDOCFLAGS="-D warnings" # https://github.com/emilk/egui/pull/1454

# Fast checks first:
typos
./scripts/lint.py
cargo fmt --all -- --check
cargo deny check
cargo doc --quiet --lib --no-deps --all-features
cargo doc --quiet --document-private-items --no-deps --all-features

cargo clippy --quiet --all-targets --all-features -- -D warnings

CLIPPY_CONF_DIR="scripts/clippy_wasm" cargo clippy --quiet --target wasm32-unknown-unknown --lib -- -D warnings

cargo check --quiet  --all-targets
cargo check --quiet  --all-targets --no-default-features
cargo check --quiet  --all-targets --all-features
cargo test  --quiet --all-targets --all-features
cargo test  --quiet --doc # slow - checks all doc-tests

echo "All checks passed."
