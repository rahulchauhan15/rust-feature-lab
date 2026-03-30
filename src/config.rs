use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub weather_api_key: String,
    pub s3_bucket: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        let _ = dotenvy::dotenv();

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .context("PORT must be a valid u16 (0-65535)")?;

        #[cfg(feature = "mock")]
        let weather_api_key = std::env::var("OPENWEATHER_API_KEY")
            .unwrap_or_else(|_| "mock-placeholder-not-used".to_string());

        #[cfg(not(feature = "mock"))]
        let weather_api_key = std::env::var("OPENWEATHER_API_KEY")
            .context("OPENWEATHER_API_KEY must be set (hint: run with --features mock for local dev)")?;

        #[cfg(feature = "mock")]
        let s3_bucket = std::env::var("S3_BUCKET").unwrap_or_else(|_| "mock-bucket".to_string());

        #[cfg(not(feature = "mock"))]
        let s3_bucket = std::env::var("S3_BUCKET")
            .context("S3_BUCKET must be set (hint: run with --features mock for local dev)")?;

        Ok(Self {
            port,
            weather_api_key,
            s3_bucket,
        })
    }
}
