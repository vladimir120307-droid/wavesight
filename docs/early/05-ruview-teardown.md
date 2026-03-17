# RuView teardown — honest technical look

_Notes after reading the RuView repo and skimming the issues._

## EN

### What RuView is

A Rust + Python + TypeScript stack that captures WiFi CSI from a mesh
of ESP32-S3 nodes and runs neural inference for presence, pose, and
vitals. Marketing claims include through-wall pose tracking. ~55 000
GitHub stars as of last week.

### What RuView gets right

- Strong Rust core, sensible crate split.
- 1400+ tests — a real engineering culture.
- 96 ADRs — admirable documentation discipline.
- Aggressive marketing that earned them attention.

### What RuView gets wrong

- **Single-antenna ESP32-S3 cannot do MIMO.** The "mesh approximates a
  MIMO array" claim is hand-wavy. Phase coherence across nodes requires
  sub-microsecond time sync, which their stack does not appear to
  provide.
- **No reproducible through-wall demo.** Every video is short, with
  cuts, no ground truth. This is suspicious.
- **Pose accuracy is ~2.5 % PCK@20** by their own README. That is
  effectively random for many practical applications.
- **No public dataset.** Cannot independently verify any claim.
- **Cognitum Seed appliance ($140)** locks users in.

### What I would do differently

1. Add ESP32-C5/C6 + UWB DWM3000 to get real spatial diversity.
2. Implement IEEE 1588 PTP across the mesh.
3. Refuse to publish a metric without a recorded ground-truth scenario.
4. Build verticals (EldGuard, SleepWave, PresenceOS), not a toolkit.
5. Run entirely on a Pi 5 or any x86 machine — no proprietary appliance.

## RU

### Что такое RuView

Стек Rust + Python + TypeScript: ESP32-S3-меш ловит WiFi CSI, нейронки
делают presence / pose / vitals. Маркетинговое обещание — детекция позы
сквозь стены. ~55 000 звёзд на GitHub на прошлой неделе.

### Что у RuView хорошо

- Сильный Rust-ядро, разумное разделение крейтов.
- 1400+ тестов — реальная инженерная культура.
- 96 ADR — образцовая дисциплина документации.
- Агрессивный маркетинг сделал им внимание.

### Что у RuView плохо

- **Однантенный ESP32-S3 не умеет MIMO.** "Меш приближает MIMO-массив"
  — это рукомахание. Фазовая когерентность между нодами требует
  субмикросекундной синхронизации, которой их стек, похоже, не даёт.
- **Нет воспроизводимого "сквозь стены" демо.** Все видео короткие, с
  монтажом, без ground truth. Подозрительно.
- **Точность позы ~2.5 % PCK@20** по их же README — фактически шум.
- **Нет публичного датасета.** Невозможно независимо проверить ничего.
- **Cognitum Seed за $140** — vendor lock-in.

### Что я бы сделал иначе

1. Добавить ESP32-C5/C6 + UWB DWM3000 для настоящего пространственного разнообразия.
2. Реализовать IEEE 1588 PTP в меше.
3. Не публиковать метрику без записанного ground-truth-сценария.
4. Строить вертикали (EldGuard, SleepWave, PresenceOS), а не toolkit.
5. Полностью на Pi 5 или x86 — никакого фирменного железа.
