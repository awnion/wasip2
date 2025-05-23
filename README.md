# wasi preview 2 exmeriments

## Requirements

- rust nightly
- `rustup target add wasm32-wasip2`
- UV (`cargo install --git https://github.com/astral-sh/uv uv`)

## Run

```bash
cargo build -p adder_wasm --release --target wasm32-wasip2

cp target/wasm32-wasip2/release/adder_wasm.wasm adder.wasm

uv run main.py
```

## Build python wasm

```bash
cd wit
uv run componentize-py --wit-path adder.wit --world adder componentize guest-adder -o guest_adder_py.wasm

cargo run --bin example --release
```

## See

- [WIT Design Documentation](https://component-model.bytecodealliance.org/design/wit.html)
- [Complete Guide to WASI Preview 2 for Rust/Python Programmers](https://ideas.reify.ing/en/blog/complete-guide-to-wasip2-for-rust-python-programmers/)
