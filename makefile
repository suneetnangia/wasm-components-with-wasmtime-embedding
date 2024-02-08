#! /bin/bash

.PHONY: all
all: build_component_release build_host_release run_host

.PHONY: build_component_release
build_component_release:
	cargo component build --release --manifest-path components/transformer/Cargo.toml && \
	cargo component build --release --manifest-path components/range_validator/Cargo.toml && \
	wasm-tools component wit target/wasm32-wasi/release/transformer.wasm && \
	wasm-tools component wit target/wasm32-wasi/release/rangevalidator.wasm && \
	wasm-tools compose target/wasm32-wasi/release/transformer.wasm -d target/wasm32-wasi/release/rangevalidator.wasm -o target/wasm32-wasi/release/composedtransformer.wasm

.PHONY: build_host_release
build_host_release:
	cargo build --release

.PHONY: run_host
run_host:
	cargo run

.PHONY: clean
clean:
	cargo clean
