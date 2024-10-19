# rs-line-count
stdin(text file) -> stdout(number of lines)

## Benchmark

- input: an index file(enwiki-20240901-pages-articles-multistream-index.txt)
- os: macOS Sequoia
- processor: M3 Max
- command(native): `cat path/to/index.txt | path/to/exe`
- command(wasi): `cat path/to/index.txt | path/to/runtime $opts path/to/wasm`

| runtime        | elapsed | rate           | score(vs wc) |
|:--------------:|:-------:|:--------------:|:------------:|
| wazero(simd)   | 0.310   | 77 M lines / s | 3.3x         |
| wasmer(simd)   | 0.315   | 76 M lines / s | 3.2x         |
| (native)       | 0.364   | 66 M lines / s | 2.8x         |
| wc             | 1.022   | 23 M lines / s | 1.0x         |
| wasmtime(simd) | 6.458   |  4 M lines / s | 0.2x         |
