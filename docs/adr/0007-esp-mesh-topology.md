# ADR-007 — ESP-MESH for inter-node networking

- **Status**: accepted
- **Date**: 2026-05-24
- **Deciders**: @vladimir120307-droid

## Context

Multiple ESP32 nodes need to:

- Share configuration and onboarding.
- Stream CSI to the edge server with bounded latency.
- Resync time periodically.
- Survive single-node failures (no SPOF).

We need a topology that is self-organising and resilient.

## Decision

We use **ESP-WIFI-MESH (ESP-MESH)** in standalone tree topology with these refinements:

- Auto-elected root reconnects to the home AP and bridges traffic to the edge server.
- Non-root nodes connect via the root, removing the requirement for every node to be in range of the AP.
- A maximum of 6 layers (Espressif recommended limit).
- CSI uplink goes from leaf → root → edge server over a single TCP/TLS connection (multiplexed via gRPC streams).
- Onboarding via BLE provisioning service on the root node.

## Consequences

### Positive
- Off-the-shelf, well-tested implementation in ESP-IDF.
- Allows deploying nodes in distant rooms even with weak AP coverage.
- Single TLS tunnel to the edge server simplifies firewall configuration.

### Negative
- ESP-MESH adds latency vs direct AP connections (typically +30 ms per hop).
- Tree topology has SPOF at the root; we mitigate with fast re-root.

### Neutral
- A future ADR may revisit topology when we have ESP32-C5 + 802.11s mesh data.

## Alternatives considered

### Each node directly on home AP
Rejected. Coverage requirements would force users to add range extenders.

### 802.11s mesh
Deferred. ESP32-C5 supports it; ESP32-S3 does not. We will reconsider at M6.

### Custom mesh over ESP-NOW
Rejected. ESP-NOW is connectionless and unencrypted by default; building a reliable layer on top is reinventing.

## References

- Espressif ESP-MESH documentation: https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-guides/esp-wifi-mesh.html
