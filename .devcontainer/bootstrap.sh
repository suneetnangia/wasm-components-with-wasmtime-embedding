# Installs Rust's wasm32-wasi target for building WebAssembly components.
rustup target add wasm32-wasi

# Installs Wasmtime, a WebAssembly runtime.
# TODO: Add wasmtime binary to path so it can be used in the terminal without opening a new one.
curl https://wasmtime.dev/install.sh -sSf | bash

# Installs Wasm Tools
cargo install cargo-component
cargo install wasm-tools

# Installs Go
rm -rf /usr/local/go
curl https://go.dev/dl/go1.22.0.linux-amd64.tar.gz -L -O
sudo tar -C /usr/local -xzf go1.22.0.linux-amd64.tar.gz
