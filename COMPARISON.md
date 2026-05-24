# Honest comparison: WaveSight vs RuView vs academic baselines

This document is meant to be **technically honest**, not promotional. We respect the work behind RuView and the academic literature it builds on (CMU DensePose-from-WiFi, Wi-Pose, Person-in-WiFi, etc.). The point of this comparison is to explain — in technical terms — what WaveSight does differently and why we believe those choices matter.

If you find a factual error here, please open an issue. We will fix it.

---

## 1. Hardware diversity

| | RuView | WaveSight | Why it matters |
|---|---|---|---|
| Primary chip | ESP32-S3 only | ESP32-S3 + **ESP32-C5/C6** + **DWM3000 UWB** | ESP32-S3 has a single antenna and a single RF chain. You cannot reconstruct phase diversity that doesn't exist in the signal. ESP32-C5 has WiFi 6 with 2x2 MIMO. DWM3000 gives real time-of-flight and angle-of-arrival. |
| Cost floor | $20 | $20 (same Entry tier) | We don't tax Entry users. |
| Cost ceiling | $9 node + $140 Cognitum Seed | $0 — runs on any Pi/PC the user already owns | No proprietary appliance lock-in. |

**Take-away**: RuView leans entirely on _algorithmic_ workarounds (mesh-as-MIMO). WaveSight adds _physical_ diversity where it matters.

---

## 2. Time synchronization

| | RuView | WaveSight | Why it matters |
|---|---|---|---|
| Method | NTP-style sync | IEEE 1588 **PTP** | PTP is the standard for sub-microsecond multi-node alignment. CSI phase coherence across nodes requires microsecond-or-better alignment to fuse meaningfully. |
| Target error | "best effort" | ≤ 1 μs across mesh | Below the wavelength-coherence threshold for 5 GHz WiFi sensing. |

---

## 3. Pose accuracy

| | RuView | WaveSight target |
|---|---|---|
| PCK@20 (camera-free) | ~2.5 % (per RuView README) | ≥ 25 % (paper-grade target, see M4) |
| Training ground truth | Implicit | Explicit RGB-D camera + IMU dataset, public on HuggingFace |
| Per-subject error reporting | No | Yes, with scatter plots |

We are honest: 25 % PCK@20 is still **research-grade**, not consumer-grade. We will not claim "camera replacement" until the numbers support it.

---

## 4. Confidence

| | RuView | WaveSight |
|---|---|---|
| UI shows confidence intervals | No | **Yes — Honest Mode by default** |
| API returns uncertainty | No | Every prediction has `confidence_low`, `confidence_high`, `epistemic_uncertainty` |
| Refuse-to-answer mode | No | Below threshold, the model returns `null` and the UI says "uncertain" |

This is the single biggest cultural difference. We will never make the dashboard look more confident than the model actually is.

---

## 5. Vertical applications

RuView ships as a research toolkit. WaveSight ships three production-shaped applications:

- **EldGuard** — fall detection for elderly. Mobile push alerts, family sharing, intentionally narrow scope, intentionally conservative (high recall, low FPR).
- **SleepWave** — sleep staging without wearables. Nightly hypnogram, weekly report.
- **PresenceOS** — per-room occupancy for Home Assistant / Matter. Energy-saving as the headline benefit.

Each vertical has its own README, its own datasets, its own evaluation metrics, and its own _failure documentation_.

---

## 6. Verification & evidence

| | RuView | WaveSight |
|---|---|---|
| Public dataset | Not yet | 50+ hours by 1.0 |
| Reproducible benchmark CLI | No | `wavesight bench` |
| Per-release benchmark CI | No | Yes — release blocked if regression > threshold |
| Video demos under controlled scenes | Few | One per shipped capability |

---

## 7. Privacy & security

| | RuView | WaveSight |
|---|---|---|
| Local-first by default | Yes | Yes |
| Signed firmware | Partial | Full SLSA Level 3 target |
| Signed measurements | Yes (Cognitum Seed) | Yes (any TPM 2.0 / ATECC608) |
| Threat model document | Limited | [SECURITY.md](SECURITY.md) |
| Consent-first deployment UI | No | Required by dashboard onboarding |

---

## 8. What RuView does better (right now)

We are not pretending RuView is bad. As of this writing:

- **Community momentum**: 55 000+ GitHub stars and a working firehose of contributions.
- **ADR depth**: 96+ architectural decision records, an excellent paper trail.
- **Test count**: 1463+ passing tests in their main repo.
- **HuggingFace pretrained weights**: available today.

We will catch up on all four, openly, and document our progress.

---

## 9. Academic baselines

For users coming from research:

- **DensePose-from-WiFi (CMU, 2023)**: the seminal paper. We cite it; our pose head is initialized from a re-implementation.
- **Wi-Pose / Person-in-WiFi**: classic mid-2010s baselines. Still useful for low-traffic scenarios.
- **mmWave radar (TI IWR6843, XeThru)**: complementary, not competing. Phase 2 of our roadmap includes mmWave fusion.
- **Nexmon CSI (Raspberry Pi)**: supported as a CSI source for research deployments.

---

_Last reviewed: 2026-05-24._
