#!/usr/bin/env bash
set -e
rustup override set nightly

#
# Debug
#
cargo build --verbose

#
# Release
#
cargo build --release --verbose
# Reduces the size of the binary by removing debug symbols.
# See: https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html
strip target/release/tiny-static-web-server