# ADR-0013 — Documentation is bilingual EN + RU

- **Status**: accepted
- **Date**: 2026-05-24
- **Deciders**: @vladimir120307-droid

## Context

The maintainer is a Russian-speaking developer building for both
English-speaking and Russian-speaking audiences. The existing leader
in this space (RuView) ships only English documentation, which
disadvantages a large practitioner community in CIS / Russia / Eastern
Europe.

## Decision

All **user-facing** documentation ships with full EN + RU parity. This
applies to:

- README.md (top section EN, mirror RU section in the same file)
- Getting-started and architecture guides (separate files,
  `docs/en/...` and `docs/ru/...`)
- Vertical READMEs (`examples/*/README.md`)
- CONTRIBUTING (EN + RU sections in the same file)
- Dashboard UI strings (i18n)

Internal documentation is EN-only:

- ADRs (no parity requirement; community is welcome to translate but
  we will not block on it)
- Source-code comments
- Test names

Pull requests adding user-facing English-only docs are blocked in
review until the Russian translation is added (or vice versa).

## Consequences

### Positive
- Larger addressable practitioner audience.
- Differentiation versus English-only RuView and academic toolkits.
- Habr / Russian developer communities are easier to engage.

### Negative
- Documentation maintenance roughly doubles.
- Translation drift risk — the two language versions need to stay in
  sync.

### Neutral
- Third languages (Chinese, Spanish, etc.) are welcomed but not
  required. Community translators get a `translator` GitHub badge.

## Alternatives considered

### English only
Rejected. We are explicitly differentiating from English-only
competitors.

### Russian only
Rejected. Would isolate the project from the global open-source
ecosystem.

### Machine translation
Rejected for user-facing docs. Maintainer reviews every Russian
sentence; machine translation in technical documentation produces
plausible-sounding errors.
