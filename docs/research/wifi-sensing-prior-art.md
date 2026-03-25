# Prior art notes (private, will be cleaned up before publishing)

- CMU "DensePose from WiFi" (2023) — seminal paper, end-to-end CSI to keypoints.
- Person-in-WiFi (Wang, 2019) — earlier attempt, single antenna.
- nexmon_csi (TU Darmstadt) — extracts CSI from Broadcom chips on Pi.
- ESP-CSI-Tool (Espressif) — official low-level CSI capture for ESP32.
- RuView (ruvnet, 2026) — viral GitHub project, 55k+ stars, ESP32-S3 mesh.
  Technically contested: single-antenna chip, no public ground-truth demo,
  pose PCK reported around 2.5%. Marketing > physics.

Differentiation ideas:
- Honest metrics, confidence intervals everywhere.
- Multi-modal fusion (WiFi + UWB + BLE).
- Vertical apps (fall / sleep / smart-home) instead of "research toolkit".
- Bilingual EN/RU docs.