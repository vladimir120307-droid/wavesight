# WaveSight dashboard

Vite + React + TypeScript + Tailwind. Subscribes to the edge server's
`/api/v1/stream` WebSocket, renders a live per-node spectrogram, and shows
a binary presence indicator computed from rolling amplitude variance.

## Run

```bash
pnpm install
pnpm dev      # http://localhost:5173 — proxies /api → http://localhost:8081
```

## Build

```bash
pnpm build
# Output in dist/. The server crate serves it as static files in production.
```

## Layout

```
src/
├── App.tsx                  # entrypoint
├── lib/
│   ├── stream.ts            # WebSocket client with auto-reconnect
│   └── state.ts             # zustand store, per-node history
└── components/
    ├── NodeCard.tsx         # one panel per connected node
    └── Spectrogram.tsx      # canvas-based live spectrogram
```

The presence indicator is intentionally rule-based at this point — it
mirrors the server-side `dsp::PresenceDetector` so the dashboard does
not depend on the trained model shipping first. Once the inference
crate ships a real model, the server pushes its calibrated output and
the dashboard switches to displaying that.
