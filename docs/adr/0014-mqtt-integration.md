# ADR-0014 — MQTT is a first-class integration target

- **Status**: accepted
- **Date**: 2026-05-24
- **Deciders**: @vladimir120307-droid

## Context

The smart-home automation audience is dominated by users who already
have an MQTT broker (Mosquitto, EMQX) feeding Home Assistant, Node-RED
and ESPHome. Re-implementing equivalents in our REST API would
fragment that ecosystem.

## Decision

WaveSight ships a **standalone MQTT bridge** binary
(`integrations/mqtt-bridge`) that subscribes to `/api/v1/stream` and
publishes the presence and event data on a configurable MQTT topic
hierarchy. The bridge also publishes Home Assistant MQTT discovery
messages so HA entities appear automatically.

The bridge is a separate process by design — it can be run in a
different network segment than the edge server, can be replaced by
community alternatives without forking the core, and degrades
gracefully if the broker is unreachable.

## Topic schema

```
wavesight/<node>/presence    {"present": bool, "energy": float, "uncertainty": float}
wavesight/<node>/rssi        integer dBm
wavesight/<node>/online      "online" | "offline"  (retained, MQTT LWT)
wavesight/<node>/fall        FallEvent JSON (on event)
wavesight/<node>/vitals      VitalsPrediction JSON (when available)
```

## Consequences

### Positive
- Zero-config integration with existing smart-home stacks.
- HA discovery removes manual YAML configuration.
- Separation of concerns: the bridge is replaceable without touching the core.

### Negative
- One more binary to release per platform.
- MQTT brokers add deployment complexity for users who do not already
  have one.

## Alternatives considered

### MQTT inside the api crate
Rejected. Couples the core to MQTT semantics; community alternatives
(Server-Sent Events, NATS) would require core changes.

### No MQTT, REST only
Rejected. Loses the smart-home audience.
