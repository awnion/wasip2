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

## See

- [WIT Design Documentation](https://component-model.bytecodealliance.org/design/wit.html)
- [Complete Guide to WASI Preview 2 for Rust/Python Programmers](https://ideas.reify.ing/en/blog/complete-guide-to-wasip2-for-rust-python-programmers/)
