<div align="center">

# WaveSight

**Honest open-source WiFi & UWB sensing platform вЂ” see motion, breath and presence without cameras, without cloud, without hype.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/built%20with-Rust-orange.svg)](https://www.rust-lang.org)
[![ESP-IDF](https://img.shields.io/badge/firmware-ESP--IDF%20v5-red.svg)](https://idf.espressif.com)
[![Docs](https://img.shields.io/badge/docs-EN%20%7C%20RU-blue.svg)](docs/)
[![Discord](https://img.shields.io/badge/chat-Discord-7289DA.svg)](#community)

<sub>English В· [Р СѓСЃСЃРєРёР№](#СЂСѓСЃСЃРєР°СЏ-РІРµСЂСЃРёСЏ)</sub>

</div>

---

## What is WaveSight?

WaveSight turns ordinary WiFi and UWB radio signals into spatial intelligence: who is in the room, are they breathing, did they fall, are they asleep. It does this **on a $20 ESP32 mesh**, **on the edge**, and **without ever shipping a frame of video anywhere**.

We built WaveSight because the existing public projects (RuView, several academic toolkits) over-promise on the marketing side and under-deliver on physics. Single-antenna ESP32 cannot magically reconstruct a MIMO array. So we did three things differently:

1. **We added real diversity** вЂ” ESP32-C5 / C6 nodes (WiFi 6 / 6E, true 2-stream MIMO) plus optional UWB anchors (DWM3000, real angle-of-arrival).
2. **We refuse to ship numbers we cannot verify** вЂ” every shipped metric has a ground-truth pipeline, error bars, and a YouTube video reproducing it.
3. **We ship three production verticals out of the box** вЂ” `EldGuard` (fall detection), `SleepWave` (sleep staging) and `PresenceOS` (occupancy for smart-home).

> WaveSight is in **alpha**. We are documenting honestly what works, what is on the roadmap, and what is research-grade. See [ROADMAP.md](ROADMAP.md).

---

## At a glance

```
                  в”Њв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”ђ
                  в”‚                       WaveSight                       в”‚
                  в”‚                                                      в”‚
   ESP32-S3 в”Ђв”ђ    в”‚   в”Њв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”ђ   в”Њв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”ђ   в”Њв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”ђ  в”‚
   ESP32-C5 в”Ђв”јв”Ђв”Ђв”Ђв–єв”‚   в”‚ csi-     в”‚в”Ђв”Ђв–єв”‚ dsp +    в”‚в”Ђв”Ђв–єв”‚ inference       в”‚  в”‚
   DWM3000  в”Ђв”    в”‚   в”‚ ingest   в”‚   в”‚ fusion   в”‚   в”‚ (Candle / ONNX) в”‚  в”‚
   (mesh)         в”‚   в””в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”   в””в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”   в””в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”¬в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”  в”‚
                  в”‚                                          в–ј           в”‚
                  в”‚   в”Њв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”ђ  в”‚
                  в”‚   в”‚  api  В·  storage  В·  cli  В·  honest-bench     в”‚  в”‚
                  в”‚   в””в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”  в”‚
                  в”‚                          в”‚                           в”‚
                  в”‚       в”Њв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”јв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”ђ        в”‚
                  в”‚       в–ј                  в–ј                  в–ј        в”‚
                  в”‚  Dashboard 3D       Mobile (Flutter)   Home Assistantв”‚
                  в”‚  (React + Three)     iOS В· Android      Matter В· MQTTв”‚
                  в””в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”
```

---

## Features

- **Multi-node WiFi CSI sensing** with sub-microsecond PTP time-sync between ESP32 nodes.
- **UWB sensor fusion** (DWM3000) for true angle-of-arrival вЂ” solves the single-antenna problem RuView hand-waves away.
- **Presence, pose, vitals, fall, sleep, activity** вЂ” one platform, six trained heads, all running on-edge.
- **3D room reconstruction** in the dashboard via Three.js вЂ” see people as figures inside a model of your floor plan.
- **Honest Mode**: every prediction shows a confidence interval. If we are unsure, we say so.
- **Reproducible benchmarks**: `wavesight bench` runs the same scenarios on your hardware against published reference numbers.
- **100% local-first**: no cloud, no telemetry by default, no account.
- **Signed firmware & measurements**: SLSA-aligned supply chain attestation.
- **Native Home Assistant, MQTT, Matter, HomeKit (via Matter), Telegram** integrations.
- **Cross-platform mobile app** (Flutter) for live monitoring and fall alerts.

---

## Hardware tiers

| Tier      | Bill of materials                                              | Approx. cost | What you get                                |
| --------- | -------------------------------------------------------------- | ------------ | ------------------------------------------- |
| **Entry**    | 2 Г— ESP32-S3                                                | $20          | Presence + simple motion                    |
| **Standard** | 3 Г— ESP32-C5 + 1 Г— DWM3000                                  | $80          | Pose, vitals, fall detection                |
| **Pro**      | 6 Г— ESP32-C5 + 3 Г— DWM3000 + Raspberry Pi 5                 | $250         | Multi-room 3D, sleep staging, full fusion   |
| **Research** | + Intel AX210 / Nordic nRF7002 reference NIC + RGB-D camera | $400+        | Ground-truth calibration & paper-grade eval |

Detailed build guides live in [`docs/hardware/`](docs/hardware/).

---

## Honest comparison vs RuView

| Aspect                     | RuView                          | **WaveSight**                                      |
| -------------------------- | ------------------------------- | ----------------------------------------------- |
| Hardware                   | ESP32-S3 only (single antenna)  | **+ ESP32-C5/C6 (MIMO) + UWB DWM3000 (true AoA)** |
| Through-wall demos         | Claimed, no public reproducible | **Public video + dataset for every claim**       |
| Pose accuracy (PCK@20)     | ~2.5% camera-free               | **Target в‰Ґ 25% вЂ” published with error bars**     |
| Setup hardware             | Cognitum Seed (~$140) required  | **Raspberry Pi 5 or any x86 PC**                 |
| Confidence intervals in UI | No                              | **Yes вЂ” Honest Mode by default**                 |
| Vertical apps              | Toolkit                         | **3 production-ready: EldGuard, SleepWave, PresenceOS** |
| Mobile app                 | None                            | **Flutter app: iOS + Android**                   |
| Languages                  | EN docs only                    | **EN + RU full parity**                          |
| Mesh time-sync             | NTP-grade                       | **IEEE 1588 PTP, sub-Ојs**                        |

The intent of this table is not to dunk on RuView вЂ” it's to make explicit the technical trade-offs we are choosing differently.

---

## Quick start

> Full step-by-step guide: [`docs/en/getting-started.md`](docs/en/getting-started.md) В· [`docs/ru/Р±С‹СЃС‚СЂС‹Р№-СЃС‚Р°СЂС‚.md`](docs/ru/Р±С‹СЃС‚СЂС‹Р№-СЃС‚Р°СЂС‚.md)

Prerequisites: Rust 1.78+, ESP-IDF 5.2+, Node 20+, Python 3.11+, two or more ESP32-S3 / C5 boards.

```bash
# 1. clone
git clone https://github.com/vladimir120307-droid/wavesight.git
cd wavesight

# 2. flash firmware on each ESP32 node
cd firmware/esp32-csi-node
idf.py set-target esp32s3
idf.py -p /dev/ttyUSB0 flash monitor

# 3. start the edge server
cd ../../server
cargo run --release --bin wavesight -- serve

# 4. start the dashboard
cd ../dashboard
pnpm install && pnpm dev
# open http://localhost:5173
```

You should see live CSI streams and a presence indicator within 60 seconds of node power-on.

---

## Project layout

```
wavesight/
в”њв”Ђв”Ђ firmware/        # ESP32 / UWB firmware (Rust + C, ESP-IDF v5)
в”њв”Ђв”Ђ server/          # Rust workspace: ingest, dsp, fusion, inference, api, cli
в”њв”Ђв”Ђ dashboard/       # React + Vite + Three.js (TypeScript)
в”њв”Ђв”Ђ mobile/          # Flutter iOS/Android app
в”њв”Ђв”Ђ training/        # PyTorch training pipelines + datasets
в”њв”Ђв”Ђ eval/            # Honest benchmarks + ground-truth tooling
в”њв”Ђв”Ђ integrations/    # Home Assistant, Matter, MQTT, Telegram
в”њв”Ђв”Ђ examples/        # EldGuard, SleepWave, PresenceOS vertical demos
в”њв”Ђв”Ђ docs/            # EN + RU documentation, ADRs, hardware guides
в””в”Ђв”Ђ scripts/         # Build, release, dev helpers
```

---

## Roadmap

See [ROADMAP.md](ROADMAP.md) for the full multi-phase plan. Headline milestones:

- **M0 вЂ” Foundation** (this commit): repo, CI, docs skeleton, ADRs.
- **M1 вЂ” First photon**: ESP32-S3 streams CSI to server, presence works.
- **M2 вЂ” Mesh & fusion**: PTP sync, multi-node Kalman fusion.
- **M3 вЂ” Vitals**: HR + breathing within В±2 BPM of reference.
- **M4 вЂ” Pose & fall**: 17-keypoint pose, EldGuard MVP.
- **M5 вЂ” Sleep & smart-home**: SleepWave + PresenceOS, Home Assistant integration.
- **M6 вЂ” Honest 1.0**: full benchmark suite, public dataset, video demos, launch.

---

## Documentation

| | EN | RU |
|---|---|---|
| Getting started | [getting-started.md](docs/en/getting-started.md) | [Р±С‹СЃС‚СЂС‹Р№-СЃС‚Р°СЂС‚.md](docs/ru/Р±С‹СЃС‚СЂС‹Р№-СЃС‚Р°СЂС‚.md) |
| Architecture | [architecture.md](docs/en/architecture.md) | [Р°СЂС…РёС‚РµРєС‚СѓСЂР°.md](docs/ru/Р°СЂС…РёС‚РµРєС‚СѓСЂР°.md) |
| Hardware guide | [hardware/](docs/hardware/) | [hardware/](docs/hardware/) |
| ADRs | [docs/adr/](docs/adr/) | вЂ” |
| Honest benchmarks | [eval/](eval/) | вЂ” |

---

## Contributing

We welcome contributions of every shape вЂ” hardware tests, dataset captures, model training, dashboard polish, translations. Start with [CONTRIBUTING.md](CONTRIBUTING.md), pick a `good-first-issue` label, and say hello in Discussions.

This is a public project, but it is also a personal mission of the maintainer to ship something genuinely useful in this space вЂ” please be patient with review cadence.

---

## Security

We take RF-sensing seriously. WaveSight could plausibly be misused to track people without consent. Please read [SECURITY.md](SECURITY.md) for our threat model, responsible-disclosure policy and the consent guidelines we ship with the dashboard.

---

## License

MIT вЂ” see [LICENSE](LICENSE). Copyright В© 2026 Cyber_Lord (Vladimir120307@gmail.com).

---
<a id="СЂСѓСЃСЃРєР°СЏ-РІРµСЂСЃРёСЏ"></a>

# WaveSight (РїРѕ-СЂСѓСЃСЃРєРё)

**Р§РµСЃС‚РЅР°СЏ open-source РїР»Р°С‚С„РѕСЂРјР° РґР»СЏ WiFi Рё UWB-СЃРµРЅСЃРёРЅРіР°. Р’РёРґРёРј РґРІРёР¶РµРЅРёРµ, РґС‹С…Р°РЅРёРµ Рё РїСЂРёСЃСѓС‚СЃС‚РІРёРµ вЂ” Р±РµР· РєР°РјРµСЂ, Р±РµР· РѕР±Р»Р°РєР°, Р±РµР· С…Р°Р№РїР°.**

## Р§С‚Рѕ СЌС‚Рѕ

WaveSight РїСЂРµРІСЂР°С‰Р°РµС‚ РѕР±С‹С‡РЅС‹Рµ WiFi Рё UWB СЂР°РґРёРѕСЃРёРіРЅР°Р»С‹ РІ РїСЂРѕСЃС‚СЂР°РЅСЃС‚РІРµРЅРЅС‹Р№ РёРЅС‚РµР»Р»РµРєС‚: РєС‚Рѕ РІ РєРѕРјРЅР°С‚Рµ, РґС‹С€РёС‚ Р»Рё С‡РµР»РѕРІРµРє, СѓРїР°Р» Р»Рё РѕРЅ, СЃРїРёС‚ Р»Рё. Р’СЃС‘ СЌС‚Рѕ СЂР°Р±РѕС‚Р°РµС‚ **РЅР° ESP32-РјРµС€Рµ Р·Р° $20**, **Р»РѕРєР°Р»СЊРЅРѕ РЅР° СѓСЃС‚СЂРѕР№СЃС‚РІРµ**, Рё **Р±РµР· РѕС‚РїСЂР°РІРєРё РµРґРёРЅРѕРіРѕ РєР°РґСЂР° РІРёРґРµРѕ РєСѓРґР° Р±С‹ С‚Рѕ РЅРё Р±С‹Р»Рѕ**.

РњС‹ РїРѕСЃС‚СЂРѕРёР»Рё WaveSight, РїРѕС‚РѕРјСѓ С‡С‚Рѕ СЃСѓС‰РµСЃС‚РІСѓСЋС‰РёРµ РїСѓР±Р»РёС‡РЅС‹Рµ РїСЂРѕРµРєС‚С‹ (RuView, СЂСЏРґ Р°РєР°РґРµРјРёС‡РµСЃРєРёС… С‚СѓР»РєРёС‚РѕРІ) РјРЅРѕРіРѕ РѕР±РµС‰Р°СЋС‚ РІ РјР°СЂРєРµС‚РёРЅРіРµ Рё РјР°Р»Рѕ РїСЂРµРґСЉСЏРІР»СЏСЋС‚ РІ С„РёР·РёРєРµ. ESP32 СЃ РѕРґРЅРѕР№ Р°РЅС‚РµРЅРЅРѕР№ РЅРµ РјРѕР¶РµС‚ РјР°РіРёС‡РµСЃРєРё СЂРµРєРѕРЅСЃС‚СЂСѓРёСЂРѕРІР°С‚СЊ MIMO-РјР°СЃСЃРёРІ. РџРѕСЌС‚РѕРјСѓ РјС‹ РїРѕС€Р»Рё РґСЂСѓРіРёРј РїСѓС‚С‘Рј:

1. **Р РµР°Р»СЊРЅРѕРµ hardware-СЂР°Р·РЅРѕРѕР±СЂР°Р·РёРµ** вЂ” ESP32-C5/C6 (WiFi 6/6E, РЅР°СЃС‚РѕСЏС‰РµРµ 2-stream MIMO) РїР»СЋСЃ РѕРїС†РёРѕРЅР°Р»СЊРЅС‹Рµ UWB-СЏРєРѕСЂСЏ (DWM3000, СЂРµР°Р»СЊРЅС‹Р№ angle-of-arrival).
2. **РќРµ РїСѓР±Р»РёРєСѓРµРј С†РёС„СЂС‹, РєРѕС‚РѕСЂС‹Рµ РЅРµ РјРѕР¶РµРј РІРѕСЃРїСЂРѕРёР·РІРµСЃС‚Рё** вЂ” РєР°Р¶РґР°СЏ РјРµС‚СЂРёРєР° СЃРЅР°Р±Р¶РµРЅР° pipeline ground-truth, error bars Рё YouTube-РІРёРґРµРѕ.
3. **РЎСЂР°Р·Сѓ С‚СЂРё production-РІРµСЂС‚РёРєР°Р»Рё** вЂ” `EldGuard` (РґРµС‚РµРєС†РёСЏ РїР°РґРµРЅРёР№), `SleepWave` (СЃС‚Р°РґРёСЂРѕРІР°РЅРёРµ СЃРЅР°) Рё `PresenceOS` (occupancy РґР»СЏ СѓРјРЅРѕРіРѕ РґРѕРјР°).

> WaveSight РІ СЃС‚Р°РґРёРё **alpha**. РњС‹ С‡РµСЃС‚РЅРѕ РґРѕРєСѓРјРµРЅС‚РёСЂСѓРµРј, С‡С‚Рѕ СѓР¶Рµ СЂР°Р±РѕС‚Р°РµС‚, С‡С‚Рѕ РІ roadmap, Рё С‡С‚Рѕ вЂ” research-grade. РЎРј. [ROADMAP.md](ROADMAP.md).

## Р’РѕР·РјРѕР¶РЅРѕСЃС‚Рё

- **Multi-node WiFi CSI-СЃРµРЅСЃРёРЅРі** СЃ PTP-СЃРёРЅС…СЂРѕРЅРёР·Р°С†РёРµР№ СЃ С‚РѕС‡РЅРѕСЃС‚СЊСЋ РјРµРЅСЊС€Рµ РјРёРєСЂРѕСЃРµРєСѓРЅРґС‹.
- **UWB sensor fusion** (DWM3000) РґР»СЏ РЅР°СЃС‚РѕСЏС‰РµРіРѕ angle-of-arrival вЂ” СЂРµС€Р°РµРј РїСЂРѕР±Р»РµРјСѓ РѕРґРЅРѕР№ Р°РЅС‚РµРЅРЅС‹, РЅР° РєРѕС‚РѕСЂСѓСЋ RuView Р·Р°РєСЂС‹РІР°РµС‚ РіР»Р°Р·Р°.
- **Presence, pose, vitals, fall, sleep, activity** вЂ” РѕРґРЅР° РїР»Р°С‚С„РѕСЂРјР°, С€РµСЃС‚СЊ РѕР±СѓС‡РµРЅРЅС‹С… РіРѕР»РѕРІ, РІСЃС‘ СЂР°Р±РѕС‚Р°РµС‚ on-edge.
- **3D-СЂРµРєРѕРЅСЃС‚СЂСѓРєС†РёСЏ РєРѕРјРЅР°С‚С‹** РІ РґР°С€Р±РѕСЂРґРµ С‡РµСЂРµР· Three.js вЂ” РІРёРґРёРј Р»СЋРґРµР№ РєР°Рє С„РёРіСѓСЂРєРё РІРЅСѓС‚СЂРё РїР»Р°РЅР° РєРІР°СЂС‚РёСЂС‹.
- **Honest Mode**: РєР°Р¶РґРѕРµ РїСЂРµРґСЃРєР°Р·Р°РЅРёРµ РїРѕРєР°Р·С‹РІР°РµС‚ РґРѕРІРµСЂРёС‚РµР»СЊРЅС‹Р№ РёРЅС‚РµСЂРІР°Р». Р•СЃР»Рё РјРѕРґРµР»СЊ РЅРµ СѓРІРµСЂРµРЅР° вЂ” РїРёС€РµРј В«РЅРµ СѓРІРµСЂРµРЅР°В».
- **Р’РѕСЃРїСЂРѕРёР·РІРѕРґРёРјС‹Рµ Р±РµРЅС‡РјР°СЂРєРё**: `wavesight bench` Р·Р°РїСѓСЃРєР°РµС‚ С‚Рµ Р¶Рµ СЃС†РµРЅР°СЂРёРё РЅР° РІР°С€РµРј Р¶РµР»РµР·Рµ Рё СЃСЂР°РІРЅРёРІР°РµС‚ СЃ РѕРїСѓР±Р»РёРєРѕРІР°РЅРЅС‹РјРё С†РёС„СЂР°РјРё.
- **100% local-first**: РЅРё РѕР±Р»Р°РєР°, РЅРё С‚РµР»РµРјРµС‚СЂРёРё, РЅРё Р°РєРєР°СѓРЅС‚РѕРІ.
- **РџРѕРґРїРёСЃР°РЅРЅС‹Рµ РїСЂРѕС€РёРІРєРё Рё РёР·РјРµСЂРµРЅРёСЏ**: supply chain attestation РїРѕ РѕР±СЂР°Р·С†Сѓ SLSA.
- **РќР°С‚РёРІРЅС‹Рµ РёРЅС‚РµРіСЂР°С†РёРё Home Assistant, MQTT, Matter, HomeKit (С‡РµСЂРµР· Matter), Telegram**.
- **РљСЂРѕСЃСЃ-РїР»Р°С‚С„РѕСЂРјРµРЅРЅРѕРµ РјРѕР±РёР»СЊРЅРѕРµ РїСЂРёР»РѕР¶РµРЅРёРµ** (Flutter) РґР»СЏ Р¶РёРІРѕРіРѕ РјРѕРЅРёС‚РѕСЂРёРЅРіР° Рё push-Р°Р»РµСЂС‚РѕРІ Рѕ РїР°РґРµРЅРёСЏС….

## Hardware-СѓСЂРѕРІРЅРё

| РЈСЂРѕРІРµРЅСЊ    | РЎРѕСЃС‚Р°РІ                                              | Р¦РµРЅР°  | Р§С‚Рѕ РїРѕР»СѓС‡Р°РµРј                                          |
| ---------- | --------------------------------------------------- | ----- | ----------------------------------------------------- |
| **Entry**     | 2 Г— ESP32-S3                                      | $20   | Presence + РїСЂРѕСЃС‚Р°СЏ РґРµС‚РµРєС†РёСЏ РґРІРёР¶РµРЅРёСЏ                  |
| **Standard**  | 3 Г— ESP32-C5 + 1 Г— DWM3000                        | $80   | РџРѕР·Р°, vitals, РґРµС‚РµРєС†РёСЏ РїР°РґРµРЅРёР№                        |
| **Pro**       | 6 Г— ESP32-C5 + 3 Г— DWM3000 + Raspberry Pi 5       | $250  | 3D РїРѕ РєРѕРјРЅР°С‚Р°Рј, sleep staging, РїРѕР»РЅС‹Р№ fusion          |
| **Research**  | + Intel AX210 / nRF7002 + RGB-D РєР°РјРµСЂР°            | $400+ | Ground-truth РєР°Р»РёР±СЂРѕРІРєР°, paper-grade evaluation       |

РџРѕРґСЂРѕР±РЅС‹Рµ СЃР±РѕСЂРѕС‡РЅС‹Рµ РіР°Р№РґС‹ Р»РµР¶Р°С‚ РІ [`docs/hardware/`](docs/hardware/).

## Р‘С‹СЃС‚СЂС‹Р№ СЃС‚Р°СЂС‚

> РџРѕР»РЅС‹Р№ РїРѕС€Р°РіРѕРІС‹Р№ РіР°Р№Рґ: [`docs/ru/Р±С‹СЃС‚СЂС‹Р№-СЃС‚Р°СЂС‚.md`](docs/ru/Р±С‹СЃС‚СЂС‹Р№-СЃС‚Р°СЂС‚.md)

РўСЂРµР±РѕРІР°РЅРёСЏ: Rust 1.78+, ESP-IDF 5.2+, Node 20+, Python 3.11+, РґРІРµ Рё Р±РѕР»РµРµ РїР»Р°С‚С‹ ESP32-S3 / C5.

```bash
git clone https://github.com/vladimir120307-droid/wavesight.git
cd wavesight

cd firmware/esp32-csi-node
idf.py set-target esp32s3
idf.py -p /dev/ttyUSB0 flash monitor

cd ../../server
cargo run --release --bin wavesight -- serve

cd ../dashboard
pnpm install && pnpm dev
```

Р’ С‚РµС‡РµРЅРёРµ РјРёРЅСѓС‚С‹ РїРѕСЃР»Рµ РІРєР»СЋС‡РµРЅРёСЏ РЅРѕРґ РІС‹ РґРѕР»Р¶РЅС‹ СѓРІРёРґРµС‚СЊ live CSI-РїРѕС‚РѕРєРё Рё РёРЅРґРёРєР°С‚РѕСЂ РїСЂРёСЃСѓС‚СЃС‚РІРёСЏ.

## Р”РѕСЂРѕР¶РЅР°СЏ РєР°СЂС‚Р°

РџРѕР»РЅС‹Р№ РїР»Р°РЅ С„Р°Р· вЂ” [ROADMAP.md](ROADMAP.md). РљР»СЋС‡РµРІС‹Рµ milestones:

- **M0 вЂ” Р¤СѓРЅРґР°РјРµРЅС‚** (СЌС‚РѕС‚ РєРѕРјРјРёС‚): СЂРµРїРѕ, CI, СЃРєРµР»РµС‚ РґРѕРєРѕРІ, ADR.
- **M1 вЂ” РџРµСЂРІС‹Р№ С„РѕС‚РѕРЅ**: ESP32-S3 С€Р»С‘С‚ CSI РЅР° СЃРµСЂРІРµСЂ, presence СЂР°Р±РѕС‚Р°РµС‚.
- **M2 вЂ” Mesh Рё fusion**: PTP-СЃРёРЅС…СЂРѕРЅРёР·Р°С†РёСЏ, РјСѓР»СЊС‚РёРЅРѕРґРѕРІС‹Р№ Kalman fusion.
- **M3 вЂ” Vitals**: HR + РґС‹С…Р°РЅРёРµ РІ РїСЂРµРґРµР»Р°С… В±2 BPM РѕС‚ СЂРµС„РµСЂРµРЅСЃР°.
- **M4 вЂ” Pose Рё fall**: 17-С‚РѕС‡РµС‡РЅР°СЏ РїРѕР·Р°, MVP EldGuard.
- **M5 вЂ” Sleep Рё smart-home**: SleepWave + PresenceOS, РёРЅС‚РµРіСЂР°С†РёСЏ СЃ Home Assistant.
- **M6 вЂ” Р§РµСЃС‚РЅР°СЏ 1.0**: РїРѕР»РЅС‹Р№ benchmark suite, РїСѓР±Р»РёС‡РЅС‹Р№ РґР°С‚Р°СЃРµС‚, РІРёРґРµРѕ-РґРµРјРѕ, Р·Р°РїСѓСЃРє.

## Р”РѕРєСѓРјРµРЅС‚Р°С†РёСЏ

| | EN | RU |
|---|---|---|
| РЎС‚Р°СЂС‚ | [getting-started.md](docs/en/getting-started.md) | [Р±С‹СЃС‚СЂС‹Р№-СЃС‚Р°СЂС‚.md](docs/ru/Р±С‹СЃС‚СЂС‹Р№-СЃС‚Р°СЂС‚.md) |
| РђСЂС…РёС‚РµРєС‚СѓСЂР° | [architecture.md](docs/en/architecture.md) | [Р°СЂС…РёС‚РµРєС‚СѓСЂР°.md](docs/ru/Р°СЂС…РёС‚РµРєС‚СѓСЂР°.md) |
| Hardware | [hardware/](docs/hardware/) | [hardware/](docs/hardware/) |
| ADR | [docs/adr/](docs/adr/) | вЂ” |

## Р’РєР»Р°Рґ

Р‘СѓРґРµРј СЂР°РґС‹ РєРѕРЅС‚СЂРёР±СЊСЋС†РёСЏРј Р»СЋР±РѕРіРѕ С„РѕСЂРјР°С‚Р° вЂ” С‚РµСЃС‚С‹ РЅР° Р¶РµР»РµР·Рµ, СЃР±РѕСЂ РґР°С‚Р°СЃРµС‚РѕРІ, РѕР±СѓС‡РµРЅРёРµ РјРѕРґРµР»РµР№, РїРµСЂРµРІРѕРґ РґРѕРєСѓРјРµРЅС‚Р°С†РёРё, РґРёР·Р°Р№РЅ РґР°С€Р±РѕСЂРґР°. РќР°С‡РЅРёС‚Рµ СЃ [CONTRIBUTING.md](CONTRIBUTING.md), РІРѕР·СЊРјРёС‚Рµ issue СЃ Р»РµР№Р±Р»РѕРј `good-first-issue`, Р·Р°С…РѕРґРёС‚Рµ РІ Discussions РїРѕР·РґРѕСЂРѕРІР°С‚СЊСЃСЏ.

## Р‘РµР·РѕРїР°СЃРЅРѕСЃС‚СЊ

RF-СЃРµРЅСЃРёРЅРі вЂ” СЃРµСЂСЊС‘Р·РЅР°СЏ РІРµС‰СЊ. WaveSight С‚РµРѕСЂРµС‚РёС‡РµСЃРєРё РјРѕР¶РЅРѕ РёСЃРїРѕР»СЊР·РѕРІР°С‚СЊ РґР»СЏ СЃР»РµР¶РєРё Р±РµР· СЃРѕРіР»Р°СЃРёСЏ. РџСЂРѕС‡РёС‚Р°Р№С‚Рµ [SECURITY.md](SECURITY.md), С‚Р°Рј РѕРїРёСЃР°РЅР° РЅР°С€Р° threat model, РїРѕР»РёС‚РёРєР° responsible disclosure Рё РІСЃС‚СЂРѕРµРЅРЅС‹Рµ РІ РґР°С€Р±РѕСЂРґ РіР°Р№РґР»Р°Р№РЅС‹ РїРѕ СЃРѕРіР»Р°СЃРёСЋ.

## Р›РёС†РµРЅР·РёСЏ

MIT вЂ” СЃРј. [LICENSE](LICENSE). В© 2026 Cyber_Lord (Vladimir120307@gmail.com).
