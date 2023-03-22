# bevy_playground

## Online Demo

[https://xuhaojun.github.io/bevy_playground/](https://xuhaojun.github.io/bevy_playground/)

## Build wasm

```sh
cargo build --release --target wasm32-unknown-unknown
~/.cargo/bin/wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/bevy_playground.wasm
```

## Dev setup

rust version: rustc 1.70.0-nightly (44f518058 2023-03-20)

```sh
sudo pacman -S mold
cargo install cargo-watch
rustup target install wasm32-unknown-unknown
```

## Run

```sh
cargo run
# cargo watch -cx "run"
```

## References

1. [ Extreme Bevy: Making a p2p web game with rust and rollback netcode ](https://johanhelsing.studio/posts/extreme-bevy)
