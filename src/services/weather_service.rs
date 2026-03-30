use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherResponse {
    pub city: String,
    pub temperature_celsius: f64,
    pub description: String,
    pub humidity_percent: u8,
    pub wind_speed_kmh: f64,
}

#[async_trait]
pub trait WeatherService: Send + Sync + 'static {
    async fn get_weather(&self, city: &str) -> anyhow::Result<WeatherResponse>;
}
