# WASM on cloud

```bash
docker run --rm --name=core-api-wasm \
  --runtime=io.containerd.wasmedge.v1 \
  --platform=wasi/wasm32 \
  docker.io/ogioldat/core-api-wasm:latest
```

Zalozenia

1. Klaster działa na 2 nodeach
   1. runtime docker engine
      1. NATS
      2. frontend ktory umoliwia odpalenie funkcji chmurowej
   2. runtime wasmedge
      1. funkcja chmurowa