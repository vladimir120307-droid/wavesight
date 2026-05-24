# WaveSight Roadmap

> _Last updated: 2026-05-24 · See also [README.md](README.md) and [docs/adr/](docs/adr/)._

The roadmap is intentionally long. We are building a serious open-source platform, not a tech demo. Each milestone (Mx) ends with a tagged release, a video demonstration and a reproducible benchmark run.

## Status legend

- ☐ planned
- ◐ in progress
- ☑ shipped (tagged release)

---

## M0 — Foundation ◐

_Goal: anyone can read the repo, understand what we are building, and run `cargo check` clean._

- ☑ Monorepo layout (firmware / server / dashboard / mobile / training / eval / integrations / examples / docs)
- ☑ Bilingual README (EN + RU) with honest comparison vs RuView
- ☑ MIT license, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY
- ☑ ADR-001..010 — foundational decisions
- ☑ Cargo workspace skeleton
- ☑ GitHub Actions CI (build, lint, test, security audit)
- ☑ Issue / PR templates, editorconfig, pre-commit
- ☐ Discord server
- ☐ Logo + favicon
- ☐ docs.wavesight.dev landing page

## M1 — First photon ☐

_Goal: one ESP32-S3 streams real CSI to the Rust server and the dashboard shows a moving line when you walk past._

- ☐ ESP32-S3 firmware (`firmware/esp32-csi-node`): WiFi join, CSI capture, NTP time-sync, WebSocket upload
- ☐ `server/crates/csi-ingest`: WebSocket + gRPC ingestion endpoints
- ☐ `server/crates/dsp`: FFT, CFR filtering, per-subcarrier amplitude/phase extraction
- ☐ `server/crates/storage`: SQLite + Parquet timeseries
- ☐ Minimal React dashboard: live spectrogram + binary presence indicator
- ☐ End-to-end test on real hardware
- ☐ YouTube demo video: walk in front of a single node
- ☐ Tagged release `v0.1.0`

## M2 — Mesh & fusion ☐

_Goal: 3+ nodes form a self-healing ESP-MESH, share PTP-aligned CSI, and the server fuses them into a single coherent stream._

- ☐ ESP-MESH topology with automatic root election
- ☐ IEEE 1588 PTP time-sync, target ≤ 1 μs error across the mesh
- ☐ `server/crates/fusion`: multi-node Kalman filter
- ☐ Time-aligned ring buffer per node
- ☐ Background subtraction / clutter map
- ☐ 2D presence heatmap in dashboard
- ☐ `eval/`: first reproducible benchmark — presence precision/recall in two rooms
- ☐ Release `v0.2.0`

## M3 — Vitals ☐

_Goal: detect heart rate and breathing within ±2 BPM of a Polar H10 reference._

- ☐ Doppler estimation in `dsp`
- ☐ Vitals model in `inference` (1D CNN, Candle)
- ☐ Reference dataset capture (10+ subjects, Polar H10 ground truth)
- ☐ Dashboard vitals chart with confidence bands
- ☐ Honest report: per-distance MAE, per-subject scatter
- ☐ Release `v0.3.0`

## M4 — Pose & fall ☐

_Goal: 17-keypoint pose at ≥ 25 % PCK@20 with camera ground-truth. EldGuard MVP catches 90 % of staged falls with ≤ 1 false positive/day._

- ☐ Pose model (transformer head over fused CSI embedding)
- ☐ Fall detection model (specialized, latency-optimised)
- ☐ `examples/eldguard`: ready-to-deploy vertical app
- ☐ Mobile push notifications via Flutter app
- ☐ Family-sharing access model
- ☐ Release `v0.4.0`

## M5 — Sleep & smart-home ☐

_Goal: SleepWave produces a 4-stage sleep hypnogram comparable to Withings Sleep. PresenceOS drives Home Assistant heating/lighting based on per-room occupancy._

- ☐ Sleep staging model (Wake / Light / Deep / REM)
- ☐ `examples/sleepwave`: nightly report PDF + dashboard panel
- ☐ Per-room occupancy with sub-second latency
- ☐ Home Assistant integration via HACS (`integrations/home-assistant/wavesight`)
- ☐ Matter bridge prototype
- ☐ Release `v0.5.0`

## M6 — UWB & 3D ☐

_Goal: DWM3000 anchors provide true angle-of-arrival; the dashboard renders a Three.js room with live human figures._

- ☐ DWM3000 firmware (TWR + AoA)
- ☐ UWB fusion into `fusion` crate
- ☐ Three.js dashboard with floor-plan upload + live figures
- ☐ VR preview (WebXR)
- ☐ Release `v0.6.0`

## M7 — Honest 1.0 ☐

_Goal: tagged 1.0 with a published benchmark paper-style report and a public dataset on HuggingFace._

- ☐ 50+ hour public dataset (consent-cleared)
- ☐ Benchmark CI gating every release
- ☐ `wavesight bench` CLI subcommand
- ☐ Reference report PDF in `eval/REPORT_v1.0.pdf`
- ☐ HackerNews + Habr launch posts
- ☐ Release `v1.0.0` 🎉

---

## Beyond 1.0

- mmWave radar fusion (XeThru / Texas Instruments IWR6843)
- Bluetooth LE RSSI / channel sounding fusion
- Local LLM voice assistant overlay (Whisper + Llama 3.1)
- Differential-privacy federated learning across consenting deployments
- Plugin system (Wasm) for community-built models
- Certification path: CE marking + FDA Class II (EldGuard medical claim)
