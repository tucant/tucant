#!/bin/sh

trunk build --config ../crates/tucant-yew/Trunk.toml --features direct --dist ../../tucant-extension/dist --public-url /dist --release

#ls -lh pkg/tucant_extension_bg.wasm

#wasm-pack build --profiling

#wasm2wat --output=pkg/tucant_extension_bg.wat pkg/tucant_extension_bg.wasm

#twiggy top pkg/tucant_extension_bg.wasm | less

#cargo llvm-lines --release --target=wasm32-unknown-unknown

#cargo tree --target=wasm32-unknown-unknown

#-rw-r--r-- 1 moritz users 1,8M Dez 27 13:14 pkg/tucant_extension_bg.wasm
