# ADR-010 — Dashboard stack: Vite + React + TypeScript + Three.js

- **Status**: accepted
- **Date**: 2026-05-24
- **Deciders**: @vladimir120307-droid

## Context

The dashboard is the primary surface where the value of WaveSight becomes visible. It must:

- Stream real-time data over WebSocket without lag.
- Render a 3D model of the user's floor plan with live human figures.
- Show calibrated uncertainty (ADR-004) clearly.
- Be hackable by community contributors (huge ML/embedded contributor base, smaller frontend base).
- Build into a static bundle servable from the Rust server (no separate Node runtime in production).

## Decision

Dashboard is **Vite + React 18 + TypeScript 5**, with:

- **Three.js** (via `@react-three/fiber` and `@react-three/drei`) for 3D room rendering.
- **Recharts** for time-series charts.
- **Zustand** for state management (lightweight vs Redux).
- **TanStack Query** for server-state caching.
- **Tailwind CSS** for styling (consistent with the dark theme already established in `esp32-smart-controller`).
- **i18next** for EN ↔ RU UI translation.
- **Vitest** + **Playwright** for tests.

The build artefact is a static SPA bundle served by the Rust `api` crate via `axum` static-file middleware.

## Consequences

### Positive
- React is the most familiar choice for community contributors.
- React Three Fiber gives us declarative 3D with React's developer ergonomics.
- Vite hot-reload is fast.
- Static-bundle deployment removes the Node-in-production worry.

### Negative
- React bundle size is non-trivial; we mitigate with code-splitting and aggressive tree-shaking.
- 3D rendering can be heavy on low-end hardware; the dashboard offers a 2D fallback mode.

### Neutral
- A future native (Tauri) wrapper is feasible without rewriting the dashboard.

## Alternatives considered

### Svelte + Threlte
Considered. Smaller bundle and elegant 3D bindings. Rejected because of smaller contributor base.

### Solid.js
Rejected. Less mature 3D ecosystem.

### Native (Tauri/Wails) instead of browser
Deferred. Browser-first is the right call for v1; we can wrap later.

## References

- @react-three/fiber: https://github.com/pmndrs/react-three-fiber
- TanStack Query: https://tanstack.com/query
