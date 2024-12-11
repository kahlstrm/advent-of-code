#!/bin/bash -eo pipefail
BIN_DATE=$(date +"%d")
if [ ! -z "$1" ]; then
  BIN_DATE=$(printf "%02d" $1)
fi
BIN_NAME=day$BIN_DATE
mkdir -p src/inputs
INPUT_FILENAME=src/inputs/$BIN_NAME
if [ ! -f "$INPUT_FILENAME" ]; then
echo "Creating $INPUT_FILENAME"
touch $INPUT_FILENAME
else
echo "$INPUT_FILENAME already exists, skipping creation"
fi
BIN_FILENAME=src/bin/$BIN_NAME.rs
if [ ! -f "$BIN_FILENAME" ]; then
echo "Creating $BIN_FILENAME"
cat << EOF > $BIN_FILENAME
const USE_TEST_INPUT: bool = false;
static TEST_INPUT: &[u8] = br#""#;
static INPUT: &[u8] = include_bytes!("../inputs/$BIN_NAME");

// https://adventofcode.com/2024/day/${BIN_DATE#0}
fn main() {
    let debug = USE_TEST_INPUT || INPUT.len() == 0;
    let lines = if debug { TEST_INPUT } else { INPUT }
        .trim_ascii()
        .split(|c| *c == b'\n')
        .filter_map(|l| match l.trim_ascii() {
            &[] => None,
            other => other.into(),
        });
    println!("{lines:#?}")
}
EOF
else
echo "$BIN_FILENAME already exists, skipping creation"
fi
