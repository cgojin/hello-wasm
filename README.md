# rust-on-browser

Running Rust on Browser, Calling Rust from Javascript, Rust to WebAssembly (WASM) example.

## Building package

```sh
# Install wasm-pack first
cargo install wasm-pack

wasm-pack build

# Building package with scope
wasm-pack build --scope cgojin
```

## References

- [Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)
- [wasm-pack](https://github.com/rustwasm/wasm-pack)
