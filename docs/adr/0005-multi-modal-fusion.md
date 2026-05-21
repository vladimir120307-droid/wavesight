# ADR-005 — Multi-modal fusion (WiFi CSI + UWB + BLE)

- **Status**: accepted
- **Date**: 2026-05-24
- **Deciders**: @vladimir120307-droid

## Context

ESP32-S3 is a single-antenna, single-RF-chain device. No amount of post-processing can recover phase diversity that the radio never observed. Approaches that compensate via dense node placement (RuView's mesh-as-MIMO claim) help in some regimes but are fundamentally limited by:

- Time-sync precision (covered in ADR-008).
- Geometry: nodes in a single plane cannot disambiguate vertical motion.
- WiFi 4 / 5 channel sampling rate (typically 100 Hz CSI per stream).

UWB radios like the DecaWave / Qorvo DWM3000 provide:

- 6 GHz centre frequency, 500 MHz bandwidth → ~10 cm range resolution.
- True time-of-flight measurements between anchor pairs.
- Angle-of-arrival on modules with multiple antennas.

WiFi 6 / 6E chips (ESP32-C5, C6) provide:

- 2x2 MIMO with real spatial diversity.
- OFDMA, allowing cleaner per-user CSI extraction.

BLE RSSI / channel sounding adds:

- Cheap, ubiquitous presence anchors via existing phones / wearables.

## Decision

WaveSight is architected as a **multi-modal fusion engine**. The `fusion` crate accepts streams from any combination of:

- WiFi CSI nodes (ESP32-S3, ESP32-C5/C6, Nexmon Pi, Intel AX210)
- UWB anchors (DWM3000)
- BLE channel sounding (post-2024 phones)
- mmWave radar (M6+ deferred — TI IWR6843, Infineon BGT60)

Fusion runs a multi-source extended Kalman filter (EKF) per tracked entity, with motion models specialised by entity class (human walking, human seated, human supine).

## Consequences

### Positive
- Resolves the single-antenna limitation honestly (more antennas, not more PR).
- Graceful degradation: with one sensor type, performance drops measurably; with three, it works robustly.
- Lets users invest as their needs grow.

### Negative
- EKF tuning per modality is non-trivial.
- Three modalities × N rooms × M scenarios = a lot of evaluation work.

### Neutral
- Entry tier (WiFi only) is still useful.

## Alternatives considered

### WiFi-only ("just like RuView")
Rejected. We don't want to inherit the physics ceiling we are critiquing.

### UWB-only
Rejected. UWB anchors don't sense vital signs or pose. Pure ranging is a smaller story.

### Sensor-fusion via deep learning end-to-end
Deferred. We will explore a learned fusion head, but the deterministic EKF gives us interpretable uncertainty for ADR-004.

## References

- DecaWave DWM3000 datasheet: https://www.qorvo.com/products/p/DWM3000
- "Pedestrian Tracking Using UWB" (Mahfouz, 2008)
- "WiFi 6 OFDMA for Sensing" (Niu, IEEE 2022)
