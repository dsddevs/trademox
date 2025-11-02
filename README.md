<h1>
  <img src="./docs/images/icon.png" width="50" align="left" style="margin-right: 10px;">
  TradeMox
</h1>

<br clear="left"/>

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache2.0-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

High-performance, developer-friendly WebSocket API for streaming market data built with Rust, Axum, and Tokio.

- Real-time streaming over WebSocket
- Strict input validation (ticker/date)
- JSON error responses
- CORS configuration via `config.yaml` or environment
- Ready for CI with GitHub Actions

---

### <img src="./docs/images/icon.png" width="24" align="top"> Quick Start

### <img src="./docs/images/icon.png" width="24" align="top"> Prerequisites
- Rust toolchain installed (`rustup`)
- A WebSocket client (e.g., `websocat`)

### <img src="./docs/images/icon.png" width="24" align="top"> Run the server (Two terminals)

1️⃣ **Terminal:**

- **On Linux/Max:**
```bash
cargo build --release
RUST_LOG=info cargo run --release
```

- **On Windows (PowerShell):**
```powershell
cargo build --release
$env:RUST_LOG = "info"; cargo run --release
```

2️⃣ **Terminal:**
```bash
# Connect with websocat
websocat "ws://127.0.0.1:3013/ws?ticker=BTC-USD&start=2025-01-01&end=2025-01-10"

# Stocks
websocat "ws://127.0.0.1:3013/ws?ticker=AAPL&start=2025-01-01&end=2025-01-10"

# Forex
websocat "ws://127.0.0.1:3013/ws?ticker=EUR-USD&start=2025-01-01&end=2025-01-10"
```

- **Health check:**
```bash
curl -i http://127.0.0.1:3013/healthz
```

---

### <img src="./docs/images/icon.png" width="24" align="top"> API

#### 1️⃣```WebSocket Endpoint```

**Path:** `/ws`  
**Method:** `GET`  
**Query parameters:**
- `ticker` (string, required; e.g. BTC-USD, AAPL)
- `start` (string, required; format YYYY-MM-DD)
- `end` (string, required; format YYYY-MM-DD)

On successful upgrade, the server streams data messages as JSON text frames.

#### 2️⃣```Validation Rules```

**Ticker:**
- Not empty
- Max length 10
- Allowed characters: A-Z and `-` (e.g., BTC-USD, MSFT)

**Dates:**
- Format must be `YYYY-MM-DD` (validated with chrono::NaiveDate)
- If a parameter is missing in the query, the server may fall back to environment defaults (see "Configuration")

Invalid values produce a JSON error response with HTTP 400.

---

### <img src="./docs/images/icon.png" width="24" align="top"> Error Responses (JSON)

- **Format:**
```json
{
  "error": "InvalidTicker | InvalidDate | InvalidQuery",
  "message": "Human readable error message",
  "field": "ticker | start | end (optional)",
  "details": "Optional details"
}
```

- **Invalid ticker:**
```json
{
  "error": "InvalidTicker",
  "message": "Invalid ticker format",
  "field": "ticker"
}
```

- **Invalid date:**
```json
{
  "error": "InvalidDate",
  "message": "Invalid date format",
  "field": "start"
}
```

- **Invalid query shape (parsing failed):**
```json
{
  "error": "InvalidQuery",
  "message": "Invalid query parameters format",
  "details": "..."
}
```

---

### <img src="./docs/images/icon.png" width="24" align="top"> Switching Stream During Session (optional)

You can send a text message with a JSON payload to switch stream parameters at runtime:

```json
{"ticker":"BTC-USD","start":"2025-01-01","end":"2025-01-10"}
```

---

### <img src="./docs/images/icon.png" width="24" align="top"> Configuration

1️⃣ **Environment variables:**
- `DEFAULT_TICKER` — default ticker if query param omitted (e.g., XAUUSD)
- `DEFAULT_START` — default start date (format YYYY-MM-DD)
- `DEFAULT_END` — default end date (format YYYY-MM-DD)
- `DEFAULT_ORIGIN` — fallback allowed origin for CORS

2️⃣ **CORS via config.yaml:**
- If present, allowed origins are loaded from `config.yaml`
- Fallback to `DEFAULT_ORIGIN` if config.yaml is missing or invalid 
- config.yaml (CORS section):
```yaml
cors:
  origins:
    - "http://localhost:3013"
    - "http://127.0.0.1:3013"
```

---

### <img src="./docs/images/icon.png" width="24" align="top"> Schema

<img width="1189" height="765" alt="Image" src="https://github.com/user-attachments/assets/d0f7bf74-3a38-4381-b2ec-85b1fcc236a1" />

---

### <img src="./docs/images/icon.png" width="24" align="top"> Development

- **Run tests:**
```bash
cargo test -- --nocapture
```

- **Run a specific test file:**
```bash
cargo test --test ws_upgrade_test -- --nocapture
```

- **Use debug logs:**
```bash
RUST_LOG=debug cargo run
```

- **On Windows (PowerShell):**
```powershell
$env:RUST_LOG = "debug"; cargo run
```

---

### <img src="./docs/images/icon.png" width="24" align="top"> CI (GitHub Actions)

- Create `.github/workflows/ci.yaml`:

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

- **Optional (multi-OS matrix):**
```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
runs-on: ${{ matrix.os }}
```

---

### <img src="./docs/images/icon.png" width="24" align="top"> Commercial Highlights

- Reliable streaming with strict parameter validation
- JSON error contracts for easy client integration
- Configurable CORS and environment-driven defaults
- Built on Rust for safety and performance
- Clean separation of concerns (upgrade, validation, messaging, streaming)

For enterprise inquiries, integration support, or custom feeds, please contact your solutions representative.

---

### <img src="./docs/images/icon.png" width="24" align="top"> Commercial Licensing

This software is available under flexible licensing options:

- **Apache-2 License**: Free for open-source and personal projects
- **Commercial License**: Available for enterprise deployments
- **Support & Consulting**: Professional services available

### <img src="./docs/images/icon.png" width="24" align="top"> Contact

- 📧 dsddevs@gmail.com
- <img src="./docs/images/telegram.png" width="24" valign="middle"> @dsddevs / +998906006989


© 2025 TradeMox | Лицензия Apache 2.0