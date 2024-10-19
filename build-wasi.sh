#!/bin/sh

export RUSTFLAGS='-C target_feature=+simd128'
cargo \
	build \
	--target wasm32-wasip1 \
	--features wsimd4k \
	--profile release-wasi
