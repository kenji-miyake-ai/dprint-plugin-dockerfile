#!/bin/bash

SCRIPT_DIR=$(realpath "$(dirname "$0")")

cargo build --target wasm32-unknown-unknown --features wasm --release
cp "${SCRIPT_DIR}/target/wasm32-unknown-unknown/release/dprint_plugin_dockerfile.wasm" "${SCRIPT_DIR}/"
