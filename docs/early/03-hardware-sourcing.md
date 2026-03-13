# Hardware sourcing — first cut

## EN

What I want to start with:

| Item | Qty | Where | Price | Notes |
|---|---|---|---|---|
| ESP32-S3-DevKitC-1 | 4 | AliExpress / Mouser | ~$10/ea | Primary CSI nodes |
| Micro-USB cables | 4 | AliExpress | $1/ea | |
| USB hub with per-port switches | 1 | Local | $20 | For flashing in parallel |
| Logitech RGB-D camera (used) | 1 | Avito | ~$60 | Ground-truth for eval |
| Generic WiFi 6 router | 1 | _have one already_ | – | AP for testing |
| DWM3000 UWB module | 2 | Mouser | $25/ea | Later — for M6 |

Rough total for Entry+Standard tier kit: ~$160.

Decisions:
- Skip ESP32-C5 for now. The chips are still scarce and the cost is
  3x. Wait until next year.
- Skip the Cognitum-style appliance entirely. My old Raspberry Pi 4 will
  do for the edge server until I get a Pi 5.

## RU

С чего стартую:

| Что | Сколько | Где | Цена | Заметка |
|---|---|---|---|---|
| ESP32-S3-DevKitC-1 | 4 | Aliexpress / Mouser | ~$10/шт | Основные CSI-ноды |
| Micro-USB кабели | 4 | Aliexpress | $1/шт | |
| USB-хаб с выключателями | 1 | Локально | $20 | Прошивать параллельно |
| RGB-D камера б/у | 1 | Авито | ~$60 | Ground-truth для eval |
| WiFi 6 роутер | 1 | _уже есть_ | – | AP для тестов |
| Модуль DWM3000 UWB | 2 | Mouser | $25/шт | Позже — для M6 |

Стартовый комплект Entry+Standard: ~$160.

Решения:
- ESP32-C5 пока пропускаю. Чипы дефицит, цена x3. Подожду полгода.
- Никакого фирменного appliance не покупаю. Pi 4 у меня лежит — пока
  сойдёт за edge-сервер. Когда выйдет Pi 5 — обновлю.

## Risks

- Параллельная прошивка через единый USB-хаб может ловить ground-loop.
  Если будет — буду шить по одной.
- AliExpress: ждать 2-3 недели. Лучше заказать прям сейчас и пока
  работать на одной плате от Mouser (быстрая доставка).
