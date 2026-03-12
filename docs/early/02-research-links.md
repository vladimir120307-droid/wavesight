# Research links — first pass

_Свалка ссылок, потом отсортирую._

## Academic / EN

- **DensePose from WiFi (CMU, Geng et al., 2023)** — the seminal paper.
  Two RX antennas, end-to-end UV mapping from CSI to 24-part body model.
  https://arxiv.org/abs/2301.00250
- **Person-in-WiFi (Wang et al., 2019)** — earlier pose attempt. Uses
  single antenna, much lower accuracy. Useful as a baseline.
- **WiFall (Wang et al., 2017)** — early fall detection paper. The
  feature engineering is dated but the protocol is reusable.
- **mmFall / mmWave radar surveys** — relevant for the M6 deferred work.
- **Nexmon CSI extractor (TU Darmstadt)** — extracts CSI on Broadcom
  chips in Raspberry Pi. Worth supporting as a "research-tier" input.

## ESP-IDF references

- ESP-IDF v5.2 wifi_csi.h API — `esp_wifi_set_csi_rx_cb`, `wifi_csi_info_t`.
- Espressif ESP-CSI-Tool — official tool for capture, code worth reading.
- esp_websocket_client managed component — handles reconnect cleanly.

## Existing projects to study

- **ruvnet/RuView** — viral 2026 project, 55k+ stars. Lots of marketing,
  questionable physics (single antenna ESP32-S3 mesh sold as MIMO).
  Detailed teardown in [05-ruview-teardown.md](05-ruview-teardown.md).
- Various academic prototypes in `wifi-sensing` GitHub topic.

## Russian-language sources

- Хабр: пара статей про CSI-сниффинг на ESP32 (старые, 2019-2020).
- VK / Telegram: канал об эмбеддед-RF — там обсуждают практическое.

## Industry context

- HFR Mercury / Cognitive Systems — commercial WiFi sensing companies.
  Closed source. Lots of patents.
- Amazon, Google — реально внедряют sensing в роутерах своих экосистем.
  Privacy-wise it's a disaster.

## Open datasets

- Не нашёл публично-доступного приличного CSI-датасета с pose ground
  truth. Это пробел который сам хочу закрыть.
