.PHONY: dev build install clean deps lint check help

# Default target
help:
	@echo "ZipRS Build System"
	@echo ""
	@echo "  make deps      Install npm dependencies"
	@echo "  make dev       Run in development mode"
	@echo "  make build     Build for production (frontend + backend)"
	@echo "  make install   Build and install to ~/.local/bin"
	@echo "  make clean     Remove build artifacts"
	@echo "  make lint      Run linters (clippy + svelte-check)"
	@echo "  make check     Type-check without building"
	@echo ""

# Install npm dependencies
deps:
	npm install

# Development mode
dev: deps
	cargo tauri dev

# Production build
build: deps
	cargo tauri build

# Install locally
install: build
	@mkdir -p $(HOME)/.local/bin
	@ln -sf $(CURDIR)/target/release/ziprs $(HOME)/.local/bin/ziprs
	@echo "Installed ziprs to ~/.local/bin/ziprs"

# Clean all build artifacts
clean:
	rm -rf dist node_modules/.vite
	cargo clean

# Lint
lint:
	cd src-tauri && cargo clippy -- -W warnings
	npx svelte-check

# Type-check only
check:
	cd src-tauri && cargo check
	npx svelte-check
