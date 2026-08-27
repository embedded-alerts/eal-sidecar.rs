# eal-sidecar.rs

k8s sidecar for Embedded Alerts.

Inherits [`ores-otel-sidecar`](https://github.com/ores-otel/ores-otel-sidecar.rs).
Bind with `EAL_SIDECAR_BIND` (default `127.0.0.1:9090`).

```sh
cargo run --bin eal-sidecar
```
