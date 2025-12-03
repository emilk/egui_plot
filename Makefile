# Configuration variables
THREADS ?= $(shell nproc)
BROWSER ?= firefox
TRUNK_OPTS ?=
CARGO_OPTS ?= -j $(THREADS)

export RUST_LOG ?= debug
export CARGO_TERM_COLOR ?= always
export RUST_BACKTRACE ?= 1
export CARGO_INCREMENTAL ?= 1
export RUSTDOCFLAGS ?= -D warnings -A rustdoc::private_intra_doc_links

# Host targets.
TARGET_LINUX ?= x86_64-unknown-linux-gnu
TARGET_WASM ?= wasm32-unknown-unknown

# By default, build in debug mode. Use BUILD_PROFILE=--release to build in release mode.
BUILD_PROFILE ?=

# Versions of tools / languages / frameworks.
RUST_NIGHTLY_VERSION ?= +nightly-2025-07-04
CARGO_TOOLS = \
	cargo-license@0.6.1 \
	cargo-nextest@0.9.99 \
	cargo-cycles@0.1.0 \
	cargo-chef@0.1.72 \
	cargo-public-api@0.50.1 \
	cargo-deny@0.18.5 \
	trunk@0.21.14 \
	wasm-pack@0.13.1

.PHONY: demo

# ---------------------------------- main --------------------------------------

default: dry_push

help:
	@echo "Usage: make <target>\n"
	@echo "Most important targets:"
	@echo "  build           Build everything for Linux and wasm."
	@echo "  test            Run all tests in the workspace."
	@echo "  tmux            Launch both backend and frontend in tmux split windows."
	@echo "  dry_push        Run all checks before pushing."
	@echo "  push            Same as dry_push, but pushes to the remote repository."
	@echo "  clean           Clean the workspace."
	@echo "\nMost important variables:"
	@echo "  BUILD_PROFILE   Set to --release to build in release mode (default: $(BUILD_PROFILE))."
	@echo "  THREADS         Number of threads to use for compilation (default: $(THREADS))."
	@echo "  BROWSER         Browser to use for frontend (default: $(BROWSER))."
	@echo "  TRUNK_OPTS      Extra options to pass to trunk (default: $(TRUNK_OPTS))."
	@echo "  CARGO_OPTS      Extra options to pass to cargo (default: $(CARGO_OPTS))."


# Run all pre-push checks.
dry_push: check_branch_name \
        fmt \
		docs \
		check_clippy \
		check_clippy_docs \
		check_cargo \
		build \
		run_checks \
		test \
		trunk \
		check_git_clean_status
	echo "All checks passed, ready to push."

# Pushes the current branch to the remote repository.
push: dry_push
	git push --set-upstream origin HEAD

# ------------------------------- build ----------------------------------------

# Builds all crates in the workspace, without running tests.
build: build_linux build_wasm

build_linux:
	cargo build --workspace --all-features --lib --tests --bins --examples --target $(TARGET_LINUX) $(CARGO_OPTS) $(BUILD_PROFILE)

build_wasm:
	cargo build -p egui_plot -p demo --all-features --lib --tests --bins --examples --target $(TARGET_WASM) $(CARGO_OPTS) $(BUILD_PROFILE)

trunk:
	cd demo && trunk build $(TRUNK_OPTS)

# -------------------------------- test ----------------------------------------

# Runs all tests in the workspace.
# Wasm tests are not run by default when pushing, but they are run in CI.
test: test_linux test_docs
test_linux:
	cargo nextest run --no-fail-fast --target $(TARGET_LINUX) -j $(THREADS) $(BUILD_PROFILE)
test_docs:
	# Nextest does not support doc tests, so we fall back to cargo test.
	cargo test --doc --target $(TARGET_LINUX) -j $(THREADS) $(BUILD_PROFILE)

# ------------------------------- run ------------------------------------------

demo:
	cargo run -p demo

# Formats all code in the workspace. Uses nightly as it can format doc-tests.
fmt:
	cargo $(RUST_NIGHTLY_VERSION) fmt --all

# Builds the documentation for all crates in the workspace.
docs:
	cargo doc --workspace --no-deps $(CARGO_OPTS)

# ------------------------------- checks ---------------------------------------

# Platform independent checks.
run_checks: check_no_commented_out_code \
			check_newlines \
			check_fmt \
			check_license \
			check_cycles \
			checks_no_unfinished \
			check_shear \
			check_deny \
			check_linter \
			check_pub_change_intentional

# Performs a compilation check of all crates in the workspace, without building.
check_cargo: check_cargo_linux check_cargo_wasm
check_cargo_linux:
	cargo check --workspace --all-features --tests --examples --target $(TARGET_LINUX) -j $(THREADS) $(BUILD_PROFILE)
check_cargo_wasm:
	cargo check -p egui_plot -p demo --tests --examples --target $(TARGET_WASM) -j $(THREADS) $(BUILD_PROFILE)
check_fmt:
	cargo fmt --all -- --check
# Checks that all files have a newline at the end.
check_newlines:
	./.github/lint_newlines.sh

# Runs clippy on all crates in the workspace.
check_clippy: check_clippy_linux check_clippy_wasm
check_clippy_linux:
	cargo clippy --workspace --no-deps --examples --lib --target $(TARGET_LINUX) -j $(THREADS) $(BUILD_PROFILE) -- -D warnings
check_clippy_wasm:
	cargo clippy -p demo -p egui_plot --no-deps --examples --lib --target $(TARGET_WASM) -j $(THREADS) $(BUILD_PROFILE) -- -D warnings
check_clippy_docs: check_clippy_docs_linux check_clippy_docs_wasm
check_clippy_docs_linux:
	cargo clippy --workspace --no-deps --message-format=short --target $(TARGET_LINUX) -j $(THREADS) $(BUILD_PROFILE) -- -D missing-docs 2>&1 | grep -P 'mod.rs|lib.rs' | grep -vP 'crate|module' && exit 1 || exit 0
check_clippy_docs_wasm:
	cargo clippy -p demo -p egui_plot --no-deps --message-format=short --target $(TARGET_WASM) -j $(THREADS) $(BUILD_PROFILE) -- -D missing-docs 2>&1 | grep -P 'mod.rs|lib.rs' | grep -vP 'crate|module' && exit 1 || exit 0

# Checks if the dependencies have approved licenses.
check_license:
	echo "skipped for now due to https://github.com/EmbarkStudios/cargo-deny/issues/804"

# Checks for dependency cycles between modules.
check_cycles:
	cargo-cycles

check_linter:
	python3 ./scripts/lint.py

check_pub_change_intentional:
	if [ "$$(cargo public-api diff latest -p egui_plot -sss --deny all)" != "" ]; then \
		echo "--------------------------------"; \
		echo "Public API changes detected, please confirm this is intentional by typing 'y'"; \
		echo "--------------------------------"; \
		cargo public-api diff latest -p egui_plot -sss; \
		read -p "Continue? (y/n): " confirm; \
		if [ "$$confirm" != "y" ]; then \
			echo "Aborting..."; \
			exit 1; \
		fi; \
	fi

# Checks for unfinished work.
checks_no_unfinished: check_todos_have_issues check_no_fixme check_no_unimplemented

check_git_clean_status:
	# Check there are no uncommitted files, if there are, exit
	git status | grep -q 'nothing to commit' || (echo "There are uncommitted files, please commit or stash them" &&  git status && exit 1)

# Checks that the current branch is not main and has a valid name in the format of xyz/abc
check_branch_name:
	@CURRENT_BRANCH=$$(git rev-parse --abbrev-ref HEAD); \
	if [ "$$CURRENT_BRANCH" = "main" ]; then \
		echo "Current branch is '$$CURRENT_BRANCH'. Please switch to a feature branch."; \
		exit 1; \
	elif ! echo "$$CURRENT_BRANCH" | grep -qE '^[a-z0-9._-]+/[a-z0-9._-]+$$'; then \
		echo "Branch name '$$CURRENT_BRANCH' is not in the format of xyz/abc."; \
		exit 1; \
	else \
		echo "On branch '$$CURRENT_BRANCH' - OK to proceed."; \
	fi

check_todos_have_issues:
	# The code should not have TODOs without issue tracking. Format that must be followed is:
	# TODO(#<issue_number>): <description>
	git ls-files | grep -P '\.rs$$' | xargs grep -Pi 'TODO(?!\(#\d+\): \w+)' || exit 0; exit 1

check_no_fixme:
	# The code should not have FIXME comments
	git ls-files | grep -P '\.rs$$' | xargs grep -Pi '(?:#|//)\s*FIXME' || exit 0; exit 1

check_no_unimplemented:
	# The code should not have unimplemented functions
	git ls-files | grep -P '\.rs$$' | xargs grep -Pi 'todo!|unimplemented!' || exit 0; exit 1

check_no_commented_out_code:
	# There should not be commented out code
	git ls-files | grep -P '\.rs$$' | xargs grep -Pi '^\s*//(?!/|\!)\S' || exit 0; exit 1

check_shear:
	cargo shear

check_deny:
	cargo deny check

clean:
	cargo clean
	trunk clean

# ------------------------------- dependencies ---------------------------------

deps-cargo-binstall:
	@if [ ! -f /usr/bin/cargo-binstall ]; then \
		wget https://github.com/cargo-bins/cargo-binstall/releases/download/v1.14.1/cargo-binstall-x86_64-unknown-linux-gnu.tgz; \
		tar -C /tmp -xzf cargo-binstall-x86_64-unknown-linux-gnu.tgz; \
		rm cargo-binstall-x86_64-unknown-linux-gnu.tgz; \
		mv /tmp/cargo-binstall /usr/bin/cargo-binstall; \
	else \
			echo "cargo-binstall already installed, skipping"; \
	fi

deps-cargo: deps-cargo-binstall
	cargo-binstall --locked -y $(CARGO_TOOLS)

# Alternative to cargo-binstall, install from source.
deps-cargo-from-src:
	cargo install --locked -j $(THREADS) $(CARGO_TOOLS)

# ------------------------------- helpers --------------------------------------

version:
	@VERSION=$$(grep -A1 '\[workspace.package\]' Cargo.toml | grep version | cut -d '"' -f2); \
	echo "egui_plot version: $$VERSION"
