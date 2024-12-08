#!/bin/bash -eo pipefail

BIN_NAME=$(date +"%d")
RELEASE_FLAG=""
for arg in "$@"
do
  if [ $arg == "-r" ];then
    RELEASE_FLAG="-r"
  else
    BIN_NAME=$(printf "%02d" $arg)
  fi
done
cargo r $RELEASE_FLAG --bin day$BIN_NAME
