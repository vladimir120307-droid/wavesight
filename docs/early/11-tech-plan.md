# Rough technical plan

Pre-coding. This is the napkin sketch I keep iterating on.

## EN

### Stack

- **Firmware**: ESP-IDF v5.2, C (with possible Rust port later via
  esp-rs). One project per board type. WebSocket uplink.
- **Server**: Rust monorepo. tokio + axum. Cargo workspace with crates
  per concern: ingest, dsp, fusion, inference, storage, api, cli.
- **ML training**: PyTorch + Lightning, distill to Candle for runtime.
- **ML runtime**: Candle in Rust, ONNX Runtime as fallback.
- **Dashboard**: Vite + React + TypeScript + Tailwind + Three.js (for
  3D room view, later).
- **Mobile**: Flutter (iOS + Android). I already know Flutter.
- **Integrations**: Home Assistant (Python custom integration), MQTT
  bridge, Matter (later).

### Pipeline

```
ESP32 ── WS /ingest ── csi-ingest ── dsp ── fusion ── inference
                                                          │
                                                          ▼
                                                    api/v1/...
                                                          │
                                          ┌───────────────┼───────────────┐
                                          ▼               ▼               ▼
                                      Dashboard       Mobile        Home Assistant
```

### What I will NOT build (anti-features)

- A cloud relay. Local-first only.
- A subscription business model.
- A proprietary appliance.
- An assistant chatbot inside the dashboard (focus, focus).
- Anything pose-related until I have honest 25 % PCK numbers.

### Verticals

Will pick three to lock scope:

1. **EldGuard** — fall detection (medical-adjacent vertical).
2. **SleepWave** — sleep staging (quantified-self vertical).
3. **PresenceOS** — occupancy for Home Assistant (smart-home vertical).

These three are different enough that they exercise the platform
in different ways but related enough that the same core delivers all
of them.

### First 4 weeks of work after name is locked

- Week 1: bootstrap monorepo, README, license, basic crate scaffolding.
- Week 2: minimal types in `wavesight-core`. ConfidenceInterval / Uncertainty.
- Week 3: csi-ingest skeleton + dsp skeleton.
- Week 4: fusion + inference + storage skeletons. Tag the foundation
  release.

## RU

### Стек

- **Firmware**: ESP-IDF v5.2, C (потом возможно порт на Rust через
  esp-rs). По одному проекту на тип платы. WebSocket-uplink.
- **Сервер**: Rust монорепо. tokio + axum. Cargo workspace с крейтами
  по слоям: ingest, dsp, fusion, inference, storage, api, cli.
- **ML-тренинг**: PyTorch + Lightning, дистилляция в Candle.
- **ML-runtime**: Candle, ONNX Runtime как запасной.
- **Dashboard**: Vite + React + TypeScript + Tailwind + Three.js (3D
  потом).
- **Mobile**: Flutter (iOS + Android). Уже знаю Flutter.
- **Интеграции**: Home Assistant (Python custom integration), MQTT,
  Matter (потом).

### Pipeline

```
ESP32 ── WS /ingest ── csi-ingest ── dsp ── fusion ── inference
                                                          │
                                                          ▼
                                                    api/v1/...
                                                          │
                                          ┌───────────────┼───────────────┐
                                          ▼               ▼               ▼
                                      Dashboard       Mobile        Home Assistant
```

### Чего НЕ строю (анти-фичи)

- Облачный реле. Только local-first.
- Подписка как бизнес-модель.
- Фирменное железо.
- Чат-ассистент внутри дашборда (фокус, фокус).
- Pose-фичи пока не получу честные 25 % PCK.

### Вертикали

Три, чтобы зафиксировать скоуп:

1. **EldGuard** — детекция падений (медицинский angle).
2. **SleepWave** — стадирование сна (quantified-self).
3. **PresenceOS** — occupancy для Home Assistant (smart-home).

Они достаточно разные, чтобы прогнать платформу по разным углам, и
достаточно связанные, чтобы одно core ядро их всех питало.

### Первые 4 недели после фиксации названия

- Неделя 1: бутстрап монорепы, README, лицензия, скелет крейтов.
- Неделя 2: минимальные типы в `wavesight-core`. ConfidenceInterval /
  Uncertainty.
- Неделя 3: скелеты csi-ingest и dsp.
- Неделя 4: скелеты fusion + inference + storage. Тег foundation.
