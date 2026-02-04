# Ultimate Dotfiles Installer - Build System

.PHONY: help build run test clean install release dev check fmt lint

# Default target
help: ## Show this help message
	@echo "🏴‍☠️ Ultimate Dotfiles Installer Build System"
	@echo ""
	@echo "Available targets:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# Install Rust if not present
install-rust: ## Install Rust toolchain
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "📦 Installing Rust toolchain..."; \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
		source ~/.cargo/env; \
	else \
		echo "✅ Rust already installed"; \
	fi

# Development setup
dev: install-rust ## Setup development environment
	@echo "🛠️  Setting up development environment..."
	cargo install cargo-watch cargo-edit
	@echo "✅ Development environment ready"

# Build the installer
build: install-rust ## Build the installer in debug mode
	@echo "🔧 Building ultimate-installer..."
	cargo build

# Build release version
release: install-rust ## Build optimized release version
	@echo "🚀 Building release version..."
	cargo build --release
	@echo "✅ Release build complete: target/release/install"

# Run the installer
run: build ## Run the installer in debug mode
	@echo "🏃 Running ultimate-installer..."
	cargo run

# Run tests
test: ## Run all tests
	@echo "🧪 Running tests..."
	cargo test

# Check code without building
check: ## Check code for errors
	@echo "🔍 Checking code..."
	cargo check

# Format code
fmt: ## Format code with rustfmt
	@echo "📐 Formatting code..."
	cargo fmt

# Run linter
lint: ## Run clippy linter
	@echo "🧹 Running linter..."
	cargo clippy -- -D warnings

# Clean build artifacts
clean: ## Clean build artifacts
	@echo "🧽 Cleaning build artifacts..."
	cargo clean

# Install the binary system-wide
install: release ## Install the binary to ~/.local/bin
	@echo "📦 Installing ultimate-installer..."
	mkdir -p ~/.local/bin
	cp target/release/install ~/.local/bin/ultimate-installer
	chmod +x ~/.local/bin/ultimate-installer
	@echo "✅ Installed to ~/.local/bin/ultimate-installer"
	@echo "💡 Make sure ~/.local/bin is in your PATH"

# Create distributable package
package: release ## Create distributable package
	@echo "📦 Creating distribution package..."
	mkdir -p dist
	cp target/release/install dist/ultimate-installer
	cp README.md dist/
	tar czf dist/ultimate-installer.tar.gz -C dist ultimate-installer README.md
	@echo "✅ Package created: dist/ultimate-installer.tar.gz"

# Watch for changes and rebuild
watch: install-rust ## Watch for changes and rebuild automatically
	cargo watch -x build

# Generate documentation
docs: ## Generate and open documentation
	cargo doc --open

# Update dependencies
update: ## Update all dependencies
	cargo update

# Audit dependencies for security issues
audit: ## Audit dependencies
	cargo audit

# Benchmark performance
bench: ## Run benchmarks
	cargo bench

# Print version information
version: ## Show version information
	@echo "Ultimate Dotfiles Installer"
	@echo "Rust version: $(shell rustc --version 2>/dev/null || echo 'Not installed')"
	@echo "Cargo version: $(shell cargo --version 2>/dev/null || echo 'Not installed')"

# Quick development cycle
dev-cycle: fmt lint test ## Format, lint, and test - good for development
	@echo "✅ Development cycle complete"

# Full quality check
quality: fmt lint test audit ## Full quality assurance check
	@echo "🌟 Quality check complete"

# Prepare for release
prepare-release: clean quality release package ## Prepare everything for release
	@echo "🎉 Release preparation complete"