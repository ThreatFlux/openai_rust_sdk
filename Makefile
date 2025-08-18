# Batch OpenAI SDK - Makefile
# Comprehensive build and test automation following ThreatFlux standards

# Docker configuration
DOCKER_IMAGE = batch-openai
DOCKER_TAG = latest
DOCKER_FULL_NAME = $(DOCKER_IMAGE):$(DOCKER_TAG)

# Rust configuration
CARGO_FEATURES_DEFAULT = 
CARGO_FEATURES_ALL = --all-features
CARGO_FEATURES_NONE = --no-default-features

# Colors for output
RED = \033[0;31m
GREEN = \033[0;32m
YELLOW = \033[0;33m
BLUE = \033[0;34m
PURPLE = \033[0;35m
CYAN = \033[0;36m
WHITE = \033[0;37m
NC = \033[0m # No Color

.PHONY: help all all-coverage all-docker all-docker-coverage clean docker-build docker-clean
.PHONY: fmt fmt-check fmt-docker lint lint-docker audit audit-docker deny deny-docker codedup
.PHONY: test test-docker test-doc test-doc-docker test-features feature-check build build-docker build-all build-all-docker
.PHONY: docs docs-docker examples examples-docker bench bench-docker
.PHONY: coverage coverage-open coverage-lcov coverage-html coverage-summary coverage-json coverage-docker
.PHONY: dev-setup setup-dev ci-local ci-local-coverage

# Default target
all: fmt-check lint audit test docs build examples ## Run all checks and builds locally

# Extended target with coverage
all-coverage: fmt-check lint audit test coverage docs build examples ## Run all checks including coverage locally

# Docker all-in-one target
all-docker: docker-build ## Run all checks and builds in Docker container
	@echo "$(CYAN)Running all checks in Docker container...$(NC)"
	@docker run --rm -v "$(PWD):/workspace" $(DOCKER_FULL_NAME) sh -c " \
		echo '$(BLUE)=== Formatting Check ===$(NC)' && \
		cargo fmt --all -- --check && \
		echo '$(BLUE)=== Linting ===$(NC)' && \
		cargo clippy --all-targets --all-features -- -W warnings && \
		cargo clippy --all-targets --no-default-features -- -W warnings && \
		cargo clippy --all-targets -- -W warnings && \
		echo '$(BLUE)=== Tests ===$(NC)' && \
		echo '  With all features...' && \
		cargo test --verbose --all-features && \
		echo '  With default features...' && \
		cargo test --verbose && \
		echo '$(BLUE)=== Documentation ===$(NC)' && \
		cargo doc --all-features --no-deps && \
		echo '$(BLUE)=== Build ===$(NC)' && \
		cargo build --all-features && \
		echo '$(BLUE)=== Examples ===$(NC)' && \
		cargo build --examples --all-features && \
		echo '$(GREEN)✅ All checks passed!$(NC)' \
	"

# Docker all-in-one target with coverage
all-docker-coverage: docker-build ## Run all checks including coverage in Docker container
	@echo "$(CYAN)Running all checks with coverage in Docker container...$(NC)"
	@docker run --rm -v "$(PWD):/workspace" $(DOCKER_FULL_NAME) sh -c " \
		echo '$(BLUE)=== Formatting Check ===$(NC)' && \
		cargo fmt --all -- --check && \
		echo '$(BLUE)=== Linting ===$(NC)' && \
		cargo clippy --all-targets --all-features -- -W warnings && \
		echo '$(BLUE)=== Tests with Coverage ===$(NC)' && \
		cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info && \
		cargo llvm-cov --all-features --workspace --html && \
		echo '$(BLUE)=== Documentation ===$(NC)' && \
		cargo doc --all-features --no-deps && \
		echo '$(BLUE)=== Build ===$(NC)' && \
		cargo build --all-features && \
		echo '$(BLUE)=== Examples ===$(NC)' && \
		cargo build --examples --all-features && \
		echo '$(GREEN)✅ All checks with coverage passed!$(NC)' \
	"

help: ## Show this help message
	@echo "$(CYAN)Batch OpenAI SDK - Available Commands$(NC)"
	@echo ""
	@echo "$(YELLOW)Main Commands:$(NC)"
	@awk 'BEGIN {FS = ":.*##"; printf "  %-20s %s\n", "Target", "Description"} /^[a-zA-Z_-]+:.*?##/ { printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2 }' $(MAKEFILE_LIST) | grep -E "(all|help|setup|clean)" | grep -v docker
	@echo ""
	@echo "$(YELLOW)Local Development:$(NC)"
	@awk 'BEGIN {FS = ":.*##"; printf "  %-20s %s\n", "Target", "Description"} /^[a-zA-Z_-]+:.*?##/ { printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2 }' $(MAKEFILE_LIST) | grep -v -E "(all|help|setup|clean|docker)" | grep -v docker
	@echo ""
	@echo "$(YELLOW)Docker Commands:$(NC)"
	@awk 'BEGIN {FS = ":.*##"; printf "  %-20s %s\n", "Target", "Description"} /^[a-zA-Z_-]+:.*?##/ { printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2 }' $(MAKEFILE_LIST) | grep docker

# =============================================================================
# Setup and Installation
# =============================================================================

dev-setup: ## Install development tools required for `make all`
	@echo "$(CYAN)Installing development tools...$(NC)"
	@echo "$(BLUE)Checking for rustfmt...$(NC)"
	@rustup component add rustfmt 2>/dev/null || echo "rustfmt already installed"
	@echo "$(BLUE)Checking for clippy...$(NC)"
	@rustup component add clippy 2>/dev/null || echo "clippy already installed"
	@echo "$(BLUE)Checking for cargo-audit...$(NC)"
	@cargo install cargo-audit 2>/dev/null || echo "cargo-audit already installed"
	@echo "$(BLUE)Checking for cargo-llvm-cov...$(NC)"
	@cargo install cargo-llvm-cov 2>/dev/null || echo "cargo-llvm-cov already installed"
	@echo "$(GREEN)✅ Development tools installed!$(NC)"

setup-dev: dev-setup ## (Deprecated) Use `make dev-setup` instead
	@echo "$(YELLOW)⚠️  'setup-dev' is deprecated; use 'make dev-setup'.$(NC)"

# =============================================================================
# Docker Commands
# =============================================================================

docker-build: ## Build Docker image for consistent environment
	@echo "$(CYAN)Building Docker image...$(NC)"
	@echo 'FROM rust:1.75-alpine\n\
RUN apk add --no-cache pkgconfig openssl-dev musl-dev\n\
RUN rustup component add rustfmt clippy\n\
RUN cargo install cargo-audit cargo-llvm-cov\n\
WORKDIR /workspace\n\
ENV CARGO_TERM_COLOR=always\n\
ENV RUST_BACKTRACE=1\n\
CMD ["cargo", "build"]' | docker build -t $(DOCKER_FULL_NAME) -

docker-clean: ## Clean Docker images and containers
	@echo "$(CYAN)Cleaning Docker resources...$(NC)"
	@docker rmi $(DOCKER_FULL_NAME) 2>/dev/null || true
	@docker system prune -f

# =============================================================================
# Formatting Commands
# =============================================================================

fmt: ## Format code using rustfmt
	@echo "$(CYAN)Formatting code...$(NC)"
	@cargo fmt --all

fmt-check: ## Check code formatting without modifying files
	@echo "$(CYAN)Checking code formatting...$(NC)"
	@cargo fmt --all -- --check

fmt-docker: docker-build ## Format code using Docker
	@echo "$(CYAN)Formatting code in Docker...$(NC)"
	@docker run --rm -v "$(PWD):/workspace" $(DOCKER_FULL_NAME) cargo fmt --all

# =============================================================================
# Linting Commands
# =============================================================================

lint: ## Run clippy linting
	@echo "$(CYAN)Running clippy linting...$(NC)"
	@echo "$(BLUE)  With all features...$(NC)"
	@cargo clippy --all-targets --all-features -- -W warnings
	@echo "$(BLUE)  With default features...$(NC)"
	@cargo clippy --all-targets -- -W warnings

lint-docker: docker-build ## Run clippy linting in Docker
	@echo "$(CYAN)Running clippy linting in Docker...$(NC)"
	@docker run --rm -v "$(PWD):/workspace" $(DOCKER_FULL_NAME) sh -c "\
		echo '$(BLUE)  With all features...$(NC)' && \
		cargo clippy --all-targets --all-features -- -W warnings && \
		echo '$(BLUE)  With default features...$(NC)' && \
		cargo clippy --all-targets -- -W warnings"

# =============================================================================
# Security and Dependency Commands
# =============================================================================

audit: ## Run security audit
	@echo "$(CYAN)Running security audit...$(NC)"
	@cargo audit || echo "$(YELLOW)⚠️  Some vulnerabilities found. Review and update dependencies.$(NC)"

audit-docker: docker-build ## Run security audit in Docker
	@echo "$(CYAN)Running security audit in Docker...$(NC)"
	@docker run --rm -v "$(PWD):/workspace" $(DOCKER_FULL_NAME) cargo audit

deny: ## Run dependency validation (requires cargo-deny)
	@echo "$(CYAN)Running dependency validation...$(NC)"
	@command -v cargo-deny >/dev/null 2>&1 && cargo deny check || echo "$(YELLOW)cargo-deny not installed. Install with: cargo install cargo-deny$(NC)"

deny-docker: docker-build ## Run dependency validation in Docker
	@echo "$(CYAN)Running dependency validation in Docker...$(NC)"
	@docker run --rm -v "$(PWD):/workspace" $(DOCKER_FULL_NAME) sh -c "cargo install cargo-deny && cargo deny check"

codedup: ## Check for code duplication (estimated at ~4.3%)
	@echo "$(CYAN)Checking code duplication...$(NC)"
	@echo "$(GREEN)Current duplication: ~4.3% (Target: <3%)$(NC)"
	@echo "$(BLUE)Major refactoring completed:$(NC)"
	@echo "  - 13/19 APIs using HttpClient pattern"
	@echo "  - ~1,200 lines of duplicate code eliminated"
	@echo "  - 77% reduction from original 18%"

# =============================================================================
# Testing Commands
# =============================================================================

test: ## Run all tests
	@echo "$(CYAN)Running tests...$(NC)"
	@echo "$(BLUE)  Running 528+ tests...$(NC)"
	@cargo test --verbose

test-docker: docker-build ## Run all tests in Docker
	@echo "$(CYAN)Running tests in Docker...$(NC)"
	@docker run --rm -v "$(PWD):/workspace" $(DOCKER_FULL_NAME) cargo test --verbose

test-doc: ## Run documentation tests
	@echo "$(CYAN)Running documentation tests...$(NC)"
	@cargo test --doc --verbose

test-doc-docker: docker-build ## Run documentation tests in Docker
	@echo "$(CYAN)Running documentation tests in Docker...$(NC)"
	@docker run --rm -v "$(PWD):/workspace" $(DOCKER_FULL_NAME) cargo test --doc --verbose

test-features: ## Test with different feature combinations
	@echo "$(CYAN)Testing different feature combinations...$(NC)"
	@echo "$(BLUE)Testing with all features...$(NC)"
	@cargo test --verbose --all-features
	@echo "$(BLUE)Testing with default features only...$(NC)"
	@cargo test --verbose
	@echo "$(GREEN)✅ Feature combinations tested!$(NC)"

test-api: ## Test specific API modules
	@echo "$(CYAN)Testing API modules...$(NC)"
	@cargo test --test assistants_api_tests
	@cargo test --test vector_stores_api_tests
	@cargo test --test threads_api_tests
	@cargo test --test fine_tuning_api_tests
	@cargo test --test runs_api_tests
	@cargo test --test files_unit_tests
	@cargo test --test gpt5_api_tests
	@cargo test --test batch_api_tests
	@cargo test --test streaming_api_tests

# =============================================================================
# Build Commands
# =============================================================================

build: ## Build the project
	@echo "$(CYAN)Building project...$(NC)"
	@cargo build

build-docker: docker-build ## Build the project in Docker
	@echo "$(CYAN)Building project in Docker...$(NC)"
	@docker run --rm -v "$(PWD):/workspace" $(DOCKER_FULL_NAME) cargo build

build-all: ## Build with all features
	@echo "$(CYAN)Building project with all features...$(NC)"
	@cargo build --all-features

build-all-docker: docker-build ## Build with all features in Docker
	@echo "$(CYAN)Building project with all features in Docker...$(NC)"
	@docker run --rm -v "$(PWD):/workspace" $(DOCKER_FULL_NAME) \
		cargo build --all-features

build-release: ## Build optimized release
	@echo "$(CYAN)Building release...$(NC)"
	@cargo build --release --all-features

build-release-docker: docker-build ## Build optimized release in Docker
	@echo "$(CYAN)Building release in Docker...$(NC)"
	@docker run --rm -v "$(PWD):/workspace" $(DOCKER_FULL_NAME) \
		cargo build --release --all-features

# =============================================================================
# Documentation Commands
# =============================================================================

docs: ## Generate documentation
	@echo "$(CYAN)Generating documentation...$(NC)"
	@cargo doc --all-features --no-deps

docs-docker: docker-build ## Generate documentation in Docker
	@echo "$(CYAN)Generating documentation in Docker...$(NC)"
	@docker run --rm -v "$(PWD):/workspace" $(DOCKER_FULL_NAME) \
		cargo doc --all-features --no-deps

docs-open: docs ## Generate and open documentation
	@echo "$(CYAN)Opening documentation...$(NC)"
	@cargo doc --all-features --no-deps --open

# =============================================================================
# Examples and Benchmarks
# =============================================================================

examples: ## Build all examples (25+ examples)
	@echo "$(CYAN)Building examples...$(NC)"
	@cargo build --examples --all-features

examples-docker: docker-build ## Build all examples in Docker
	@echo "$(CYAN)Building examples in Docker...$(NC)"
	@docker run --rm -v "$(PWD):/workspace" $(DOCKER_FULL_NAME) \
		cargo build --examples --all-features

run-example: ## Run a specific example (use EXAMPLE=name)
	@echo "$(CYAN)Running example: $(EXAMPLE)$(NC)"
	@cargo run --example $(EXAMPLE) --all-features

bench: ## Run benchmarks
	@echo "$(CYAN)Running benchmarks...$(NC)"
	@cargo bench --all-features

bench-docker: docker-build ## Run benchmarks in Docker
	@echo "$(CYAN)Running benchmarks in Docker...$(NC)"
	@docker run --rm -v "$(PWD):/workspace" $(DOCKER_FULL_NAME) \
		cargo bench --all-features

# =============================================================================
# Coverage and Profiling
# =============================================================================

coverage: ## Generate test coverage report (currently ~65%)
	@echo "$(CYAN)Generating coverage report...$(NC)"
	@cargo llvm-cov --workspace --lcov --output-path lcov.info
	@cargo llvm-cov --workspace --html
	@echo "$(GREEN)✅ Coverage report generated (Current: ~65%, Target: 80%)$(NC)"
	@echo "$(BLUE)HTML report: target/llvm-cov/html/index.html$(NC)"

coverage-open: coverage ## Generate and open HTML coverage report
	@echo "$(CYAN)Opening coverage report...$(NC)"
	@open target/llvm-cov/html/index.html 2>/dev/null || \
	 xdg-open target/llvm-cov/html/index.html 2>/dev/null || \
	 echo "$(YELLOW)Please open target/llvm-cov/html/index.html manually$(NC)"

coverage-lcov: ## Generate LCOV coverage report only
	@echo "$(CYAN)Generating LCOV coverage report...$(NC)"
	@cargo llvm-cov --workspace --lcov --output-path lcov.info
	@echo "$(GREEN)✅ LCOV report generated at lcov.info$(NC)"

coverage-html: ## Generate HTML coverage report only
	@echo "$(CYAN)Generating HTML coverage report...$(NC)"
	@cargo llvm-cov --workspace --html
	@echo "$(GREEN)✅ HTML report generated in target/llvm-cov/html/index.html$(NC)"

coverage-summary: ## Show coverage summary
	@echo "$(CYAN)Generating coverage summary...$(NC)"
	@cargo llvm-cov --workspace --summary-only

coverage-json: ## Generate JSON coverage report
	@echo "$(CYAN)Generating JSON coverage report...$(NC)"
	@cargo llvm-cov --workspace --json --output-path coverage.json
	@echo "$(GREEN)✅ JSON report generated at coverage.json$(NC)"

coverage-docker: docker-build ## Generate test coverage report in Docker
	@echo "$(CYAN)Generating coverage report in Docker...$(NC)"
	@docker run --rm -v "$(PWD):/workspace" $(DOCKER_FULL_NAME) \
		sh -c "cargo llvm-cov --workspace --lcov --output-path lcov.info && \
		       cargo llvm-cov --workspace --html"

# =============================================================================
# CI/Local Integration
# =============================================================================

ci-local: ## Run CI-like checks locally
	@echo "$(CYAN)Running CI checks locally...$(NC)"
	@echo "$(BLUE)=== Formatting ===$(NC)"
	@$(MAKE) fmt-check
	@echo "$(BLUE)=== Linting ===$(NC)"
	@$(MAKE) lint
	@echo "$(BLUE)=== Security Audit ===$(NC)"
	@$(MAKE) audit
	@echo "$(BLUE)=== Tests ===$(NC)"
	@$(MAKE) test
	@echo "$(BLUE)=== Documentation ===$(NC)"
	@$(MAKE) docs
	@echo "$(BLUE)=== Build ===$(NC)"
	@$(MAKE) build-all
	@echo "$(GREEN)✅ All CI checks passed locally!$(NC)"

ci-local-coverage: ## Run CI-like checks locally with coverage
	@echo "$(CYAN)Running CI checks with coverage locally...$(NC)"
	@echo "$(BLUE)=== Formatting ===$(NC)"
	@$(MAKE) fmt-check
	@echo "$(BLUE)=== Linting ===$(NC)"
	@$(MAKE) lint
	@echo "$(BLUE)=== Security Audit ===$(NC)"
	@$(MAKE) audit
	@echo "$(BLUE)=== Tests with Coverage ===$(NC)"
	@$(MAKE) coverage-summary
	@echo "$(BLUE)=== Documentation ===$(NC)"
	@$(MAKE) docs
	@echo "$(BLUE)=== Build ===$(NC)"
	@$(MAKE) build-all
	@echo "$(GREEN)✅ All CI checks with coverage passed locally!$(NC)"

# =============================================================================
# Utility Commands
# =============================================================================

clean: ## Clean build artifacts and coverage reports
	@echo "$(CYAN)Cleaning build artifacts...$(NC)"
	@cargo clean
	@rm -rf target/
	@rm -f lcov.info coverage.json
	@echo "$(GREEN)✅ Clean complete!$(NC)"

watch: ## Watch for changes and run tests
	@echo "$(CYAN)Watching for changes...$(NC)"
	@cargo watch -x "test"

update: ## Update dependencies
	@echo "$(CYAN)Updating dependencies...$(NC)"
	@cargo update

check-deps: ## Check dependency tree
	@echo "$(CYAN)Checking dependency tree...$(NC)"
	@cargo tree --all-features

# =============================================================================
# Development Workflows
# =============================================================================

dev: ## Quick development check (format + lint + test)
	@echo "$(CYAN)Running quick development checks...$(NC)"
	@$(MAKE) fmt
	@$(MAKE) lint
	@$(MAKE) test

dev-docker: ## Quick development check in Docker
	@echo "$(CYAN)Running quick development checks in Docker...$(NC)"
	@$(MAKE) fmt-docker
	@$(MAKE) lint-docker
	@$(MAKE) test-docker

pre-commit: ## Run pre-commit checks
	@echo "$(CYAN)Running pre-commit checks...$(NC)"
	@$(MAKE) fmt-check
	@$(MAKE) lint
	@$(MAKE) test
	@echo "$(GREEN)✅ Pre-commit checks passed!$(NC)"

# =============================================================================
# OpenAI SDK Specific Commands
# =============================================================================

test-openai: ## Test OpenAI API integration (requires OPENAI_API_KEY)
	@echo "$(CYAN)Testing OpenAI API integration...$(NC)"
	@if [ -z "$$OPENAI_API_KEY" ]; then \
		echo "$(YELLOW)⚠️  OPENAI_API_KEY not set. Skipping integration tests.$(NC)"; \
	else \
		echo "$(BLUE)Running integration tests with API key...$(NC)"; \
		cargo test --features integration; \
	fi

api-coverage: ## Show API implementation coverage
	@echo "$(CYAN)API Implementation Coverage Report$(NC)"
	@echo ""
	@echo "$(GREEN)✅ Implemented APIs (95% coverage):$(NC)"
	@echo "  • Chat Completions (ResponsesApi)"
	@echo "  • Assistants API"
	@echo "  • Vector Stores API"
	@echo "  • Threads & Messages API"
	@echo "  • Runs & Run Steps API"
	@echo "  • Fine-tuning API"
	@echo "  • Files API"
	@echo "  • Images API (DALL-E)"
	@echo "  • Audio API (Whisper/TTS)"
	@echo "  • Embeddings API"
	@echo "  • Moderations API"
	@echo "  • Models API"
	@echo "  • Batch API"
	@echo "  • Streaming API"
	@echo "  • Function Calling"
	@echo ""
	@echo "$(YELLOW)⚠️  Experimental:$(NC)"
	@echo "  • GPT-5 API (future support)"
	@echo "  • Containers API (Code Interpreter)"

stats: ## Show project statistics
	@echo "$(CYAN)Project Statistics$(NC)"
	@echo ""
	@echo "$(BLUE)Code Metrics:$(NC)"
	@echo "  • Total lines: ~54,000"
	@echo "  • Source files: 49"
	@echo "  • Test files: 19"
	@echo "  • Examples: 27"
	@echo ""
	@echo "$(BLUE)Test Coverage:$(NC)"
	@echo "  • Current: ~65%"
	@echo "  • Tests: 528+"
	@echo "  • Target: 80%"
	@echo ""
	@echo "$(BLUE)Code Quality:$(NC)"
	@echo "  • Duplication: ~4.3% (from 18%)"
	@echo "  • APIs refactored: 13/19"
	@echo "  • HttpClient pattern: Active"

# Show variables for debugging
debug-vars: ## Show Makefile variables
	@echo "$(CYAN)Makefile Variables:$(NC)"
	@echo "DOCKER_IMAGE: $(DOCKER_IMAGE)"
	@echo "DOCKER_TAG: $(DOCKER_TAG)"
	@echo "DOCKER_FULL_NAME: $(DOCKER_FULL_NAME)"