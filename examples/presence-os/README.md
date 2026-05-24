# PresenceOS

> Per-room occupancy for Home Assistant / Matter. Energy saving as the
> headline benefit.

## EN

### What it does

PresenceOS turns the WaveSight platform into a high-resolution
occupancy sensor for every room in the house. Unlike PIR sensors, it
detects a person sitting still (e.g. reading) and distinguishes person
from pet by motion signature. Home Assistant binary-sensor entities
appear automatically via MQTT discovery.

### Why this matters

A typical Home Assistant heating automation triggered by PIR fails when
the occupant sits down — heat shuts off because "the room is empty".
PresenceOS does not have this failure mode.

### Setup

1. Deploy 2–6 ESP32-S3 nodes (one per room, plus one in a central
   hallway).
2. Run WaveSight edge server.
3. Install the Home Assistant integration (see
   `integrations/home-assistant/`).
4. Use the new `binary_sensor.<room>_presence` entities in your
   automations.

### Recommended automations

- Light control: `binary_sensor.<room>_presence` → light on/off.
- Heating: `binary_sensor.<room>_presence` + temperature schedule.
- Security: alert when nobody home but motion detected.

## RU

### Что делает

PresenceOS превращает платформу WaveSight в высокоразрешающий датчик
присутствия для каждой комнаты. В отличие от PIR-датчиков, видит
неподвижно сидящего человека (например, читающего) и отличает
человека от животного по паттерну движения. Сущности
`binary_sensor` в Home Assistant появляются автоматически через
MQTT discovery.

### Зачем это нужно

Типичная автоматизация отопления в HA по PIR ломается когда человек
сел — отопление выключается "комната пустая". У PresenceOS такой
проблемы нет.

### Установка

1. Разместить 2-6 ESP32-S3 нод (по одной в комнату + одна в
   центральном коридоре).
2. Запустить WaveSight edge-сервер.
3. Поставить интеграцию HA (см. `integrations/home-assistant/`).
4. Использовать новые `binary_sensor.<room>_presence` в автоматизациях.

### Рекомендованные автоматизации

- Свет: `binary_sensor.<room>_presence` → on/off.
- Отопление: `binary_sensor.<room>_presence` + расписание температур.
- Безопасность: alert когда никого дома, но детект движения.
