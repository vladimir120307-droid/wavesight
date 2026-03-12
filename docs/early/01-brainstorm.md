# Project brainstorm — first dump

_Личный документ, не для публикации. Пишу мысли как есть._

## EN

I have been carrying this idea for a while: build something that turns
ordinary WiFi into a sensor for the home. There are academic papers
(CMU's DensePose from WiFi, Person-in-WiFi, several IEEE pieces) and one
or two viral GitHub projects that gloss over the physics. Nobody seems
to be shipping an honest open-source platform that:

- Works on a $20 ESP32 mesh.
- Is privacy-respecting (no cloud).
- Has actual verified demos, not Twitter mock-ups.
- Provides production-shaped verticals (fall detection, sleep, smart-home occupancy).

I think there is a niche here. Even if the through-wall claims of the
hype projects are bogus, the simpler signals (presence in the same room,
breathing while at rest, distinguishing pet from human) are tractable.

Goals for me personally:
- Build something I would actually deploy in my own apartment.
- Ship in the open from day one.
- Bilingual docs (EN + RU) so I can reach both audiences I care about.
- No marketing-grade lies. If a number is not measured, do not publish it.

## RU

Идея давно крутится: сделать сенсор для дома на обычном WiFi. Есть
академические работы (DensePose-from-WiFi от CMU, Person-in-WiFi, ряд
IEEE-статей), и парочка вирусных GitHub-проектов которые игнорируют
физику. Ни одного честного open-source-решения, которое бы:

- Работало на ESP32-меше за $20.
- Уважало приватность (без облака).
- Имело реальные подтверждённые демо, а не twitter-мокапы.
- Содержало production-вертикали (детекция падений, сон, occupancy
  для умного дома).

Ниша есть. Даже если громкие "сквозь стены"-обещания у хайповых проектов
липовые, более простые сигналы (присутствие в одной комнате, дыхание
лежащего человека, отличить кота от человека) — вполне реализуемы.

Личные цели:
- Сделать вещь, которую я сам поставлю себе в квартиру.
- С первого дня в открытом доступе.
- Документация EN + RU — это две аудитории, которые меня интересуют.
- Никаких маркетинговых преувеличений. Если число не измерено — не публиковать.

## Open questions

- Hardware: ESP32-S3 точно. Брать ли сразу ESP32-C5/C6 для MIMO? Дорого, может позже.
- Сервер: Rust однозначно. Tokio + axum.
- ML: Candle на edge, PyTorch для тренинга.
- Dashboard: React + Three.js — но Three.js потом, сначала 2D.
- Лицензия: MIT.

## Next steps

- Перебрать литературу, собрать ссылки.
- Прикинуть бюджет на железо.
- Подумать про название.
