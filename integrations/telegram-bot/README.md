# WaveSight Telegram bot

Pushes fall-detection alerts to family members via Telegram. Designed
for the EldGuard vertical — see `examples/eldguard/`.

## Why a separate bot

The dashboard is local-first by default (ADR-006). A Telegram push is an
explicit user opt-in: it sends a single event ("fall detected at
node-X, uncertainty 0.18") to one or more chat IDs. No video, no
biometrics, no metadata beyond the event itself leaves the network.

## Setup

1. Create a Telegram bot via `@BotFather`, save the token.
2. Add the bot to a private group with the family members who should
   receive alerts.
3. Get the group's chat ID (`@userinfobot` works).
4. Run:

```bash
cargo run -p telegram-bot -- \
    --server http://localhost:8081 \
    --token "$TELEGRAM_TOKEN" \
    --chat-id "$GROUP_CHAT_ID" \
    --alert-types fall
```

## Privacy

- The bot does **not** stream live presence — only on-event alerts.
- Outbound traffic is exactly one HTTPS request per alert.
- Set `--throttle-minutes 10` to deduplicate aftershock notifications.
