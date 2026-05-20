# ADR-004 — Honest Mode is the default

- **Status**: accepted
- **Date**: 2026-05-24
- **Deciders**: @vladimir120307-droid

## Context

RF-sensing models routinely produce predictions with very high epistemic uncertainty. A pose prediction at 6 m through a wall, fused from a noisy two-node mesh, is not the same kind of prediction as a vitals reading 2 m line-of-sight. Yet dashboards in this space (including RuView's) tend to render both as identical-looking numbers.

This is misleading and, in the medical-adjacent verticals we target (fall detection, sleep), genuinely dangerous.

## Decision

Every prediction produced by the inference layer **must** carry:

- A point estimate.
- A `confidence_low` / `confidence_high` interval (e.g. 90 % credible interval).
- An `epistemic_uncertainty` scalar in [0, 1].

The dashboard renders the confidence interval as a visible band. When `epistemic_uncertainty > 0.7`, the UI displays "uncertain" rather than the point estimate.

The API returns `null` for the point estimate when below the configured confidence threshold; clients must handle this case.

## Consequences

### Positive
- Aligns the UI with reality. Users build correct mental models.
- Forces model authors to expose calibrated uncertainty, which improves model quality.
- Distinct competitive position vs RuView and academic toolkits.

### Negative
- More complex API and UI than a single "is there a person" boolean.
- Calibrated uncertainty is harder than point predictions; we may ship M1 with naive uncertainty and refine later.

### Neutral
- A developer-facing toggle can disable Honest Mode for downstream products that have their own UX layer, but the API still returns the uncertainty fields.

## Alternatives considered

### Show only point estimates, hide uncertainty
Rejected. This is the status quo we are explicitly differentiating from.

### Show uncertainty only behind an "advanced" toggle
Rejected. If we believe uncertainty matters, hiding it is dishonest.

## References

- "On Calibration of Modern Neural Networks" (Guo et al., 2017)
- "Concrete Problems in AI Safety" (Amodei et al., 2016)
