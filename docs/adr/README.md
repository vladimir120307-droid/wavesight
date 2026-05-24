# Architecture Decision Records

This directory tracks substantive architectural decisions made during the development of WaveSight. Each ADR is a small Markdown document explaining the context, the decision, and the trade-offs.

We target **100+ ADRs by v1.0**. This is not a vanity number — it is the trail of breadcrumbs the next maintainer (or curious community member) follows to understand _why_ things are the way they are.

## Index

| # | Title | Status |
|---|---|---|
| [0001](0001-why-wavesight.md) | Why WaveSight exists | accepted |
| [0002](0002-rust-server-core.md) | Rust as the server-core language | accepted |
| [0003](0003-monorepo-layout.md) | Monorepo layout | accepted |
| [0004](0004-honest-mode.md) | Honest Mode is the default | accepted |
| [0005](0005-multi-modal-fusion.md) | Multi-modal fusion (WiFi + UWB + BLE) | accepted |
| [0006](0006-privacy-local-first.md) | Privacy is local-first, no cloud by default | accepted |
| [0007](0007-esp-mesh-topology.md) | ESP-MESH for inter-node networking | accepted |
| [0008](0008-ptp-time-sync.md) | IEEE 1588 PTP for inter-node time sync | accepted |
| [0009](0009-ml-stack.md) | ML stack: PyTorch training, Candle inference | accepted |
| [0010](0010-dashboard-stack.md) | Dashboard stack: Vite + React + TypeScript + Three.js | accepted |
| [0011](0011-storage-retention.md) | Raw CSI retention is 24 hours, aggregates indefinite | accepted |
| [0012](0012-mobile-flutter.md) | Flutter for the mobile app | accepted |
| [0013](0013-bilingual-docs.md) | Documentation is bilingual EN + RU | accepted |
| [0014](0014-mqtt-integration.md) | MQTT is a first-class integration target | accepted |
| [0015](0015-fall-medical-disclaimer.md) | EldGuard is not a medical device | accepted |

## Writing a new ADR

1. Copy [`_template.md`](_template.md) to `NNNN-short-slug.md` with the next free number.
2. Fill in Context, Decision, Consequences and Alternatives.
3. Open a PR. ADRs are merged after one maintainer review.
4. After merge, update this index.

ADRs are immutable once accepted. To change a decision, write a new ADR that supersedes the old one and update the old one's status to `superseded by ADR-NNNN`.
