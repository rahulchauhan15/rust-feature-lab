# Cargo Feature Flags: Mock vs Live Services POC 

[![Rust](https://img.shields.io/badge/rust-1.80-blue?logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/axum-0.7-orange?logo=rust)](https://github.com/tokio-rs/axum)
[![License: MIT](https://img.shields.io/badge/license-MIT-green)](LICENSE)

A practical **Proof-of-Concept** demonstrating [Cargo feature flags](https://doc.rust-lang.org/cargo/reference/features.html) for toggling **mock** vs **live** external service integrations in a production-like Axum web server.

**Real-world use case**: Dev/test with static mocks (no keys/API costs), flip to live for prod/CI via `cargo build --features` + runtime env overrides.

##  Features
- **Weather API**: Mock static data vs [OpenWeatherMap](https://openweathermap.org/api) live calls.
- **S3 Storage**: Mock in-memory vs live AWS S3 uploads (via reqwest).
- **Compile-time + Runtime switching**: `#[cfg(feature=\"mock\")]` guards + `MOCK_*=1` env vars.
- **Zero deps bloat**: Mock impls are empty structs—no extra crates.
- **Graceful**: Env overrides work even in live builds.

##  Quickstart (Mock Mode - No Keys Needed)

1. **Clone & Setup**
   ```bash
   git clone <repo>
   cd cargo-feature-mock-poc
   cp .env.example .env   # Optional, no keys for mock
   ```

2. **Run Mock**
   ```bash
   ./scripts/run-mock.sh
   ```
   Or manual:
   ```bash
   MOCK_WEATHER=1 MOCK_S3=1 cargo run --features mock
   ```

3. **Test Endpoints**
   ```bash
   # Weather (mock: always 22.5°C Berlin ☀️)
   curl http://localhost:3000/weather/Berlin

   # S3 Upload (mock response)
   curl -X POST http://localhost:3000/s3/upload/myfile.txt \\
     -H 'Content-Type: text/plain' --data 'Hello S3 Mock!'
   ```

   **Expected Weather**:
   ```json
   {
     \"city\": \"Berlin\",
     \"temperature_celsius\": 22.5,
     \"description\": \"mock: clear sky (no real API call was made)\",
     \"humidity_percent\": 55,
     \"wind_speed_kmh\": 14.4
   }
   ```

##  Live Mode (Real Services)

1. **Set Keys** (edit `.env`)
   ```
   OPENWEATHER_API_KEY=abc123...  # Free @ openweathermap.org
   S3_BUCKET=my-real-bucket
   ```

2. **Run Live**
   ```bash
   ./scripts/run-live.sh
   ```

3. **Test**: Same curls → real data/S3 ETAG.

##  How Switching Works

| Mode | Cargo Command | Env Vars | Builders Do |
|------|---------------|----------|-------------|
| **Dev/Mock** | `cargo run --features mock` | `MOCK_*=1` | cfg! guards → Mock impls |
| **Live/Prod** | `cargo run` | None | Default Live impls |
| **Hybrid** | `cargo run --features mock` | None | Live (ignores feature) |

**Key Insight**: Features enable/disable mock _code_ at compile-time (smaller binaries), env provides runtime override for flexibility (e.g. prod emergencies).

### Architecture (Mermaid)

```mermaid
graph TD
    Main[main.rs] -->|AppConfig| Config
    Main -->|build_*| Services
    Services -->|trait Arc<dyn>| AppState
    Services --> MockWeather['MockWeatherService<br/>(static data)'] 
    Services --> LiveWeather['LiveWeatherService<br/>(reqwest → OWM)']
    Services --> MockS3['MockS3Service']
    Services --> LiveS3['LiveS3Service']
    AppState --> Router[router.rs → handlers]
    Handlers --> Weather['/weather/:city']
    Handlers --> S3['/s3/upload']
```

##  Cargo Features Deep Dive

See `src/services/mod.rs`:
```rust
#[cfg(feature = "mock")] mod mock_weather;  // Compile mock only if enabled

pub fn build_weather_service(...) -> Arc<dyn WeatherService> {
    if cfg!(feature = "mock") && std::env::var_os("MOCK_WEATHER").is_some() {
        return Arc::new(mock_weather::MockWeatherService);  // Double-check
    }
    // Live
}
```
- **Pros**: No dead code in prod builds. `cargo build --release` slims binary.
- **Gotcha**: Recompile on feature change (`cargo clean` if issues).
- **Extend**: Add `live` feature? Unnecessary—default is live.

`Cargo.toml`:
```toml
[features]
mock = []  # No deps needed for mocks
```

##  Development

- **Check**: `cargo check`
- **Fmt/Clippy**: `cargo fmt && cargo clippy`
- **Add Test**: See TODO.md for integration tests idea.
- **Docker?** Easy: Add `Dockerfile` w/ multi-stage + feature arg.

##  Why This Pattern?

- **Battery-included**: Works OOB for dev/prod.
- **No Feature Hell**: One flag + env = simple.
- **Trait Magic**: Dyn dispatches calls without generics overhead.

Questions? Open issue/PR. Learn by hacking handlers/services!

---
*Crafted for Rust devs learning features the practical way.*

