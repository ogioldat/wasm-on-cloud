FROM scratch
COPY spin.toml /spin.toml
COPY target/wasm32-wasi/release/cloud_function.wasm /target/wasm32-wasi/release/cloud_function.wasm
ENTRYPOINT ["/spin.toml"]