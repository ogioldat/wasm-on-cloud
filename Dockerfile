FROM scratch
COPY ./spin.toml ./spin.toml
COPY ./target/wasm32-wasi/release/restaurant.wasm ./target/wasm32-wasi/release/restaurant.wasm
COPY ./menu-api/target/wasm32-wasi/release/menu_api.wasm ./menu-api/target/wasm32-wasi/release/menu_api.wasm
