# WaveSight Documentation

This is the documentation root. Documentation is bilingual (EN + RU) for user-facing content; internal docs (ADRs, source-code comments) are EN-only.

## Quick links

### English

- [Getting started](en/getting-started.md) — flash, start, see your first CSI stream
- [Architecture overview](en/architecture.md) — how the pieces fit together
- [API reference](en/api.md) — REST, WebSocket, gRPC
- [Honest Mode explained](en/honest-mode.md) — how we report confidence
- [Hardware compatibility](hardware/compatibility.md) — what boards work
- [Verticals](en/verticals.md) — EldGuard, SleepWave, PresenceOS

### Русский

- [Быстрый старт](ru/быстрый-старт.md) — прошивка, запуск, первый CSI-поток
- [Архитектура](ru/архитектура.md) — как устроены компоненты
- [Справочник API](ru/api.md) — REST, WebSocket, gRPC
- [Honest Mode — пояснение](ru/honest-mode.md) — как мы показываем уверенность
- [Совместимость железа](hardware/compatibility.md) — какие платы поддерживаются
- [Вертикали](ru/вертикали.md) — EldGuard, SleepWave, PresenceOS

### Internal (English-only)

- [Architecture Decision Records](adr/) — full ADR index
- [Threat model](../SECURITY.md)
- [Hardware build guides](hardware/)
- [Benchmark methodology](../eval/README.md)

## Translation policy

User-facing content **must** ship with EN + RU parity. PRs that add EN-only user-facing docs are blocked until RU is added (or vice versa). Translations of internal ADRs are optional but welcome.

Help us translate to other languages! Open an issue with `i18n` label.
