# Makefile for dodecet-wasm package

.PHONY: all build test clean publish help

# Default target
all: build

# Build the package
build:
	@echo "🔨 Building dodecet-wasm..."
	wasm-pack build --target web --release

# Development build
build-dev:
	@echo "🔨 Building dodecet-wasm (dev)..."
	wasm-pack build --target web

# Build for Node.js
build-node:
	@echo "🔨 Building dodecet-wasm (Node.js)..."
	wasm-pack build --target nodejs

# Build for bundlers
build-bundler:
	@echo "🔨 Building dodecet-wasm (bundler)..."
	wasm-pack build --target bundler

# Run tests
test:
	@echo "🧪 Running tests..."
	wasm-pack test --node

# Test in Firefox
test-firefox:
	@echo "🧪 Running tests in Firefox..."
	wasm-pack test --firefox

# Test in Chrome
test-chrome:
	@echo "🧪 Running tests in Chrome..."
	wasm-pack test --chrome

# Clean build artifacts
clean:
	@echo "🧹 Cleaning..."
	rm -rf pkg
	rm -f Cargo.toml~
	cargo clean

# Publish to npm (dry run)
publish-dry:
	@echo "📦 Publishing (dry run)..."
	cd pkg && npm publish --dry-run

# Publish to npm
publish:
	@echo "📦 Publishing to npm..."
	cd pkg && npm publish --access public

# Install dependencies
install:
	@echo "📥 Installing wasm-pack..."
	cargo install wasm-pack

# Show help
help:
	@echo "Available targets:"
	@echo "  all          - Build the package (default)"
	@echo "  build        - Build for web"
	@echo "  build-dev    - Build for web (dev mode)"
	@echo "  build-node   - Build for Node.js"
	@echo "  build-bundler- Build for bundlers"
	@echo "  test         - Run tests in Node.js"
	@echo "  test-firefox - Run tests in Firefox"
	@echo "  test-chrome  - Run tests in Chrome"
	@echo "  clean        - Clean build artifacts"
	@echo "  publish      - Publish to npm"
	@echo "  install      - Install wasm-pack"
	@echo "  help         - Show this help message"
