#!/bin/bash

# SSP Installation Script for Linux/macOS
# This script builds and installs SSP to your system

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Installation directory
INSTALL_DIR="$HOME/.local/bin"

echo -e "${BLUE}╔═══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  SSP - Show Structure of Project     ║${NC}"
echo -e "${BLUE}║  Installation Script                  ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════╝${NC}"
echo

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust/Cargo is not installed!${NC}"
    echo -e "${YELLOW}Please install Rust from: https://rustup.rs/${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} Rust/Cargo found"

# Check Rust version
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
echo -e "${GREEN}✓${NC} Rust version: $RUST_VERSION"

# Build the project
echo
echo -e "${BLUE}Building SSP in release mode...${NC}"
cargo build --release

if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Build failed!${NC}"
    exit 1
fi

echo -e "${GREEN}✓${NC} Build successful"

# Create installation directory if it doesn't exist
if [ ! -d "$INSTALL_DIR" ]; then
    echo -e "${YELLOW}Creating installation directory: $INSTALL_DIR${NC}"
    mkdir -p "$INSTALL_DIR"
fi

# Copy binary
echo -e "${BLUE}Installing SSP to $INSTALL_DIR...${NC}"
cp target/release/ssp "$INSTALL_DIR/ssp"
chmod +x "$INSTALL_DIR/ssp"

echo -e "${GREEN}✓${NC} Binary installed"

# Check if the directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo
    echo -e "${YELLOW}⚠  $INSTALL_DIR is not in your PATH${NC}"
    echo
    echo "Add the following line to your shell configuration file:"
    echo
    
    # Detect shell
    if [ -n "$BASH_VERSION" ]; then
        SHELL_CONFIG="$HOME/.bashrc"
        echo -e "${GREEN}  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc${NC}"
        echo -e "${GREEN}  source ~/.bashrc${NC}"
    elif [ -n "$ZSH_VERSION" ]; then
        SHELL_CONFIG="$HOME/.zshrc"
        echo -e "${GREEN}  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.zshrc${NC}"
        echo -e "${GREEN}  source ~/.zshrc${NC}"
    else
        echo -e "${GREEN}  export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}"
    fi
    
    echo
    read -p "Would you like to add it automatically? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        if [ -n "$BASH_VERSION" ]; then
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
            echo -e "${GREEN}✓${NC} Added to ~/.bashrc"
            echo -e "${YELLOW}Run: source ~/.bashrc${NC}"
        elif [ -n "$ZSH_VERSION" ]; then
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc"
            echo -e "${GREEN}✓${NC} Added to ~/.zshrc"
            echo -e "${YELLOW}Run: source ~/.zshrc${NC}"
        fi
    fi
else
    echo -e "${GREEN}✓${NC} $INSTALL_DIR is in PATH"
fi

# Test installation
echo
echo -e "${BLUE}Testing installation...${NC}"
if command -v ssp &> /dev/null; then
    VERSION=$(ssp --help | head -n1 || echo "Unknown version")
    echo -e "${GREEN}✓${NC} SSP installed successfully!"
    echo -e "  Version: ${VERSION}"
else
    echo -e "${YELLOW}⚠  SSP command not immediately available${NC}"
    echo -e "${YELLOW}  Please restart your terminal or run: source ~/.bashrc (or ~/.zshrc)${NC}"
fi

# Show usage
echo
echo -e "${GREEN}╔═══════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  Installation Complete!               ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════╝${NC}"
echo
echo -e "Quick start:"
echo -e "  ${BLUE}ssp${NC}              - Show current directory structure"
echo -e "  ${BLUE}ssp -l${NC}           - Show with line counts"
echo -e "  ${BLUE}ssp -a${NC}           - Analyze code"
echo -e "  ${BLUE}ssp --help${NC}       - Show all options"
echo
echo -e "For full documentation, visit:"
echo -e "  ${BLUE}https://github.com/Flaykky/show-struct-of-folder${NC}"
echo
