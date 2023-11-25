# !/bin/zsh
# cargo build --target wasm32-wasi --release
cd ./cloud-function
spin build
docker buildx build \
--platform wasi/wasm32 \
-t docker.io/ogioldat/cloud-function:latest \
-f Dockerfile \
--provenance=false .
docker push docker.io/ogioldat/cloud-function:latest

# docker build -t docker.io/ogioldat/core-api:latest -f Dockerfile .
# docker push docker.io/ogioldat/core-api:latest 