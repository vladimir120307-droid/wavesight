# WaveSight

Open-source platform for WiFi-based sensing. Work in progress.

The goal is to turn ordinary WiFi (and later UWB) radio signals into
useful spatial signals: presence, motion, breathing, fall detection,
sleep staging. Local-first, no cloud.

More to come.
## Sketch

Target hardware:

- 2x ESP32-S3 nodes minimum (sniff CSI from the home WiFi router).
- Later: ESP32-C5/C6 for true MIMO, DWM3000 UWB for angle-of-arrival.

Target verticals:

- Presence + occupancy for smart-home automations.
- Fall detection for elderly people.
- Sleep monitoring without a wearable.