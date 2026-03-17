#!/bin/bash

# Dodecet Encoder Build Script
# Builds WASM package and prepares npm package

set -e

echo "🚀 Building Dodecet Encoder WASM Package"
echo "=========================================="

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check prerequisites
echo -e "${YELLOW}Checking prerequisites...${NC}"

if ! command -v rustc &> /dev/null; then
    echo -e "${RED}Error: Rust not found. Please install Rust first.${NC}"
    echo "Visit: https://rustup.rs/"
    exit 1
fi

if ! command -v wasm-pack &> /dev/null; then
    echo -e "${YELLOW}Installing wasm-pack...${NC}"
    cargo install wasm-pack
fi

if ! command -v node &> /dev/null; then
    echo -e "${YELLOW}Warning: Node.js not found. Skipping npm package preparation.${NC}"
fi

echo -e "${GREEN}✅ Prerequisites check complete${NC}"

# Build WASM package
echo -e "\n${YELLOW}Building WASM package...${NC}"
wasm-pack build \
    --target web \
    --out-dir pkg \
    --release

echo -e "${GREEN}✅ WASM package built successfully${NC}"

# Prepare npm package
if command -v node &> /dev/null; then
    echo -e "\n${YELLOW}Preparing npm package...${NC}"

    # Copy package.json to pkg directory
    cp package.json pkg/

    # Copy TypeScript definitions to pkg directory
    mkdir -p pkg/types
    cp typescript/dodecet_encoder.d.ts pkg/

    # Copy documentation
    mkdir -p pkg/docs
    cp docs/WASM_INTEGRATION_GUIDE.md pkg/docs/

    # Copy examples
    mkdir -p pkg/examples
    cp examples/*.html pkg/examples/
    cp examples/*.js pkg/examples/

    echo -e "${GREEN}✅ npm package prepared successfully${NC}"
fi

# Run tests
echo -e "\n${YELLOW}Running tests...${NC}"
cargo test --lib
cargo test --features wasm

echo -e "${GREEN}✅ All tests passed${NC}"

# Run WASM tests (if Node.js is available)
if command -v node &> /dev/null; then
    echo -e "\n${YELLOW}Running WASM tests...${NC}"
    wasm-pack test --node

    echo -e "${GREEN}✅ WASM tests passed${NC}"
fi

# Display summary
echo -e "\n${GREEN}=========================================="
echo "✅ Build complete!"
echo "==========================================${NC}"
echo ""
echo "Package contents:"
echo "  - pkg/dodecet_encoder.js        (JavaScript wrapper)"
echo "  - pkg/dodecet_encoder_bg.wasm   (WASM module)"
echo "  - pkg/dodecet_encoder.d.ts      (TypeScript definitions)"
echo "  - pkg/package.json              (npm package configuration)"
echo "  - pkg/docs/                     (Documentation)"
echo "  - pkg/examples/                 (Usage examples)"
echo ""
echo "To publish to npm:"
echo "  cd pkg && npm publish"
echo ""
echo "To test locally:"
echo "  npm install ./pkg"
echo ""
