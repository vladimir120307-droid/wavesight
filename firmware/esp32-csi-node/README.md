# WaveSight ESP32-S3 CSI node firmware

This is the firmware that runs on each ESP32-S3 sensing node. It joins the
configured WiFi network, taps Channel State Information from passing
packets, encodes each frame as JSON, and streams the result over WebSocket
to the WaveSight edge server.

## Build

Requires [ESP-IDF v5.2 or newer](https://docs.espressif.com/projects/esp-idf/en/v5.2/esp32s3/get-started/index.html).

```bash
idf.py set-target esp32s3
idf.py menuconfig    # set "WaveSight node" → SSID, password, server URI
idf.py -p /dev/ttyUSB0 flash monitor
```

## Configuration

All knobs live under `menuconfig → WaveSight node`:

| Key | Default | Notes |
|---|---|---|
| `WAVESIGHT_NODE_NAME` | `node-1` | Unique identifier reported to the server |
| `WAVESIGHT_WIFI_SSID` | `MyWiFi` | Network to join and tap CSI from |
| `WAVESIGHT_WIFI_PASSWORD` | _empty_ | WPA2/3 passphrase, leave empty for open networks |
| `WAVESIGHT_SERVER_URI` | `ws://192.168.1.10:8080/ingest` | Edge-server WebSocket endpoint |
| `WAVESIGHT_CSI_QUEUE_LEN` | `128` | Frames buffered between ISR and uploader |
| `WAVESIGHT_BATCH_SIZE` | `8` | Frames per outbound JSON message |

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│              ESP32-S3                                    │
│                                                          │
│   WiFi RX  ──► esp_wifi_set_csi_rx_cb ──► FreeRTOS queue │
│                       (core 0)              (capacity N) │
│                                                  │       │
│                                                  ▼       │
│                                         uploader task    │
│                                            (core 1)      │
│                                                  │       │
│              batch + JSON encode + base64 ◄──────┘       │
│                       │                                  │
│                       ▼                                  │
│              esp_websocket_client_send_text              │
│                       │                                  │
└───────────────────────┼──────────────────────────────────┘
                        │
                        ▼   ws://server:8080/ingest
                 WaveSight edge server
```

## Wire format

Each WebSocket message is one JSON object:

```json
{
  "node": "node-1",
  "batch": [
    {
      "seq":    12345,
      "ts_us":  9876543210,
      "rssi":  -47,
      "ch":     6,
      "bw":     0,
      "iq":     "<base64-encoded interleaved int8 I/Q>"
    }
  ]
}
```

`bw` is `0` for HT20 (64 subcarriers) and `1` for HT40 (128 or 256).

The server expects all frames in a batch to come from the same node and to
arrive in monotonically increasing `seq` order; gaps are tolerated (the
server logs but does not error).

## Power & timing notes

- CPU pinned to 240 MHz. CSI ISR runs on the WiFi-affinity core (0); the
  uploader runs on core 1 so JSON encoding never blocks RX.
- Queue overflow drops the oldest frame, not the newest — this favours
  fresh data over completeness, which is the right trade-off for a
  near-real-time sensing pipeline.
- Disconnect drops the in-flight batch; the firmware does *not* cache.
  See ADR notes about not building a phase-incoherent cache.

## Hardware compatibility

| Board | Status |
|---|---|
| ESP32-S3-DevKitC-1 | ✅ primary target |
| ESP32-S3 N16R8 modules | ✅ |
| ESP32-S2 | ❌ no CSI support |
| ESP32-C3 | ❌ single-core, insufficient for our pipeline |
| ESP32-C5 | ⚙️ planned (M6 — true MIMO) |
| ESP32-C6 | ⚙️ planned (M6 — WiFi 6) |
