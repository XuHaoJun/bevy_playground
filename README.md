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

## Android

uncomment all code in `lib.rs`

1. should turn off optimize, that cause build long time, i don't why.
2. copy main.rs to lib.rs and add `#[bevy_main]` on `fn main`

```sh
ANDROID_SDK_ROOT="/home/<user>/Android/Sdk/" ANDROID_NDK_ROOT="/home/<user>/Android/Sdk/ndk/<version>" cargo apk run
```

egui still have problem(clipboard) cause android build failed, you can fork it and apply that patch.
[Disable clipboard feature on Android](https://github.com/paulotten/egui/commit/68cdf23d93661c4c4508f8d83118eaba4055570a)

## References

1. [Extreme Bevy: Making a p2p web game with rust and rollback netcode](https://johanhelsing.studio/posts/extreme-bevy)
