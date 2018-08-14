#!/usr/bin/env bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

set -e
rustup override set nightly

#
# Creating the signature
#
BRANCH=$(git branch | sed -n -e 's/^\* \(.*\)/\1/p')
HASH=$(git rev-parse HEAD)
export SIGNATURE="$BRANCH:$HASH"

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
strip $DIR/target/release/tiny-static-web-server