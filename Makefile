# SPDX-License-Identifier: GPL-2.0
EXAMPLE	+= counter
.PHONY: all init update check build test start run clean
all: init fmt lint check build
init:
	@cargo install cargo-web
update: init
	@cargo update
fmt:
	@rustfmt --edition 2018 --check **/*.rs
lint:
	@cargo clippy -- -D warnings
check build test start: init
	@cargo web $@ --target wasm32-unknown-unknown
run: start
run-% start-%:
	@cargo web start --target wasm32-unknown-unknown --example $*
$(EXAMPLE):
	@cargo web build --target wasm32-unknown-unknown --example $@
clean:
	@cargo clean
