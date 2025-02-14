#!/bin/bash

set -e -x 

BINARY_NAME="bit32"  
TARGET_DIR="/usr/local/bin"

cd "$PROJECT_DIR"

cargo build --release

sudo ln -sf "$(pwd)/target/release/$BINARY_NAME" "$TARGET_DIR/$BINARY_NAME"