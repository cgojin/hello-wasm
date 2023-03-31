# rust-on-browser

Running Rust on Browser, JavaScript call Rust, Rust to WebAssembly (WASM) example.

## Building package

```sh
# Add wasm32-unknown-unknown target, first only
rustup target add wasm32-unknown-unknown

# Install wasm-pack, first only
cargo install wasm-pack

# Build bundler (npm) package
wasm-pack build
# or
wasm-pack build --out-dir pkg --target bundler

# Build package with scope
wasm-pack build --scope cgojin

# Build nodejs package
wasm-pack build --out-dir pkg-nodejs --target nodejs
```

## Publishing package

```sh
# https://www.npmjs.com/signup
npm adduser

cd pkg
# publish the package to npmjs.com
npm publish --access=public
```

## Running wasm in JavaScript (vite)

```sh
cd site-vite

# Installing dependencies first
npm install

# Start http server
npm run dev

# Building
npm run build
```

## Running wasm in JavaScript (webpack)

```sh
cd site-webpack

# Installing dependencies first
npm install

# Start http server
npm start

# Building
npm run build
```

## Running wasm in Node.js

```sh
cd nodejs
node main.js
```

## Show wasm export/import methods

```sh
# Install wasm-tools, first only
cargo install wasm-tools

# Show export methods
wasm-tools print target/wasm32-unknown-unknown/release/rust_on_browser.wasm | grep "export"
wasm-tools print pkg/rust_on_browser_bg.wasm | grep "export"
    (export "memory" (memory 0))
    (export "greet" (func 28))
    (export "__wbindgen_malloc" (func 43))
    (export "__wbindgen_realloc" (func 46))

# Show import methods
wasm-tools print target/wasm32-unknown-unknown/release/rust_on_browser.wasm | grep "import"
wasm-tools print pkg/rust_on_browser_bg.wasm | grep "import"
    (import "./rust_on_browser_bg.js" "__wbg_alert_d0dac0ef1b2f8911" (func (;0;) (type 0)))

cargo build --lib --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir pkg2 --target web ./target/wasm32-unknown-unknown/release/rust_on_browser.wasm
```

## References

- [Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)
- [wasm-pack](https://github.com/rustwasm/wasm-pack)
- [Webpack Guides](https://webpack.js.org/guides/)
