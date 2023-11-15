# WASM on cloud

```bash
docker run --rm --name=core-api-wasm \
  --runtime=io.containerd.wasmedge.v1 \
  --platform=wasi/wasm32 \
  docker.io/ogioldat/core-api-wasm:latest
```