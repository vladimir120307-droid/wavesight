# WaveSight mobile app

Flutter (iOS + Android) companion for the WaveSight platform. Subscribes
to the edge server's live stream, shows per-node presence, and surfaces
fall-detection events as push notifications.

## Build

```bash
flutter pub get
flutter run                       # connected device or emulator
flutter build apk --release       # Android
flutter build ipa --release       # iOS
```

## Configuration

Server URL is set on first launch (Settings screen). Default is
`http://wavesight.local:8081`. The app reaches the server over the
local network; no cloud relay.

## Status

- ✅ App scaffold (Flutter + Riverpod + go_router)
- ✅ Theming (dark mode default to match dashboard)
- ✅ Settings screen
- ⚙️ Live WebSocket client (next)
- ⚙️ Fall-event push via local notifications (next)
- ☐ Family sharing
- ☐ iOS / Android release builds in CI
