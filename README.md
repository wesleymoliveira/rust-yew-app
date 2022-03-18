# yew-app

Rust app with WebAssembly by yew and stdweb

# Getting started

- Install [Rust]
- Install WebAssembly target
  ```
  rustup target add wasm32-unknown-unknown
  ```
- Install Trunk
  ```
  cargo install --locked trunk
  cargo install wasm-bindgen-cli
  ```
- Serve and build the app
  Go to `/crates/wasm` and run:
  ```
  trunk serve
  ```
