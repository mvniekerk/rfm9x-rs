#!/bin/sh
set -x
set -e

cargo install --force --git https://github.com/rust-embedded/svd2rust svd2rust
cargo install --force --version 0.7.0 form

