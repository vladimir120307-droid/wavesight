# ADR-006 — Privacy is local-first, no cloud by default

- **Status**: accepted
- **Date**: 2026-05-24
- **Deciders**: @vladimir120307-droid

## Context

RF-sensing in the home is intimately invasive: presence, breathing, sleep, motion, gait. A vendor with cloud access to those streams effectively has continuous physiological surveillance of every occupant. We consider that an unacceptable default.

Regulatory winds are also blowing this way: GDPR Article 9 (special-category data), upcoming EU AI Act sensitive-application rules, FCC scrutiny of WiFi sensing.

## Decision

WaveSight is **local-first by design and by default**:

1. All inference, storage and dashboard rendering happens on the user's own hardware.
2. No telemetry of any kind is shipped without an explicit opt-in, per-feature.
3. Cloud integrations (Telegram, push notifications) are listed individually in onboarding; user must approve each.
4. The mesh is encrypted (ChaCha20-Poly1305) with a PSK provisioned during onboarding.
5. Signed measurement frames carry monotonic counters; replays are rejected.
6. Raw CSI retention defaults to 24 hours, then aggregated derivatives only.

## Consequences

### Positive
- Strong differentiator vs cloud-based competitors.
- Easier story for regulators and enterprise / healthcare buyers.
- No infrastructure cost for the maintainer.

### Negative
- No "log in from anywhere" out of the box; users must self-host a reverse proxy / VPN.
- Crash reports / usage analytics are weak by design.

### Neutral
- An optional opt-in federated learning service (M7+) may aggregate gradients with differential privacy; raw data still never leaves the device.

## Alternatives considered

### Hybrid (default cloud, optional local)
Rejected. Default behaviour drives adoption; cloud-by-default defeats the privacy premise.

### Local-only forever (no cloud features ever)
Rejected. Family sharing of fall alerts is a legitimate cloud-touching use case; we want to support it, just not by default.

## References

- GDPR Article 9
- EU AI Act final text (2024)
- Apple "Differential Privacy" white paper
