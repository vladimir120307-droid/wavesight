# Home Assistant integration

Native custom integration for Home Assistant. Polls the WaveSight edge
server every 2 seconds and exposes per-node presence as binary sensors
plus diagnostic sensors for RSSI and frame rate.

## Install via HACS

1. Add `https://github.com/vladimir120307-droid/wavesight` as a custom
   repository in HACS, category "Integration".
2. Install "WaveSight" from the HACS UI.
3. Restart Home Assistant.
4. Settings → Devices & Services → Add Integration → "WaveSight".
5. Enter your edge server URL (`http://<wavesight-host>:8081`).

## Установка через HACS (RU)

1. Добавьте `https://github.com/vladimir120307-droid/wavesight` как custom
   repository в HACS, категория "Integration".
2. Поставьте "WaveSight" через HACS UI.
3. Перезапустите Home Assistant.
4. Настройки → Устройства и службы → Добавить интеграцию → "WaveSight".
5. Укажите URL edge-сервера (`http://<хост-wavesight>:8081`).

## Entities

| Entity | Purpose |
|---|---|
| `binary_sensor.<node>_presence` | Occupancy of the room around `<node>` |
| `sensor.<node>_rssi` | Live RSSI in dBm |
| `sensor.<node>_frame_rate` | Frames/s observed in the last second |

## Automations example

```yaml
automation:
  - alias: Turn on light when someone enters living room
    trigger:
      - platform: state
        entity_id: binary_sensor.node_1_presence
        to: "on"
    action:
      - service: light.turn_on
        target:
          entity_id: light.living_room
```
