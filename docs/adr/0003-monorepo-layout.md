# ADR-003 — Monorepo layout

- **Status**: accepted
- **Date**: 2026-05-24
- **Deciders**: @vladimir120307-droid

## Context

WaveSight has multiple deliverables: ESP32 firmware (Rust + C), Rust server, TypeScript dashboard, Flutter mobile app, Python training, documentation, vertical examples. Splitting these across many repos creates:

- Cross-cutting refactors that span 4+ PRs.
- Version-skew between firmware protocol and server expectation.
- Difficult onboarding ("which 11 repos do I clone?").
- Hard-to-coordinate releases.

## Decision

Single monorepo with the following top-level layout:

```
wavesight/
├── firmware/       # ESP32 / UWB firmware (Rust + C)
├── server/         # Rust workspace
├── dashboard/      # TypeScript / Vite / React / Three.js
├── mobile/         # Flutter
├── training/       # Python
├── eval/           # Honest benchmarks
├── integrations/   # Home Assistant, Matter, MQTT, Telegram
├── examples/       # EldGuard, SleepWave, PresenceOS
├── docs/           # EN + RU + ADR + hardware
└── scripts/        # Build, release, dev helpers
```

Releases are tagged at the repo root and versioned per-component via Cargo / pnpm workspaces.

## Consequences

### Positive
- Atomic cross-cutting commits.
- Single source of truth for the wire protocol between firmware and server (`server/crates/csi-ingest/proto/`).
- Easy "clone and explore" experience.
- One CI configuration governs the whole project.

### Negative
- Repo clone size grows over time. We mitigate with `git lfs` for binary assets.
- CI runtime can balloon; we mitigate with path-filter triggers per workflow.

### Neutral
- Contributors specialising in one area (e.g. dashboard) can ignore the rest with the `--filter` flags.

## Alternatives considered

### Multi-repo
Rejected. Coordination overhead would dominate development time.

### Bazel monorepo
Rejected. Bazel adds significant complexity; native Cargo + pnpm + uv covers our needs.

### Nx monorepo
Rejected. Designed for JS-heavy projects; awkward fit for Rust + ESP-IDF.

## References

- pnpm workspaces: https://pnpm.io/workspaces
- Cargo workspaces: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
