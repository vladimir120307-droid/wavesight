# WaveSight Roadmap

This roadmap is intentionally rough; we will refine as we go.

## M1 — First photon

Goal: one ESP32-S3 streams real CSI to the Rust server and the dashboard
shows a moving line when you walk past.

- ESP32-S3 firmware: WiFi join, CSI capture, WebSocket upload.
- Server: csi-ingest + dsp + minimal storage.
- Dashboard: live spectrogram + binary presence indicator.
- End-to-end test on real hardware.
- Tagged release v0.1.0.

## Later

- Mesh + multi-node fusion.
- Vital signs (HR / breathing).
- Pose + fall detection.
- Sleep staging.
- Home Assistant integration.
- Honest 1.0 release with public dataset.