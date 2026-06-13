# ThreatFlux Rust Project Makefile
# Standardized build, test, and development commands for openai_rust_sdk.

CARGO ?= cargo
RUST_MSRV ?= 1.96.0
RUST_TOOLCHAIN ?= 1.96.0

DOCKER_IMAGE ?= openai-rust-sdk
DOCKER_TAG ?= latest
GITHUB_OWNER ?= $(shell git config --get remote.origin.url | sed -E 's#(git@github.com:|https://github.com/)##; s#/.+##' | tr '[:upper:]' '[:lower:]')
DOCKER_REGISTRY ?= ghcr.io/$(if $(GITHUB_OWNER),$(GITHUB_OWNER),local)
BINARY_NAME ?= openai_rust_sdk
BINARY_PACKAGE ?=
CLI_NAME ?= openai-rust-sdk
SBOM_MANIFEST_PATH ?= Cargo.toml
PUBLISH_PACKAGES ?= openai_rust_sdk

CLIPPY_FLAGS := -D warnings \
	-D clippy::all \
	-D clippy::pedantic \
	-D clippy::nursery \
	-D clippy::cargo \
	-A clippy::multiple_crate_versions \
	-A clippy::module_name_repetitions \
	-A clippy::missing_errors_doc \
	-A clippy::missing_panics_doc \
	-A clippy::must_use_candidate

RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
CYAN := \033[0;36m
NC := \033[0m

.DEFAULT_GOAL := help

.PHONY: help
help: ## Display this help message
	@echo "$(CYAN)OpenAI Rust SDK - Available Commands$(NC)"
	@echo ""
	@echo "$(YELLOW)Toolchain:$(NC)"
	@echo "  Rust MSRV:      $(GREEN)$(RUST_MSRV)$(NC)"
	@echo "  Rust toolchain: $(GREEN)$(RUST_TOOLCHAIN)$(NC)"
	@echo ""
	@echo "$(YELLOW)Quick Start:$(NC)"
	@echo "  $(GREEN)make dev-setup$(NC)       Install all development tools"
	@echo "  $(GREEN)make template-check$(NC)  Validate bootstrap placeholders"
	@echo "  $(GREEN)make ci$(NC)              Run all CI checks locally"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-18s$(NC) %s\n", $$1, $$2}'

.PHONY: dev-setup
dev-setup: ## Install development tools
	@echo "$(CYAN)Installing development tools...$(NC)"
	@rustup toolchain install $(RUST_TOOLCHAIN) --profile minimal >/dev/null 2>&1 || true
	@rustup component add rustfmt clippy llvm-tools-preview 2>/dev/null || true
	@cargo install cargo-llvm-cov --locked 2>/dev/null || echo "cargo-llvm-cov already installed"
	@cargo install cargo-audit --locked 2>/dev/null || echo "cargo-audit already installed"
	@cargo install cargo-deny --locked 2>/dev/null || echo "cargo-deny already installed"
	@cargo install cargo-cyclonedx --locked 2>/dev/null || echo "cargo-cyclonedx already installed"
	@cargo install cargo-hack --locked 2>/dev/null || echo "cargo-hack already installed"
	@python3 -m pip install --user pre-commit 2>/dev/null || echo "pre-commit already available"
	@echo "$(GREEN)Development tools installed!$(NC)"

.PHONY: setup-dev
setup-dev: dev-setup ## Alias: install development tools

.PHONY: install-hooks
install-hooks: ## Install git hooks
	@echo "$(CYAN)Installing git hooks...$(NC)"
	@mkdir -p .git/hooks
	@printf '#!/bin/sh\nmake pre-commit\n' > .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "$(GREEN)Git hooks installed!$(NC)"

.PHONY: build
build: ## Build the project (debug)
	@echo "$(CYAN)Building project...$(NC)"
	@$(CARGO) build --all-features
	@echo "$(GREEN)Build completed!$(NC)"

.PHONY: build-release
build-release: ## Build the project (release)
	@echo "$(CYAN)Building release...$(NC)"
	@if [ -n "$(BINARY_PACKAGE)" ]; then \
		$(CARGO) build --release -p $(BINARY_PACKAGE) --bin $(BINARY_NAME) --all-features; \
	else \
		$(CARGO) build --release --bin $(BINARY_NAME) --all-features || $(CARGO) build --release --all-features; \
	fi
	@echo "$(GREEN)Release build completed!$(NC)"

.PHONY: check
check: ## Check compilation without building
	@echo "$(CYAN)Checking compilation...$(NC)"
	@$(CARGO) check --all-features --all-targets

.PHONY: fmt
fmt: ## Format code
	@echo "$(CYAN)Formatting code...$(NC)"
	@$(CARGO) fmt --all
	@echo "$(GREEN)Formatting completed!$(NC)"

.PHONY: fmt-check
fmt-check: ## Check code formatting
	@echo "$(CYAN)Checking code format...$(NC)"
	@$(CARGO) fmt --all -- --check
	@echo "$(GREEN)Format check passed!$(NC)"

.PHONY: lint
lint: ## Run clippy linter
	@echo "$(CYAN)Running clippy...$(NC)"
	@$(CARGO) clippy --all-features --all-targets -- -D warnings
	@echo "$(GREEN)Linting passed!$(NC)"

.PHONY: lint-strict
lint-strict: ## Run clippy with strict flags
	@echo "$(CYAN)Running strict clippy...$(NC)"
	@$(CARGO) clippy --all-features --all-targets -- $(CLIPPY_FLAGS)
	@echo "$(GREEN)Strict linting passed!$(NC)"

.PHONY: lint-fix
lint-fix: ## Run clippy and apply fixes
	@echo "$(CYAN)Applying clippy fixes...$(NC)"
	@$(CARGO) clippy --all-features --all-targets --fix --allow-dirty --allow-staged -- -D warnings
	@echo "$(GREEN)Fixes applied!$(NC)"

.PHONY: test
test: ## Run all tests
	@echo "$(CYAN)Running tests...$(NC)"
	@$(CARGO) test --all-features
	@echo "$(GREEN)Tests passed!$(NC)"

.PHONY: test-verbose
test-verbose: ## Run tests with output
	@echo "$(CYAN)Running tests (verbose)...$(NC)"
	@$(CARGO) test --all-features -- --nocapture

.PHONY: test-doc
test-doc: ## Run documentation tests
	@echo "$(CYAN)Running doc tests...$(NC)"
	@$(CARGO) test --doc --all-features
	@echo "$(GREEN)Doc tests passed!$(NC)"

.PHONY: test-features
test-features: ## Test feature combinations
	@echo "$(CYAN)Testing feature combinations...$(NC)"
	@echo "$(BLUE)  No default features...$(NC)"
	@$(CARGO) check --workspace --no-default-features
	@echo "$(BLUE)  All features...$(NC)"
	@$(CARGO) check --workspace --all-features
	@echo "$(BLUE)  Default features only...$(NC)"
	@$(CARGO) check --workspace
	@echo "$(GREEN)Feature checks passed!$(NC)"

.PHONY: feature-check
feature-check: test-features ## Alias: test feature combinations

.PHONY: test-features-full
test-features-full: ## Test full feature powerset
	@echo "$(CYAN)Testing full feature powerset...$(NC)"
	@cargo hack check --workspace --feature-powerset --no-dev-deps
	@echo "$(GREEN)Feature powerset passed!$(NC)"

.PHONY: coverage
coverage: ## Generate code coverage report
	@echo "$(CYAN)Generating coverage...$(NC)"
	@cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
	@echo "$(GREEN)Coverage report: lcov.info$(NC)"

.PHONY: coverage-html
coverage-html: ## Generate HTML coverage report
	@echo "$(CYAN)Generating HTML coverage...$(NC)"
	@cargo llvm-cov --all-features --workspace --html
	@echo "$(GREEN)Report: target/llvm-cov/html/index.html$(NC)"

.PHONY: coverage-summary
coverage-summary: ## Show coverage summary
	@echo "$(CYAN)Coverage summary:$(NC)"
	@cargo llvm-cov --all-features --workspace --summary-only

.PHONY: audit
audit: ## Run security audit
	@echo "$(CYAN)Running security audit...$(NC)"
	@cargo audit
	@echo "$(GREEN)Security audit passed!$(NC)"

.PHONY: deny
deny: ## Check licenses and advisories
	@echo "$(CYAN)Running cargo-deny...$(NC)"
	@cargo deny check
	@echo "$(GREEN)Deny checks passed!$(NC)"

.PHONY: sbom
sbom: ## Generate a CycloneDX SBOM
	@echo "$(CYAN)Generating SBOM...$(NC)"
	@mkdir -p sbom
	@rm -f sbom/*.json
	@cargo cyclonedx --manifest-path $(SBOM_MANIFEST_PATH) --all-features --format json --spec-version 1.5 --override-filename $(BINARY_NAME)-sbom
	@find . -maxdepth 4 -name '$(BINARY_NAME)-sbom.json' -exec mv {} sbom/ \;
	@echo "$(GREEN)SBOM written to sbom/$(BINARY_NAME)-sbom.json$(NC)"

.PHONY: security
security: audit deny ## Run all security checks
	@echo "$(GREEN)All security checks passed!$(NC)"

.PHONY: docs
docs: ## Build documentation
	@echo "$(CYAN)Building documentation...$(NC)"
	@RUSTDOCFLAGS="-D warnings" $(CARGO) doc --all-features --no-deps
	@echo "$(GREEN)Documentation built!$(NC)"

.PHONY: docs-open
docs-open: ## Build and open documentation
	@$(CARGO) doc --all-features --no-deps --open

.PHONY: bench
bench: ## Run benchmarks
	@echo "$(CYAN)Running benchmarks...$(NC)"
	@$(CARGO) bench --all-features

.PHONY: bench-check
bench-check: ## Check benchmarks compile
	@echo "$(CYAN)Checking benchmarks...$(NC)"
	@$(CARGO) bench --all-features --no-run
	@echo "$(GREEN)Benchmarks compile!$(NC)"

.PHONY: msrv
msrv: ## Check minimum supported Rust version
	@echo "$(CYAN)Checking MSRV ($(RUST_MSRV))...$(NC)"
	@rustup toolchain install $(RUST_MSRV) --profile minimal >/dev/null 2>&1 || true
	@rustup run $(RUST_MSRV) cargo check --workspace --all-features
	@echo "$(GREEN)MSRV check passed!$(NC)"

.PHONY: docker-build
docker-build: ## Build Docker image
	@echo "$(CYAN)Building Docker image...$(NC)"
	@docker build \
		--build-arg BINARY_NAME=$(BINARY_NAME) \
		--build-arg BINARY_PACKAGE=$(BINARY_PACKAGE) \
		--build-arg CLI_NAME=$(CLI_NAME) \
		--build-arg SBOM_MANIFEST_PATH=$(SBOM_MANIFEST_PATH) \
		-t $(DOCKER_REGISTRY)/$(DOCKER_IMAGE):$(DOCKER_TAG) .
	@echo "$(GREEN)Docker image built: $(DOCKER_REGISTRY)/$(DOCKER_IMAGE):$(DOCKER_TAG)$(NC)"

.PHONY: docker-push
docker-push: ## Push Docker image to registry
	@echo "$(CYAN)Pushing Docker image...$(NC)"
	@docker push $(DOCKER_REGISTRY)/$(DOCKER_IMAGE):$(DOCKER_TAG)
	@echo "$(GREEN)Docker image pushed!$(NC)"

.PHONY: pre-commit
pre-commit: fmt-check lint test-doc ## Pre-commit checks

.PHONY: template-check
template-check: ## Fail if template placeholders are still present
	@echo "$(CYAN)Checking for template placeholders...$(NC)"
	@python3 scripts/check_template_placeholders.py
	@echo "$(GREEN)No unresolved template placeholders found!$(NC)"

.PHONY: ci
ci: template-check fmt-check lint test test-features docs security ## Full CI checks

.PHONY: ci-quick
ci-quick: template-check fmt-check lint check ## Quick CI checks

.PHONY: ci-local
ci-local: ci ## Alias: run local CI checks

.PHONY: ci-local-coverage
ci-local-coverage: fmt-check lint coverage-summary docs build ## Alias: run local CI checks with coverage

.PHONY: all
all: ci coverage bench-check ## Full validation suite

.PHONY: release-check
release-check: ## Check release readiness
	@echo "$(CYAN)Checking release readiness...$(NC)"
	@$(CARGO) check --all-features
	@$(CARGO) test --all-features
	@$(CARGO) clippy --all-features --all-targets -- -D warnings
	@python3 scripts/check_template_placeholders.py
	@echo "$(GREEN)Release readiness checks passed!$(NC)"

.PHONY: test-api
test-api: ## Test selected API modules
	@echo "$(CYAN)Testing API modules...$(NC)"
	@cargo test --test assistants_api_tests
	@cargo test --test vector_stores_tests
	@cargo test --test threads_api_tests
	@cargo test --test fine_tuning_api_tests
	@cargo test --test runs_api_tests
	@cargo test --test files_unit_tests
	@cargo test --test gpt5_api_tests
	@cargo test --test batch_api_tests
	@cargo test --test streaming_api_tests

.PHONY: test-openai
test-openai: ## Test OpenAI API integration when OPENAI_API_KEY is set
	@echo "$(CYAN)Testing OpenAI API integration...$(NC)"
	@if [ -z "$$OPENAI_API_KEY" ]; then \
		echo "$(YELLOW)OPENAI_API_KEY not set. Skipping integration tests.$(NC)"; \
	else \
		cargo test --test openai_live_test -- --nocapture; \
	fi

.PHONY: api-coverage
api-coverage: ## Show API implementation coverage
	@echo "$(CYAN)API Implementation Coverage Report$(NC)"
	@echo ""
	@echo "$(GREEN)Implemented APIs (95% coverage):$(NC)"
	@echo "  - Chat Completions (ResponsesApi)"
	@echo "  - Assistants API"
	@echo "  - Vector Stores API"
	@echo "  - Threads & Messages API"
	@echo "  - Runs & Run Steps API"
	@echo "  - Fine-tuning API"
	@echo "  - Files API"
	@echo "  - Images API"
	@echo "  - Audio API"
	@echo "  - Embeddings API"
	@echo "  - Moderations API"
	@echo "  - Models API"
	@echo "  - Batch API"
	@echo "  - Streaming API"
	@echo "  - Function Calling"

.PHONY: stats
stats: ## Show project statistics
	@echo "$(CYAN)Project Statistics$(NC)"
	@echo ""
	@echo "$(BLUE)Code Metrics:$(NC)"
	@echo "  - Total lines: ~54,000"
	@echo "  - Source files: 49"
	@echo "  - Test files: 19"
	@echo "  - Examples: 27"
	@echo ""
	@echo "$(BLUE)Test Coverage:$(NC)"
	@echo "  - Current: ~65%"
	@echo "  - Tests: 528+"
	@echo "  - Target: 80%"
	@echo ""
	@echo "$(BLUE)Code Quality:$(NC)"
	@echo "  - Duplication: ~4.3% (from 18%)"
	@echo "  - APIs refactored: 13/19"
	@echo "  - HttpClient pattern: Active"

.PHONY: clean
clean: ## Clean build artifacts
	@echo "$(CYAN)Cleaning artifacts...$(NC)"
	@$(CARGO) clean
	@rm -rf lcov.info coverage coverage.json sbom dist
	@echo "$(GREEN)Cleaned!$(NC)"

.PHONY: debug-vars
debug-vars: ## Show Makefile variables
	@echo "$(CYAN)Makefile Variables:$(NC)"
	@echo "RUST_MSRV: $(RUST_MSRV)"
	@echo "RUST_TOOLCHAIN: $(RUST_TOOLCHAIN)"
	@echo "DOCKER_IMAGE: $(DOCKER_IMAGE)"
	@echo "DOCKER_TAG: $(DOCKER_TAG)"
	@echo "DOCKER_REGISTRY: $(DOCKER_REGISTRY)"
	@echo "BINARY_NAME: $(BINARY_NAME)"
	@echo "CLI_NAME: $(CLI_NAME)"

.PHONY: f l t b c
f: fmt   ## Alias: format
l: lint  ## Alias: lint
t: test  ## Alias: test
b: build ## Alias: build
c: check ## Alias: check
