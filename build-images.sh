# !/bin/zsh
# cargo build --target wasm32-wasi --release
cd ./apps/core-api
docker buildx build \
--platform wasi/wasm32 \
-t docker.io/ogioldat/core-api-wasm:latest \
-f wasm.Dockerfile \
--provenance=false .
docker push docker.io/ogioldat/core-api-wasm:latest   

# docker build -t docker.io/ogioldat/core-api:latest -f Dockerfile .
# docker push docker.io/ogioldat/core-api:latest 
