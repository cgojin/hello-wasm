// const js = import("@cgojin/rust-on-browser");
const js = import("../pkg");

js.then(rust_module => {
    rust_module.greet("WebAssembly");
});
