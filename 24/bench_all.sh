#!/bin/bash
set -eo pipefail

ls src/bin | while read LINE; do
BIN_NAME=${LINE%.rs}
cargo b -q -r --bin $BIN_NAME
hyperfine --warmup 5 -N target/release/$BIN_NAME
done
