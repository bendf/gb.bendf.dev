#!/bin/bash

# Rust crate build
cargo build --target=wasm32-unknown-unknown

# Build wasm-bindgen bindings
wasm-bindgen --target web --out-dir ./dist ./target/wasm32-unknown-unknown/debug/squaregb.wasm
