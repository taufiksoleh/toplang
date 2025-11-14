.PHONY: help build build-release run test bench clean fmt lint doc check install examples version

# Default target
.DEFAULT_GOAL := help

# Colors for output
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[1;33m
NC := \033[0m # No Color

##@ General

help: ## Display this help message
	@echo "$(BLUE)TopLang Makefile - Available Commands$(NC)"
	@echo ""
	@awk 'BEGIN {FS = ":.*##"; printf "Usage:\n  make $(YELLOW)<target>$(NC)\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2 } /^##@/ { printf "\n$(BLUE)%s$(NC)\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Building

build: ## Build debug version (fast compilation)
	@echo "$(BLUE)Building TopLang (debug)...$(NC)"
	@cargo build
	@echo "$(GREEN)Build complete: target/debug/topc$(NC)"

build-release: ## Build release version (optimized)
	@echo "$(BLUE)Building TopLang (release with optimizations)...$(NC)"
	@cargo build --release
	@echo "$(GREEN)Build complete: target/release/topc$(NC)"

install: build-release ## Install topc to system (requires sudo)
	@echo "$(BLUE)Installing topc to /usr/local/bin...$(NC)"
	@sudo cp target/release/topc /usr/local/bin/
	@echo "$(GREEN)Installed! Run 'topc --version' to verify.$(NC)"

##@ Running

run: build ## Run with example file
	@echo "$(BLUE)Running examples/hello.top...$(NC)"
	@./target/debug/topc examples/hello.top

run-vm: build ## Run with bytecode VM (faster)
	@echo "$(BLUE)Running examples/hello.top with VM...$(NC)"
	@./target/debug/topc examples/hello.top --bytecode

run-release: build-release ## Run with release build and VM
	@echo "$(BLUE)Running examples/hello.top with optimized VM...$(NC)"
	@./target/release/topc examples/hello.top --bytecode

examples: build-release ## Run all example programs
	@echo "$(BLUE)Running all examples...$(NC)"
	@for file in examples/*.top; do \
		echo "$(YELLOW)Running $$file...$(NC)"; \
		./target/release/topc "$$file" --bytecode || exit 1; \
		echo ""; \
	done
	@echo "$(GREEN)All examples completed successfully!$(NC)"

##@ Testing

test: ## Run all tests
	@echo "$(BLUE)Running tests...$(NC)"
	@cargo test
	@echo "$(GREEN)Tests passed!$(NC)"

test-verbose: ## Run tests with verbose output
	@echo "$(BLUE)Running tests (verbose)...$(NC)"
	@cargo test -- --nocapture

test-examples: build-release ## Test all example files
	@echo "$(BLUE)Testing all examples...$(NC)"
	@./test_examples.sh
	@echo "$(GREEN)Example tests passed!$(NC)"

##@ Benchmarking

bench: build-release ## Run performance benchmarks (TopLang vs Python)
	@echo "$(BLUE)Running benchmarks...$(NC)"
	@./benchmarks/run_benchmarks.sh

bench-vm: build-release ## Run VM benchmarks (Interpreter vs VM vs Python)
	@echo "$(BLUE)Running VM benchmarks...$(NC)"
	@./benchmarks/run_vm_benchmarks.sh

bench-fibonacci: build-release ## Quick benchmark with fibonacci only
	@echo "$(BLUE)Benchmarking fibonacci...$(NC)"
	@echo "Interpreter:"
	@time ./target/release/topc benchmarks/toplang/fibonacci.top > /dev/null
	@echo "\nVM:"
	@time ./target/release/topc benchmarks/toplang/fibonacci.top --bytecode > /dev/null
	@echo "\nPython:"
	@time python3 benchmarks/python/fibonacci.py > /dev/null

##@ Code Quality

fmt: ## Format code with rustfmt
	@echo "$(BLUE)Formatting code...$(NC)"
	@cargo fmt
	@echo "$(GREEN)Code formatted!$(NC)"

lint: ## Run clippy linter
	@echo "$(BLUE)Running clippy...$(NC)"
	@cargo clippy --all-targets --all-features -- -D warnings

lint-fix: ## Run clippy with auto-fix
	@echo "$(BLUE)Running clippy with auto-fix...$(NC)"
	@cargo clippy --all-targets --all-features --fix

check: fmt lint test ## Run all code quality checks
	@echo "$(GREEN)All checks passed!$(NC)"

doc: ## Generate documentation
	@echo "$(BLUE)Generating documentation...$(NC)"
	@cargo doc --no-deps --open

doc-check: ## Check documentation for warnings
	@echo "$(BLUE)Checking documentation...$(NC)"
	@cargo doc --no-deps 2>&1 | grep -E "(warning|error)" || echo "$(GREEN)Documentation is clean!$(NC)"

##@ Debugging

show-bytecode: build-release ## Show bytecode for hello.top
	@echo "$(BLUE)Showing bytecode for examples/hello.top...$(NC)"
	@./target/release/topc examples/hello.top --bytecode --show-bytecode

debug-vm: build-release ## Run with VM debugging enabled
	@echo "$(BLUE)Running with VM debug mode...$(NC)"
	@./target/release/topc examples/hello.top --bytecode --debug-vm

show-ast: build ## Show AST for hello.top
	@echo "$(BLUE)Showing AST for examples/hello.top...$(NC)"
	@./target/release/topc examples/hello.top --show-ast

show-tokens: build ## Show tokens for hello.top
	@echo "$(BLUE)Showing tokens for examples/hello.top...$(NC)"
	@./target/release/topc examples/hello.top --show-tokens

verbose: build ## Run with verbose output
	@echo "$(BLUE)Running with verbose output...$(NC)"
	@./target/release/topc examples/hello.top --bytecode --verbose

##@ Maintenance

clean: ## Clean build artifacts
	@echo "$(BLUE)Cleaning build artifacts...$(NC)"
	@cargo clean
	@rm -f test_*.top 2>/dev/null || true
	@echo "$(GREEN)Clean complete!$(NC)"

clean-cache: ## Clean cargo cache (use when encountering build issues)
	@echo "$(BLUE)Cleaning cargo cache...$(NC)"
	@cargo clean
	@rm -rf target/
	@echo "$(GREEN)Cache cleaned!$(NC)"

update: ## Update dependencies
	@echo "$(BLUE)Updating dependencies...$(NC)"
	@cargo update
	@echo "$(GREEN)Dependencies updated!$(NC)"

version: ## Show version information
	@echo "$(BLUE)TopLang Version Information:$(NC)"
	@./target/release/topc --version 2>/dev/null || echo "Build release first: make build-release"
	@echo "\nRust toolchain:"
	@rustc --version
	@cargo --version

##@ Development Workflow

dev: fmt lint test ## Development workflow (format, lint, test)
	@echo "$(GREEN)Development checks complete!$(NC)"

quick: build run-vm ## Quick build and run with VM
	@echo "$(GREEN)Quick run complete!$(NC)"

release: clean build-release test bench ## Full release build with tests and benchmarks
	@echo "$(GREEN)Release build complete!$(NC)"

ci: fmt lint test doc-check ## CI pipeline checks
	@echo "$(GREEN)CI checks passed!$(NC)"

##@ Git Operations

git-status: ## Show detailed git status
	@echo "$(BLUE)Git Status:$(NC)"
	@git status
	@echo "\n$(BLUE)Recent commits:$(NC)"
	@git log --oneline -5

commit: fmt lint test ## Stage all, run checks, and prepare for commit
	@echo "$(BLUE)Preparing commit...$(NC)"
	@git add -A
	@git status
	@echo "\n$(YELLOW)Ready to commit. Run:$(NC)"
	@echo "  git commit -m 'your message'"
	@echo "  git push origin <branch>"

##@ Profiling & Performance

profile-release: build-release ## Profile release build
	@echo "$(BLUE)Profiling release build...$(NC)"
	@echo "Running fibonacci benchmark 10 times..."
	@time for i in {1..10}; do ./target/release/topc benchmarks/toplang/fibonacci.top --bytecode > /dev/null; done

size: build-release ## Show binary size
	@echo "$(BLUE)Binary sizes:$(NC)"
	@ls -lh target/release/topc
	@echo "\nStripped size:"
	@strip target/release/topc -o target/release/topc-stripped 2>/dev/null || true
	@ls -lh target/release/topc-stripped 2>/dev/null || echo "Strip not available"

##@ Utilities

watch: ## Watch for changes and rebuild (requires cargo-watch)
	@command -v cargo-watch >/dev/null 2>&1 || { echo "Installing cargo-watch..."; cargo install cargo-watch; }
	@echo "$(BLUE)Watching for changes...$(NC)"
	@cargo watch -x build

watch-test: ## Watch for changes and run tests (requires cargo-watch)
	@command -v cargo-watch >/dev/null 2>&1 || { echo "Installing cargo-watch..."; cargo install cargo-watch; }
	@echo "$(BLUE)Watching for changes and running tests...$(NC)"
	@cargo watch -x test

setup: ## Initial setup (install tools)
	@echo "$(BLUE)Setting up development environment...$(NC)"
	@rustup update
	@rustup component add rustfmt clippy
	@echo "$(GREEN)Setup complete!$(NC)"
	@echo "\nOptional tools (install manually if needed):"
	@echo "  cargo install cargo-watch    # Auto-rebuild on changes"
	@echo "  cargo install cargo-bloat    # Binary size analysis"

##@ Quick Reference

quick-start: build-release run-vm ## New user quick start
	@echo ""
	@echo "$(GREEN)Welcome to TopLang!$(NC)"
	@echo ""
	@echo "$(BLUE)Quick Reference:$(NC)"
	@echo "  Run a file:        ./target/release/topc myfile.top --bytecode"
	@echo "  See bytecode:      make show-bytecode"
	@echo "  Run benchmarks:    make bench-vm"
	@echo "  Run tests:         make test"
	@echo "  Code quality:      make check"
	@echo ""
	@echo "$(BLUE)Development:$(NC)"
	@echo "  make dev           # Format, lint, test"
	@echo "  make quick         # Quick build and run"
	@echo "  make help          # Show all commands"
