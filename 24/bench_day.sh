#!/bin/bash -eo pipefail

BIN_NAME=$(date +"%d")
if [ ! -z "$1" ]; then
  BIN_NAME=$(printf "%02d" $1)
fi
BIN_NAME=day$BIN_NAME
cargo b -q -r --bin $BIN_NAME
hyperfine --warmup 5 -N target/release/$BIN_NAME
