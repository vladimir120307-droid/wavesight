# Academic baselines — quick survey

A skim, not a literature review. Mostly to set expectations.

## Pose estimation from CSI

| Paper | Year | Approach | Reported metric |
|---|---|---|---|
| Wi-Pose (Jiang et al.) | 2018 | CNN on amplitude | ~50 % PCK@40 |
| Person-in-WiFi (Wang) | 2019 | 4-rx, mask + skeleton | ~60 % AP, but on 2 subjects |
| DensePose-WiFi (CMU) | 2023 | DenseNet + UV mapping | ~30 % UV-error, controlled scene |
| RuView README | 2026 | ESP32-S3 mesh + transformer | 2.5 % PCK@20, _camera-free_ |

DensePose-WiFi numbers are the academic ceiling. RuView is two orders
of magnitude worse than that — which is expected, because they removed
the camera ground-truth that the academic work uses for supervision.

My target for M4: 25 % PCK@20 with camera ground-truth during training.
Aggressive but plausible.

## Vital signs from CSI

| Paper | Year | Metric (HR) | Metric (BR) |
|---|---|---|---|
| Liu et al. 2014 | 2014 | MAE ±5 BPM | MAE ±1 BPM |
| FullBreath | 2017 | – | MAE ±0.5 BPM |
| EQ-Radio (MIT) | 2016 | MAE ±1.5 BPM | MAE ±0.5 BPM |

These are the gold-standard numbers I want to approach with
commodity ESP32 hardware. Realistic for BR; HR is harder because the
signal is much smaller.

## Fall detection from CSI

| Paper | Year | TPR | FPR |
|---|---|---|---|
| WiFall (Wang) | 2017 | 94 % | 13 % |
| FallSense (2020) | 2020 | 96 % | 3 % |
| RT-Fall | 2021 | 91 % | 5 % |

For EldGuard we need TPR ≥ 90 % and FPR ≤ 1/day. Doable, the literature
supports it.

## Sleep staging

Less surveyed by me — known that Whoop, Withings Sleep, and Apple do
this with PPG / accelerometer. RF-based papers exist but are rare.
Going to revisit this when I get to M5.

## Takeaway

The 1-D problems (presence, BR, fall) are tractable on commodity
hardware. The 2-D problem (pose) is a research stretch — I should be
honest that any pose number on $20 hardware is a stretch and report
error bars accordingly.
