#! /bin/bash

.PHONY: all
all: build_component_release build_host_release run_host

.PHONY: build_component_release
build_component_release:
	cargo component build --release --manifest-path components/transformer/Cargo.toml && wasm-tools component wit target/wasm32-wasi/release/transformer.wasm

.PHONY: build_host_release
build_host_release:
	cargo build --release


.PHONY: run_host
run_host:
	cargo run

.PHONY: clean
clean:
	cargo clean
