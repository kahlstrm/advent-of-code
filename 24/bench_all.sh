#!/bin/bash
set -eo pipefail

OUTPUT_FILE="$1"
MARKDOWN_ARG=()

if [ -n "$OUTPUT_FILE" ]; then
    MARKDOWN_ARG=(--export-markdown "$OUTPUT_FILE")
fi

commands=()
names=()
for file in src/bin/*.rs; do
    BIN_NAME=$(basename "$file" .rs)
    cargo build -q --release --bin "$BIN_NAME"
    names+=("-nDay $((10#${BIN_NAME#day}))")
    commands+=("target/release/$BIN_NAME")
done

hyperfine --warmup 5 -N "${names[@]}" "${commands[@]}" "${MARKDOWN_ARG[@]}"
