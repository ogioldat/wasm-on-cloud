FROM scratch
COPY ./target/wasm32-wasi/release/core-api.wasm /core-api.wasm
ENTRYPOINT [ "core-api.wasm" ]