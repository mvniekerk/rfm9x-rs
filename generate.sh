#!/usr/bin/env bash
set -x
set -e

TOP=$PWD

rm -rf "$TOP"/registers/src
cd "$TOP"/registers
svd2rust --nightly -i "$TOP"/svd/rfm9x.svd
form -i lib.rs -o src
rm lib.rs device.x build.rs
cargo fmt
find . -type f | grep rs | xargs rustfmt
#rm -rf "$TOP"/src/generated
