#!/bin/bash

set -e  # stop during error

echo "Building the project..."
cargo build --release

BINARY_NAME="ssp"
TARGET_DIR="./target/release"

if [ ! -f "$TARGET_DIR/$BINARY_NAME" ]; then
    echo "Error: Binary '$BINARY_NAME' not found in $TARGET_DIR"
    exit 1
fi

INSTALL_DIR="/usr/local/bin"

echo "Installing $BINARY_NAME to $INSTALL_DIR..."

# copying binary with root
sudo cp "$TARGET_DIR/$BINARY_NAME" "$INSTALL_DIR/"

sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo "Installation complete! You can now use 'ssp' from anywhere."
