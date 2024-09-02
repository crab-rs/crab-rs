# Development

Run the following command in the root of the project to start the Dioxus dev server:

## pre
```bash
# install cargo-make
cargo install cargo-make 
# install dioxus-cli
cargo install dioxus-cli
```

## run dev-sever with hot-reloading
```bash
cargo make dev_server
```

## build wasm & upload
```bash
cargo make upload_wasm
```


## run on ios simulator
```bash
cargo make run_ios_sim
```


## build ipa file for ios
```bash
cargo make build_ipa
```


## build for macos
```bash
cargo make build_macos
```

## run  macos
```bash
cargo make run_macos
```



