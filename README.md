# Silf

A Telegram bot written in Rust that acts as a personal secretary using the **Telegram Business Connections API**. It monitors incoming messages on a connected business account, automatically responds to greetings and portfolio inquiries, tracks deleted messages, and lets the owner post stories via chat commands.

## Technologies

| | |
|---|---|
| **Rust** (edition 2024) | Language |
| **teloxide** | Async Telegram Bot framework |
| **tokio** | Async runtime |
| **axum** | Webhook HTTP server (via teloxide integration) |
| **dashmap** | Concurrent in-memory session store |
| **chrono** | Date/time handling |
| **regex** | Keyword detection for greetings and portfolio requests |
| **reqwest** | HTTP client for `postStory` multipart upload |

## Prerequisites

- Rust 1.90+ (stable)
- OpenSSL dev libraries (`libssl-dev` on Debian/Ubuntu)
- A Telegram Bot Token from [@BotFather](https://t.me/BotFather)
- A Telegram Business account with the bot set as secretary (for business connection features)

## Setup

```bash
git clone <repo-url>
cd silf
cp .env.example .env
```

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `BOT_TOKEN` | ✅ | — | Telegram bot token |
| `DEBUG` | No | `false` | Set `true` to use long-polling instead of webhook |
| `WEBHOOK_URL` | In production | — | Public HTTPS URL for webhook (e.g. `https://example.com/`) |
| `PORT` | No | `8080` | HTTP port for the webhook listener |
| `ADMIN` | No | hardcoded fallback | Telegram user ID of the bot owner |
| `RUST_LOG` | No | — | Log level (e.g. `info`, `debug`) |

## Running

**Development (polling):**
```bash
DEBUG=true BOT_TOKEN=your_token cargo run
```

**Production (webhook):**
```bash
BOT_TOKEN=your_token WEBHOOK_URL=https://your.domain/ PORT=8080 ./target/release/silf
```

**Docker:**
```bash
docker build -t silf .
docker run --env-file .env silf
```

## Tests

No automated test suite. Validate by running in polling mode with a test bot token (`DEBUG=true`).

## Project Structure

```
src/
├── main.rs                    # Entry point — loads config, starts dispatcher
├── app.rs                     # AppState: in-memory sessions + greeting tracker
├── config/settings.rs         # Configuration loaded from environment
├── routes/set.rs              # Route registration for the dispatcher
├── handlers/
│   ├── start.rs               # /start command
│   ├── messages.rs            # General message dispatcher (story state machine)
│   ├── business/
│   │   ├── connection.rs      # 🔒 Business connection lifecycle (protected)
│   │   ├── start.rs           # Incoming business message handler
│   │   ├── deleted.rs         # Deleted message notification to admin
│   │   └── info.rs            # Portfolio auto-reply
│   └── story/
│       ├── idle.rs            # Story flow entry trigger
│       └── story.rs           # Multi-step story upload state machine
├── services/
│   ├── action.rs              # Chat action (typing indicator) helper
│   └── story.rs               # Telegram postStory multipart API call
├── states/
│   ├── manager.rs             # In-memory session + business message store
│   ├── session.rs             # Per-user session (state + business connection ID)
│   ├── state.rs               # User state enum
│   ├── story.rs               # Story draft state
│   └── business.rs            # Business message snapshot (for deletion tracking)
└── utils/
    ├── run.rs                 # Polling / webhook startup
    ├── message.rs             # Streaming word-by-word text edit helper
    └── greeting.rs            # Greeting + portfolio keyword detection (regex)
```

## Business Connection ID Integration

When a Telegram Business account connects this bot as a secretary:

1. Telegram sends a `BusinessConnection` update containing a unique `business_connection_id`.
2. The bot stores this ID in the user's in-memory session (`handlers/business/connection.rs`).
3. Subsequent messages from the business account's chats include the same ID, enabling the bot to reply on the owner's behalf by attaching `business_connection_id` to outgoing messages.
4. When a message is deleted in a business chat, the bot looks up its stored snapshot and notifies the admin.
5. If the connection is revoked, the session is cleared and all cached messages for that connection are removed.

The entire Business Connection ID subsystem (`connection.rs`, session `business_connection_id` field, `StateManager` business message store) is treated as protected infrastructure and must not be altered without careful review.
