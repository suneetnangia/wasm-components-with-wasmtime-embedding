#! /bin/bash

.PHONY: all
all: build_component_release build_host_release run_host
go: build_module_release build_go_host_release run_go_host

.PHONY: build_component_release
build_component_release:
	cargo component build --release --manifest-path components/transformer/Cargo.toml && \
	cargo component build --release --manifest-path components/range_validator/Cargo.toml && \
	wasm-tools component wit target/wasm32-wasi/release/transformer.wasm && \
	wasm-tools component wit target/wasm32-wasi/release/rangevalidator.wasm && \
	wasm-tools compose target/wasm32-wasi/release/transformer.wasm -d target/wasm32-wasi/release/rangevalidator.wasm -o target/wasm32-wasi/release/composedtransformer.wasm

.PHONY: build_module_release
build_module_release:
	cargo build --release --target wasm32-wasi --manifest-path modules/wasm/Cargo.toml

.PHONY: build_host_release
build_host_release:
	cargo build --release

.PHONY: build_go_host_release
build_go_host_release:
	cd src/go && \
	go build && \
	cd ../..

.PHONY: run_host
run_host:
	cargo run

.PHONY: run_go_host
run_go_host:
	cd src/go && \
	./go && \
	cd ../..

.PHONY: clean
clean:
	cargo clean

.PHONY: clean
clean_go:
	cd src/go && \
	go clean && \
	cd ../..

