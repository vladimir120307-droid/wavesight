# ADR-002 — Rust as the server-core language

- **Status**: accepted
- **Date**: 2026-05-24
- **Deciders**: @vladimir120307-droid

## Context

The server core ingests CSI frames from many nodes at multi-kHz, runs DSP, fuses streams, and serves a dashboard. It needs:

- Predictable low-latency processing (CSI windows are typically 10–100 ms; we want end-to-end under 50 ms).
- Memory safety — we are processing untrusted network frames.
- Cross-platform binaries — Linux, macOS, Windows, ARM, x86.
- Solid async + concurrency story.
- Strong DSP and ML inference ecosystem.

## Decision

The server core is written in **Rust**. Specifically:

- `tokio` for async runtime
- `axum` for HTTP / WebSocket
- `tonic` for gRPC
- `candle` for on-edge ML inference (PyTorch-compatible)
- `ndarray` + `rustfft` for DSP
- `serde` + `rkyv` for serialization

## Consequences

### Positive
- Memory safety eliminates a large class of CSI-parser RCE risks.
- Single static binary per platform; trivial deployment.
- Excellent cross-compilation story (including ESP32 with `esp-rs`).
- Candle ML stack covers most needs without Python in the hot path.

### Negative
- Smaller talent pool than Python or Go among ML practitioners.
- Build times can be slow; we mitigate with `sccache` and split crates.
- Some bleeding-edge ML ops are only in PyTorch; we keep Python training for that, distill to Candle for inference.

### Neutral
- Python is still first-class for `training/` — Rust just runs inference.

## Alternatives considered

### Go
Rejected. Weaker DSP ecosystem and no PyTorch-compatible inference story comparable to Candle.

### Python (FastAPI + ONNX Runtime)
Rejected. GIL would bottleneck multi-node ingest at our target throughput, and packaging on edge devices is fragile.

### C++
Rejected. Memory-safety risk on a network-facing service is unacceptable.

## References

- Candle: https://github.com/huggingface/candle
- `esp-rs` Rust on ESP32: https://github.com/esp-rs
