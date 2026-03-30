use anyhow::Result;
use async_trait::async_trait;

use super::weather_service::{WeatherResponse, WeatherService};


const MOCK_TEMPERATURE_CELSIUS: f64 = 22.5;
const MOCK_HUMIDITY_PERCENT:    u8  = 55;
const MOCK_WIND_SPEED_KMH:      f64 = 14.4;
const MOCK_DESCRIPTION: &str = "mock: clear sky (no real API call was made)";

pub struct MockWeatherService;

#[async_trait]
impl WeatherService for MockWeatherService {
    async fn get_weather(&self, city: &str) -> Result<WeatherResponse> {
        tracing::warn!(
            city,
            temperature = MOCK_TEMPERATURE_CELSIUS,
            "MockWeatherService: returning static fixture — no real API call made"
        );

        Ok(WeatherResponse {
            city:                city.to_string(),
            temperature_celsius: MOCK_TEMPERATURE_CELSIUS,
            description:         MOCK_DESCRIPTION.to_string(),
            humidity_percent:    MOCK_HUMIDITY_PERCENT,
            wind_speed_kmh:      MOCK_WIND_SPEED_KMH,
        })
    }
}

