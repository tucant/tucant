#!/bin/sh

cargo build --release --color always --target=wasm32-unknown-unknown
wasm-bindgen --target web --out-dir=wasm-bindgen ../target/wasm32-unknown-unknown/release/tucant_extension.wasm
#npm run build

twiggy top ../target/wasm32-unknown-unknown/release/tucant_extension.wasm  | less

wasm-pack build --profiling

wasm2wat --output=pkg/tucant_extension_bg.wat pkg/tucant_extension_bg.wasm