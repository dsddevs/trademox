# 📊 TradeMox — WebSocket Trading API

High-performance, developer-friendly WebSocket API for streaming market data built with Rust, Axum, and Tokio.

- Real-time streaming over WebSocket
- Strict input validation (ticker/date)
- JSON error responses
- CORS configuration via `config.yaml` or environment
- Ready for CI with GitHub Actions

---

## 💼 Quick Start

### Prerequisites
- Rust toolchain installed (`rustup`)
- A WebSocket client (e.g., `websocat`)

### Run the server (Two terminals)

**Terminal 1:**
```bash
# Build and run (Linux/Mac)
cargo build --release
RUST_LOG=info cargo run --release
```

**On Windows (PowerShell):**
```powershell
cargo build --release
$env:RUST_LOG = "info"; cargo run --release
```

**Terminal 2:**
```bash
# Connect with websocat
websocat "ws://127.0.0.1:3013/ws?ticker=BTC-USD&start=2025-01-01&end=2025-01-10"
```

**Examples:**
```bash
# Stocks
websocat "ws://127.0.0.1:3013/ws?ticker=AAPL&start=2025-01-01&end=2025-01-10"

# Forex
websocat "ws://127.0.0.1:3013/ws?ticker=EUR-USD&start=2025-01-01&end=2025-01-10"
```

**Health check:**
```bash
curl -i http://127.0.0.1:3013/healthz
```

---

## 💼 API

### ```WebSocket Endpoint```

**Path:** `/ws`  
**Method:** `GET`  
**Query parameters:**
- `ticker` (string, required; e.g. BTC-USD, AAPL)
- `start` (string, required; format YYYY-MM-DD)
- `end` (string, required; format YYYY-MM-DD)

On successful upgrade, the server streams data messages as JSON text frames.

### ```Validation Rules```

**Ticker:**
- Not empty
- Max length 10
- Allowed characters: A-Z and `-` (e.g., BTC-USD, MSFT)

**Dates:**
- Format must be `YYYY-MM-DD` (validated with chrono::NaiveDate)
- If a parameter is missing in the query, the server may fall back to environment defaults (see "Configuration")

Invalid values produce a JSON error response with HTTP 400.

---

## 💼 Error Responses (JSON)

**Format:**
```json
{
  "error": "InvalidTicker | InvalidDate | InvalidQuery",
  "message": "Human readable error message",
  "field": "ticker | start | end (optional)",
  "details": "Optional details"
}
```

**Examples:**

**Invalid ticker:**
```json
{
  "error": "InvalidTicker",
  "message": "Invalid ticker format",
  "field": "ticker"
}
```

**Invalid date:**
```json
{
  "error": "InvalidDate",
  "message": "Invalid date format",
  "field": "start"
}
```

**Invalid query shape (parsing failed):**
```json
{
  "error": "InvalidQuery",
  "message": "Invalid query parameters format",
  "details": "..."
}
```

---

## 💼 Switching Stream During Session (optional)

You can send a text message with a JSON payload to switch stream parameters at runtime:

```json
{"ticker":"BTC-USD","start":"2025-01-01","end":"2025-01-10"}
```

---

## 💼 Configuration

**Environment variables:**
- `DEFAULT_TICKER` — default ticker if query param omitted (e.g., XAUUSD)
- `DEFAULT_START` — default start date (format YYYY-MM-DD)
- `DEFAULT_END` — default end date (format YYYY-MM-DD)
- `DEFAULT_ORIGIN` — fallback allowed origin for CORS

**CORS via config.yaml:**
- If present, allowed origins are loaded from `config.yaml`
- Fallback to `DEFAULT_ORIGIN` if config.yaml is missing or invalid

**Example config.yaml (CORS section):**
```yaml
cors:
  origins:
    - "http://localhost:3013"
    - "http://127.0.0.1:3013"
```

---

## 💼 SCHEMA

<img width="1189" height="765" alt="Image" src="https://github.com/user-attachments/assets/8f7e6d48-89ee-47b5-87a1-9f506acf993b" />

---

## 💼 Development

**Run tests:**
```bash
cargo test -- --nocapture
```

**Run a specific test file:**
```bash
cargo test --test ws_upgrade_test -- --nocapture
```

**Use debug logs:**
```bash
RUST_LOG=debug cargo run
```

**On Windows (PowerShell):**
```powershell
$env:RUST_LOG = "debug"; cargo run
```

---

## 💼 CI (GitHub Actions)

Create `.github/workflows/ci.yaml`:

```yaml
name: CI

on: [push, pull_request]

env:
  CARGO_TERM_COLOR: always

jobs:
  build_test:
    name: Build and Test
    strategy:
      matrix:
        os: [ ubuntu-latest, macos-latest, windows-latest ]
        target: [x86_64, aarch64]
    runs-on: ${{ matrix.os }}


    steps:
      - uses: actions/checkout@v4
      # Install Rust
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: clippy, rustfmt
          override: true

      # Cache dependencies
      - uses: Swatinem/rust-cache@v2

      # Check formatting
      - name: Check formatting
        run: cargo fmt --all -- --check

      # Linter
      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

      # Project builder
      - name: Build
        run: cargo build --release

      # Testing
      - name: Run tests
        run: cargo test --all --all-features --

      # Coverage generation
      - name: Generate coverage
        run: cargo tarpaulin --out Xml --output-dir target/coverage

      # Coverage uploader
      - name: Upload coverage
        uses: codecov/codecov-action@v3

      # Documentation checker
      - name: Check docs
        run: cargo doc --no-deps --all-features
```

**Optional (multi-OS matrix):**
```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
runs-on: ${{ matrix.os }}
```

---

## 💼 Commercial Highlights

- Reliable streaming with strict parameter validation
- JSON error contracts for easy client integration
- Configurable CORS and environment-driven defaults
- Built on Rust for safety and performance
- Clean separation of concerns (upgrade, validation, messaging, streaming)

For enterprise inquiries, integration support, or custom feeds, please contact your solutions representative.

---

## 💼 Commercial Licensing

This software is available under flexible licensing options:

- **Apache-2 License**: Free for open-source and personal projects
- **Commercial License**: Available for enterprise deployments
- **Support & Consulting**: Professional services available

## 📫 Contact
- 📧 Email:    dsddevs@gmail.com
- 📞 Telegram: @dsddevs / +998906006989

© 2025 TradeMox | Лицензия Apache 2.0