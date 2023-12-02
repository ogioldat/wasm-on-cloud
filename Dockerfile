FROM scratch
COPY ./spin.toml ./spin.toml
COPY ./target/wasm32-wasi/release/restaurant.wasm ./target/wasm32-wasi/release/restaurant.wasm
COPY ./menu-api/target/wasm32-wasi/release/menu_api.wasm ./menu-api/target/wasm32-wasi/release/menu_api.wasm
COPY ./kitchen-api/target/wasm32-wasi/release/kitchen_api.wasm ./kitchen-api/target/wasm32-wasi/release/kitchen_api.wasm
COPY ./order-api/target/wasm32-wasi/release/order_api.wasm ./order-api/target/wasm32-wasi/release/order_api.wasm
