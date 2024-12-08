#!/bin/bash
set -euo pipefail

ls src/bin | while read LINE; do
bin_name=${LINE%.rs}
echo running $bin_name
cargo r -q -r --bin $bin_name
done
