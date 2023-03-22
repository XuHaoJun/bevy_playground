# bevy_playground

## Online Demo

[https://xuhaojun.github.io/bevy_playground/](https://xuhaojun.github.io/bevy_playground/)

## Build wasm

```sh
cargo build --release --target wasm32-unknown-unknown
~/.cargo/bin/wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/bevy_playground.wasm
```
