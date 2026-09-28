#!/bin/sh

set -eu

project_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
target_dir="$project_dir/target/wasm32-wasip1/release"
wasm_file="$target_dir/swc_jsx_dom_runtime.wasm"
optimized_file="$wasm_file.tmp"
output_file="$project_dir/index.wasm"

trap 'rm -f "$optimized_file"' EXIT HUP INT TERM

cargo build \
    --manifest-path "$project_dir/Cargo.toml" \
    --target wasm32-wasip1 \
    --release

cargo run \
    --manifest-path "$project_dir/tools/optimize-wasm/Cargo.toml" \
    --release \
    -- "$wasm_file" "$optimized_file"

mv "$optimized_file" "$output_file"
printf '\nBuilt and optimized WASM artifact: %s\n\n' "$output_file"

trap - EXIT HUP INT TERM
