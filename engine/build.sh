#!/bin/sh

set -ex

rustup target add wasm32-unknown-unknown
# This must match the same version as in Cargo.toml
cargo install --version 0.2.69 wasm-bindgen-cli

cargo build --target wasm32-unknown-unknown --release
rm -rf wasm
mkdir -p wasm
wasm-bindgen \
	target/wasm32-unknown-unknown/release/engine.wasm \
	--out-dir wasm \
	--target web \
	--omit-imports \
	--no-typescript \
	--debug

