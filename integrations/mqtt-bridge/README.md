# WaveSight MQTT bridge

Bridges the WaveSight `/api/v1/stream` WebSocket onto an MQTT broker so
existing smart-home stacks that already speak MQTT (Home Assistant via
MQTT discovery, OpenHAB, Node-RED, ESPHome) can consume presence events
without writing custom code.

## Topics

| Topic | Direction | Payload |
|---|---|---|
| `wavesight/<node>/presence` | publish | `{"present": bool, "energy": float, "uncertainty": float}` |
| `wavesight/<node>/rssi` | publish | int dBm |
| `wavesight/<node>/online` | publish (retained, LWT) | `"online"` / `"offline"` |

## Run

```bash
cargo run -p mqtt-bridge -- \
    --server http://localhost:8081 \
    --broker mqtt://localhost:1883 \
    --topic-prefix wavesight
```

Set `MQTT_USERNAME` / `MQTT_PASSWORD` env vars for authenticated brokers.

## Home Assistant MQTT discovery

The bridge publishes Home Assistant MQTT discovery messages on startup,
so binary_sensor entities appear automatically without manual YAML.
