# wasi preview 2 exmeriments

## Requirements

- rust nightly
- `rustup target add wasm32-wasip2`
- UV (`cargo install --git https://github.com/astral-sh/uv uv`)

## Run

```bash
cargo build --release --target wasm32-wasip2

cp target/wasm32-wasip2/release/ww.wasm ww.wasm

uv run main.py
```