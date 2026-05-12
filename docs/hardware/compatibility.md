# Hardware compatibility

Tested boards, modules and routers. PRs welcome — see
[issue template](../../.github/ISSUE_TEMPLATE/hardware_report.yml).

## ESP32 family — CSI nodes

| Board / module | CSI capture | MIMO | Notes |
|---|---|---|---|
| ESP32-S3-DevKitC-1 | ✅ tested | ❌ 1×1 | Primary target. 240 MHz, dual-core, 8 MB flash recommended. |
| ESP32-S3 N16R8 module | ✅ tested | ❌ 1×1 | 16 MB flash gives more OTA headroom. |
| ESP32-C5-DevKitC | ⚙️ planned | ✅ 2×2 | WiFi 6, real MIMO. Adds true spatial diversity. M6 target. |
| ESP32-C6 | ⚙️ planned | ✅ 2×2 | WiFi 6, alternative to C5. Lower clock — may need DSP offload to server. |
| ESP32 (original) | ❌ unsupported | – | Single-core, insufficient for our DSP path. |
| ESP32-C3 | ❌ unsupported | – | Single-core. RuView ships these, we do not. |
| ESP32-S2 | ❌ unsupported | – | No CSI capture in IDF. |

## UWB anchors (M6+)

| Module | TWR | AoA | Notes |
|---|---|---|---|
| Qorvo DWM3000 | ✅ planned | ✅ planned | Reference module. AoA requires multi-antenna evaluation board. |
| Qorvo DWM1000 | ⚙️ partial | ❌ | Older silicon, ranging only. Useful for early prototypes. |

## Reference NICs (research tier)

| NIC | CSI source | Notes |
|---|---|---|
| Intel AX210 | csi-tool fork | Linux x86 only. Sub-microsecond timestamping. |
| Nordic nRF7002 | nRF Connect SDK | Pi 5 + module via QSPI. |
| Raspberry Pi (Broadcom) | nexmon_csi | Patched firmware. Works on Pi 3B+ and Pi 4. |

## Edge-server hosts

| Host | Status |
|---|---|
| Raspberry Pi 5 (8 GB) | ✅ recommended |
| Raspberry Pi 4 (4 GB+) | ✅ works, slower DSP throughput |
| Intel NUC | ✅ |
| Apple Silicon Mac | ✅ |
| Generic x86-64 Linux | ✅ |
| Windows 11 | ✅ via WSL2 |
| Android (Termux) | ❌ untested |

## Routers / APs

The CSI side observes traffic on whatever AP the ESP32 nodes are
associated with. We are AP-agnostic. Verified working APs:

- ASUS RT-AX86U (WiFi 6)
- TP-Link Archer C7 (WiFi 5)
- Mikrotik hAP ac²
- OpenWrt-flashed Xiaomi AX3600 (with nexmon_csi)

If you tested on hardware not listed here, please open an issue with
the [hardware report template](../../.github/ISSUE_TEMPLATE/hardware_report.yml).
