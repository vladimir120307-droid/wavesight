# Security Policy

WaveSight is a sensing platform that can observe people inside their homes. We take security and consent very seriously.

## Reporting a vulnerability

Please **do not** open public issues for security problems.

- Email: **vladimir120307@gmail.com**
- PGP key: see `docs/security/pgp.asc` (TBD before 1.0)
- Expected first response: 72 hours

You can also use GitHub's private vulnerability reporting in the Security tab of this repository.

If your report leads to a fix that ships in a release, you will be credited in the release notes (with your permission) and listed in [`docs/security/hall-of-fame.md`](docs/security/hall-of-fame.md).

A symbolic bug bounty is available starting from release `v0.4.0` for verified high-impact issues. See [`docs/security/bounty.md`](docs/security/bounty.md).

## Threat model

WaveSight protects against:

1. **Physical eavesdropping on the mesh.** All inter-node traffic is encrypted with ChaCha20-Poly1305. PSK is provisioned during onboarding.
2. **Firmware tampering.** Firmware images are signed; bootloader enforces signature check. A/B partitions allow rollback.
3. **Measurement spoofing.** Signed measurement frames with monotonic counters; replays are rejected.
4. **Server compromise leaking history.** Stored timeseries can be configured to retain only aggregated derivatives (e.g. "occupancy at 5-minute resolution") rather than raw CSI.
5. **Cloud exfiltration.** There is no default cloud. Egress is explicitly opt-in per integration.

WaveSight **does not** protect against:

- A nation-state-level attacker with physical access to your hardware.
- An attacker who has already root on the machine hosting the edge server.
- Side-channel inference attacks on the underlying WiFi medium itself (this is a research-grade open problem).

## Consent guidelines

The dashboard ships with mandatory onboarding that requires the deploying user to acknowledge:

- Everyone who lives in the monitored space has consented to monitoring.
- Visitors are informed.
- A clear physical indicator (LED, signage) is visible in monitored rooms.

These cannot be skipped via the UI. They can be skipped by editing the source — please don't, and if you do, you are operating outside the supported deployment model.

## Supply chain

- Cargo dependencies pinned via `Cargo.lock` and audited weekly by `cargo-audit` in CI.
- npm dependencies pinned via `pnpm-lock.yaml` and audited by `pnpm audit` in CI.
- Python dependencies pinned via `requirements.lock` and audited by `pip-audit`.
- Release artifacts attested via Sigstore / SLSA Level 3 (target by v1.0).
- ESP-IDF version pinned in `firmware/esp32-csi-node/sdkconfig.defaults`.

## Coordinated disclosure timeline

| Severity | Public disclosure |
|---|---|
| Critical | 30 days or upon patch availability, whichever sooner |
| High | 60 days |
| Medium | 90 days |
| Low | At next regular release |

We will negotiate extensions for complex issues affecting downstream users.
