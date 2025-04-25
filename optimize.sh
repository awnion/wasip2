#!/bin/bash

set -euxo pipefail

CARGO_TARGET_DIR=$(cargo metadata --format-version=1 | jq -r '.target_directory')

wasm-opt -Oz -o optimized.wasm "$CARGO_TARGET_DIR"/wasm32-unknown-unknown/release/ww.wasm
