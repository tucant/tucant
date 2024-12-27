#!/bin/sh

wasm-pack build --profiling

wasm2wat --output=pkg/tucant_extension_bg.wat pkg/tucant_extension_bg.wasm

twiggy top pkg/tucant_extension_bg.wasm | less

cargo llvm-lines --release --target=wasm32-unknown-unknown