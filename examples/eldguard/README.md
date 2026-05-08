# EldGuard

> Fall detection for older adults living alone. Uses the WaveSight
> WiFi-sensing platform. Mobile-app push alerts to family. No cameras
> in the home.

## EN

### Why

Roughly one in three adults over 65 experiences a fall each year. The
existing solutions are either invasive (wearable pendants — refused by
many seniors because of stigma) or privacy-poor (cameras everywhere).
RF-based detection is unobtrusive, works in the dark, and the sensor
doesn't go on the body.

### How it works

WaveSight's `fall` inference head is specialised for this vertical: a
narrow band of motion patterns (sudden vertical descent followed by
prolonged absence of motion) triggers a fall event. The event is
pushed to family members via the Telegram bot or the mobile app.

### Setup

1. Deploy 1–2 ESP32-S3 nodes in the rooms where falls are most likely
   (bathroom, bedroom, hallway).
2. Run the WaveSight edge server on a Raspberry Pi 5 inside the home.
3. Pair the mobile app with the server (Settings → Pair).
4. Add family members' Telegram IDs in `examples/eldguard/config.toml`.

### What EldGuard is NOT

- Not a substitute for an emergency response system. We notify family;
  family decides whether to call 911.
- Not certified medical equipment. We will pursue CE / FDA certification
  when accuracy numbers justify it; until then we ship as an early
  intervention helper, not a guaranteed alarm.

## RU

### Зачем

Примерно каждый третий взрослый старше 65 лет падает раз в год.
Существующие решения либо инвазивны (носимые медальоны — пожилые часто
отказываются от них из-за стигмы), либо приватность плохая (камеры).
RF-детекция незаметна, работает в темноте, ничего на тело надевать
не надо.

### Как это работает

`fall`-голова WaveSight специализирована под эту вертикаль: узкая
полоса паттернов движения (резкое вертикальное падение с последующим
длительным отсутствием движения) триггерит событие. Событие шлётся
родственникам через Telegram-бота или мобильное приложение.

### Установка

1. Разместить 1-2 ноды ESP32-S3 в комнатах с высоким риском (ванная,
   спальня, коридор).
2. Запустить WaveSight edge-сервер на Raspberry Pi 5 внутри дома.
3. Запарить мобильное приложение с сервером (Настройки → Pair).
4. Добавить Telegram ID родственников в `examples/eldguard/config.toml`.

### Чем EldGuard НЕ является

- Не заменяет систему экстренного реагирования. Мы извещаем семью;
  семья решает, вызывать ли 112.
- Не сертифицированное медицинское оборудование. CE / FDA-сертификацию
  планируем когда точность позволит; пока — раннее предупреждение, не
  гарантированный аларм.

## Configuration

See [`config.toml`](config.toml) for tunable parameters (room
assignments, severity thresholds, family chat IDs).
