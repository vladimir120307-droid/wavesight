# ADR-001 — Why WaveSight exists

- **Status**: accepted
- **Date**: 2026-05-24
- **Deciders**: @vladimir120307-droid

## Context

WiFi-based sensing is having a moment. The most visible public project, [RuView](https://github.com/ruvnet/RuView), crossed 55 000 GitHub stars in mid-May 2026. Yet:

1. The pose-estimation accuracy reported in its own README is ~2.5 % PCK@20 — barely above random for many practical applications.
2. The hardware architecture (ESP32-S3 mesh) relies on single-antenna nodes presented as a substitute for true MIMO arrays. Independent reviewers (Cybernews, TechTimes) have noted this is contested.
3. There is no publicly reproducible video demonstration of the headline "through-wall pose" capability.
4. There is no public dataset to validate community-built models.
5. Documentation is English-only, which excludes a large practitioner community in CIS / Russia / Eastern Europe.

We see space for a project that is technically more honest, hardware more diverse, and accessible to a bilingual audience.

## Decision

We will build **WaveSight**: an open-source RF-sensing platform with these non-negotiable principles:

1. **Honest metrics** — no marketed number without a reproducible benchmark and a video demo.
2. **Hardware diversity** — first-class support for ESP32-C5/C6 (true MIMO) and DWM3000 UWB (true AoA), not just ESP32-S3.
3. **Vertical applications** — ship three production-ready applications (EldGuard, SleepWave, PresenceOS), not just a research toolkit.
4. **Bilingual documentation** — full EN + RU parity for all user-facing content.
5. **Privacy-first** — local processing by default, signed measurements, threat-modelled consent flow.

## Consequences

### Positive
- Clear differentiation from RuView on technical and ethical grounds.
- Larger addressable audience by being bilingual.
- Three verticals give concrete user stories to anchor architectural decisions.

### Negative / trade-offs
- Bilingual docs roughly double documentation maintenance cost.
- Hardware diversity multiplies firmware test surface.
- "Honest mode" with confidence intervals is harder to market than confident-looking numbers.

### Neutral
- We will publicly compare against RuView at every release. This is intentional and not adversarial.

## Alternatives considered

### Fork RuView
Rejected. We would inherit its license complexity, architectural assumptions (single-chip), and reputational baggage.

### Build closed-source commercial product first
Rejected. The space rewards open ecosystem players; we are betting on community gravity.

### Narrow to a single vertical (just fall detection)
Rejected. A single vertical is fragile commercially and unattractive technically. The three-vertical platform play creates compounding network effects across users, datasets and models.

## References

- RuView GitHub: https://github.com/ruvnet/RuView
- "DensePose from WiFi" (Carnegie Mellon, 2023)
- Cybernews coverage of RuView: https://cybernews.com/security/viral-github-project-wifi-see-through-walls/
- TechTimes coverage: https://www.techtimes.com/articles/316748/20260517/ruview-passes-55000-github-stars-wifi-sensing-law-it-outran-has-not-arrived.htm
