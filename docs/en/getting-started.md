# Getting started

This guide walks you through the **Entry tier** setup: two ESP32-S3 boards, your existing WiFi router, and a PC running the WaveSight edge server. You will go from zero to a live presence indicator in your dashboard in approximately 30 minutes.

## What you need

### Hardware (~$20 + your existing PC)

- 2 × ESP32-S3 development boards (DevKitC-1 or similar, dual-core, 4 MB+ flash)
- 2 × micro-USB or USB-C cables
- A WiFi router with 2.4 GHz or 5 GHz (any modern AP works)
- A Linux, macOS or Windows machine with at least 4 GB RAM and a free USB port

### Software

- [Rust 1.78+](https://rustup.rs)
- [ESP-IDF 5.2+](https://docs.espressif.com/projects/esp-idf/en/v5.2/esp32s3/get-started/index.html)
- [Node 20+](https://nodejs.org) and [pnpm](https://pnpm.io/installation)
- [Python 3.11+](https://www.python.org) (only for training, optional at this stage)
- `git`

## Step 1 — Clone the repo

```bash
git clone https://github.com/vladimir120307-droid/wavesight.git
cd wavesight
```

## Step 2 — Configure firmware

Open `firmware/esp32-csi-node/main/Kconfig.projbuild` (after `idf.py menuconfig`) and set:

- **WiFi SSID** — your home network
- **WiFi password**
- **Server endpoint** — `ws://<your-pc-ip>:8080/ingest` (we'll start the server in step 4)
- **Node name** — `node-1` on the first board, `node-2` on the second

```bash
cd firmware/esp32-csi-node
idf.py set-target esp32s3
idf.py menuconfig    # configure WiFi + server endpoint
```

## Step 3 — Flash both ESP32 boards

```bash
# First board, plugged into /dev/ttyUSB0 (Linux/macOS) or COM3 (Windows)
idf.py -p /dev/ttyUSB0 flash monitor

# Press Ctrl-] to exit monitor when you see "WiFi connected, sending CSI"

# Repeat for the second board
idf.py -p /dev/ttyUSB1 flash monitor
```

You should see in the log:

```
I (4321) wavesight: WiFi connected to MyHomeWiFi
I (4327) wavesight: CSI capture started, 100 frames/s
I (4399) wavesight: Server connection established
```

## Step 4 — Start the edge server

```bash
cd ../../server
cargo run --release --bin wavesight -- serve --listen 0.0.0.0:8080
```

The server is now waiting for CSI streams. You should see:

```
INFO wavesight: edge server listening on 0.0.0.0:8080
INFO csi_ingest: node-1 connected
INFO csi_ingest: node-2 connected
INFO fusion: 2-node mesh active, time sync RMS ≈ 2 μs
```

## Step 5 — Start the dashboard

In a separate terminal:

```bash
cd dashboard
pnpm install
pnpm dev
```

Open `http://localhost:5173`. After a few seconds you should see:

- Two green node markers on the topology view
- A live CSI spectrogram for each node
- A "Presence" indicator that switches to **detected** when you walk between the boards

## Step 6 — Calibrate

Click **Calibrate** in the dashboard. Stand still in the room for 30 seconds, then leave the room for 30 seconds. WaveSight builds a clutter map of the empty room which dramatically improves presence accuracy.

## Next steps

- [Architecture overview](architecture.md) — understand what just happened.
- [Add a UWB anchor](../hardware/uwb-anchor.md) — upgrade to Standard tier for vital signs.
- [Home Assistant integration](../../integrations/home-assistant/README.md) — wire WaveSight into your smart-home automations.
- [Honest Mode explained](honest-mode.md) — read about the confidence interval in the presence indicator.

## Troubleshooting

| Symptom | Likely cause | Fix |
|---|---|---|
| Node never connects | WiFi creds wrong, or server unreachable | `idf.py monitor` and check logs |
| Presence flickers | Insufficient calibration | Re-run the calibration step with the room fully empty |
| Dashboard shows red node | Connection dropped | Check WiFi signal at the node; consider moving closer to AP |
| CSI rate < 100 fps | AP throttling | Try 5 GHz or a less congested 2.4 GHz channel |

Stuck? Open a [Discussion](https://github.com/vladimir120307-droid/wavesight/discussions) — we love helping.
