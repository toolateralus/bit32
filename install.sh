#!/bin/bash

set -e -x 

BINARY_NAME="bit32"  
TARGET_DIR="/usr/local/bin"

cd "$PROJECT_DIR"

if [[ "${1,,}" == "debug" ]]; then
    cargo build
    BINARY_PATH="target/debug/$BINARY_NAME"
else
    cargo build --release
    BINARY_PATH="target/release/$BINARY_NAME"
fi

sudo ln -sf "$(pwd)/$BINARY_PATH" "$TARGET_DIR/$BINARY_NAME"