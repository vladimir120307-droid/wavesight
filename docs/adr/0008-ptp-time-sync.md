# ADR-008 — IEEE 1588 PTP for inter-node time sync

- **Status**: accepted
- **Date**: 2026-05-24
- **Deciders**: @vladimir120307-droid

## Context

Multi-node CSI fusion requires that frames from different nodes be aligned in time tightly enough that phase relationships across nodes remain meaningful. The relevant scale is the wavelength-coherence time of the carrier:

- 2.4 GHz: ~0.4 ns per wavelength.
- 5 GHz: ~0.2 ns per wavelength.
- 6 GHz: ~0.17 ns per wavelength.

Strict phase coherence is unattainable on ESP32-class hardware. However, amplitude- and Doppler-domain fusion only requires alignment at the **sub-microsecond** level (the CSI integration window is typically 10 ms; we want jitter <1 % of that).

NTP over WiFi typically achieves ±5–50 ms — three orders of magnitude too coarse.

## Decision

We implement a software-only **IEEE 1588 Precision Time Protocol (PTP)** sync layer:

- Root node hosts the PTP grandmaster (synced to NTP for absolute time).
- Each non-root node runs a PTP slave with hardware-assisted timestamping where the chip supports it (ESP32-C5, C6); software timestamping with periodic correction on ESP32-S3.
- Target error: ≤ 1 μs across the mesh under normal conditions.
- Frames carry PTP timestamp; server uses this for fusion alignment, not local arrival time.

## Consequences

### Positive
- Aligns with industry-standard timing; downstream tooling (Wireshark, network analysers) work.
- Sub-microsecond timing unlocks coherent Doppler analysis.

### Negative
- ESP32-S3 software-only PTP is bursty; we may see occasional 5–10 μs excursions and need to flag them.
- Adds complexity to onboarding (PTP master election).

### Neutral
- Software PTP libraries exist (`linuxptp` for the server side); ESP32 implementation is from-scratch and a non-trivial engineering item for M2.

## Alternatives considered

### NTP only
Rejected. Insufficient precision for fusion.

### Roundtrip-corrected NTP-like custom protocol
Rejected. We would be reinventing PTP poorly.

### Wired PPS (1 Hz pulse over GPIO between adjacent nodes)
Considered for `Research` tier only. Excellent precision but not consumer-friendly.

## References

- IEEE 1588-2019
- "Precision Time Protocol on ESP32" (Sandsmark, 2023 — community work we may build on)
