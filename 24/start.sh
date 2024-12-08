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
static TEST_INPUT: &str = r#""#;
static INPUT: &str = include_str!("../inputs/$BIN_NAME");

// https://adventofcode.com/2024/day/${BIN_DATE#0}
fn main() {
    let lines = if INPUT.len() == 0 { TEST_INPUT } else { INPUT }
        .lines()
        .filter_map(|l| match l.trim() {
            "" => None,
            other => Some(other),
        });
    println!("{lines:#?}")
}
EOF
else
echo "$BIN_FILENAME already exists, skipping creation"
fi
