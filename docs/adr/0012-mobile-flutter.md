# ADR-0012 — Flutter for the mobile app

- **Status**: accepted
- **Date**: 2026-05-24
- **Deciders**: @vladimir120307-droid

## Context

The EldGuard vertical needs a mobile app for push notifications on
fall events. The maintainer already knows Flutter from previous work;
the mobile app does not need bleeding-edge platform features.

## Decision

The mobile app is **Flutter**, single codebase for iOS and Android.
State management is **Riverpod**, routing is **go_router**, push
notifications are **flutter_local_notifications** (local) plus the
WebSocket subscription to the edge server for live events.

## Consequences

### Positive
- One codebase covers both platforms.
- The maintainer is already productive in Flutter.
- Dart's null-safety helps with reliability in the medical-adjacent
  EldGuard vertical.

### Negative
- Flutter binaries are larger than native (~15 MB minimum).
- iOS background-execution rules force us to fall back on a foreground
  notification service if continuous monitoring is required.

### Neutral
- A future native port (Kotlin / Swift) is feasible but not on the
  roadmap.

## Alternatives considered

### React Native
Rejected. The dashboard is React already; reusing the codebase sounds
appealing but the native runtime story on iOS for background sockets
is worse than Flutter's.

### Native (Kotlin + Swift)
Rejected. Doubles the maintenance burden for a project staffed by one
person.

### Tauri Mobile
Rejected. Too immature in mid-2026.

## References

- Riverpod: https://riverpod.dev
- go_router: https://pub.dev/packages/go_router
