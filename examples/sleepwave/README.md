# SleepWave

> Sleep staging without a wearable. Uses the WaveSight platform.
> Local-only nightly report, no cloud.

## EN

### What it does

While you sleep, one ESP32-S3 in the bedroom captures CSI. The
`sleep` inference head produces a 4-stage hypnogram (Wake / Light /
Deep / REM) at 30-second resolution. Every morning, SleepWave
generates a one-page PDF report and exposes the data over the
dashboard's sleep panel.

### Hardware

- 1 × ESP32-S3 node placed ~2 m from the bed (no closer — RSSI
  saturation degrades the signal).
- WaveSight edge server (any Linux box, Pi 5 recommended).

### Privacy

The hypnogram never leaves your local network. The report PDF lives
in `~/.wavesight/sleepwave/reports/`. Sharing is opt-in and explicit
(export → email).

### Limits

Sleep staging from RF is **not** EEG-grade. Expected agreement with a
wrist-worn reference (Withings ScanWatch, Oura) is ~70 % at the
4-stage level. Honest reporting (ADR-004) in the UI will show the
confidence band on each epoch.

## RU

### Что делает

Пока вы спите, одна ESP32-S3 в спальне ловит CSI. `sleep`-голова
выдаёт 4-стадийную гипнограмму (Wake / Light / Deep / REM) с
разрешением 30 секунд. Утром SleepWave собирает PDF-отчёт и
показывает данные на панели сна в дашборде.

### Железо

- 1 × ESP32-S3 нода в ~2 м от кровати (ближе нельзя — RSSI-сатурация
  ухудшает сигнал).
- WaveSight edge-сервер (любой Linux, рекомендуется Pi 5).

### Приватность

Гипнограмма никогда не уходит за пределы вашей локальной сети. PDF
лежит в `~/.wavesight/sleepwave/reports/`. Шеринг — opt-in (export
→ email).

### Ограничения

RF-стадирование сна **не** EEG-grade. Ожидаемое согласие с
референсным wearable (Withings ScanWatch, Oura) — ~70% на 4-стадийном
уровне. Honest Mode (ADR-004) в UI покажет confidence band на каждой
эпохе.
