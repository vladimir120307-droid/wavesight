# ADR-0011 — Raw CSI retention is 24 hours, aggregates indefinite

- **Status**: accepted
- **Date**: 2026-05-24
- **Deciders**: @vladimir120307-droid

## Context

A 4-node CSI stream at 100 Hz with HT20 captures generates roughly
1 GB / day of raw amplitude+phase data per node. Indefinite retention
would require multiple terabytes / year for a typical 3-room deployment,
which we cannot expect users to host on a Raspberry Pi.

At the same time, aggregated derivatives (per-minute presence,
per-night sleep staging, fall events) are tiny — a year of all three
verticals fits comfortably in a few hundred megabytes.

## Decision

Raw CSI is stored in SQLite as a **rolling 24-hour buffer**, configurable
via `StorageConfig::raw_retention_hours`. A background prune task runs
every 10 minutes and deletes rows older than the cutoff.

Aggregated derivatives are stored as **Parquet files** in
`~/.wavesight/data/parquet/`, partitioned by date. No retention limit.

## Consequences

### Positive
- Raspberry Pi 5 with a 256 GB SSD can host the platform for years.
- Privacy story improves — raw RF samples that could in principle be
  re-analyzed for unintended signals self-delete daily.
- Parquet aggregates are portable and queryable from pandas / polars.

### Negative
- A user who finds a bug in the inference pipeline cannot rerun
  yesterday's CSI through a fixed model — the raw data is gone.
- Researchers wanting longer raw retention must bump
  `raw_retention_hours` and provision their own storage.

### Neutral
- We may add an opt-in "research mode" that retains raw forever in a
  future ADR if community demand justifies the support cost.

## Alternatives considered

### Indefinite raw retention
Rejected. Storage cost makes the platform unusable on commodity edge
hardware.

### No raw retention (only stream)
Rejected. A 24-hour buffer is necessary for replay-debugging during
development and for the rolling clutter map.

### S3-tier cloud archival
Rejected. Violates ADR-006 (privacy local-first).
