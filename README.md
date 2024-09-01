# Development

Run the following command in the root of the project to start the Dioxus dev server:

## pre
```bash
# install cargo-make
cargo install cargo-make 
# install dioxus-cli
cargo install dioxus-cli

# install targets
rustup target add aarch64-apple-darwin
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add wasm32-unknown-unknown

```

## run dev-sever with hot-reloading
```bash
cargo make dev_server
```

## build wasm & upload
```bash
cargo make upload_wasm
```
## build ipa file for ios
```bash
cargo make build_ipa
```



