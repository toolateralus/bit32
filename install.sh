#!/bin/bash

set -e -x 

BINARY_NAME="bit32"  
TARGET_DIR="/usr/local/bin"
FONT_DIR="/usr/share/fonts/truetype/ModernDOS"
FONT_FILE="ModernDOS8x16.ttf"

cd "$PROJECT_DIR"

if [[ "${1,,}" == "debug" ]]; then
    cargo build
    BINARY_PATH="target/debug/$BINARY_NAME"
else
    cargo build --release
    BINARY_PATH="target/release/$BINARY_NAME"
fi

sudo ln -sf "$(pwd)/$BINARY_PATH" "$TARGET_DIR/$BINARY_NAME"

if [[ ! -d "$FONT_DIR" || ! -f "$FONT_DIR/$FONT_FILE" ]]; then
    sudo mkdir -p "$FONT_DIR"
    sudo cp res/ModernDOS/"$FONT_FILE" "$FONT_DIR/$FONT_FILE"
fi