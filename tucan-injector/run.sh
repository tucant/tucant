#!/bin/sh

cargo build --color always --target=wasm32-unknown-unknown
wasm-bindgen --out-dir=wasm-bindgen ../target/wasm32-unknown-unknown/debug/tucan_injector.wasm
npm run build