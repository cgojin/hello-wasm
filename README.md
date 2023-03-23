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

## Publishing package

```sh
# https://www.npmjs.com/signup
npm adduser

cd pkg
# publish the package to npmjs.com
npm publish --access=public
```

## Running example (webpack)

```sh
cd site-webpack

# Installing dependencies first
npm install

# Start http server
npm start

# Building
npm run build
```

## References

- [Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)
- [wasm-pack](https://github.com/rustwasm/wasm-pack)
- [Webpack Guides](https://webpack.js.org/guides/)
