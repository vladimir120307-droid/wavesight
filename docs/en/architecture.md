# Architecture Overview

This document explains what happens between the moment your body disturbs a WiFi signal and the moment a dot lights up in the dashboard.

```
┌──────────────────────────────────────────────────────────────────────┐
│                              YOUR ROOM                                │
│                                                                      │
│    ┌────────────┐                          ┌────────────┐            │
│    │  ESP32-S3  │ ◄────WiFi packets────►   │  WiFi AP   │            │
│    │   node-1   │                          │  (router)  │            │
│    │ (CSI tap)  │                          └─────┬──────┘            │
│    └─────┬──────┘                                │                   │
│          │ WebSocket (ws/wss)                    │                   │
│          ▼                                       │                   │
│    ┌─────────────────────────────────────────────┴──────┐            │
│    │   EDGE SERVER (Rust, your PC or Raspberry Pi 5)    │            │
│    │                                                    │            │
│    │   csi-ingest → dsp → fusion → inference → api      │            │
│    │                                              │     │            │
│    │                                              ▼     │            │
│    │                                    storage (SQLite + Parquet)   │
│    └────────────────────┬─────────────────────────┬─────┘            │
│                         │                         │                  │
│                         ▼                         ▼                  │
│                  ┌─────────────┐          ┌──────────────────┐       │
│                  │  Dashboard  │          │   Mobile (Flutter)│      │
│                  │ (React/3JS) │          │   iOS + Android   │      │
│                  └─────────────┘          └──────────────────┘       │
└──────────────────────────────────────────────────────────────────────┘
```

## Stage 1: WiFi → CSI

Your ESP32 nodes are configured as **passive monitors** on your existing WiFi network. Whenever a packet flies between any device and the AP, the ESP32's WiFi radio extracts the [Channel State Information](https://en.wikipedia.org/wiki/Channel_state_information) — a per-subcarrier complex-valued snapshot of the propagation environment.

Format: 64 subcarriers (HT20) or 256 subcarriers (HT40), complex int8 per subcarrier, ~100 Hz update rate.

## Stage 2: ESP32 → server (CSI ingest)

Each node:

1. Buffers CSI frames into 1 KB chunks.
2. Stamps each chunk with the PTP-synchronised local time.
3. Encrypts with ChaCha20-Poly1305 using a mesh-wide pre-shared key.
4. Pushes over WebSocket (TLS by default) to the edge server.

The server's `csi-ingest` crate runs an `axum` WebSocket handler, decrypts, decodes the protobuf frame, and pushes onto a per-node Tokio channel.

## Stage 3: DSP pipeline

`dsp` crate transforms raw CSI into stable, model-friendly representations:

1. **Calibration** — remove constant per-subcarrier phase / amplitude offsets.
2. **Filtering** — low-pass to denoise, bandpass for breath / heart rate bands.
3. **FFT / STFT** — short-time Fourier transform on amplitude streams.
4. **Doppler estimation** — phase-difference-of-arrival between consecutive frames → radial velocity per subcarrier.
5. **Background subtraction** — subtract the calibrated empty-room clutter map.

Output: a `DspFrame` carrying amplitude, phase, doppler, and metadata.

## Stage 4: Multi-node fusion

`fusion` crate takes time-aligned `DspFrame`s from N nodes and runs a multi-source Extended Kalman Filter:

- State: per-tracked-entity position, velocity, motion class.
- Observation: signal energy direction + UWB AoA + BLE RSSI (when available).
- Process noise tuned per entity class.

Output: a `FusedScene` containing zero or more tracked entities with confidence intervals.

## Stage 5: Inference

`inference` crate runs Candle (or ONNX Runtime, behind a feature flag) on the fused scene to produce:

- Presence (binary + confidence)
- Pose (17 keypoints + per-keypoint visibility)
- Vital signs (HR, BR with confidence bands)
- Activity (sit/stand/walk/lie/fall)
- Sleep stage (Wake/Light/Deep/REM)

All heads share a common 128-d encoder; only the heads are task-specific. Models are versioned and live under `server/models/`.

## Stage 6: API & dashboard

`api` crate exposes:

- **REST**: `/api/v1/scene`, `/api/v1/vitals`, etc., for occasional polling.
- **WebSocket**: `/api/v1/stream` for live updates.
- **gRPC**: `wavesight.v1.SceneService` for power users.

The dashboard subscribes to `/api/v1/stream` and renders the scene in 3D using React Three Fiber.

## Where data lives

- **Raw CSI**: 24-hour rolling buffer in SQLite, then aggregated.
- **Aggregated derivatives**: indefinite in Parquet under `~/.wavesight/data/`.
- **Models**: `server/models/`, versioned via `semver`.
- **Configuration**: TOML under `~/.config/wavesight/config.toml`.

## What is NOT in the architecture

- No cloud round-trip in the default deployment.
- No persistent video.
- No identity inference beyond what the user explicitly enables (e.g. "this is mom's room").

For implementation details, see the [ADRs](../adr/).
