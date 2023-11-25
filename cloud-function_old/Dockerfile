FROM scratch
COPY ./spin.toml .
COPY ./target/wasm32-wasi/release/cloud-function.wasm ./target/wasm32-wasi/release/cloud-function.wasm
COPY ./spin_static_fs.wasm ./spin_static_fs.wasm
COPY ./static ./static