#!/bin/bash

# Exit on error
set -e

echo "Building hex tool..."
cargo build --release

# Create installation directory
INSTALL_DIR="$HOME/.skylark/bin"
echo "Creating installation directory at $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"

echo "Installing to $INSTALL_DIR/hex..."
cp target/release/hex "$INSTALL_DIR/hex"

# Check if directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "Adding $INSTALL_DIR to PATH in ~/.zshrc..."
    echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> ~/.zshrc
    echo "Please run 'source ~/.zshrc' or restart your terminal to use the hex command."
fi

echo "Installation complete! You can now use 'hex' command."
echo "Example: hex 0xff -x" 