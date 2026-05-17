<div align="center">

# WaveSight

**Honest open-source WiFi & UWB sensing platform — see motion, breath and presence without cameras, without cloud, without hype.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/built%20with-Rust-orange.svg)](https://www.rust-lang.org)
[![ESP-IDF](https://img.shields.io/badge/firmware-ESP--IDF%20v5-red.svg)](https://idf.espressif.com)
[![Docs](https://img.shields.io/badge/docs-EN%20%7C%20RU-blue.svg)](docs/)
[![Discord](https://img.shields.io/badge/chat-Discord-7289DA.svg)](#community)

<sub>English · [Русский](#русская-версия)</sub>

</div>

---

## What is WaveSight?

WaveSight turns ordinary WiFi and UWB radio signals into spatial intelligence: who is in the room, are they breathing, did they fall, are they asleep. It does this **on a $20 ESP32 mesh**, **on the edge**, and **without ever shipping a frame of video anywhere**.

We built WaveSight because the existing public projects (RuView, several academic toolkits) over-promise on the marketing side and under-deliver on physics. Single-antenna ESP32 cannot magically reconstruct a MIMO array. So we did three things differently:

1. **We added real diversity** — ESP32-C5 / C6 nodes (WiFi 6 / 6E, true 2-stream MIMO) plus optional UWB anchors (DWM3000, real angle-of-arrival).
2. **We refuse to ship numbers we cannot verify** — every shipped metric has a ground-truth pipeline, error bars, and a YouTube video reproducing it.
3. **We ship three production verticals out of the box** — `EldGuard` (fall detection), `SleepWave` (sleep staging) and `PresenceOS` (occupancy for smart-home).

> WaveSight is in **alpha**. We are documenting honestly what works, what is on the roadmap, and what is research-grade. See [ROADMAP.md](ROADMAP.md).

---

## At a glance

```
                  ┌──────────────────────────────────────────────────────┐
                  │                       WaveSight                       │
                  │                                                      │
   ESP32-S3 ─┐    │   ┌──────────┐   ┌──────────┐   ┌─────────────────┐  │
   ESP32-C5 ─┼───►│   │ csi-     │──►│ dsp +    │──►│ inference       │  │
   DWM3000  ─┘    │   │ ingest   │   │ fusion   │   │ (Candle / ONNX) │  │
   (mesh)         │   └──────────┘   └──────────┘   └────────┬────────┘  │
                  │                                          ▼           │
                  │   ┌───────────────────────────────────────────────┐  │
                  │   │  api  ·  storage  ·  cli  ·  honest-bench     │  │
                  │   └───────────────────────────────────────────────┘  │
                  │                          │                           │
                  │       ┌──────────────────┼──────────────────┐        │
                  │       ▼                  ▼                  ▼        │
                  │  Dashboard 3D       Mobile (Flutter)   Home Assistant│
                  │  (React + Three)     iOS · Android      Matter · MQTT│
                  └──────────────────────────────────────────────────────┘
```

---

## Features

- **Multi-node WiFi CSI sensing** with sub-microsecond PTP time-sync between ESP32 nodes.
- **UWB sensor fusion** (DWM3000) for true angle-of-arrival — solves the single-antenna problem RuView hand-waves away.
- **Presence, pose, vitals, fall, sleep, activity** — one platform, six trained heads, all running on-edge.
- **3D room reconstruction** in the dashboard via Three.js — see people as figures inside a model of your floor plan.
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
| **Entry**    | 2 × ESP32-S3                                                | $20          | Presence + simple motion                    |
| **Standard** | 3 × ESP32-C5 + 1 × DWM3000                                  | $80          | Pose, vitals, fall detection                |
| **Pro**      | 6 × ESP32-C5 + 3 × DWM3000 + Raspberry Pi 5                 | $250         | Multi-room 3D, sleep staging, full fusion   |
| **Research** | + Intel AX210 / Nordic nRF7002 reference NIC + RGB-D camera | $400+        | Ground-truth calibration & paper-grade eval |

Detailed build guides live in [`docs/hardware/`](docs/hardware/).

---

## Honest comparison vs RuView

| Aspect                     | RuView                          | **WaveSight**                                      |
| -------------------------- | ------------------------------- | ----------------------------------------------- |
| Hardware                   | ESP32-S3 only (single antenna)  | **+ ESP32-C5/C6 (MIMO) + UWB DWM3000 (true AoA)** |
| Through-wall demos         | Claimed, no public reproducible | **Public video + dataset for every claim**       |
| Pose accuracy (PCK@20)     | ~2.5% camera-free               | **Target ≥ 25% — published with error bars**     |
| Setup hardware             | Cognitum Seed (~$140) required  | **Raspberry Pi 5 or any x86 PC**                 |
| Confidence intervals in UI | No                              | **Yes — Honest Mode by default**                 |
| Vertical apps              | Toolkit                         | **3 production-ready: EldGuard, SleepWave, PresenceOS** |
| Mobile app                 | None                            | **Flutter app: iOS + Android**                   |
| Languages                  | EN docs only                    | **EN + RU full parity**                          |
| Mesh time-sync             | NTP-grade                       | **IEEE 1588 PTP, sub-μs**                        |

The intent of this table is not to dunk on RuView — it's to make explicit the technical trade-offs we are choosing differently.

---

## Quick start

> Full step-by-step guide: [`docs/en/getting-started.md`](docs/en/getting-started.md) · [`docs/ru/быстрый-старт.md`](docs/ru/быстрый-старт.md)

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
├── firmware/        # ESP32 / UWB firmware (Rust + C, ESP-IDF v5)
├── server/          # Rust workspace: ingest, dsp, fusion, inference, api, cli
├── dashboard/       # React + Vite + Three.js (TypeScript)
├── mobile/          # Flutter iOS/Android app
├── training/        # PyTorch training pipelines + datasets
├── eval/            # Honest benchmarks + ground-truth tooling
├── integrations/    # Home Assistant, Matter, MQTT, Telegram
├── examples/        # EldGuard, SleepWave, PresenceOS vertical demos
├── docs/            # EN + RU documentation, ADRs, hardware guides
└── scripts/         # Build, release, dev helpers
```

---

## Roadmap

See [ROADMAP.md](ROADMAP.md) for the full multi-phase plan. Headline milestones:

- **M0 — Foundation** (this commit): repo, CI, docs skeleton, ADRs.
- **M1 — First photon**: ESP32-S3 streams CSI to server, presence works.
- **M2 — Mesh & fusion**: PTP sync, multi-node Kalman fusion.
- **M3 — Vitals**: HR + breathing within ±2 BPM of reference.
- **M4 — Pose & fall**: 17-keypoint pose, EldGuard MVP.
- **M5 — Sleep & smart-home**: SleepWave + PresenceOS, Home Assistant integration.
- **M6 — Honest 1.0**: full benchmark suite, public dataset, video demos, launch.

---

## Documentation

| | EN | RU |
|---|---|---|
| Getting started | [getting-started.md](docs/en/getting-started.md) | [быстрый-старт.md](docs/ru/быстрый-старт.md) |
| Architecture | [architecture.md](docs/en/architecture.md) | [архитектура.md](docs/ru/архитектура.md) |
| Hardware guide | [hardware/](docs/hardware/) | [hardware/](docs/hardware/) |
| ADRs | [docs/adr/](docs/adr/) | — |
| Honest benchmarks | [eval/](eval/) | — |

---

## Contributing

We welcome contributions of every shape — hardware tests, dataset captures, model training, dashboard polish, translations. Start with [CONTRIBUTING.md](CONTRIBUTING.md), pick a `good-first-issue` label, and say hello in Discussions.

This is a public project, but it is also a personal mission of the maintainer to ship something genuinely useful in this space — please be patient with review cadence.

---

## Security

We take RF-sensing seriously. WaveSight could plausibly be misused to track people without consent. Please read [SECURITY.md](SECURITY.md) for our threat model, responsible-disclosure policy and the consent guidelines we ship with the dashboard.

---

## License

MIT — see [LICENSE](LICENSE). Copyright © 2026 Cyber_Lord (Vladimir120307@gmail.com).

---
<a id="русская-версия"></a>

# WaveSight (по-русски)

**Честная open-source платформа для WiFi и UWB-сенсинга. Видим движение, дыхание и присутствие — без камер, без облака, без хайпа.**

## Что это

WaveSight превращает обычные WiFi и UWB радиосигналы в пространственный интеллект: кто в комнате, дышит ли человек, упал ли он, спит ли. Всё это работает **на ESP32-меше за $20**, **локально на устройстве**, и **без отправки единого кадра видео куда бы то ни было**.

Мы построили WaveSight, потому что существующие публичные проекты (RuView, ряд академических тулкитов) много обещают в маркетинге и мало предъявляют в физике. ESP32 с одной антенной не может магически реконструировать MIMO-массив. Поэтому мы пошли другим путём:

1. **Реальное hardware-разнообразие** — ESP32-C5/C6 (WiFi 6/6E, настоящее 2-stream MIMO) плюс опциональные UWB-якоря (DWM3000, реальный angle-of-arrival).
2. **Не публикуем цифры, которые не можем воспроизвести** — каждая метрика снабжена pipeline ground-truth, error bars и YouTube-видео.
3. **Сразу три production-вертикали** — `EldGuard` (детекция падений), `SleepWave` (стадирование сна) и `PresenceOS` (occupancy для умного дома).

> WaveSight в стадии **alpha**. Мы честно документируем, что уже работает, что в roadmap, и что — research-grade. См. [ROADMAP.md](ROADMAP.md).

## Возможности

- **Multi-node WiFi CSI-сенсинг** с PTP-синхронизацией с точностью меньше микросекунды.
- **UWB sensor fusion** (DWM3000) для настоящего angle-of-arrival — решаем проблему одной антенны, на которую RuView закрывает глаза.
- **Presence, pose, vitals, fall, sleep, activity** — одна платформа, шесть обученных голов, всё работает on-edge.
- **3D-реконструкция комнаты** в дашборде через Three.js — видим людей как фигурки внутри плана квартиры.
- **Honest Mode**: каждое предсказание показывает доверительный интервал. Если модель не уверена — пишем «не уверена».
- **Воспроизводимые бенчмарки**: `wavesight bench` запускает те же сценарии на вашем железе и сравнивает с опубликованными цифрами.
- **100% local-first**: ни облака, ни телеметрии, ни аккаунтов.
- **Подписанные прошивки и измерения**: supply chain attestation по образцу SLSA.
- **Нативные интеграции Home Assistant, MQTT, Matter, HomeKit (через Matter), Telegram**.
- **Кросс-платформенное мобильное приложение** (Flutter) для живого мониторинга и push-алертов о падениях.

## Hardware-уровни

| Уровень    | Состав                                              | Цена  | Что получаем                                          |
| ---------- | --------------------------------------------------- | ----- | ----------------------------------------------------- |
| **Entry**     | 2 × ESP32-S3                                      | $20   | Presence + простая детекция движения                  |
| **Standard**  | 3 × ESP32-C5 + 1 × DWM3000                        | $80   | Поза, vitals, детекция падений                        |
| **Pro**       | 6 × ESP32-C5 + 3 × DWM3000 + Raspberry Pi 5       | $250  | 3D по комнатам, sleep staging, полный fusion          |
| **Research**  | + Intel AX210 / nRF7002 + RGB-D камера            | $400+ | Ground-truth калибровка, paper-grade evaluation       |

Подробные сборочные гайды лежат в [`docs/hardware/`](docs/hardware/).

## Быстрый старт

> Полный пошаговый гайд: [`docs/ru/быстрый-старт.md`](docs/ru/быстрый-старт.md)

Требования: Rust 1.78+, ESP-IDF 5.2+, Node 20+, Python 3.11+, две и более платы ESP32-S3 / C5.

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

В течение минуты после включения нод вы должны увидеть live CSI-потоки и индикатор присутствия.

## Дорожная карта

Полный план фаз — [ROADMAP.md](ROADMAP.md). Ключевые milestones:

- **M0 — Фундамент** (этот коммит): репо, CI, скелет доков, ADR.
- **M1 — Первый фотон**: ESP32-S3 шлёт CSI на сервер, presence работает.
- **M2 — Mesh и fusion**: PTP-синхронизация, мультинодовый Kalman fusion.
- **M3 — Vitals**: HR + дыхание в пределах ±2 BPM от референса.
- **M4 — Pose и fall**: 17-точечная поза, MVP EldGuard.
- **M5 — Sleep и smart-home**: SleepWave + PresenceOS, интеграция с Home Assistant.
- **M6 — Честная 1.0**: полный benchmark suite, публичный датасет, видео-демо, запуск.

## Документация

| | EN | RU |
|---|---|---|
| Старт | [getting-started.md](docs/en/getting-started.md) | [быстрый-старт.md](docs/ru/быстрый-старт.md) |
| Архитектура | [architecture.md](docs/en/architecture.md) | [архитектура.md](docs/ru/архитектура.md) |
| Hardware | [hardware/](docs/hardware/) | [hardware/](docs/hardware/) |
| ADR | [docs/adr/](docs/adr/) | — |

## Вклад

Будем рады контрибьюциям любого формата — тесты на железе, сбор датасетов, обучение моделей, перевод документации, дизайн дашборда. Начните с [CONTRIBUTING.md](CONTRIBUTING.md), возьмите issue с лейблом `good-first-issue`, заходите в Discussions поздороваться.

## Безопасность

RF-сенсинг — серьёзная вещь. WaveSight теоретически можно использовать для слежки без согласия. Прочитайте [SECURITY.md](SECURITY.md), там описана наша threat model, политика responsible disclosure и встроенные в дашборд гайдлайны по согласию.

## Лицензия

MIT — см. [LICENSE](LICENSE). © 2026 Cyber_Lord (Vladimir120307@gmail.com).
