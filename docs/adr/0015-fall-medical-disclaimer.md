# ADR-0015 — EldGuard is not a medical device

- **Status**: accepted
- **Date**: 2026-05-24
- **Deciders**: @vladimir120307-droid

## Context

The EldGuard vertical detects falls and notifies family members. This
is medical-adjacent functionality. In several jurisdictions (FDA in
the US, CE Mark in the EU, Roszdravnadzor in Russia), marketing a
device as a "fall alarm" or "medical alert" requires regulatory
certification.

The project is open-source, maintained by one person, with no current
plans to pursue certification.

## Decision

EldGuard is marketed and documented as a **family-notification helper**,
not a medical device or emergency response system. Every user-facing
surface (README, mobile app first-launch screen, Telegram first
message) carries the disclaimer:

> EldGuard is not a substitute for a professional emergency response
> system. It notifies designated family members on detected events; the
> family decides whether to escalate.

The disclaimer is not removable through configuration.

## Consequences

### Positive
- Removes regulatory liability while still offering meaningful value.
- Honest marketing — we tell users exactly what they get.
- Lower bar for community contributions (contributors do not need to
  worry about FDA implications).

### Negative
- Some users may want an "alarm" experience and feel let down by the
  family-notification framing.
- Hospitals, assisted-living facilities and similar buyers are
  effectively excluded until certification.

### Neutral
- A future fork (or a separate commercial product) could pursue
  certification on top of the open source core. We do not preclude
  this — but we do not pursue it.

## Alternatives considered

### Pursue FDA / CE certification
Rejected. Cost (six-figure USD) and timeline (multiple years) are
incompatible with a hobby project.

### Disable EldGuard until certification
Rejected. EldGuard delivers real value as a notification helper today.
Removing it harms users without protecting them.

## References

- FDA "Mobile Medical Applications" guidance (2019)
- EU Medical Device Regulation 2017/745
