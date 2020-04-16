# SPDX-License-Identifier: GPL-2.0
EXAMPLE	+= counter
.PHONY: all init update check build test start run clean
all: init fmt lint doc check build test
init:
	@rustup target add wasm32-unknown-unknown
	@cargo install cargo-web
update: init
	@cargo update
fmt:
	@rustfmt --edition 2018 --check **/*.rs
lint:
	@cargo clippy -- -D warnings
doc:
	@cargo $@
check build test start: init
	@cargo web $@ --target wasm32-unknown-unknown
run: start
$(EXAMPLE):
	@cargo web build --target wasm32-unknown-unknown --example $@
run-% start-%:
	@cargo web start --target wasm32-unknown-unknown --example $*
clean:
	@cargo clean
