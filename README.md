# rs-line-count
stdin(text file) -> stdout(number of lines)

## Benchmark

- input: an index file(enwiki-20240901-pages-articles-multistream-index.txt)
- command(native): `cat path/to/index.txt | path/to/exe`
- command(wasi): `cat path/to/index.txt | path/to/runtime $opts path/to/wasm`

### macOS Sequioia(M3 Max)

| runtime               | elapsed | rate           | score(vs wc) |
|:---------------------:|:-------:|:--------------:|:------------:|
| wazero 1.8.1(simd)    | 0.278   | 82 M lines / s | 3.7x         |
| wasmer 4.4.0(simd)    | 1.338   | 17 M lines / s | 0.8x         |
| (rust 1.82.0 native)  | 0.370   | 62 M lines / s | 2.8x         |
| wc                    | 1.022   | 22 M lines / s | 1.0x         |
| wasmtime 25.0.2(simd) | 6.042   |  4 M lines / s | 0.2x         |

### ubuntu jammy(i7-13700)

| runtime               | elapsed | rate           | score(vs wc) |
|:---------------------:|:-------:|:--------------:|:------------:|
| wazero 1.8.1(simd)    | 0.386   | 59 M lines / s | 0.9x         |
| wasmer 4.4.0(simd)    | 1.161   | 20 M lines / s | 0.3x         |
| (rust 1.82.0 native)  | 0.388   | 59 M lines / s | 0.9x         |
| wc                    | 0.354   | 65 M lines / s | 1.0x         |
| wasmtime 10.0.1(simd) | 0.399   | 57 M lines / s | 0.9x         |
| wasmtime 25.0.2(simd) | 5.076   |  5 M lines / s | 0.1x         |
