# Verticals

WaveSight ships three production-ready vertical applications. Each one
lives under `examples/<name>/` and consumes the same core platform.

## EldGuard — fall detection

Mobile-app push alerts when an elderly person falls. Family-shared. No
camera in the house, no cloud, no wearable on the body.

- **Audience**: adult children of ageing parents.
- **Inference head**: `fall` (latency-optimised).
- **Mobile**: Flutter app pushes notifications via APNs / FCM.
- **Telegram**: optional bridge for extended family.

Docs: [`examples/eldguard/`](../../examples/eldguard/).

## SleepWave — sleep staging

A 4-stage hypnogram (Wake / Light / Deep / REM) produced from a single
ESP32 in the bedroom. Nightly PDF report. Local-only.

- **Audience**: quantified-self users, sleep hackers.
- **Inference head**: `sleep`.
- **Comparison**: target ~70 % agreement with Withings ScanWatch.

Docs: [`examples/sleepwave/`](../../examples/sleepwave/).

## PresenceOS — Home Assistant occupancy

Per-room presence as Home Assistant `binary_sensor` entities, with
sub-second latency. Detects stationary occupants (sitting, reading) —
where PIR sensors fail.

- **Audience**: smart-home automation users.
- **Inference head**: `presence`.
- **Integration**: native HACS custom integration + MQTT discovery.

Docs: [`examples/presence-os/`](../../examples/presence-os/).

## Why three and not one

A platform is hard to evaluate without concrete use cases. Building
three verticals at once forces architectural decisions to be honest
about cross-cutting concerns (uncertainty handling, latency budgets,
authentication, mobile push). A single-vertical project tends to bake
assumptions into the platform that come back to bite later.

The verticals share:

- The same `wavesight-core` types.
- The same `csi-ingest` → `dsp` → `fusion` pipeline.
- The same dashboard.

The verticals differ in:

- Which inference head they consume.
- What downstream UI they expose (mobile push vs PDF report vs HA entity).
- Their evaluation methodology (FPR vs accuracy vs latency).
