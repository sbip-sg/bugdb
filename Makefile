.PHONY: all
.DEFAULT_GOAL := all

########################################################
## Make' targets

all: debug

# Build all targets in debug mode.
debug:
	cargo +stable build
	@echo "Output files were compiled to the folder: target/debug"

# Build all targets in release mode
release:
	cargo +stable build --release
	@echo "Output files were compiled to the folder: target/release"

# Build API documentation in Rustdoc
docs:
	cargo +stable doc --no-deps
	@echo "API docs were generated to: target/doc/core/index.html"
	$(BROWSER) ${MAKEFILE_DIR}target/doc/core/index.html > /dev/null 2>&1 &

# Format code
format:
	cargo +stable fmt

# Check source code linting rules
linting: format
	cargo +stable clippy --tests --benches --features linting

# Run unit tests
test:
	cargo +stable test --workspace --features linting

# Clean code
clean:
	cargo +stable clean
